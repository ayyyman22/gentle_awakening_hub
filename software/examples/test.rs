#include "nrf_gpio.h"  // For handling GPIO pins in the microcontroller

#define LED_PIN 22  // Physical pin 22, which corresponds to P0.09

int main(void) {
    // Configure pin 22 as output
    nrf_gpio_cfg_output(LED_PIN);

    // Turn on the LED (set the pin to high)
    nrf_gpio_pin_set(LED_PIN);

    // Infinite loop to keep the LED on
    while (1) {
        // You can add other code here if necessary
    }

    return 0;
}
