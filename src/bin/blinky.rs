//! 闪烁一个LED灯
//!
//! 这个代码假设LD1（绿色）连接到pb0，LD3（红色）连接到pb14。这个假设对于nucleo-h743zi和nucleo-h743zi2这两种开发板是正确的。
#![no_std]
#![no_main]

use defmt_rtt as _; // 全局日志器

use defmt::info;
use panic_probe as _;

use stm32h7xx_hal::{block, prelude::*, timer::Timer};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    info!("开始闪烁");
    // 从外设访问包中获取设备特定外设的访问权限
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // 获取RCC设备的所有权，并将它们转换为相应的HAL结构体
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // 冻结系统中所有时钟的配置，并
    // 获取核心时钟分配和复位（CCDR）对象
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // 获取GPIOB外设
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // 将gpio B引脚0配置为推挽输出。
    let mut ld1 = gpiob.pb0.into_push_pull_output();

    // 将gpio B引脚14配置为推挽输出。
    let mut ld3 = gpiob.pb14.into_push_pull_output();
    ld3.set_high();

    // 配置定时器每秒触发一次更新
    let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.Hz());

    // 等待定时器触发更新并改变LED灯的状态
    info!("进入主循环");
    loop {
        ld1.set_high();
        block!(timer.wait()).unwrap();
        ld1.set_low();
        block!(timer.wait()).unwrap();
    }
}
