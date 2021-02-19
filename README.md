# bmp280-server
A thin server around a BMP280 or BME280 sensor, using the bmp280 crate. 
Sensor data will be served as JSON to HTTP GETs at /measurement. 

This was useful for my learning `warp`, and some thread-safe primitives. 
The additional config is for compiling for ARMv6 (Raspberry Pi Zero etc.) 
