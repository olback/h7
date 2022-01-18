use {
    super::dsi_consts::{
        DSI_CLTCR_HS2LP_TIME, DSI_CLTCR_LP2HS_TIME, DSI_DLTCR_HS2LP_TIME, DSI_DLTCR_LP2HS_TIME,
        DSI_DLTCR_MRD_TIME, DSI_PCONFR_SW_TIME,
    },
    stm32h7xx_hal::pac::DSIHOST,
};

/// DSI PHY Timings definition
#[derive(Debug, Clone)]
pub struct DsiPhyTimerConfig {
    pub ClockLaneHS2LPTime: u32,
    pub ClockLaneLP2HSTime: u32,
    pub DataLaneHS2LPTime: u32,
    pub DataLaneLP2HSTime: u32,
    pub DataLaneMaxReadTime: u32,
    pub StopWaitTime: u32,
}

impl DsiPhyTimerConfig {
    pub unsafe fn apply(&self, dsihost: &DSIHOST) {
        let max_time = if self.ClockLaneLP2HSTime > self.ClockLaneHS2LPTime {
            self.ClockLaneLP2HSTime
        } else {
            self.ClockLaneHS2LPTime
        };

        // hdsi->Instance->CLTCR &= ~(DSI_CLTCR_LP2HS_TIME | DSI_CLTCR_HS2LP_TIME);
        dsihost.cltcr.write(|w| {
            w.bits(dsihost.cltcr.read().bits() & !(DSI_CLTCR_LP2HS_TIME | DSI_CLTCR_HS2LP_TIME))
        });

        //   hdsi->Instance->CLTCR |= (maxTime | ((maxTime) << 16U));
        dsihost
            .cltcr
            .write(|w| w.bits(dsihost.cltcr.read().bits() | (max_time | (max_time << 16))));

        // hdsi->Instance->DLTCR &= ~(DSI_DLTCR_MRD_TIME | DSI_DLTCR_LP2HS_TIME | DSI_DLTCR_HS2LP_TIME)
        dsihost.dltcr.write(|w| {
            w.bits(
                dsihost.dltcr.read().bits()
                    & !(DSI_DLTCR_MRD_TIME | DSI_DLTCR_LP2HS_TIME | DSI_DLTCR_HS2LP_TIME),
            )
        });

        // hdsi->Instance->DLTCR |= (PhyTimers->DataLaneMaxReadTime | ((PhyTimers->DataLaneLP2HSTime) << 16U) | ((PhyTimers->DataLaneHS2LPTime) << 24U))
        dsihost.dltcr.write(|w| {
            w.bits(
                dsihost.dltcr.read().bits()
                    | (self.DataLaneMaxReadTime
                        | ((self.DataLaneLP2HSTime) << 16)
                        | ((self.DataLaneHS2LPTime) << 24)),
            )
        });

        // hdsi->Instance->PCONFR &= ~DSI_PCONFR_SW_TIME;
        dsihost
            .pconfr
            .write(|w| w.bits(dsihost.pconfr.read().bits() & !DSI_PCONFR_SW_TIME));

        // hdsi->Instance->PCONFR |= ((PhyTimers->StopWaitTime) << 8U);
        dsihost
            .pconfr
            .write(|w| w.bits(dsihost.pconfr.read().bits() | self.StopWaitTime));
    }
}
