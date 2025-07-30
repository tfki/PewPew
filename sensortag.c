
#include <stdlib.h>
#include <unistd.h>

#include <ti/drivers/rf/RF.h>
#include <ti/drivers/PIN.h>
#include <ti/drivers/pin/PINCC26XX.h>

#include DeviceFamily_constructPath(driverlib/rf_prop_mailbox.h)

#include "Board.h"
#include "smartrf_settings/smartrf_settings.h"

#include <stdio.h>
#include <string.h>

#include <xdc/std.h>
#include <xdc/runtime/System.h>

#include <ti/sysbios/BIOS.h>
#include <ti/sysbios/knl/Task.h>
#include <ti/sysbios/knl/Semaphore.h>
#include <ti/sysbios/knl/Event.h>
#include <ti/sysbios/knl/Clock.h>

#include <ti/drivers/PIN.h>
#include <ti/display/Display.h>
#include <ti/display/DisplayExt.h>
#include <ti/drivers/UART.h>
#include <ti/drivers/timer/GPTimerCC26XX.h>
#include <ti/drivers/GPIO.h>

#include "sensors/SensorI2C.h"
#include "sensors/SensorOpt3001.h"
#include "sensors/SensorBmp280.h"
#include "sensors/SensorHdc1000.h"
#include "sensors/SensorMpu9250.h"

#include <ti/devices/DeviceFamily.h>
#include DeviceFamily_constructPath(driverlib/cpu.h)
#include DeviceFamily_constructPath(driverlib/sys_ctrl.h)

static uint16_t latestAdcValue;
static float latestGyroValue[3];
static float latestAccValue[3];
static float latestTemp;
static float latestHum;
static uint32_t latestPress;

static uint16_t my_id = 0;

static RF_Object rfObject;
static RF_Handle rfHandle;
static int time_counter = 0;
static bool button_pressed = false;

GPTimerCC26XX_Handle hTimer;
void timerCallback(GPTimerCC26XX_Handle handle, GPTimerCC26XX_IntMask interruptMask) {
    // interrupt callback code goes here. Minimize processing in interrupt.
    time_counter++;
}

void rf_send(uint8_t* data, size_t length) {
    RF_cmdPropTx.pktLen = length;
    RF_cmdPropTx.pPkt = data;
    RF_cmdPropTx.startTrigger.triggerType = TRIG_NOW;

    /* rf_send packet */
    RF_EventMask terminationReason = RF_runCmd(rfHandle, (RF_Op*)&RF_cmdPropTx,
                                               RF_PriorityNormal, NULL, 0);
    switch(terminationReason)
    {
        case RF_EventLastCmdDone:
            // A stand-alone radio operation command or the last radio
            // operation command in a chain finished.
            break;
        case RF_EventCmdCancelled:
            // Command cancelled before it was started; it can be caused
        // by RF_cancelCmd() or RF_flushCmd().
            break;
        case RF_EventCmdAborted:
            // Abrupt command termination caused by RF_cancelCmd() or
            // RF_flushCmd().
            break;
        case RF_EventCmdStopped:
            // Graceful command termination caused by RF_cancelCmd() or
            // RF_flushCmd().
            break;
        default:
            // Uncaught error event
            break;
    }
    uint32_t cmdStatus = ((volatile RF_Op*)&RF_cmdPropTx)->status;
    switch(cmdStatus)
    {
        case PROP_DONE_OK:
            // Packet transmitted successfully
            break;
        case PROP_DONE_STOPPED:
            // received CMD_STOP while transmitting packet and finished
            // transmitting packet
            break;
        case PROP_DONE_ABORT:
            // Received CMD_ABORT while transmitting packet
            break;
        case PROP_ERROR_PAR:
            // Observed illegal parameter
            break;
        case PROP_ERROR_NO_SETUP:
            // Command sent without setting up the radio in a supported
            // mode using CMD_PROP_RADIO_SETUP or CMD_RADIO_SETUP
            break;
        case PROP_ERROR_NO_FS:
            // Command sent without the synthesizer being programmed
            break;
        case PROP_ERROR_TXUNF:
            // TX underflow observed during operation
            break;
        default:
            // Uncaught error event - these could come from the
            // pool of states defined in rf_mailbox.h
            break;
    }
}

static void rf_send_button_pressed_message() {
    const size_t length = 8;
    uint8_t buffer[length];
    uint8_t id = 2;

    memcpy(&buffer[0], &my_id, 2);
    memcpy(&buffer[2], &time_counter, 4);
    memcpy(&buffer[6], &id, 1); // 321 = button pressed packet

    for (int i = 0; i < length - 1; i++) {
        if (buffer[i] == 255) {
            buffer[i] = 254;
        }
    }

    buffer[length - 1] = 255;
    rf_send(buffer, length);
}

static void rf_send_brightness_message(uint16_t brightness) {
    const size_t length = 10;
    uint8_t buffer[length];
    uint8_t id = 1;

    memcpy(&buffer[0], &my_id, 2);
    memcpy(&buffer[2], &time_counter, 4);
    memcpy(&buffer[6], &id, 1); // 123 = brightness packet
    memcpy(&buffer[7], &brightness, 2);

    for (int i = 0; i < length - 1; i++) {
        if (buffer[i] == 255) {
            buffer[i] = 254;
        }
    }

    buffer[length - 1] = 255;
    rf_send(buffer, length);
}

static void btn_callback(uint_least8_t index) {
    if (my_id == 0) {
        my_id = time_counter % UINT16_MAX;
    }

    button_pressed = true;
}


void *main_thread(void *arg0)
{
    GPIO_init();
    GPIO_setConfig(Board_GPIO_BUTTON0, GPIO_CFG_IN_INT_FALLING | GPIO_CFG_IN_PU);
    GPIO_setCallback(Board_GPIO_BUTTON0, &btn_callback);
    GPIO_enableInt(Board_GPIO_BUTTON0);

    {
        GPTimerCC26XX_Params params;
        GPTimerCC26XX_Params_init(&params);
        params.width          = GPT_CONFIG_16BIT;
        params.mode           = GPT_MODE_PERIODIC;
        params.direction      = GPTimerCC26XX_DIRECTION_UP;
        params.debugStallMode = GPTimerCC26XX_DEBUG_STALL_OFF;
        hTimer = GPTimerCC26XX_open(CC1350STK_GPTIMER0A, &params);
        if(hTimer == NULL) {
            while (1) {}
        }

        // 50000 ~ breaks the timer
        // 100000 ~ 4ms
        // 200000 ~ 4ms wtf
        // 500000 ~ 8/12ms wtf
        // 5000000  ~ 102/106 ms
        GPTimerCC26XX_setLoadValue(hTimer, 100000);
        GPTimerCC26XX_registerInterrupt(hTimer, timerCallback, GPT_INT_TIMEOUT);
        GPTimerCC26XX_start(hTimer);
    }

    RF_Params rfParams;
    RF_Params_init(&rfParams);

    rfHandle = RF_open(&rfObject, &RF_prop, (RF_RadioSetup*)&RF_cmdPropRadioDivSetup, &rfParams);

    /* Set the frequency */
    RF_postCmd(rfHandle, (RF_Op*)&RF_cmdFs, RF_PriorityNormal, NULL, 0);

    if (SensorI2C_open())
    {
        /* Put unused external sensors and flash into Sleep */
        SensorBmp280_init();            // Pressure Sensor
        SensorBmp280_enable(true);

        SensorHdc1000_init();           // Humidity
        SensorHdc1000_start();
        
        SensorMpu9250_init();           // Gyroscope and accelerometer
        SensorMpu9250_powerOn();
        SensorMpu9250_enable(MPU_AX_ALL);
        
        /* Init Light sensor */
        SensorOpt3001_init();
        SensorOpt3001_enable(true);
    }
    else
    {
        while (1) {}
    }

    uint16_t last_raw_lux = 0;
    while (1)
    {
        if (my_id == 0) continue;

        uint16_t raw_lux;

        /* Read sensor */
        SensorOpt3001_read(&raw_lux);

        if (raw_lux != last_raw_lux) {
            rf_send_brightness_message(raw_lux);
            last_raw_lux = raw_lux;
        }
        if (button_pressed) {
            rf_send_button_pressed_message();
            button_pressed = false;
        }

        /*
        latestAdcValue = (uint16_t) lux;

        uint16_t tmp[3];

        if (SensorMpu9250_gyroRead(tmp)) {
            latestGyroValue[0] = SensorMpu9250_gyroConvert(tmp[0]);
            latestGyroValue[1] = SensorMpu9250_gyroConvert(tmp[1]);
            latestGyroValue[2] = SensorMpu9250_gyroConvert(tmp[2]);
        }

        if (SensorMpu9250_accRead(tmp)) {
            latestAccValue[0] = SensorMpu9250_accConvert(tmp[0]);
            latestAccValue[1] = SensorMpu9250_accConvert(tmp[1]);
            latestAccValue[2] = SensorMpu9250_accConvert(tmp[2]);
        }

        uint8_t rawPres;
        int32_t tempBMP;
        SensorBmp280_read(&rawPres);
        SensorBmp280_convert(&rawPres, &tempBMP, &latestPress);

        uint16_t rawtmp;
        uint16_t rawhum;
        SensorHdc1000_read(&rawtmp, &rawhum);
        SensorHdc1000_convert(rawtmp, rawhum, &latestTemp, &latestHum);

        NodeRadioTask_rf_sendAdcData(latestAdcValue);
        */

        //sprintf(buffer, "%.2f lux\n", lux);
        //size_t len = strlen(buffer);
        //UART_write(uart, buffer, len);
        //last_lux = lux;
    }
}
