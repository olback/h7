use {
    super::dsi_consts::{
        DSI_LOOSELY_PACKED_ENABLE, DSI_MCR_CMDM, DSI_RGB666, DSI_VMCR_VMT, DSI_VPCR_VPSIZE,
        DSI_WCFGR_DSIM, *,
    },
    stm32h7xx_hal::pac::DSIHOST,
};

macro_rules! op_eq {
    ($dsi:ident, $reg:ident, $op:tt, $data:expr) => {
        $dsi
            .$reg
            .write(|w| w.bits($dsi.$reg.read().bits() $op ($data)));
    };
}

pub struct DsiVideoConfig {
    pub virtual_channel_id: u32,
    pub color_coding: u32,
    pub loosely_packed: u32,
    pub mode: u32,
    pub packet_size: u32,
    pub number_of_chunks: u32,
    pub null_packet_size: u32,
    pub hspolarity: u32,
    pub vspolarity: u32,
    pub depolarity: u32,
    pub horizontal_sync_active: u32,
    pub horizontal_back_porch: u32,
    pub horizontal_line: u32,
    pub vertical_sync_active: u32,
    pub vertical_back_porch: u32,
    pub vertical_front_porch: u32,
    pub vertical_active: u32,
    pub lpcommand_enable: u32,
    pub lplargest_packet_size: u32,
    pub lpvactlargest_packet_size: u32,
    pub lphorizontal_front_porch_enable: u32,
    pub lphorizontal_back_porch_enable: u32,
    pub lpvertical_active_enable: u32,
    pub lpvertical_front_porch_enable: u32,
    pub lpvertical_back_porch_enable: u32,
    pub lpvertical_sync_active_enable: u32,
    pub frame_btaacknowledge_enable: u32,
}

// HAL_StatusTypeDef HAL_DSI_ConfigVideoMode(DSI_HandleTypeDef *hdsi, DSI_VidCfgTypeDef *VidCfg)
// {
//   /* Process locked */
//   __HAL_LOCK(hdsi);

//   /* Check the parameters */
//   assert_param(IS_DSI_COLOR_CODING(VidCfg->ColorCoding));
//   assert_param(IS_DSI_VIDEO_MODE_TYPE(VidCfg->Mode));
//   assert_param(IS_DSI_LP_COMMAND(VidCfg->LPCommandEnable));
//   assert_param(IS_DSI_LP_HFP(VidCfg->LPHorizontalFrontPorchEnable));
//   assert_param(IS_DSI_LP_HBP(VidCfg->LPHorizontalBackPorchEnable));
//   assert_param(IS_DSI_LP_VACTIVE(VidCfg->LPVerticalActiveEnable));
//   assert_param(IS_DSI_LP_VFP(VidCfg->LPVerticalFrontPorchEnable));
//   assert_param(IS_DSI_LP_VBP(VidCfg->LPVerticalBackPorchEnable));
//   assert_param(IS_DSI_LP_VSYNC(VidCfg->LPVerticalSyncActiveEnable));
//   assert_param(IS_DSI_FBTAA(VidCfg->FrameBTAAcknowledgeEnable));
//   assert_param(IS_DSI_DE_POLARITY(VidCfg->DEPolarity));
//   assert_param(IS_DSI_VSYNC_POLARITY(VidCfg->VSPolarity));
//   assert_param(IS_DSI_HSYNC_POLARITY(VidCfg->HSPolarity));
//   /* Check the LooselyPacked variant only in 18-bit mode */
//   if (VidCfg->ColorCoding == DSI_RGB666)
//   {
//     assert_param(IS_DSI_LOOSELY_PACKED(VidCfg->LooselyPacked));
//   }

//   /* Select video mode by resetting CMDM and DSIM bits */
//   hdsi->Instance->MCR &= ~DSI_MCR_CMDM;
//   hdsi->Instance->WCFGR &= ~DSI_WCFGR_DSIM;

//   /* Configure the video mode transmission type */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_VMT;
//   hdsi->Instance->VMCR |= VidCfg->Mode;

//   /* Configure the video packet size */
//   hdsi->Instance->VPCR &= ~DSI_VPCR_VPSIZE;
//   hdsi->Instance->VPCR |= VidCfg->PacketSize;

//   /* Set the chunks number to be transmitted through the DSI link */
//   hdsi->Instance->VCCR &= ~DSI_VCCR_NUMC;
//   hdsi->Instance->VCCR |= VidCfg->NumberOfChunks;

//   /* Set the size of the null packet */
//   hdsi->Instance->VNPCR &= ~DSI_VNPCR_NPSIZE;
//   hdsi->Instance->VNPCR |= VidCfg->NullPacketSize;

//   /* Select the virtual channel for the LTDC interface traffic */
//   hdsi->Instance->LVCIDR &= ~DSI_LVCIDR_VCID;
//   hdsi->Instance->LVCIDR |= VidCfg->VirtualChannelID;

//   /* Configure the polarity of control signals */
//   hdsi->Instance->LPCR &= ~(DSI_LPCR_DEP | DSI_LPCR_VSP | DSI_LPCR_HSP);
//   hdsi->Instance->LPCR |= (VidCfg->DEPolarity | VidCfg->VSPolarity | VidCfg->HSPolarity);

//   /* Select the color coding for the host */
//   hdsi->Instance->LCOLCR &= ~DSI_LCOLCR_COLC;
//   hdsi->Instance->LCOLCR |= VidCfg->ColorCoding;

//   /* Select the color coding for the wrapper */
//   hdsi->Instance->WCFGR &= ~DSI_WCFGR_COLMUX;
//   hdsi->Instance->WCFGR |= ((VidCfg->ColorCoding) << 1U);

//   /* Enable/disable the loosely packed variant to 18-bit configuration */
//   if (VidCfg->ColorCoding == DSI_RGB666)
//   {
//     hdsi->Instance->LCOLCR &= ~DSI_LCOLCR_LPE;
//     hdsi->Instance->LCOLCR |= VidCfg->LooselyPacked;
//   }

//   /* Set the Horizontal Synchronization Active (HSA) in lane byte clock cycles */
//   hdsi->Instance->VHSACR &= ~DSI_VHSACR_HSA;
//   hdsi->Instance->VHSACR |= VidCfg->HorizontalSyncActive;

//   /* Set the Horizontal Back Porch (HBP) in lane byte clock cycles */
//   hdsi->Instance->VHBPCR &= ~DSI_VHBPCR_HBP;
//   hdsi->Instance->VHBPCR |= VidCfg->HorizontalBackPorch;

//   /* Set the total line time (HLINE=HSA+HBP+HACT+HFP) in lane byte clock cycles */
//   hdsi->Instance->VLCR &= ~DSI_VLCR_HLINE;
//   hdsi->Instance->VLCR |= VidCfg->HorizontalLine;

//   /* Set the Vertical Synchronization Active (VSA) */
//   hdsi->Instance->VVSACR &= ~DSI_VVSACR_VSA;
//   hdsi->Instance->VVSACR |= VidCfg->VerticalSyncActive;

//   /* Set the Vertical Back Porch (VBP)*/
//   hdsi->Instance->VVBPCR &= ~DSI_VVBPCR_VBP;
//   hdsi->Instance->VVBPCR |= VidCfg->VerticalBackPorch;

//   /* Set the Vertical Front Porch (VFP)*/
//   hdsi->Instance->VVFPCR &= ~DSI_VVFPCR_VFP;
//   hdsi->Instance->VVFPCR |= VidCfg->VerticalFrontPorch;

//   /* Set the Vertical Active period*/
//   hdsi->Instance->VVACR &= ~DSI_VVACR_VA;
//   hdsi->Instance->VVACR |= VidCfg->VerticalActive;

//   /* Configure the command transmission mode */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_LPCE;
//   hdsi->Instance->VMCR |= VidCfg->LPCommandEnable;

//   /* Low power largest packet size */
//   hdsi->Instance->LPMCR &= ~DSI_LPMCR_LPSIZE;
//   hdsi->Instance->LPMCR |= ((VidCfg->LPLargestPacketSize) << 16U);

//   /* Low power VACT largest packet size */
//   hdsi->Instance->LPMCR &= ~DSI_LPMCR_VLPSIZE;
//   hdsi->Instance->LPMCR |= VidCfg->LPVACTLargestPacketSize;

//   /* Enable LP transition in HFP period */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_LPHFPE;
//   hdsi->Instance->VMCR |= VidCfg->LPHorizontalFrontPorchEnable;

//   /* Enable LP transition in HBP period */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_LPHBPE;
//   hdsi->Instance->VMCR |= VidCfg->LPHorizontalBackPorchEnable;

//   /* Enable LP transition in VACT period */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVAE;
//   hdsi->Instance->VMCR |= VidCfg->LPVerticalActiveEnable;

//   /* Enable LP transition in VFP period */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVFPE;
//   hdsi->Instance->VMCR |= VidCfg->LPVerticalFrontPorchEnable;

//   /* Enable LP transition in VBP period */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVBPE;
//   hdsi->Instance->VMCR |= VidCfg->LPVerticalBackPorchEnable;

//   /* Enable LP transition in vertical sync period */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVSAE;
//   hdsi->Instance->VMCR |= VidCfg->LPVerticalSyncActiveEnable;

//   /* Enable the request for an acknowledge response at the end of a frame */
//   hdsi->Instance->VMCR &= ~DSI_VMCR_FBTAAE;
//   hdsi->Instance->VMCR |= VidCfg->FrameBTAAcknowledgeEnable;

//   /* Process unlocked */
//   __HAL_UNLOCK(hdsi);

//   return HAL_OK;
// }

impl DsiVideoConfig {
    pub unsafe fn apply(&self, dsihost: &DSIHOST) {
        if self.color_coding == DSI_RGB666 {
            assert!(self.loosely_packed == DSI_LOOSELY_PACKED_ENABLE);
        }

        //   /* Select video mode by resetting CMDM and DSIM bits */
        //   hdsi->Instance->MCR &= ~DSI_MCR_CMDM;
        op_eq!(dsihost, mcr, &, !DSI_MCR_CMDM);
        //   hdsi->Instance->WCFGR &= ~DSI_WCFGR_DSIM;
        op_eq!(dsihost, wcfgr, &, !DSI_WCFGR_DSIM);

        //   /* Configure the video mode transmission type */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_VMT;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_VMT);
        //   hdsi->Instance->VMCR |= VidCfg->Mode;
        op_eq!(dsihost, vmcr, |, self.mode);

        //   /* Configure the video packet size */
        //   hdsi->Instance->VPCR &= ~DSI_VPCR_VPSIZE;
        op_eq!(dsihost, vpcr, &, !DSI_VPCR_VPSIZE);
        //   hdsi->Instance->VPCR |= VidCfg->PacketSize;
        op_eq!(dsihost, vpcr, |, self.packet_size);

        //   /* Set the chunks number to be transmitted through the DSI link */
        //   hdsi->Instance->VCCR &= ~DSI_VCCR_NUMC;
        op_eq!(dsihost, vccr, &, !DSI_VCCR_NUMC);
        //   hdsi->Instance->VCCR |= VidCfg->NumberOfChunks;
        op_eq!(dsihost, vccr, |, self.number_of_chunks);

        //   /* Set the size of the null packet */
        //   hdsi->Instance->VNPCR &= ~DSI_VNPCR_NPSIZE;
        op_eq!(dsihost, vnpcr, &, !DSI_VNPCR_NPSIZE);
        //   hdsi->Instance->VNPCR |= VidCfg->NullPacketSize;
        op_eq!(dsihost, vnpcr, |, self.null_packet_size);

        //   /* Select the virtual channel for the LTDC interface traffic */
        //   hdsi->Instance->LVCIDR &= ~DSI_LVCIDR_VCID;
        op_eq!(dsihost, lvcidr, &, !DSI_LVCIDR_VCID);
        //   hdsi->Instance->LVCIDR |= VidCfg->VirtualChannelID;
        op_eq!(dsihost, lvcidr, |, self.virtual_channel_id);

        //   /* Configure the polarity of control signals */
        //   hdsi->Instance->LPCR &= ~(DSI_LPCR_DEP | DSI_LPCR_VSP | DSI_LPCR_HSP);
        op_eq!(dsihost, lpcr, &, !(DSI_LPCR_DEP | DSI_LPCR_VSP | DSI_LPCR_HSP));
        //   hdsi->Instance->LPCR |= (VidCfg->DEPolarity | VidCfg->VSPolarity | VidCfg->HSPolarity);
        op_eq!(dsihost, lpcr, |, (self.depolarity | self.vspolarity | self.hspolarity));

        //   /* Select the color coding for the host */
        //   hdsi->Instance->LCOLCR &= ~DSI_LCOLCR_COLC;
        op_eq!(dsihost, lcolcr, &, !DSI_LCOLCR_COLC);
        //   hdsi->Instance->LCOLCR |= VidCfg->ColorCoding;
        op_eq!(dsihost, lcolcr, |, self.color_coding);

        //   /* Select the color coding for the wrapper */
        //   hdsi->Instance->WCFGR &= ~DSI_WCFGR_COLMUX;
        op_eq!(dsihost, wcfgr, &, !DSI_WCFGR_COLMUX);
        //   hdsi->Instance->WCFGR |= ((VidCfg->ColorCoding) << 1U);
        op_eq!(dsihost, wcfgr, |, (self.color_coding << 1));

        //   /* Enable/disable the loosely packed variant to 18-bit configuration */
        //   if (VidCfg->ColorCoding == DSI_RGB666)
        //   {
        //     hdsi->Instance->LCOLCR &= ~DSI_LCOLCR_LPE;
        //     hdsi->Instance->LCOLCR |= VidCfg->LooselyPacked;
        //   }
        if self.color_coding == DSI_RGB666 {
            op_eq!(dsihost, lcolcr, &, !DSI_LCOLCR_LPE);
            op_eq!(dsihost, lcolcr, |, self.loosely_packed);
        }

        //   /* Set the Horizontal Synchronization Active (HSA) in lane byte clock cycles */
        //   hdsi->Instance->VHSACR &= ~DSI_VHSACR_HSA;
        op_eq!(dsihost, vhsacr, &, !DSI_VHSACR_HSA);
        //   hdsi->Instance->VHSACR |= VidCfg->HorizontalSyncActive;
        op_eq!(dsihost, vhsacr, |, self.horizontal_sync_active);

        //   /* Set the Horizontal Back Porch (HBP) in lane byte clock cycles */
        //   hdsi->Instance->VHBPCR &= ~DSI_VHBPCR_HBP;
        op_eq!(dsihost, vhbpcr, &, !DSI_VHBPCR_HBP);
        //   hdsi->Instance->VHBPCR |= VidCfg->HorizontalBackPorch;
        op_eq!(dsihost, vhbpcr, |, self.horizontal_back_porch);

        //   /* Set the total line time (HLINE=HSA+HBP+HACT+HFP) in lane byte clock cycles */
        //   hdsi->Instance->VLCR &= ~DSI_VLCR_HLINE;
        op_eq!(dsihost, vlcr, &, !DSI_VLCR_HLINE);
        //   hdsi->Instance->VLCR |= VidCfg->HorizontalLine;
        op_eq!(dsihost, vlcr, |, self.horizontal_line);

        //   /* Set the Vertical Synchronization Active (VSA) */
        //   hdsi->Instance->VVSACR &= ~DSI_VVSACR_VSA;
        op_eq!(dsihost, vvsacr, &, !DSI_VVSACR_VSA);
        //   hdsi->Instance->VVSACR |= VidCfg->VerticalSyncActive;
        op_eq!(dsihost, vvsacr, |, self.vertical_sync_active);

        //   /* Set the Vertical Back Porch (VBP)*/
        //   hdsi->Instance->VVBPCR &= ~DSI_VVBPCR_VBP;
        op_eq!(dsihost, vvbpcr, &, !DSI_VVBPCR_VBP);
        //   hdsi->Instance->VVBPCR |= VidCfg->VerticalBackPorch;
        op_eq!(dsihost, vvbpcr, |, self.vertical_back_porch);

        //   /* Set the Vertical Front Porch (VFP)*/
        //   hdsi->Instance->VVFPCR &= ~DSI_VVFPCR_VFP;
        op_eq!(dsihost, vvfpcr, &, !DSI_VVFPCR_VFP);
        //   hdsi->Instance->VVFPCR |= VidCfg->VerticalFrontPorch;
        op_eq!(dsihost, vvfpcr, |, self.vertical_front_porch);

        //   /* Set the Vertical Active period*/
        //   hdsi->Instance->VVACR &= ~DSI_VVACR_VA;
        op_eq!(dsihost, vvacr, &, !DSI_VVACR_VA);
        //   hdsi->Instance->VVACR |= VidCfg->VerticalActive;
        op_eq!(dsihost, vvacr, |, self.vertical_active);

        //   /* Configure the command transmission mode */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_LPCE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_LPCE);
        //   hdsi->Instance->VMCR |= VidCfg->LPCommandEnable;
        op_eq!(dsihost, vmcr, |, self.lpcommand_enable);

        //   /* Low power largest packet size */
        //   hdsi->Instance->LPMCR &= ~DSI_LPMCR_LPSIZE;
        op_eq!(dsihost, lpmcr, &, !DSI_LPMCR_LPSIZE);
        //   hdsi->Instance->LPMCR |= ((VidCfg->LPLargestPacketSize) << 16U);
        op_eq!(dsihost, lpmcr, |, (self.lplargest_packet_size << 16));

        //   /* Low power VACT largest packet size */
        //   hdsi->Instance->LPMCR &= ~DSI_LPMCR_VLPSIZE;
        op_eq!(dsihost, lpmcr, &, !DSI_LPMCR_VLPSIZE);
        //   hdsi->Instance->LPMCR |= VidCfg->LPVACTLargestPacketSize;
        op_eq!(dsihost, lpmcr, |, self.lpvactlargest_packet_size);

        //   /* Enable LP transition in HFP period */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_LPHFPE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_LPHFPE);
        //   hdsi->Instance->VMCR |= VidCfg->LPHorizontalFrontPorchEnable;
        op_eq!(dsihost, vmcr, |, self.lphorizontal_front_porch_enable);

        //   /* Enable LP transition in HBP period */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_LPHBPE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_LPHBPE);
        //   hdsi->Instance->VMCR |= VidCfg->LPHorizontalBackPorchEnable;
        op_eq!(dsihost, vmcr, |, self.lphorizontal_back_porch_enable);

        //   /* Enable LP transition in VACT period */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVAE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_LPVAE);
        //   hdsi->Instance->VMCR |= VidCfg->LPVerticalActiveEnable;
        op_eq!(dsihost, vmcr, |, self.lpvertical_active_enable);

        //   /* Enable LP transition in VFP period */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVFPE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_LPVFPE);
        //   hdsi->Instance->VMCR |= VidCfg->LPVerticalFrontPorchEnable;
        op_eq!(dsihost, vmcr, |, self.lpvertical_front_porch_enable);

        //   /* Enable LP transition in VBP period */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVBPE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_LPVBPE);
        //   hdsi->Instance->VMCR |= VidCfg->LPVerticalBackPorchEnable;
        op_eq!(dsihost, vmcr, |, self.lpvertical_back_porch_enable);

        //   /* Enable LP transition in vertical sync period */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_LPVSAE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_LPVSAE);
        //   hdsi->Instance->VMCR |= VidCfg->LPVerticalSyncActiveEnable;
        op_eq!(dsihost, vmcr, |, self.lpvertical_sync_active_enable);

        //   /* Enable the request for an acknowledge response at the end of a frame */
        //   hdsi->Instance->VMCR &= ~DSI_VMCR_FBTAAE;
        op_eq!(dsihost, vmcr, &, !DSI_VMCR_FBTAAE);
        //   hdsi->Instance->VMCR |= VidCfg->FrameBTAAcknowledgeEnable;
        op_eq!(dsihost, vmcr, |, self.frame_btaacknowledge_enable);
    }
}
