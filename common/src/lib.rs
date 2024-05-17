#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
#[repr(align(8))]
pub struct State {
    data: [u8; 105012usize],
    guest_memory_base: usize,
}
impl State {
    // Returns the ISA state with initial values and configuration set
    pub fn init(guest_memory_base: usize) -> Self {
        let mut celf = Self {
            data: [0; 105012usize],
            guest_memory_base,
        };
        celf
    }
    pub fn write_register<T>(&mut self, offset: isize, value: T) {
        unsafe { *(self.data.as_ptr().byte_offset(offset) as *mut T) = value };
    }
    pub fn read_register<T: Copy>(&self, offset: isize) -> T {
        unsafe { *(self.data.as_ptr().byte_offset(offset) as *const T) }
    }
    pub fn guest_memory_base(&self) -> usize {
        self.guest_memory_base
    }
}
pub const REG_ERXMISC1_EL1: isize = 1400;
pub const REG_FEAT_VMID16_IMPLEMENTED: isize = 1408;
pub const REG_V9AP0_IMPLEMENTED: isize = 1416;
pub const REG_FEAT_SVE_PMULL128_IMPLEMENTED: isize = 1424;
pub const REG_U__DBG_ROM_ADDR: isize = 1432;
pub const REG_U_ERXMISC7: isize = 1440;
pub const REG_GICH_EISR: isize = 1448;
pub const REG_U_VTCR: isize = 1456;
pub const REG_SCTLR2_EL3: isize = 1464;
pub const REG_PMU_EVENT_EXC_TAKEN: isize = 48;
pub const REG_ICC_CTLR_EL1_NS: isize = 1472;
pub const REG_ID_ISAR5_EL1: isize = 1480;
pub const REG_FEAT_EVT_IMPLEMENTED: isize = 1488;
pub const REG_U_PMINTENSET: isize = 1496;
pub const REG_FEAT_EL3_IMPLEMENTED: isize = 1512;
pub const REG_U_ICV_CTLR: isize = 1528;
pub const REG_AMPIDR2: isize = 1504;
pub const REG_PMVCIDSR: isize = 1536;
pub const REG_SPEADDRPOSPREVBRANCHTARGET: isize = 1048;
pub const REG_PMSDSFR_EL1: isize = 1520;
pub const REG_FEAT_FGT2_IMPLEMENTED: isize = 1544;
pub const REG_RLPIDEN: isize = 1552;
pub const REG_FEAT_ETEV1P2_IMPLEMENTED: isize = 1560;
pub const REG_FEAT_AES_IMPLEMENTED: isize = 1568;
pub const REG_U__MAX_IMPLEMENTED_SMEVECLEN: isize = 1576;
pub const REG_ICC_AP1R_EL1_S: isize = 1600;
pub const REG_MECID_A0_EL2: isize = 1592;
pub const REG_EDCIDR2: isize = 1640;
pub const REG_FEAT_SHA256_IMPLEMENTED: isize = 1632;
pub const REG_LORC_EL1: isize = 1648;
pub const REG_U_PMEVCNTR: isize = 1656;
pub const REG_U__EXCLUSIVE_GRANULE_SIZE: isize = 1784;
pub const REG_PMU_EVENT_LL_CACHE_HITM_RD: isize = 312;
pub const REG_FEAT_FGT_IMPLEMENTED: isize = 1792;
pub const REG_U_Z: isize = 1800;
pub const REG_FEAT_GICV4_IMPLEMENTED: isize = 9992;
pub const REG_FEAT_SEL2_IMPLEMENTED: isize = 10000;
pub const REG_U_ICH_AP1R: isize = 10008;
pub const REG_CFG_MPAM_NONE: isize = 1264;
pub const REG_FEAT_SME2P1_IMPLEMENTED: isize = 10024;
pub const REG_U__ETEBASE: isize = 10032;
pub const REG_GICC_BPR: isize = 10040;
pub const REG_CONTEXTIDR_EL1: isize = 10048;
pub const REG_GICR_STATUSR: isize = 10056;
pub const REG_CNTHVS_CVAL_EL2: isize = 10064;
pub const REG_SPEMAXCOUNTERS: isize = 1000;
pub const REG_STACK_LIMIT: isize = 10072;
pub const REG_GICC_ABPR: isize = 10080;
pub const REG_GICC_AIAR: isize = 10096;
pub const REG_U_CTR: isize = 10088;
pub const REG_FEAT_RASSAV1P1_IMPLEMENTED: isize = 10104;
pub const REG_ERXADDR_EL1: isize = 10112;
pub const REG_FEAT_PACQARMA5_IMPLEMENTED: isize = 10120;
pub const REG_OSLSR_EL1: isize = 10128;
pub const REG_GICR_SETLPIR: isize = 10136;
pub const REG_PMSLATFR_EL1: isize = 10144;
pub const REG_U_DBGDTRRXEXT: isize = 10152;
pub const REG_U_HDCR: isize = 10160;
pub const REG_BRBINFINJ_EL1: isize = 10168;
pub const REG_PMCEID1_EL0: isize = 10176;
pub const REG_SP_EL1: isize = 10184;
pub const REG_CP15SDISABLE: isize = 10192;
pub const REG_ICC_SRE_EL3: isize = 10200;
pub const REG_FEAT_HPMN0_IMPLEMENTED: isize = 10208;
pub const REG_GCSPR_EL2: isize = 10216;
pub const REG_U_ERXMISC1: isize = 10224;
pub const REG_GICV_RPR: isize = 10232;
pub const REG_ICH_LR_EL2: isize = 10240;
pub const REG_U__HIGHEST_EL_AARCH32: isize = 10368;
pub const REG_SMCR_EL2: isize = 10376;
pub const REG_SPERECORDSIZE: isize = 10384;
pub const REG_FEAT_RNG_TRAP_IMPLEMENTED: isize = 10400;
pub const REG_FEAT_DOPD_IMPLEMENTED: isize = 10408;
pub const REG_ERXCTLR_EL1: isize = 10416;
pub const REG_CFG_ID_AA64PFR0_EL1_EL1: isize = 1232;
pub const REG_U__CYCLE_COUNT: isize = 10424;
pub const REG_PMUACR_EL1: isize = 10440;
pub const REG_U_ICC_CTLR_S: isize = 10456;
pub const REG_FEAT_HAFT_IMPLEMENTED: isize = 10464;
pub const REG_U_CNTV_CTL: isize = 10448;
pub const REG_FEAT_PMUV3_EXT32_IMPLEMENTED: isize = 10472;
pub const REG_ACTLR2_S: isize = 10480;
pub const REG_U_ID_MMFR1: isize = 10488;
pub const REG_GICD_CTLR: isize = 10496;
pub const REG_CNTHPS_CTL_EL2: isize = 10504;
pub const REG_AMCFGR_EL0: isize = 10512;
pub const REG_PMCIDR2: isize = 10520;
pub const REG_SPESAMPLEINSTISNV2: isize = 10528;
pub const REG_VBAR_S: isize = 10536;
pub const REG_MAIR_EL2: isize = 10544;
pub const REG_FEAT_PACIMP_IMPLEMENTED: isize = 10552;
pub const REG_PMULASTTHRESHOLDVALUE: isize = 10560;
pub const REG_R25: isize = 10592;
pub const REG_ICV_IGRPEN1_EL1: isize = 10600;
pub const REG_ID_AA64AFR0_EL1: isize = 10608;
pub const REG_ACTLR_EL2: isize = 10616;
pub const REG_FEAT_DGH_IMPLEMENTED: isize = 10624;
pub const REG_GITS_TYPER: isize = 10632;
pub const REG_U__MONOMORPHIZE_READS: isize = 10640;
pub const REG_MPAMVPM1_EL2: isize = 10648;
pub const REG_RNDRRS: isize = 10656;
pub const REG_SPERECORDDATA: isize = 10664;
pub const REG_GICR_VSGIR: isize = 10728;
pub const REG_LOG2_TAG_GRANULE: isize = 456;
pub const REG_TCR_EL3: isize = 10736;
pub const REG_PMEVCNTR_EL0: isize = 10744;
pub const REG_U_MAIR0_NS: isize = 11000;
pub const REG_EDRCR: isize = 11008;
pub const REG_IFSR_S: isize = 11016;
pub const REG_GPRS: isize = 520;
pub const REG_FEAT_FLAGM2_IMPLEMENTED: isize = 11024;
pub const REG_MPAMIDR_EL1: isize = 11032;
pub const REG_ICH_MISR_EL2: isize = 11040;
pub const REG_U_AIFSR_NS: isize = 11048;
pub const REG_GICC_AHPPIR: isize = 11056;
pub const REG_ZCR_EL3: isize = 11064;
pub const REG_U_ERXFR: isize = 11072;
pub const REG_U_ID_DFR0: isize = 11080;
pub const REG_CPTR_EL2: isize = 11088;
pub const REG_APIBKEYLO_EL1: isize = 11096;
pub const REG_NUM_PMU_COUNTERS: isize = 11104;
pub const REG_PMPCSCTL: isize = 11120;
pub const REG_U__RD_BASE: isize = 11128;
pub const REG_SPESAMPLEADDRESSVALID: isize = 11136;
pub const REG_VSTCR_EL2: isize = 11168;
pub const REG_U__MAX_IMPLEMENTED_SVEVECLEN: isize = 11176;
pub const REG_PMU_EVENT_L2D_CACHE_RD: isize = 144;
pub const REG_U_CNTHCTL: isize = 11192;
pub const REG_SPEADDRPOSPCVIRTUAL: isize = 1016;
pub const REG_FEAT_ETMV4P1_IMPLEMENTED: isize = 11200;
pub const REG_PMEVTYPER_EL0: isize = 11208;
pub const REG_TRFCR_EL1: isize = 11464;
pub const REG_GICC_HPPIR: isize = 11472;
pub const REG_GCR_EL1: isize = 11480;
pub const REG_CFG_MPAM_V1P1: isize = 1280;
pub const REG_R23: isize = 11488;
pub const REG_FEAT_TIDCP1_IMPLEMENTED: isize = 11496;
pub const REG_DACR_S: isize = 11504;
pub const REG_EDPIDR1: isize = 11512;
pub const REG_U_SDER32_EL3: isize = 11520;
pub const REG_SPESAMPLESUBCLASSVALID: isize = 11528;
pub const REG_ICC_AP1R_EL1_NS: isize = 11536;
pub const REG_U_DBGDTR_EL0: isize = 11568;
pub const REG_FEAT_LSE128_IMPLEMENTED: isize = 11576;
pub const REG_U__RME_L0GPTSZ: isize = 11584;
pub const REG_AMAIR_EL3: isize = 11592;
pub const REG_FEAT_AMUV1_IMPLEMENTED: isize = 11600;
pub const REG_FEAT_PMUV3_EDGE_IMPLEMENTED: isize = 11608;
pub const REG_TTBR0_NS: isize = 11616;
pub const REG_FEAT_AIE_IMPLEMENTED: isize = 11624;
pub const REG_ICC_CTLR_EL3: isize = 11632;
pub const REG_PMMIR: isize = 11640;
pub const REG_TRFCR_EL2: isize = 11648;
pub const REG_R28: isize = 11656;
pub const REG_PMU_EVENT_SAMPLE_COLLISION: isize = 176;
pub const REG_FEAT_PCSRV8P2_IMPLEMENTED: isize = 11664;
pub const REG_TPIDRPRW_S: isize = 11672;
pub const REG_V8AP0_IMPLEMENTED: isize = 11680;
pub const REG_FEAT_AA64EL2_IMPLEMENTED: isize = 11688;
pub const REG_LR_MON: isize = 11696;
pub const REG_GCSCR_EL3: isize = 11704;
pub const REG_U_IFSR_NS: isize = 11712;
pub const REG_RCW128_PROTECTED_BIT: isize = 1192;
pub const REG_SCTLR2_EL1: isize = 11720;
pub const REG_ICC_NMIAR1_EL1: isize = 11728;
pub const REG_PMCNTENSET_EL0: isize = 11736;
pub const REG_ID_PFR2_EL1: isize = 11744;
pub const REG_U_AMEVTYPER0: isize = 11752;
pub const REG_U_ICH_LRC: isize = 11768;
pub const REG_EDDEVTYPE: isize = 11832;
pub const REG_FEAT_IDST_IMPLEMENTED: isize = 11840;
pub const REG_ISWFESLEEP: isize = 11848;
pub const REG_U_ICC_IAR1: isize = 11856;
pub const REG_FEAT_AA64EL3_IMPLEMENTED: isize = 11864;
pub const REG_U_ICH_MISR: isize = 11872;
pub const REG_FEAT_PMUV3_ICNTR_IMPLEMENTED: isize = 11880;
pub const REG_HPFAR_EL2: isize = 11888;
pub const REG_APGAKEYLO_EL1: isize = 11896;
pub const REG_ICC_SRE_EL1_S: isize = 11904;
pub const REG_U_ERXSTATUS: isize = 11912;
pub const REG_GICR_WAKER: isize = 11920;
pub const REG_FEAT_SVE_IMPLEMENTED: isize = 11928;
pub const REG_S2PIR_EL2: isize = 11936;
pub const REG_SPMACCESSR_EL1: isize = 11944;
pub const REG_U_ICC_AP0R: isize = 11952;
pub const REG_CNTFID0: isize = 11968;
pub const REG_TPIDR_EL2: isize = 11976;
pub const REG_ICC_IGRPEN1_EL3: isize = 11984;
pub const REG_ESR_EL3: isize = 11992;
pub const REG_FEAT_CSSC_IMPLEMENTED: isize = 12008;
pub const REG_GICR_VSGIPENDR: isize = 12000;
pub const REG_R6: isize = 12016;
pub const REG_FEAT_SPEV1P1_IMPLEMENTED: isize = 12024;
pub const REG_FEAT_SCTLR2_IMPLEMENTED: isize = 12032;
pub const REG_FEAT_MTE_TAGGED_FAR_IMPLEMENTED: isize = 12040;
pub const REG_ICV_IGRPEN0_EL1: isize = 12048;
pub const REG_GICD_TYPER2: isize = 12056;
pub const REG_U_CCSIDR: isize = 12064;
pub const REG_DBGCLAIMSET_EL1: isize = 12072;
pub const REG_SP_EL3: isize = 12080;
pub const REG_CPACR_EL1: isize = 12088;
pub const REG_U_HVBAR: isize = 12096;
pub const REG_PMVIDSR: isize = 12104;
pub const REG_FEAT_TRBE_MPAM_IMPLEMENTED: isize = 12112;
pub const REG_ICV_IAR0_EL1: isize = 12120;
pub const REG_FEAT_BRBEV1P1_IMPLEMENTED: isize = 12128;
pub const REG_SPIDEN: isize = 12136;
pub const REG_FEAT_PMUV3P1_IMPLEMENTED: isize = 12144;
pub const REG_FEAT_SME_FA64_IMPLEMENTED: isize = 12152;
pub const REG_U_HAMAIR0: isize = 12160;
pub const REG_FEAT_TWED_IMPLEMENTED: isize = 12168;
pub const REG_PIR_EL3: isize = 12176;
pub const REG_DBGBCR_EL1: isize = 12184;
pub const REG_STACK_BASE: isize = 12696;
pub const REG_U_ICC_RPR: isize = 12704;
pub const REG_AMAIR0_S: isize = 12712;
pub const REG_GICV_STATUSR: isize = 12720;
pub const REG_PMITCTRL: isize = 12728;
pub const REG_PMSIRR_EL1: isize = 12736;
pub const REG_U_PC: isize = 12744;
pub const REG_U_ICC_ASGI1R: isize = 12752;
pub const REG_NUM_AMU_COUNTER_GROUPS: isize = 12760;
pub const REG_ICC_PMR_EL1: isize = 12776;
pub const REG_FEAT_RASSAV2_IMPLEMENTED: isize = 12784;
pub const REG_U_MPAM3_EL3: isize = 12792;
pub const REG_FEAT_PAN3_IMPLEMENTED: isize = 12800;
pub const REG_CNTHCTL_EL2: isize = 12808;
pub const REG_TAG_GRANULE: isize = 1344;
pub const REG_TCR_EL2: isize = 12816;
pub const REG_ICV_CTLR_EL1: isize = 12824;
pub const REG_AMAIR_EL2: isize = 12832;
pub const REG_U_MVFR1: isize = 12840;
pub const REG_U_ICC_AP1R_NS: isize = 12848;
pub const REG_U_CCSIDR2: isize = 12864;
pub const REG_U_AMCGCR: isize = 12872;
pub const REG_TFSR_EL1: isize = 12880;
pub const REG_U_HSR: isize = 12888;
pub const REG_FEAT_RASV2_IMPLEMENTED: isize = 12896;
pub const REG_PMSNEVFR_EL1: isize = 12904;
pub const REG_FEAT_CSV2_1P2_IMPLEMENTED: isize = 12912;
pub const REG_PMU_EVENT_SAMPLE_WRAP: isize = 224;
pub const REG_DEBUGEXCEPTION_BREAKPOINT: isize = 1312;
pub const REG_FPCR: isize = 12920;
pub const REG_U_PMCCNTR: isize = 12928;
pub const REG_ERXMISC3_EL1: isize = 12936;
pub const REG_PMICNTR_EL0: isize = 12944;
pub const REG_U__DCZID_LOG2_BLOCK_SIZE: isize = 12952;
pub const REG_EDPIDR2: isize = 12968;
pub const REG_U_DCLONE: isize = 12976;
pub const REG_CTIAUTHSTATUS: isize = 13232;
pub const REG_U__SYNCABORTONTTWNONCACHE: isize = 13240;
pub const REG_U__SYNCABORTONREADNORMNONCACHE: isize = 13248;
pub const REG_U_ICV_DIR: isize = 13256;
pub const REG_U_AIDR: isize = 13264;
pub const REG_PMSSCR_EL1: isize = 13272;
pub const REG_U_CNTP_CTL_NS: isize = 13280;
pub const REG_FEAT_AA32EL3_IMPLEMENTED: isize = 13288;
pub const REG_U_AMEVTYPER1: isize = 13296;
pub const REG_DLR_EL0: isize = 13360;
pub const REG_AFSR0_EL2: isize = 13368;
pub const REG_U_TTBCR2_NS: isize = 13376;
pub const REG_GIC_BASE: isize = 1360;
pub const REG_U_ICV_BPR1: isize = 13384;
pub const REG_U__MPAM_PMG_MAX: isize = 13392;
pub const REG_FEAT_HPDS2_IMPLEMENTED: isize = 13400;
pub const REG_FEAT_PMUV3P9_IMPLEMENTED: isize = 13408;
pub const REG_U_HADFSR: isize = 13416;
pub const REG_U_ICH_ELRSR: isize = 13424;
pub const REG_APGAKEYHI_EL1: isize = 13432;
pub const REG_AMCNTENSET1_EL0: isize = 13440;
pub const REG_APDAKEYHI_EL1: isize = 13448;
pub const REG_PHYSICALCOUNT: isize = 13456;
pub const REG_U__GICITSCONTROLBASE: isize = 13472;
pub const REG_DEBUGHALT_STEP_EXCLUSIVE: isize = 1120;
pub const REG_ID_AA64PFR2_EL1: isize = 13480;
pub const REG_U_AMCFGR: isize = 13488;
pub const REG_BRBIDR0_EL1: isize = 13496;
pub const REG_SPESAMPLETIMESTAMP: isize = 13504;
pub const REG_GICR_SYNCR: isize = 13512;
pub const REG_U_NMRR_NS: isize = 13520;
pub const REG_SPESAMPLESUBCLASS: isize = 13528;
pub const REG_U_MPAM1_EL1: isize = 13536;
pub const REG_U_ID_MMFR5: isize = 13544;
pub const REG_ICV_EOIR0_EL1: isize = 13552;
pub const REG_FEAT_EXS_IMPLEMENTED: isize = 13560;
pub const REG_ICV_HPPIR0_EL1: isize = 13568;
pub const REG_FEAT_BBM_IMPLEMENTED: isize = 13576;
pub const REG_U__SME_ONLY: isize = 13584;
pub const REG_POR_EL1: isize = 13592;
pub const REG_U__THISINSTRENC: isize = 13600;
pub const REG_HFGITR_EL2: isize = 13608;
pub const REG_PMECR_EL1: isize = 13616;
pub const REG_EDAA32PFR: isize = 13624;
pub const REG_PMU_EVENT_SAMPLE_FILTRATE: isize = 168;
pub const REG_DISR_EL1: isize = 13632;
pub const REG_U_ID_ISAR6: isize = 13640;
pub const REG_M32_MONITOR: isize = 384;
pub const REG_VNCR_EL2: isize = 13648;
pub const REG_FEAT_PFAR_IMPLEMENTED: isize = 13656;
pub const REG_ICC_EOIR0_EL1: isize = 13664;
pub const REG_GICR_IIDR: isize = 13672;
pub const REG_CTICIDR0: isize = 13680;
pub const REG_SPMACCESSR_EL3: isize = 13688;
pub const REG_CNTEL0ACR: isize = 13696;
pub const REG_PMBSR_EL1: isize = 13704;
pub const REG_U_AMCR: isize = 13712;
pub const REG_U_ICV_RPR: isize = 13720;
pub const REG_U__IMPDEF_TG1: isize = 13728;
pub const REG_CTIDEVTYPE: isize = 13736;
pub const REG_EDCIDR1: isize = 13744;
pub const REG_CTIDEVCTL: isize = 13752;
pub const REG_PMU_EVENT_LDST_ALIGN_LAT: isize = 192;
pub const REG_GPT_CONTIG: isize = 864;
pub const REG_U_HTRFCR: isize = 13760;
pub const REG_FEAT_RASV1P1_IMPLEMENTED: isize = 13768;
pub const REG_SPESAMPLEADDRESS: isize = 13776;
pub const REG_U__LAST_BRANCH_VALID: isize = 14032;
pub const REG_EDPRSR: isize = 14040;
pub const REG_CFG_MPIDR: isize = 14048;
pub const REG_FEAT_DEBUGV8P2_IMPLEMENTED: isize = 14056;
pub const REG_FEAT_LRCPC_IMPLEMENTED: isize = 14064;
pub const REG_PMPIDR2: isize = 14072;
pub const REG_U_IFAR_NS: isize = 14080;
pub const REG_U_HAIFSR: isize = 14088;
pub const REG_U_DBGWCR: isize = 14096;
pub const REG_CNTPS_CVAL_EL1: isize = 14160;
pub const REG_DEBUGEXCEPTION_BKPT: isize = 1320;
pub const REG_U_TTBR1_EL1: isize = 14168;
pub const REG_SPESAMPLEDATASOURCEVALID: isize = 14184;
pub const REG_AMDEVTYPE: isize = 14192;
pub const REG_POR_EL3: isize = 14200;
pub const REG_U_EDSCR2: isize = 14208;
pub const REG_U__SUPPORTED_VA_SIZE: isize = 14216;
pub const REG_FEAT_HCX_IMPLEMENTED: isize = 14232;
pub const REG_U__CNTBASE_FREQUENCY: isize = 14240;
pub const REG_GITS_CBASER: isize = 14248;
pub const REG_U__MPAM_FRAC: isize = 14256;
pub const REG_FEAT_ADERR_IMPLEMENTED: isize = 14264;
pub const REG_U_PMCNTEN: isize = 14272;
pub const REG_TPIDR_EL1: isize = 14280;
pub const REG_U_TPIDRURW_NS: isize = 14288;
pub const REG_GPT_REALM: isize = 896;
pub const REG_FEAT_AMUV1P1_IMPLEMENTED: isize = 14296;
pub const REG_FEAT_CSV2_1P1_IMPLEMENTED: isize = 14304;
pub const REG_FEAT_ANERR_IMPLEMENTED: isize = 14312;
pub const REG_APDBKEYHI_EL1: isize = 14320;
pub const REG_NUM_GIC_PREEMPTION_BITS: isize = 14328;
pub const REG_U__SET_MOPS_OPTION_A_SUPPORTED: isize = 14344;
pub const REG_FEAT_LS64_V_IMPLEMENTED: isize = 14352;
pub const REG_HEAP_LIMIT: isize = 14360;
pub const REG_U_PMCEID0: isize = 14368;
pub const REG_SP_REL_ACCESS_PC: isize = 14376;
pub const REG_ID_ISAR1_EL1: isize = 14384;
pub const REG_U_ERRIDR: isize = 14392;
pub const REG_U__HAS_SME_PRIORITY_CONTROL: isize = 14400;
pub const REG_GICR_CLRLPIR: isize = 14408;
pub const REG_ERXGSR_EL1: isize = 14416;
pub const REG_FEAT_TRC_SR_IMPLEMENTED: isize = 14424;
pub const REG_FEAT_RNG_IMPLEMENTED: isize = 14432;
pub const REG_GITS_MPIDR: isize = 14440;
pub const REG_FEAT_PMUV3P5_IMPLEMENTED: isize = 14448;
pub const REG_FEAT_LVA3_IMPLEMENTED: isize = 14456;
pub const REG_FEAT_MTE_STORE_ONLY_IMPLEMENTED: isize = 14464;
pub const REG_FEAT_PCSRV8P9_IMPLEMENTED: isize = 14472;
pub const REG_FEAT_SPE_FDS_IMPLEMENTED: isize = 14480;
pub const REG_PMU_EVENT_SAMPLE_FEED_OP: isize = 256;
pub const REG_U_AMAIR1_NS: isize = 14488;
pub const REG_ICC_IGRPEN0_EL1: isize = 14496;
pub const REG_U_PMINTEN: isize = 14504;
pub const REG_GICR_CTLR: isize = 14512;
pub const REG_DBGDEVID: isize = 14520;
pub const REG_U_TTBR0_EL1: isize = 14528;
pub const REG_U__CNTBASEN: isize = 14544;
pub const REG_U_FFR: isize = 14552;
pub const REG_CNTPOFF_EL2: isize = 14584;
pub const REG_APDAKEYLO_EL1: isize = 14592;
pub const REG_ID_AA64ISAR1_EL1: isize = 14600;
pub const REG_AFSR1_EL3: isize = 14608;
pub const REG_FEAT_SHA512_IMPLEMENTED: isize = 14616;
pub const REG_AMEVCNTR0: isize = 14624;
pub const REG_AMCGCR_EL0: isize = 14656;
pub const REG_MAX_ZERO_BLOCK_SIZE: isize = 1088;
pub const REG_FEAT_EL1_IMPLEMENTED: isize = 14664;
pub const REG_U_ID_ISAR3: isize = 14672;
pub const REG_U_PMSWINC: isize = 14680;
pub const REG_FEAT_IVIPT_IMPLEMENTED: isize = 14688;
pub const REG_SEE: isize = 14696;
pub const REG_EDESR: isize = 14712;
pub const REG_U_IFAR_S: isize = 14720;
pub const REG_U_ID_PFR0: isize = 14728;
pub const REG_PMSIDR_EL1: isize = 14736;
pub const REG_FEAT_SB_IMPLEMENTED: isize = 14744;
pub const REG_U_CNTHP_CVAL: isize = 14752;
pub const REG_FEAT_PCSRV8_IMPLEMENTED: isize = 14760;
pub const REG_R29: isize = 14768;
pub const REG_TCR2_EL1: isize = 14776;
pub const REG_FEAT_LSE_IMPLEMENTED: isize = 14784;
pub const REG_APIAKEYHI_EL1: isize = 14792;
pub const REG_ZCR_EL3_LEN_VALUE: isize = 14800;
pub const REG_FEAT_SVE_BITPERM_IMPLEMENTED: isize = 14816;
pub const REG_HTTBR: isize = 14824;
pub const REG_ICH_AP0R_EL2: isize = 14832;
pub const REG_ID_AA64ISAR2_EL1: isize = 14864;
pub const REG_CNTHVS_CTL_EL2: isize = 14872;
pub const REG_SPESAMPLECONTEXTEL2VALID: isize = 14880;
pub const REG_DOMAIN_NOACCESS: isize = 784;
pub const REG_ICC_ASGI1R_EL1: isize = 14888;
pub const REG_ID_AA64MMFR0_EL1: isize = 14896;
pub const REG_HACR_EL2: isize = 14904;
pub const REG_FEAT_CONSTPACFIELD_IMPLEMENTED: isize = 14912;
pub const REG_PMU_EVENT_SAMPLE_POP: isize = 152;
pub const REG_FEAT_GICV3_IMPLEMENTED: isize = 14920;
pub const REG_FEAT_CHK_IMPLEMENTED: isize = 14928;
pub const REG_FEAT_ETEV1P1_IMPLEMENTED: isize = 14936;
pub const REG_U__BRANCHTAKEN: isize = 14944;
pub const REG_TFSRE0_EL1: isize = 14952;
pub const REG_MDRAR_EL1: isize = 14960;
pub const REG_PMCEID0_EL0: isize = 14968;
pub const REG_GITS_CREADR: isize = 14976;
pub const REG_PMIIDR: isize = 14984;
pub const REG_U_ID_ISAR4: isize = 14992;
pub const REG_U__CNTCTLBASE: isize = 15000;
pub const REG_GICM_CLRSPI_NSR: isize = 15024;
pub const REG_U_ERXMISC4: isize = 15008;
pub const REG_RVBAR: isize = 15032;
pub const REG_CFG_ID_AA64PFR0_EL1_EL0: isize = 1224;
pub const REG_GITS_CTLR: isize = 15016;
pub const REG_U_EDSCR: isize = 15040;
pub const REG_SDCR: isize = 15048;
pub const REG_IFSR32_EL2: isize = 15056;
pub const REG_ICV_PMR_EL1: isize = 15064;
pub const REG_CFG_MPAM_FRAC_NONE: isize = 1288;
pub const REG_ZCR_EL2: isize = 15072;
pub const REG_U_AMEVCNTR1: isize = 15080;
pub const REG_M32_ABORT: isize = 392;
pub const REG_FEAT_FRINTTS_IMPLEMENTED: isize = 15208;
pub const REG_U_SPSR_SVC: isize = 15216;
pub const REG_U__EMPAM_TIDR_IMPLEMENTED: isize = 15224;
pub const REG_DBGDEVID1: isize = 15232;
pub const REG_FEAT_TRC_EXT_IMPLEMENTED: isize = 15240;
pub const REG_U_ERXMISC0: isize = 15248;
pub const REG_FEAT_F32MM_IMPLEMENTED: isize = 15256;
pub const REG_V8AP3_IMPLEMENTED: isize = 15264;
pub const REG_ERRIDR_EL1: isize = 15272;
pub const REG_GICC_AEOIR: isize = 15280;
pub const REG_GICC_DIR: isize = 15288;
pub const REG_FEAT_ECV_IMPLEMENTED: isize = 15296;
pub const REG_U_CPACR: isize = 15304;
pub const REG_FEAT_SPEV1P2_IMPLEMENTED: isize = 15312;
pub const REG_U__SYNCABORTONPREFETCH: isize = 15320;
pub const REG_VTCR_EL2: isize = 15328;
pub const REG_POR_EL2: isize = 15336;
pub const REG_PMCCNTSVR_EL1: isize = 15344;
pub const REG_PMXEVCNTR_EL0: isize = 15352;
pub const REG_SP_MON: isize = 15360;
pub const REG_TTBCR_S: isize = 15368;
pub const REG_ICH_VMCR_EL2: isize = 15376;
pub const REG_U_FPSCR: isize = 15384;
pub const REG_ICV_RPR_EL1: isize = 15392;
pub const REG_AFSR1_EL2: isize = 15400;
pub const REG_ACTLR_S: isize = 15408;
pub const REG_FEAT_LPA_IMPLEMENTED: isize = 15416;
pub const REG_DEFAULTPARTID: isize = 768;
pub const REG_EDPFR: isize = 15424;
pub const REG_FEAT_ETMV4P4_IMPLEMENTED: isize = 15432;
pub const REG_SPESAMPLEPREVIOUSBRANCHADDRESS: isize = 15440;
pub const REG_PMINTENCLR_EL1: isize = 15448;
pub const REG_EDLSR: isize = 15456;
pub const REG_MPAMVPM2_EL2: isize = 15464;
pub const REG_AMPIDR1: isize = 15472;
pub const REG_RTPIDEN: isize = 15480;
pub const REG_FEAT_DOTPROD_IMPLEMENTED: isize = 15488;
pub const REG_GICR_PENDBASER: isize = 15496;
pub const REG_U_ID_ISAR2: isize = 15504;
pub const REG_GICC_IAR: isize = 15512;
pub const REG_U_MAIR1_S: isize = 15520;
pub const REG_U_ICC_BPR0: isize = 15528;
pub const REG_DEBUGHALT_HALTINSTRUCTION: isize = 1152;
pub const REG_SPSR_FIQ: isize = 15536;
pub const REG_AMCR_EL0: isize = 15544;
pub const REG_FEAT_DPB_IMPLEMENTED: isize = 15552;
pub const REG_U_SCTLR_NS: isize = 15560;
pub const REG_ICC_IAR0_EL1: isize = 15568;
pub const REG_FPSID: isize = 15576;
pub const REG_FEAT_CSV3_IMPLEMENTED: isize = 15584;
pub const REG_FEAT_S1POE_IMPLEMENTED: isize = 15592;
pub const REG_FEAT_LSMAOC_IMPLEMENTED: isize = 15600;
pub const REG_GCSCRE0_EL1: isize = 15608;
pub const REG_LST_64BV: isize = 1200;
pub const REG_AMIIDR: isize = 15616;
pub const REG_U__BLOCK_BBM_IMPLEMENTED: isize = 15624;
pub const REG_PMU_EVENT_SAMPLE_FEED_ST: isize = 248;
pub const REG_U_ERXCTLR: isize = 15640;
pub const REG_GICC_CTLR: isize = 15648;
pub const REG_CPTR_EL3_EZ_VALUE: isize = 15656;
pub const REG_R2: isize = 15672;
pub const REG_ACTLR_EL3: isize = 15680;
pub const REG_FEAT_VPIPT_IMPLEMENTED: isize = 15688;
pub const REG_U_ICC_HPPIR0: isize = 15696;
pub const REG_MEMATTR_WT: isize = 472;
pub const REG_PMBIDR_EL1: isize = 15704;
pub const REG_CTIITCTRL: isize = 15712;
pub const REG_VMECID_A_EL2: isize = 15720;
pub const REG_U_HAMAIR1: isize = 15728;
pub const REG_SPSR_EL2: isize = 15736;
pub const REG_COLD_RESET: isize = 1392;
pub const REG_LORSA_EL1: isize = 15744;
pub const REG_TCR2_EL2: isize = 15752;
pub const REG_APDBKEYLO_EL1: isize = 15760;
pub const REG_RVBAR_EL3: isize = 15768;
pub const REG_PMPIDR0: isize = 15776;
pub const REG_U_ICH_LR: isize = 15784;
pub const REG_U__CLOCK_DIVIDER: isize = 15848;
pub const REG_PMCCFILTR_EL0: isize = 15864;
pub const REG_OSDTRRX_EL1: isize = 15872;
pub const REG_DBGDSAR: isize = 15880;
pub const REG_U_VPIDR: isize = 15888;
pub const REG_CNTID: isize = 15896;
pub const REG_FEAT_SVE2_IMPLEMENTED: isize = 15904;
pub const REG_FEAT_SME2_IMPLEMENTED: isize = 15912;
pub const REG_HEAP_BASE: isize = 15920;
pub const REG_FEAT_ETMV4P2_IMPLEMENTED: isize = 15928;
pub const REG_U__MECID_WIDTH: isize = 15936;
pub const REG_BRBTGT_EL1: isize = 15944;
pub const REG_GICV_HPPIR: isize = 16200;
pub const REG_FEAT_PMUV3_IMPLEMENTED: isize = 16208;
pub const REG_FEAT_SSBS_IMPLEMENTED: isize = 16216;
pub const REG_U_HDFAR: isize = 16224;
pub const REG_U_ICV_IAR1: isize = 16232;
pub const REG_ISR_EL1: isize = 16240;
pub const REG_FEAT_NTLBPA_IMPLEMENTED: isize = 16248;
pub const REG_FAR_EL1: isize = 16256;
pub const REG_RVBAR_EL1: isize = 16264;
pub const REG_U_CNTKCTL: isize = 16272;
pub const REG_TPIDR_EL3: isize = 16280;
pub const REG_ID_PFR0_EL1: isize = 16288;
pub const REG_FEAT_RPRES_IMPLEMENTED: isize = 16296;
pub const REG_U_PRRR_NS: isize = 16304;
pub const REG_DEBUGHALT_EXCEPTIONCATCH: isize = 1168;
pub const REG_FEAT_TCR2_IMPLEMENTED: isize = 16312;
pub const REG_U_ICC_IAR0: isize = 16320;
pub const REG_SPECOUNTERPOSTRANSLATIONLATENCY: isize = 1072;
pub const REG_FEAT_SHA1_IMPLEMENTED: isize = 16328;
pub const REG_FEAT_AA32HPD_IMPLEMENTED: isize = 16336;
pub const REG_PMU_EVENT_L3D_CACHE_HITM_RD: isize = 304;
pub const REG_FEAT_LSE2_IMPLEMENTED: isize = 16344;
pub const REG_CFG_RMR_AA64: isize = 16352;
pub const REG_MEMHINT_NO: isize = 488;
pub const REG_U_PMCNTENSET: isize = 16360;
pub const REG_ICC_SRE_EL2: isize = 16368;
pub const REG_HFGWTR2_EL2: isize = 16376;
pub const REG_PMPIDR3: isize = 16384;
pub const REG_U_DBGBVR: isize = 16392;
pub const REG_SCTLR_S: isize = 16456;
pub const REG_FEAT_FHM_IMPLEMENTED: isize = 16464;
pub const REG_EDWAR: isize = 16472;
pub const REG_R1: isize = 16480;
pub const REG_U_CONTEXTIDR_NS: isize = 16488;
pub const REG_DEBUGHALT_OSUNLOCKCATCH: isize = 1128;
pub const REG_AFSR0_EL1: isize = 16496;
pub const REG_RCWSMASK_EL1: isize = 16504;
pub const REG_SCXTNUM_EL2: isize = 16520;
pub const REG_ERXPFGCDN_EL1: isize = 16528;
pub const REG_BRBFCR_EL1: isize = 16536;
pub const REG_U__IMPDEF_TG0: isize = 16544;
pub const REG_SPMSELR_EL0: isize = 16552;
pub const REG_U_PMUSERENR: isize = 16560;
pub const REG_FCSEIDR: isize = 16568;
pub const REG_GICD_SETSPI_SR: isize = 16576;
pub const REG_PMU_EVENT_L1D_CACHE_LMISS_RD: isize = 136;
pub const REG_CFG_MPAM_FRAC_V0P1: isize = 1296;
pub const REG_DACR32_EL2: isize = 16584;
pub const REG_HFGRTR_EL2: isize = 16592;
pub const REG_TPIDRURO_S: isize = 16600;
pub const REG_FEAT_DEBUGV8P9_IMPLEMENTED: isize = 16608;
pub const REG_FEAT_MEC_IMPLEMENTED: isize = 16616;
pub const REG_MPAM0_EL1: isize = 16624;
pub const REG_FEAT_TLBIOS_IMPLEMENTED: isize = 16632;
pub const REG_SPEADDRPOSDATAVIRTUAL: isize = 1032;
pub const REG_CNTHP_CVAL_EL2: isize = 16640;
pub const REG_GPCCR_EL3: isize = 16648;
pub const REG_AFSR0_EL3: isize = 16656;
pub const REG_AMEVCNTVOFF1_EL2: isize = 16664;
pub const REG_U_AMUSERENR: isize = 16792;
pub const REG_U_ICC_EOIR1: isize = 16800;
pub const REG_EDCIDR3: isize = 16808;
pub const REG_DBGDIDR: isize = 16816;
pub const REG_FEAT_LVA_IMPLEMENTED: isize = 16824;
pub const REG_MDCCSR_EL0: isize = 16832;
pub const REG_CPTR_EL3: isize = 16840;
pub const REG_CNTP_CVAL_S: isize = 16848;
pub const REG_GPTRANGE_32MB: isize = 944;
pub const REG_AIDR_EL1: isize = 16856;
pub const REG_U_AMCNTENSET0: isize = 16864;
pub const REG_U_DACR_NS: isize = 16872;
pub const REG_EDLAR: isize = 16880;
pub const REG_FEAT_AA64EL1_IMPLEMENTED: isize = 16888;
pub const REG_U_ICH_AP0R: isize = 16896;
pub const REG_ERRNFR: isize = 16912;
pub const REG_R15: isize = 16944;
pub const REG_U_PMCCFILTR: isize = 16952;
pub const REG_PMCFGR: isize = 16960;
pub const REG_PSTATE: isize = 16968;
pub const REG_PMU_EVENT_L2D_LFB_HIT_RD: isize = 328;
pub const REG_EDDEVARCH: isize = 17000;
pub const REG_U_ID_ISAR1: isize = 17008;
pub const REG_TCMTR: isize = 17016;
pub const REG_EDHSR: isize = 17024;
pub const REG_U__CNTREADBASE: isize = 17032;
pub const REG_ICC_IGRPEN1_EL1_NS: isize = 17040;
pub const REG_GICH_VTR: isize = 17048;
pub const REG_GICD_SGIR: isize = 17056;
pub const REG_FEAT_ADVSIMD_IMPLEMENTED: isize = 17064;
pub const REG_SCTLR_EL3: isize = 17072;
pub const REG_U_ERXMISC3: isize = 17080;
pub const REG_U_ELR_HYP: isize = 17088;
pub const REG_U_PMSELR: isize = 17096;
pub const REG_R19: isize = 17104;
pub const REG_CNTHVS_TVAL_EL2: isize = 17112;
pub const REG_AIFSR_S: isize = 17120;
pub const REG_U_PMCEID2: isize = 17128;
pub const REG_SPESAMPLECLASS: isize = 17136;
pub const REG_NIDEN: isize = 17144;
pub const REG_VBAR_EL1: isize = 17152;
pub const REG_FEAT_ECBHB_IMPLEMENTED: isize = 17160;
pub const REG_ICC_HPPIR1_EL1: isize = 17168;
pub const REG_ICH_ELRSR_EL2: isize = 17176;
pub const REG_FEAT_MOPS_IMPLEMENTED: isize = 17184;
pub const REG_CLIDR_EL1: isize = 17192;
pub const REG_CNTV_CTL_EL0: isize = 17200;
pub const REG_U_MAIR1_NS: isize = 17208;
pub const REG_FEAT_SPE_IMPLEMENTED: isize = 17216;
pub const REG_ELR_EL2: isize = 17224;
pub const REG_DBGDTRTX_EL0: isize = 17232;
pub const REG_TPIDRRO_EL0: isize = 17240;
pub const REG_ICC_EOIR1_EL1: isize = 17248;
pub const REG_PMCIDR0: isize = 17256;
pub const REG_FEAT_SME_I16I64_IMPLEMENTED: isize = 17264;
pub const REG_MEMATTR_WB: isize = 480;
pub const REG_FEAT_FP_IMPLEMENTED: isize = 17272;
pub const REG_FEAT_MTE_ASYM_FAULT_IMPLEMENTED: isize = 17280;
pub const REG_FEAT_SPE_CRR_IMPLEMENTED: isize = 17288;
pub const REG_FEAT_TRBE_IMPLEMENTED: isize = 17296;
pub const REG_SMCR_EL1: isize = 17304;
pub const REG_MPAMVPMV_EL2: isize = 17312;
pub const REG_U_VDISR: isize = 17320;
pub const REG_ICC_BPR0_EL1: isize = 17328;
pub const REG_ID_ISAR0_EL1: isize = 17336;
pub const REG_ICC_BPR1_EL1_NS: isize = 17344;
pub const REG_ICH_VTR_EL2: isize = 17352;
pub const REG_HDFGWTR_EL2: isize = 17360;
pub const REG_FEAT_MTE_PERM_IMPLEMENTED: isize = 17368;
pub const REG_MPIDR_EL1: isize = 17376;
pub const REG_DEBUGHALT_RESETCATCH: isize = 1136;
pub const REG_PMPCSR: isize = 17384;
pub const REG_U_ICC_SGI0R: isize = 17392;
pub const REG_AMEVCNTVOFF0_EL2: isize = 17400;
pub const REG_ERXFR_EL1: isize = 17528;
pub const REG_GICR_VPENDBASER: isize = 17536;
pub const REG_U_ICC_BPR1_NS: isize = 17544;
pub const REG_SPESAMPLEDATASOURCE: isize = 17552;
pub const REG_GICM_SETSPI_NSR: isize = 17560;
pub const REG_NUM_GIC_LIST_REGS: isize = 17568;
pub const REG_U_PMINTENCLR: isize = 17584;
pub const REG_GICM_TYPER: isize = 17592;
pub const REG_FEAT_DEBUGV8P8_IMPLEMENTED: isize = 17600;
pub const REG_MPAMHCR_EL2: isize = 17608;
pub const REG_PMU_EVENT_SAMPLE_FEED: isize = 160;
pub const REG_SPESAMPLETIMESTAMPVALID: isize = 17616;
pub const REG_FEAT_CMOW_IMPLEMENTED: isize = 17624;
pub const REG_FEAT_ETEV1P3_IMPLEMENTED: isize = 17632;
pub const REG_V8AP1_IMPLEMENTED: isize = 17640;
pub const REG_U_DBGDSCREXT: isize = 17648;
pub const REG_MAIR_EL3: isize = 17656;
pub const REG_FINAL_LEVEL: isize = 800;
pub const REG_HDFGWTR2_EL2: isize = 17664;
pub const REG_FEAT_ABLE_IMPLEMENTED: isize = 17672;
pub const REG_GICV_IAR: isize = 17680;
pub const REG_U_PMOVS: isize = 17688;
pub const REG_CTIPIDR2: isize = 17696;
pub const REG_V8AP8_IMPLEMENTED: isize = 17704;
pub const REG_EL1: isize = 440;
pub const REG_FEAT_RME_IMPLEMENTED: isize = 17712;
pub const REG_U_DBGDRAR: isize = 17720;
pub const REG_GITS_PARTIDR: isize = 17728;
pub const REG_U_P: isize = 17736;
pub const REG_PMU_EVENT_CHAIN: isize = 80;
pub const REG_GCSPR_EL3: isize = 18248;
pub const REG_FEAT_ASMV8P2_IMPLEMENTED: isize = 18256;
pub const REG_U__VLPI_BASE: isize = 18264;
pub const REG_BRBCR_EL2: isize = 18272;
pub const REG_U__UNPRED_TSIZE_ABORTS: isize = 18280;
pub const REG_CNTCR: isize = 18288;
pub const REG_CNTHP_TVAL_EL2: isize = 18296;
pub const REG_U_ICV_HPPIR1: isize = 18304;
pub const REG_ELR_EL1: isize = 18312;
pub const REG_R4: isize = 18320;
pub const REG_U__ICACHE_CCSIDR_RESET: isize = 18328;
pub const REG_U_HSCTLR: isize = 18384;
pub const REG_ICC_CTLR_EL1_S: isize = 18392;
pub const REG_U_TPIDRURO_NS: isize = 18400;
pub const REG_U_ERXADDR2: isize = 18408;
pub const REG_MDSELR_EL1: isize = 18416;
pub const REG_DEFAULT_MECID: isize = 832;
pub const REG_SPSR_UND: isize = 18424;
pub const REG_TTBR1_EL2: isize = 18432;
pub const REG_U_VTTBR_EL2: isize = 18448;
pub const REG_SPESAMPLECOUNTER: isize = 18464;
pub const REG_GICH_VMCR: isize = 18976;
pub const REG_CTILAR: isize = 18984;
pub const REG_PMDEVTYPE: isize = 18992;
pub const REG_GICC_EOIR: isize = 19000;
pub const REG_GPTRANGE_16KB: isize = 920;
pub const REG_ID_ISAR4_EL1: isize = 19008;
pub const REG_FEAT_CSV2_2_IMPLEMENTED: isize = 19016;
pub const REG_FEAT_SYSREG128_IMPLEMENTED: isize = 19024;
pub const REG_R9: isize = 19032;
pub const REG_SPESAMPLEOPTYPE: isize = 19040;
pub const REG_CTIPIDR0: isize = 19048;
pub const REG_CTR_EL0: isize = 19056;
pub const REG_SPMACCESSR_EL2: isize = 19064;
pub const REG_FEAT_CSV2_3_IMPLEMENTED: isize = 19072;
pub const REG_FEAT_SPMU_IMPLEMENTED: isize = 19080;
pub const REG_U__TLB_ENABLED: isize = 19088;
pub const REG_U_VBAR_NS: isize = 19096;
pub const REG_MAIR2_EL3: isize = 19104;
pub const REG_R14: isize = 19112;
pub const REG_TTBR1_S: isize = 19120;
pub const REG_V8AP5_IMPLEMENTED: isize = 19128;
pub const REG_PMSELR_EL0: isize = 19136;
pub const REG_DEBUGHALT_SOFTWAREACCESS: isize = 1160;
pub const REG_HDFGRTR_EL2: isize = 19144;
pub const REG_AMEVTYPER1_EL0: isize = 19152;
pub const REG_MEMATTR_NC: isize = 464;
pub const REG_CNTHV_CTL_EL2: isize = 19280;
pub const REG_ICC_RPR_EL1: isize = 19288;
pub const REG_AMDEVARCH: isize = 19296;
pub const REG_GCSCR_EL2: isize = 19304;
pub const REG_EDPCSR: isize = 19312;
pub const REG_U_ERXFR2: isize = 19320;
pub const REG_VDISR_EL2: isize = 19328;
pub const REG_FEAT_MTE_ASYNC_IMPLEMENTED: isize = 19336;
pub const REG_U_CNTP_CVAL_NS: isize = 19344;
pub const REG_DBGDEVID2: isize = 19352;
pub const REG_NUM_WATCHPOINTS: isize = 19360;
pub const REG_CNTSR: isize = 19376;
pub const REG_AMCIDR1: isize = 19384;
pub const REG_DBGWVR_EL1: isize = 19392;
pub const REG_ICH_AP1R_EL2: isize = 19904;
pub const REG_FEAT_FCMA_IMPLEMENTED: isize = 19936;
pub const REG_FEAT_GICV3P1_IMPLEMENTED: isize = 19944;
pub const REG_DEBUGHALT_STEP_NORMAL: isize = 1112;
pub const REG_U__SYNCABORTONTTWCACHE: isize = 19952;
pub const REG_FEAT_S1PIE_IMPLEMENTED: isize = 19960;
pub const REG_OSECCR_EL1: isize = 19968;
pub const REG_GPT_NOACCESS: isize = 840;
pub const REG_PMU_EVENT_SAMPLE_FEED_BR: isize = 232;
pub const REG_FEAT_ETMV4P5_IMPLEMENTED: isize = 19976;
pub const REG_PRRR_S: isize = 19984;
pub const REG_ICC_MSRE: isize = 19992;
pub const REG_U_ERXMISC5: isize = 20000;
pub const REG_PFAR_EL2: isize = 20008;
pub const REG_CTICIDR1: isize = 20016;
pub const REG_TTBR1_NS: isize = 20024;
pub const REG_SPSR_ABT: isize = 20032;
pub const REG_U_ICV_IAR0: isize = 20040;
pub const REG_MAIR2_EL1: isize = 20048;
pub const REG_FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED: isize = 20056;
pub const REG_R21: isize = 20064;
pub const REG_MDCCINT_EL1: isize = 20072;
pub const REG_AMCIDR2: isize = 20080;
pub const REG_U_ICH_HCR: isize = 20088;
pub const REG_PMU_EVENT_BRB_FILTRATE: isize = 216;
pub const REG_RGSR_EL1: isize = 20096;
pub const REG_U_MIDR: isize = 20104;
pub const REG_ID_AA64DFR0_EL1: isize = 20112;
pub const REG_U_ID_PFR1: isize = 20120;
pub const REG_ELR_EL3: isize = 20128;
pub const REG_U__SYNCABORTONSOREAD: isize = 20136;
pub const REG_INSTRUCTION_COUNTER_ID: isize = 8;
pub const REG_ID_AA64AFR1_EL1: isize = 20144;
pub const REG_FEAT_AA64EL0_IMPLEMENTED: isize = 20152;
pub const REG_FEAT_EBEP_IMPLEMENTED: isize = 20168;
pub const REG_GPTRANGE_64GB: isize = 976;
pub const REG_SPESAMPLECONTEXTEL1VALID: isize = 20160;
pub const REG_EDECR: isize = 20176;
pub const REG_GICR_VPROPBASER: isize = 20184;
pub const REG_AMAIR1_S: isize = 20208;
pub const REG_U_CSSELR_NS: isize = 20192;
pub const REG_U_MVFR0: isize = 20200;
pub const REG_PMU_EVENT_SAMPLE_FEED_LAT: isize = 272;
pub const REG_ID_MMFR5_EL1: isize = 20216;
pub const REG_PMCIDR3: isize = 20224;
pub const REG_U_DBGCLAIMCLR: isize = 20232;
pub const REG_U_ADFSR_NS: isize = 20240;
pub const REG_V8AP6_IMPLEMENTED: isize = 20248;
pub const REG_PMU_EVENT_L3D_LFB_HIT_RD: isize = 336;
pub const REG_U_HPFAR: isize = 20256;
pub const REG_EDPIDR0: isize = 20264;
pub const REG_U_DBGOSLSR: isize = 20272;
pub const REG_PIRE0_EL1: isize = 20280;
pub const REG_FEAT_LRCPC3_IMPLEMENTED: isize = 20288;
pub const REG_PMU_EVENT_SAMPLE_FEED_EVENT: isize = 264;
pub const REG_FEAT_SVE_AES_IMPLEMENTED: isize = 20296;
pub const REG_SPSR_EL3: isize = 20304;
pub const REG_GICM_CLRSPI_SR: isize = 20312;
pub const REG_U__SYNCABORTONWRITENORMCACHE: isize = 20320;
pub const REG_CP15SDISABLE2: isize = 20328;
pub const REG_FEAT_CRC32_IMPLEMENTED: isize = 20336;
pub const REG_FEAT_TTST_IMPLEMENTED: isize = 20344;
pub const REG_TTBCR2_S: isize = 20352;
pub const REG_U_ICC_IGRPEN0: isize = 20360;
pub const REG_R20: isize = 20368;
pub const REG_CNTPS_CTL_EL1: isize = 20376;
pub const REG_U_HTPIDR: isize = 20384;
pub const REG_GICR_PARTIDR: isize = 20392;
pub const REG_FEAT_PMUV3_EXT_IMPLEMENTED: isize = 20400;
pub const REG_R13: isize = 20408;
pub const REG_ID_DFR0_EL1: isize = 20416;
pub const REG_GICD_CLRSPI_SR: isize = 20424;
pub const REG_PMMIR_EL1: isize = 20432;
pub const REG_DBGEN: isize = 20440;
pub const REG_FEAT_IESB_IMPLEMENTED: isize = 20448;
pub const REG_FEAT_BTI_IMPLEMENTED: isize = 20456;
pub const REG_ICC_SGI1R_EL1: isize = 20464;
pub const REG_R30: isize = 20472;
pub const REG_PMBLIMITR_EL1: isize = 20480;
pub const REG_U_TPIDRPRW_NS: isize = 20488;
pub const REG_FEAT_GTG_IMPLEMENTED: isize = 20496;
pub const REG_U_CNTHV_CTL: isize = 20504;
pub const REG_GITS_MPAMIDR: isize = 20512;
pub const REG_U_DBGDTRRXINT: isize = 20520;
pub const REG_FEAT_AA32EL0_IMPLEMENTED: isize = 20528;
pub const REG_FEAT_DOUBLEFAULT_IMPLEMENTED: isize = 20536;
pub const REG_SPEADDRPOSBRANCHTARGET: isize = 1024;
pub const REG_U__ISLA_VECTOR_GPR: isize = 20544;
pub const REG_U__GICCPUINTERFACEBASE: isize = 20552;
pub const REG_RC: isize = 20560;
pub const REG_SPECOUNTERPOSTOTALLATENCY: isize = 1056;
pub const REG_VMECID_P_EL2: isize = 20600;
pub const REG_U__GIC_PENDING: isize = 20608;
pub const REG_ICC_DIR_EL1: isize = 20616;
pub const REG_GPTBR_EL3: isize = 20624;
pub const REG_U_ICC_EOIR0: isize = 20632;
pub const REG_U_MAIR0_S: isize = 20640;
pub const REG_U_ICC_SRE_S: isize = 20648;
pub const REG_FEAT_SPECRES2_IMPLEMENTED: isize = 20656;
pub const REG_U__MOPS_FORWARD_COPY: isize = 20664;
pub const REG_VMPIDR_EL2: isize = 20672;
pub const REG_U_ICV_BPR0: isize = 20680;
pub const REG_FEAT_PMUV3_SS_IMPLEMENTED: isize = 20688;
pub const REG_FPSR: isize = 20696;
pub const REG_U_HIFAR: isize = 20704;
pub const REG_U_ICV_EOIR1: isize = 20712;
pub const REG_U_HMAIR1: isize = 20720;
pub const REG_SPESAMPLEPREVIOUSBRANCHADDRESSVALID: isize = 20728;
pub const REG_BRANCHTYPETAKEN: isize = 20736;
pub const REG_ICV_AP1R_EL1: isize = 20744;
pub const REG_AMAIR2_EL3: isize = 20776;
pub const REG_SCTLR_EL2: isize = 20784;
pub const REG_VPIDR_EL2: isize = 20792;
pub const REG_CNTP_CVAL_EL0: isize = 20800;
pub const REG_U_ICV_AP0R: isize = 20808;
pub const REG_NUM_BRBE_RECORDS: isize = 20824;
pub const REG_GCSPR_EL0: isize = 20840;
pub const REG_U__HAS_SVE_EXTENDED_BF16: isize = 20848;
pub const REG_GPT_NONSECURE: isize = 880;
pub const REG_V8AP2_IMPLEMENTED: isize = 20864;
pub const REG_U_ACTLR2_NS: isize = 20872;
pub const REG_SPESAMPLECONTEXTEL2: isize = 20880;
pub const REG_AMEVTYPER0_EL0: isize = 20888;
pub const REG_SCR: isize = 20920;
pub const REG_MAIR2_EL2: isize = 20928;
pub const REG_GICC_STATUSR: isize = 20936;
pub const REG_ID_AA64MMFR4_EL1: isize = 20944;
pub const REG_BTYPECOMPATIBLE: isize = 20952;
pub const REG_FEAT_S2PIE_IMPLEMENTED: isize = 20960;
pub const REG_U_DBGOSDLR: isize = 20968;
pub const REG_DBGAUTHSTATUS_EL1: isize = 20976;
pub const REG_MPAMVPM7_EL2: isize = 20984;
pub const REG_ICH_HCR_EL2: isize = 20992;
pub const REG_GICV_DIR: isize = 21000;
pub const REG_LST_64B: isize = 1208;
pub const REG_FEAT_EBF16_IMPLEMENTED: isize = 21008;
pub const REG_PMCR_EL0: isize = 21016;
pub const REG_FPEXC32_EL2: isize = 21024;
pub const REG_ICV_HPPIR1_EL1: isize = 21032;
pub const REG_FEAT_FP16_IMPLEMENTED: isize = 21040;
pub const REG_U_TRFCR: isize = 21048;
pub const REG_U__EMPAM_SDEFLT_IMPLEMENTED: isize = 21056;
pub const REG_CNTHV_TVAL_EL2: isize = 21064;
pub const REG_PMSCR_EL1: isize = 21072;
pub const REG_ID_AFR0_EL1: isize = 21080;
pub const REG_DBGCLAIMCLR_EL1: isize = 21088;
pub const REG_APIAKEYLO_EL1: isize = 21096;
pub const REG_FEAT_UAO_IMPLEMENTED: isize = 21104;
pub const REG_SDER32_EL2: isize = 21112;
pub const REG_EDDFR1: isize = 21120;
pub const REG_FEAT_GICV3_NMI_IMPLEMENTED: isize = 21128;
pub const REG_SPSR_MON: isize = 21136;
pub const REG_U__MPAM_HAS_ALTSP: isize = 21144;
pub const REG_PMU_EVENT_L2D_CACHE_LMISS_RD: isize = 184;
pub const REG_ICV_AP0R_EL1: isize = 21152;
pub const REG_SCXTNUM_EL3: isize = 21184;
pub const REG_U__MPAM_VPMR_MAX: isize = 21192;
pub const REG_R18: isize = 21200;
pub const REG_U__SGI_BASE: isize = 21208;
pub const REG_R0: isize = 21216;
pub const REG_V9AP3_IMPLEMENTED: isize = 21224;
pub const REG_U__APPLY_EFFECTIVE_SHAREABILITY: isize = 21232;
pub const REG_RECORDS_SRC: isize = 21240;
pub const REG_U_DFAR_S: isize = 21752;
pub const REG_HAFGRTR_EL2: isize = 21760;
pub const REG_U__SYNCABORTONREADNORMCACHE: isize = 21768;
pub const REG_LOREA_EL1: isize = 21776;
pub const REG_AMAIR2_EL1: isize = 21784;
pub const REG_ERRSELR_EL1: isize = 21792;
pub const REG_ICC_MCTLR: isize = 21800;
pub const REG_U__MPAM_PARTID_MAX: isize = 21808;
pub const REG_FEAT_RDM_IMPLEMENTED: isize = 21816;
pub const REG_U__SYNCABORTONDEVICEWRITE: isize = 21824;
pub const REG_FEAT_ETMV4P6_IMPLEMENTED: isize = 21832;
pub const REG_R27: isize = 21840;
pub const REG_U_DORMANTCTLREG: isize = 21848;
pub const REG_U_ID_MMFR0: isize = 21856;
pub const REG_U_ERXADDR: isize = 21864;
pub const REG_EDITCTRL: isize = 21872;
pub const REG_U__IGNORE_RVBAR_IN_AARCH32: isize = 21880;
pub const REG_CNTP_CTL_S: isize = 21888;
pub const REG_FEAT_EL2_IMPLEMENTED: isize = 21896;
pub const REG_CTICONTROL: isize = 21904;
pub const REG_GCSPR_EL1: isize = 21912;
pub const REG_U__CURRENTCOND: isize = 21920;
pub const REG_BRBSRCINJ_EL1: isize = 21928;
pub const REG_CONTEXTIDR_S: isize = 21936;
pub const REG_GITS_STATUSR: isize = 21944;
pub const REG_U_HCR2: isize = 21952;
pub const REG_AMCIDR0: isize = 21960;
pub const REG_EVENTREGISTER: isize = 21968;
pub const REG_FEAT_ETS2_IMPLEMENTED: isize = 21976;
pub const REG_U_DBGPRCR: isize = 21984;
pub const REG_U_DLR: isize = 21992;
pub const REG_FEAT_SME_IMPLEMENTED: isize = 22000;
pub const REG_U__SPE_LFSR: isize = 22008;
pub const REG_MEMHINT_RWA: isize = 512;
pub const REG_CNTSCR: isize = 22016;
pub const REG_U_AMEVCNTR0_EL0: isize = 22024;
pub const REG_CNTKCTL_EL1: isize = 22056;
pub const REG_CFG_ID_AA64PFR0_EL1_EL2: isize = 1240;
pub const REG_U__ISB_IS_BRANCH: isize = 22064;
pub const REG_GICR_MPAMIDR: isize = 22072;
pub const REG_LORID_EL1: isize = 22080;
pub const REG_U_ICC_SRE_NS: isize = 22088;
pub const REG_U_ICV_IGRPEN0: isize = 22096;
pub const REG_FEAT_DPB2_IMPLEMENTED: isize = 22104;
pub const REG_ID_AA64MMFR3_EL1: isize = 22112;
pub const REG_BRBINF_EL1: isize = 22120;
pub const REG_GICH_ELRSR: isize = 22376;
pub const REG_GICH_MISR: isize = 22384;
pub const REG_TCR_EL1: isize = 22392;
pub const REG_CNTVOFF_EL2: isize = 22400;
pub const REG_VTTBR: isize = 22408;
pub const REG_UART_BASE: isize = 1352;
pub const REG_SPESAMPLEINFLIGHT: isize = 22416;
pub const REG_REVIDR_EL1: isize = 22424;
pub const REG_U_DBGBXVR: isize = 22432;
pub const REG_TPIDRURW_S: isize = 22496;
pub const REG_AMCIDR3: isize = 22504;
pub const REG_PMU_EVENT_L1D_CACHE: isize = 32;
pub const REG_FEAT_XS_IMPLEMENTED: isize = 22512;
pub const REG_MPAMVPM4_EL2: isize = 22520;
pub const REG_HCRX_EL2: isize = 22528;
pub const REG_OSDTRTX_EL1: isize = 22536;
pub const REG_MPAMVPM6_EL2: isize = 22544;
pub const REG_ID_AA64PFR1_EL1: isize = 22552;
pub const REG_ERXPFGF_EL1: isize = 22560;
pub const REG_FEAT_NV2_IMPLEMENTED: isize = 22568;
pub const REG_MAX_VL: isize = 808;
pub const REG_FEAT_HAFDBS_IMPLEMENTED: isize = 22576;
pub const REG_FEAT_PAUTH_IMPLEMENTED: isize = 22584;
pub const REG_ICH_EISR_EL2: isize = 22592;
pub const REG_ERXMISC0_EL1: isize = 22600;
pub const REG_JOSCR: isize = 22608;
pub const REG_AMAIR2_EL2: isize = 22616;
pub const REG_PMAUTHSTATUS: isize = 22624;
pub const REG_PMCNTENCLR_EL0: isize = 22632;
pub const REG_U__LAST_CYCLE_COUNT: isize = 22640;
pub const REG_FEAT_F64MM_IMPLEMENTED: isize = 22656;
pub const REG_FEAT_PAUTH2_IMPLEMENTED: isize = 22664;
pub const REG_CNTHPS_CVAL_EL2: isize = 22672;
pub const REG_U__TRCCLAIM_TAGS: isize = 22680;
pub const REG_AFSR1_EL1: isize = 22688;
pub const REG_U_AMCNTENCLR1: isize = 22696;
pub const REG_GICD_SETSPI_NSR: isize = 22704;
pub const REG_MDCR_EL3: isize = 22712;
pub const REG_U_VMPIDR: isize = 22720;
pub const REG_GICV_AHPPIR: isize = 22728;
pub const REG_DEBUGHALT_BREAKPOINT: isize = 1096;
pub const REG_AMPIDR0: isize = 22736;
pub const REG_PMSEVFR_EL1: isize = 22744;
pub const REG_SPEMAXRECORDSIZE: isize = 1008;
pub const REG_V8AP7_IMPLEMENTED: isize = 22752;
pub const REG_U__INSTRUCTIONSTEP: isize = 22760;
pub const REG_FEAT_SVE2P1_IMPLEMENTED: isize = 22768;
pub const REG_NUM_BREAKPOINTS: isize = 22776;
pub const REG_AMCNTENCLR0_EL0: isize = 22792;
pub const REG_EDDFR: isize = 22800;
pub const REG_U__SPE_LFSR_INITIALIZED: isize = 22808;
pub const REG_VBAR_EL2: isize = 22816;
pub const REG_VSTTBR_EL2: isize = 22824;
pub const REG_EDVIDSR: isize = 22832;
pub const REG_PMZR_EL0: isize = 22840;
pub const REG_ADFSR_S: isize = 22848;
pub const REG_U_ID_PFR2: isize = 22856;
pub const REG_U_ICC_AP1R_S: isize = 22864;
pub const REG_PMU_EVENT_L1D_TLB: isize = 96;
pub const REG_U_ICC_SGI1R: isize = 22880;
pub const REG_U_CNTFRQ: isize = 22888;
pub const REG_CSSELR_EL1: isize = 22896;
pub const REG_DEBUGHALT_WATCHPOINT: isize = 1144;
pub const REG_MECID_P0_EL2: isize = 22904;
pub const REG_CNTFRQ_EL0: isize = 22912;
pub const REG_MAIR_EL1: isize = 22920;
pub const REG_R5: isize = 22928;
pub const REG_U_HRMR: isize = 22936;
pub const REG_MAX_PL: isize = 816;
pub const REG_U_HACTLR2: isize = 22944;
pub const REG_ESR_EL1: isize = 22952;
pub const REG_ICC_SRE_EL1_NS: isize = 22960;
pub const REG_U_PAR_EL1: isize = 22968;
pub const REG_R3: isize = 22984;
pub const REG_SHOULDADVANCESS: isize = 22992;
pub const REG_FEAT_SME_F64F64_IMPLEMENTED: isize = 23000;
pub const REG_BRBTS_EL1: isize = 23008;
pub const REG_U_ICV_AP1R: isize = 23016;
pub const REG_FEAT_MTE4_IMPLEMENTED: isize = 23032;
pub const REG_U_DBGDSCRINT: isize = 23040;
pub const REG_U_DSPSR2: isize = 23048;
pub const REG_GPTRANGE_2MB: isize = 936;
pub const REG_SPESAMPLECOUNTERVALID: isize = 23056;
pub const REG_U_DISR: isize = 23088;
pub const REG_R26: isize = 23096;
pub const REG_VBAR_EL3: isize = 23104;
pub const REG_MECID_A1_EL2: isize = 23112;
pub const REG_RMR_EL2: isize = 23120;
pub const REG_U_ID_DFR1: isize = 23128;
pub const REG_U_ICV_PMR: isize = 23136;
pub const REG_U_CNTV_CVAL: isize = 23144;
pub const REG_R10: isize = 23152;
pub const REG_FEAT_BF16_IMPLEMENTED: isize = 23160;
pub const REG_FEAT_THE_IMPLEMENTED: isize = 23168;
pub const REG_CFG_PMCR_IDCODE: isize = 1256;
pub const REG_TTBR0_EL3: isize = 23176;
pub const REG_ICC_IAR1_EL1: isize = 23184;
pub const REG_R16: isize = 23192;
pub const REG_DOMAIN_CLIENT: isize = 792;
pub const REG_U_PMOVSSET: isize = 23200;
pub const REG_U_DBGDTRTXEXT: isize = 23208;
pub const REG_CTICIDR3: isize = 23216;
pub const REG_FEAT_PMUV3_EXT64_IMPLEMENTED: isize = 23224;
pub const REG_FEAT_SEBEP_IMPLEMENTED: isize = 23232;
pub const REG_U_REVIDR: isize = 23240;
pub const REG_PMU_EVENT_L1D_CACHE_HITM_RD: isize = 288;
pub const REG_FEAT_I8MM_IMPLEMENTED: isize = 23248;
pub const REG_U__CNTEL0BASEN: isize = 23256;
pub const REG_FEAT_ETE_IMPLEMENTED: isize = 23264;
pub const REG_U__GICDISTBASE: isize = 23272;
pub const REG_CCSIDR_EL1: isize = 23280;
pub const REG_FEAT_EPAC_IMPLEMENTED: isize = 23288;
pub const REG_U_DBGWVR: isize = 23296;
pub const REG_PMU_EVENT_LL_CACHE: isize = 112;
pub const REG_U__FEAT_RPRES: isize = 23360;
pub const REG_ID_ISAR3_EL1: isize = 23368;
pub const REG_U__GMID_LOG2_BLOCK_SIZE: isize = 23376;
pub const REG_GICM_SETSPI_SR: isize = 23392;
pub const REG_GITS_SGIR: isize = 23400;
pub const REG_U__PMUBASE: isize = 23408;
pub const REG_U_VDFSR: isize = 23416;
pub const REG_PMU_EVENT_INST_RETIRED: isize = 40;
pub const REG_TPIDR_EL0: isize = 23424;
pub const REG_EDDEVID: isize = 23432;
pub const REG_GICV_EOIR: isize = 23440;
pub const REG_ICV_DIR_EL1: isize = 23448;
pub const REG_U_HTCR: isize = 23456;
pub const REG_U_PMEVTYPER: isize = 23464;
pub const REG_ERXPFGCTL_EL1: isize = 23592;
pub const REG_U_PMCEID1: isize = 23600;
pub const REG_U_AMCNTENCLR0: isize = 23608;
pub const REG_RCWMASK_EL1: isize = 23616;
pub const REG_CNTV_CVAL_EL0: isize = 23632;
pub const REG_U__CPY_MOPS_OPTION_A_SUPPORTED: isize = 23640;
pub const REG_BRBSRC_EL1: isize = 23648;
pub const REG_GITS_IIDR: isize = 23904;
pub const REG_R24: isize = 23912;
pub const REG_FEAT_CSV2_IMPLEMENTED: isize = 23920;
pub const REG_RNDR: isize = 23928;
pub const REG_U__SYNCABORTONSOWRITE: isize = 23936;
pub const REG_GICM_IIDR: isize = 23944;
pub const REG_U_ZA: isize = 23952;
pub const REG_GICD_TYPER: isize = 89488;
pub const REG_RMR_EL1: isize = 89496;
pub const REG_GICC_PMR: isize = 89504;
pub const REG_PMU_EVENT_CPU_CYCLES: isize = 64;
pub const REG_FEAT_MTE_IMPLEMENTED: isize = 89512;
pub const REG_FEAT_MPAMV0P1_IMPLEMENTED: isize = 89520;
pub const REG_U__CPYF_MOPS_OPTION_A_SUPPORTED: isize = 89528;
pub const REG_ICV_EOIR1_EL1: isize = 89536;
pub const REG_ICC_MGRPEN1: isize = 89544;
pub const REG_U_ERXCTLR2: isize = 89552;
pub const REG_PIR_EL2: isize = 89560;
pub const REG_FEAT_SPECRES_IMPLEMENTED: isize = 89568;
pub const REG_U_CNTHP_CTL: isize = 89576;
pub const REG_FEAT_TRBE_EXT_IMPLEMENTED: isize = 89584;
pub const REG_RVBAR_EL2: isize = 89592;
pub const REG_U_ID_MMFR2: isize = 89600;
pub const REG_ID_MMFR0_EL1: isize = 89608;
pub const REG_PMU_EVENT_L2D_CACHE_HITM_RD: isize = 296;
pub const REG_FEAT_XNX_IMPLEMENTED: isize = 89616;
pub const REG_PMU_EVENT_SVE_PRED_PARTIAL_SPEC: isize = 208;
pub const REG_AMAIR_EL1: isize = 89624;
pub const REG_PMUEVENTACCUMULATOR: isize = 89632;
pub const REG_SP_EL0: isize = 90128;
pub const REG_U_ICH_VMCR: isize = 90136;
pub const REG_U__MPAM_MAJOR: isize = 90144;
pub const REG_FEAT_E0PD_IMPLEMENTED: isize = 90152;
pub const REG_EDPIDR4: isize = 90160;
pub const REG_MECID_P1_EL2: isize = 90168;
pub const REG_U_DBGBCR: isize = 90176;
pub const REG_FEAT_GICV3_LEGACY_IMPLEMENTED: isize = 90240;
pub const REG_SMPRIMAP_EL2: isize = 90248;
pub const REG_U__SUPPORTED_PA_SIZE: isize = 90256;
pub const REG_SCTLR_EL1: isize = 90272;
pub const REG_U__SYNCABORTONDEVICEREAD: isize = 90280;
pub const REG_FEAT_DEBUGV8P1_IMPLEMENTED: isize = 90288;
pub const REG_GPT_TABLE: isize = 848;
pub const REG_FEAT_TME_IMPLEMENTED: isize = 90296;
pub const REG_DBGPRCR_EL1: isize = 90304;
pub const REG_ID_MMFR4_EL1: isize = 90312;
pub const REG_PMINTENSET_EL1: isize = 90320;
pub const REG_V8AP4_IMPLEMENTED: isize = 90328;
pub const REG_U_CNTHPS_CTL: isize = 90336;
pub const REG_CFG_MPAM_FRAC_V1P1: isize = 1304;
pub const REG_ICV_BPR0_EL1: isize = 90344;
pub const REG_CPTR_EL3_ESM_VALUE: isize = 90352;
pub const REG_FEAT_AFP_IMPLEMENTED: isize = 90368;
pub const REG_GITS_CWRITER: isize = 90376;
pub const REG_PMU_EVENT_SW_INCR: isize = 16;
pub const REG_DEBUGHALT_EDBGRQ: isize = 1104;
pub const REG_U_ICC_IGRPEN1_NS: isize = 90384;
pub const REG_U__MPAM_HAS_HCR: isize = 90392;
pub const REG_U__EMPAM_FORCE_NS_RAO: isize = 90400;
pub const REG_ID_ISAR6_EL1: isize = 90408;
pub const REG_SCXTNUM_EL0: isize = 90416;
pub const REG_MVFR0_EL1: isize = 90424;
pub const REG_GIC_PENDING_NONE: isize = 1384;
pub const REG_U_DFAR_NS: isize = 90432;
pub const REG_U_HACR: isize = 90440;
pub const REG_FEAT_PMUV3P8_IMPLEMENTED: isize = 90448;
pub const REG_U_DBGCLAIMSET: isize = 90456;
pub const REG_GICR_INMIR0: isize = 90464;
pub const REG_NUM_AMU_CG0_MONITORS: isize = 90472;
pub const REG_PMU_EVENT_L1D_LFB_HIT_RD: isize = 320;
pub const REG_CTIPIDR4: isize = 90488;
pub const REG_AMUSERENR_EL0: isize = 90496;
pub const REG_MPAM2_EL2: isize = 90504;
pub const REG_PMBPTR_EL1: isize = 90512;
pub const REG_U_ZT0: isize = 90520;
pub const REG_FEAT_SVE_SHA3_IMPLEMENTED: isize = 90584;
pub const REG_U_HSTR: isize = 90592;
pub const REG_ID_AA64MMFR2_EL1: isize = 90600;
pub const REG_ID_AA64ISAR0_EL1: isize = 90608;
pub const REG_U_DBGOSECCR: isize = 90616;
pub const REG_AMPIDR4: isize = 90624;
pub const REG_ICC_SGI0R_EL1: isize = 90632;
pub const REG_BRBCR_EL1: isize = 90640;
pub const REG_SPSR_EL1: isize = 90648;
pub const REG_U_PMCR: isize = 90656;
pub const REG_U_ICC_IGRPEN1_S: isize = 90664;
pub const REG_U_ICH_EISR: isize = 90672;
pub const REG_U__GIC_ACTIVE: isize = 90680;
pub const REG_ESR_EL2: isize = 90688;
pub const REG_FEAT_PAN2_IMPLEMENTED: isize = 90696;
pub const REG_SCR_EL3: isize = 90704;
pub const REG_PAR_S: isize = 90712;
pub const REG_FEAT_WFXT_IMPLEMENTED: isize = 90720;
pub const REG_RCW64_PROTECTED_BIT: isize = 1184;
pub const REG_ID_MMFR3_EL1: isize = 90728;
pub const REG_CSSELR_S: isize = 90736;
pub const REG_U_ICC_HSRE: isize = 90744;
pub const REG_CNTNSAR: isize = 90752;
pub const REG_FEAT_PMUV3_TH_IMPLEMENTED: isize = 90760;
pub const REG_FEAT_HBC_IMPLEMENTED: isize = 90768;
pub const REG_FEAT_SME_F16F16_IMPLEMENTED: isize = 90776;
pub const REG_M32_UNDEF: isize = 408;
pub const REG_PMU_EVENT_DTLB_WALK: isize = 128;
pub const REG_NUM_AMU_CG1_MONITORS: isize = 90784;
pub const REG_OSLAR_EL1: isize = 90800;
pub const REG_MECIDR_EL2: isize = 90808;
pub const REG_MVFR2_EL1: isize = 90816;
pub const REG_PMU_EVENT_INST_SPEC: isize = 72;
pub const REG_EL3: isize = 424;
pub const REG_U_PMCEID3: isize = 90824;
pub const REG_CNTP_CTL_EL0: isize = 90832;
pub const REG_FEAT_CLRBHB_IMPLEMENTED: isize = 90840;
pub const REG_FEAT_MTE2_IMPLEMENTED: isize = 90848;
pub const REG_U_PMCNTENCLR: isize = 90856;
pub const REG_MPAMVPM3_EL2: isize = 90864;
pub const REG_ID_MMFR1_EL1: isize = 90872;
pub const REG_ICV_NMIAR1_EL1: isize = 90880;
pub const REG_FEAT_SVE_B16B16_IMPLEMENTED: isize = 90888;
pub const REG_DEBUGEXCEPTION_WATCHPOINT: isize = 1336;
pub const REG_V9AP2_IMPLEMENTED: isize = 90896;
pub const REG_FEAT_FPACCOMBINE_IMPLEMENTED: isize = 90904;
pub const REG_BTYPENEXT: isize = 90912;
pub const REG_FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED: isize = 90920;
pub const REG_SMCR_EL3_LEN_VALUE: isize = 90928;
pub const REG_ID_PFR1_EL1: isize = 90944;
pub const REG_U_ERXMISC6: isize = 90952;
pub const REG_SMCR_EL3: isize = 90960;
pub const REG_SP_EL2: isize = 90968;
pub const REG_U_ICV_EOIR0: isize = 90976;
pub const REG_PMU_EVENT_DSNP_HIT_RD: isize = 280;
pub const REG_FEAT_SVE_SM4_IMPLEMENTED: isize = 90984;
pub const REG_U_CNTVOFF: isize = 90992;
pub const REG_U__MTE_IMPLEMENTED: isize = 91000;
pub const REG_CONTEXTIDR_EL2: isize = 91008;
pub const REG_SPSR_IRQ: isize = 91016;
pub const REG_U_TTBR0_EL2: isize = 91024;
pub const REG_JMCR: isize = 91040;
pub const REG_MEMHINT_WA: isize = 496;
pub const REG_ICV_IAR1_EL1: isize = 91048;
pub const REG_EL0: isize = 448;
pub const REG_U__EMPAM_FORCE_NS_IMPLEMENTED: isize = 91056;
pub const REG_U_SPSR_HYP: isize = 91064;
pub const REG_SPEMAXADDRS: isize = 992;
pub const REG_ICC_AP0R_EL1: isize = 91072;
pub const REG_GICC_RPR: isize = 91104;
pub const REG_U_HACTLR: isize = 91112;
pub const REG_GICR_ISENABLER0: isize = 91120;
pub const REG_SMPRI_EL1: isize = 91128;
pub const REG_TSTATE: isize = 91136;
pub const REG_MVBAR: isize = 100208;
pub const REG_GPTRANGE_16GB: isize = 968;
pub const REG_U__GICC_IIDR: isize = 1376;
pub const REG_CNTV_TVAL_EL0: isize = 100216;
pub const REG_MPAMVPM0_EL2: isize = 100224;
pub const REG_VARIANTIMPLEMENTED: isize = 100232;
pub const REG_U_DBGVCR: isize = 100248;
pub const REG_ID_AA64SMFR0_EL1: isize = 100256;
pub const REG_FEAT_PMULL_IMPLEMENTED: isize = 100264;
pub const REG_FEAT_PAN_IMPLEMENTED: isize = 100272;
pub const REG_GPT_SECURE: isize = 872;
pub const REG_MFAR_EL3: isize = 100280;
pub const REG_RECORDS_INF: isize = 100288;
pub const REG_CTIPIDR3: isize = 100800;
pub const REG_FEAT_FPAC_IMPLEMENTED: isize = 100808;
pub const REG_GMID_EL1: isize = 100816;
pub const REG_VSESR_EL2: isize = 100824;
pub const REG_CNTHPS_TVAL_EL2: isize = 100832;
pub const REG_NMRR_S: isize = 100840;
pub const REG_U_ID_MMFR4: isize = 100848;
pub const REG_U_ICH_VTR: isize = 100856;
pub const REG_EDDEVID1: isize = 100864;
pub const REG_PMCIDR1: isize = 100872;
pub const REG_GICR_INVALLR: isize = 100880;
pub const REG_FEAT_EDHSR_IMPLEMENTED: isize = 100888;
pub const REG_FEAT_NV_IMPLEMENTED: isize = 100896;
pub const REG_PMU_EVENT_SVE_PRED_EMPTY_SPEC: isize = 200;
pub const REG_FEAT_SYSINSTR128_IMPLEMENTED: isize = 100904;
pub const REG_CNTHP_CTL_EL2: isize = 100912;
pub const REG_APIBKEYHI_EL1: isize = 100920;
pub const REG_CNTP_TVAL_EL0: isize = 100928;
pub const REG_FEAT_S2FWB_IMPLEMENTED: isize = 100936;
pub const REG_FEAT_AA32EL2_IMPLEMENTED: isize = 100944;
pub const REG_R8: isize = 100952;
pub const REG_U_ICC_CTLR_NS: isize = 100960;
pub const REG_U_EDECCR: isize = 100968;
pub const REG_CCSIDR2_EL1: isize = 100976;
pub const REG_VMID_NONE: isize = 1080;
pub const REG_MPAMVPM5_EL2: isize = 100984;
pub const REG_HFGWTR_EL2: isize = 100992;
pub const REG_SMIDR_EL1: isize = 101000;
pub const REG_U_ERXMISC2: isize = 101008;
pub const REG_FEAT_LS64_ACCDATA_IMPLEMENTED: isize = 101016;
pub const REG_FEAT_ITE_IMPLEMENTED: isize = 101024;
pub const REG_CTIDEVARCH: isize = 101032;
pub const REG_S2POR_EL1: isize = 101040;
pub const REG_PMU_EVENT_LL_LFB_HIT_RD: isize = 344;
pub const REG_GICD_CLRSPI_NSR: isize = 101048;
pub const REG_GCSCR_EL1: isize = 101056;
pub const REG_M32_SVC: isize = 376;
pub const REG_FEAT_GCS_IMPLEMENTED: isize = 101064;
pub const REG_FEAT_DEBUGV8P4_IMPLEMENTED: isize = 101072;
pub const REG_U_TTBCR_NS: isize = 101080;
pub const REG_LORN_EL1: isize = 101088;
pub const REG_FEAT_PACQARMA3_IMPLEMENTED: isize = 101096;
pub const REG_U_RMR: isize = 101104;
pub const REG_SPECOUNTERPOSISSUELATENCY: isize = 1064;
pub const REG_FEAT_PMUV3P7_IMPLEMENTED: isize = 101112;
pub const REG_R7: isize = 101120;
pub const REG_U__EMULATOR_TERMINATION_OPCODE: isize = 101128;
pub const REG_U_PMOVSR: isize = 101144;
pub const REG_GPTRANGE_1GB: isize = 960;
pub const REG_U__MONOMORPHIZE_WRITES: isize = 101152;
pub const REG_U__EXCLUSIVEMONITORSET: isize = 101160;
pub const REG_FEAT_FLAGM_IMPLEMENTED: isize = 101168;
pub const REG_TLBTR: isize = 101176;
pub const REG_FEAT_SHA3_IMPLEMENTED: isize = 101184;
pub const REG_GPTRANGE_512GB: isize = 984;
pub const REG_FEAT_TLBIRANGE_IMPLEMENTED: isize = 101192;
pub const REG_ISWFISLEEP: isize = 101200;
pub const REG_PMSFCR_EL1: isize = 101208;
pub const REG_ICC_IGRPEN1_EL1_S: isize = 101216;
pub const REG_HDFGRTR2_EL2: isize = 101224;
pub const REG_CTIPIDR1: isize = 101232;
pub const REG_U_MPIDR: isize = 101240;
pub const REG_RECORDS_TGT: isize = 101248;
pub const REG_EDPIDR3: isize = 101760;
pub const REG_EDDEVID2: isize = 101768;
pub const REG_PMIAR_EL1: isize = 101776;
pub const REG_GICR_PROPBASER: isize = 101784;
pub const REG_V9AP4_IMPLEMENTED: isize = 101792;
pub const REG_TTBR0_S: isize = 101800;
pub const REG_GICV_CTLR: isize = 101808;
pub const REG_PMSICR_EL1: isize = 101816;
pub const REG_ID_AA64PFR0_EL1: isize = 101824;
pub const REG_FEAT_TTL_IMPLEMENTED: isize = 101832;
pub const REG_FEAT_LS64_IMPLEMENTED: isize = 101840;
pub const REG_FEAT_HPDS_IMPLEMENTED: isize = 101848;
pub const REG_V8AP9_IMPLEMENTED: isize = 101856;
pub const REG_U_DBGDTRTXINT: isize = 101864;
pub const REG_JIDR: isize = 101872;
pub const REG_DBGWFAR: isize = 101880;
pub const REG_GICV_AIAR: isize = 101888;
pub const REG_ZCR_EL1: isize = 101896;
pub const REG_FEAT_ETMV4_IMPLEMENTED: isize = 101904;
pub const REG_RMR_EL3: isize = 101912;
pub const REG_AMCNTENCLR1_EL0: isize = 101920;
pub const REG_PMEVCNTSVR_EL1: isize = 101928;
pub const REG_NUM_GIC_PRIORITY_BITS: isize = 102176;
pub const REG_U_ICV_HPPIR0: isize = 102192;
pub const REG_DEBUGEXCEPTION_VECTORCATCH: isize = 1328;
pub const REG_PMLSR: isize = 102200;
pub const REG_DCZID_EL0: isize = 102208;
pub const REG_U_ICV_IGRPEN1: isize = 102216;
pub const REG_U__DCACHE_CCSIDR_RESET: isize = 102224;
pub const REG_FEAT_RPRFM_IMPLEMENTED: isize = 102280;
pub const REG_DBGVCR32_EL2: isize = 102288;
pub const REG_CTIDEVID: isize = 102296;
pub const REG_BRBTGTINJ_EL1: isize = 102304;
pub const REG_FEAT_DOUBLELOCK_IMPLEMENTED: isize = 102312;
pub const REG_U_ID_MMFR3: isize = 102320;
pub const REG_U_SDER: isize = 102328;
pub const REG_FEAT_SM4_IMPLEMENTED: isize = 102336;
pub const REG_MPAMSM_EL1: isize = 102344;
pub const REG_M32_HYP: isize = 400;
pub const REG_FEAT_TRF_IMPLEMENTED: isize = 102352;
pub const REG_PIRE0_EL2: isize = 102360;
pub const REG_U_ICC_HPPIR1: isize = 102368;
pub const REG_EDCIDR0: isize = 102376;
pub const REG_FEAT_CNTSC_IMPLEMENTED: isize = 102384;
pub const REG_U__TRICKBOX_ENABLED: isize = 102392;
pub const REG_MEMHINT_RA: isize = 504;
pub const REG_AMPIDR3: isize = 102400;
pub const REG_GPTRANGE_512MB: isize = 952;
pub const REG_FEAT_CCIDX_IMPLEMENTED: isize = 102408;
pub const REG_U_ICC_DIR: isize = 102416;
pub const REG_PMLAR: isize = 102424;
pub const REG_GPT_ROOT: isize = 888;
pub const REG_FEAT_SM3_IMPLEMENTED: isize = 102432;
pub const REG_CFG_RVBAR: isize = 102440;
pub const REG_U_FPEXC: isize = 102448;
pub const REG_PMU_EVENT_LL_CACHE_MISS: isize = 120;
pub const REG_PMU_EVENT_L1D_CACHE_REFILL: isize = 24;
pub const REG_ICV_BPR1_EL1: isize = 102456;
pub const REG_ACCDATA_EL1: isize = 102464;
pub const REG_ERXMISC2_EL1: isize = 102472;
pub const REG_FEAT_VHE_IMPLEMENTED: isize = 102480;
pub const REG_NSACR: isize = 102488;
pub const REG_U__CTIBASE: isize = 102496;
pub const REG_CTILSR: isize = 102504;
pub const REG_U_ISR: isize = 102512;
pub const REG_INGUARDEDPAGE: isize = 102520;
pub const REG_SPEADDRPOSDATAPHYSICAL: isize = 1040;
pub const REG_ICC_BPR1_EL1_S: isize = 102528;
pub const REG_EL2: isize = 432;
pub const REG_U_ERRSELR: isize = 102536;
pub const REG_GICV_AEOIR: isize = 102544;
pub const REG_HCR_EL2: isize = 102552;
pub const REG_ZT0_LEN: isize = 824;
pub const REG_ID_ISAR2_EL1: isize = 102560;
pub const REG_MECID_RL_A_EL3: isize = 102568;
pub const REG_FEAT_EL0_IMPLEMENTED: isize = 102576;
pub const REG_DSPSR_EL0: isize = 102584;
pub const REG_FEAT_D128_IMPLEMENTED: isize = 102592;
pub const REG_U_DFSR_NS: isize = 102600;
pub const REG_GICD_STATUSR: isize = 102608;
pub const REG_FAR_EL2: isize = 102616;
pub const REG_PMUSERENR_EL0: isize = 102624;
pub const REG_FEAT_SSBS2_IMPLEMENTED: isize = 102632;
pub const REG_CFG_ID_AA64PFR0_EL1_EL3: isize = 1248;
pub const REG_U_ID_ISAR5: isize = 102640;
pub const REG_SPESAMPLECOUNTERPENDING: isize = 102648;
pub const REG_SCTLR2_EL2: isize = 102680;
pub const REG_POR_EL0: isize = 102688;
pub const REG_R12: isize = 102696;
pub const REG_FEAT_PRFMSLC_IMPLEMENTED: isize = 102704;
pub const REG_R22: isize = 102712;
pub const REG_GPT_BLOCK: isize = 856;
pub const REG_U_MVFR2: isize = 102720;
pub const REG_GICV_PMR: isize = 102728;
pub const REG_GICR_INVLPIR: isize = 102736;
pub const REG_U_ACTLR_NS: isize = 102744;
pub const REG_DEFAULTPMG: isize = 776;
pub const REG_FEAT_LOR_IMPLEMENTED: isize = 102752;
pub const REG_V9AP1_IMPLEMENTED: isize = 102760;
pub const REG_R11: isize = 102768;
pub const REG_GPT_ANY: isize = 904;
pub const REG_PMICNTSVR_EL1: isize = 102776;
pub const REG_HFGRTR2_EL2: isize = 102784;
pub const REG_U__NUM_CTX_BREAKPOINTS: isize = 102792;
pub const REG_U_ID_AFR0: isize = 102808;
pub const REG_U_CONFIGREG: isize = 102816;
pub const REG_PAR_NS: isize = 102824;
pub const REG_PMDEVID: isize = 102832;
pub const REG_PFAR_EL1: isize = 102840;
pub const REG_U_HCR: isize = 102848;
pub const REG_OSDLR_EL1: isize = 102856;
pub const REG_FEAT_SPEV1P4_IMPLEMENTED: isize = 102864;
pub const REG_FEATUREIMPL: isize = 102872;
pub const REG_ICC_HPPIR0_EL1: isize = 103136;
pub const REG_ID_AA64MMFR1_EL1: isize = 103144;
pub const REG_CNTHV_CVAL_EL2: isize = 103152;
pub const REG_ID_MMFR2_EL1: isize = 103160;
pub const REG_U_HCPTR: isize = 103168;
pub const REG_SCXTNUM_EL1: isize = 103176;
pub const REG_DBGDTRRX_EL0: isize = 103184;
pub const REG_U__SETG_MOPS_OPTION_A_SUPPORTED: isize = 103192;
pub const REG_CFG_MPAM_V0P1: isize = 1272;
pub const REG_U_DSPSR: isize = 103200;
pub const REG_EDPRCR: isize = 103208;
pub const REG_PMU_EVENT_BR_MIS_PRED_RETIRED: isize = 88;
pub const REG_DEBUGHALT_STEP_NOSYNDROME: isize = 1176;
pub const REG_FEAT_DIT_IMPLEMENTED: isize = 103216;
pub const REG_FEAT_MPAM_IMPLEMENTED: isize = 103224;
pub const REG_PMU_EVENT_SAMPLE_FEED_LD: isize = 240;
pub const REG_U_ID_ISAR0: isize = 103232;
pub const REG_AMEVCNTR1_EL0: isize = 103240;
pub const REG_U_HMAIR0: isize = 103368;
pub const REG_FEAT_AA32EL1_IMPLEMENTED: isize = 103376;
pub const REG_M32_SYSTEM: isize = 416;
pub const REG_ERXSTATUS_EL1: isize = 103384;
pub const REG_GICH_HCR: isize = 103392;
pub const REG_DFSR_S: isize = 103400;
pub const REG_M32_FIQ: isize = 360;
pub const REG_FEAT_GICV4P1_IMPLEMENTED: isize = 103408;
pub const REG_MIDR_EL1: isize = 103416;
pub const REG_DBGBVR_EL1: isize = 103424;
pub const REG_FEAT_RAS_IMPLEMENTED: isize = 103936;
pub const REG_PMSWINC_EL0: isize = 103944;
pub const REG_CNTPS_TVAL_EL1: isize = 103952;
pub const REG_M32_USER: isize = 352;
pub const REG_PMCGCR0: isize = 103960;
pub const REG_FEAT_NMI_IMPLEMENTED: isize = 103968;
pub const REG_GPTRANGE_64KB: isize = 928;
pub const REG_FEAT_LPA2_IMPLEMENTED: isize = 103976;
pub const REG_DBGWCR_EL1: isize = 103984;
pub const REG_PMICFILTR_EL0: isize = 104496;
pub const REG_FEAT_MTE3_IMPLEMENTED: isize = 104504;
pub const REG_U_AMCNTENSET1: isize = 104512;
pub const REG_U__G1_ACTIVITY_MONITOR_IMPLEMENTED: isize = 104520;
pub const REG_FEAT_SPEV1P3_IMPLEMENTED: isize = 104528;
pub const REG_GICV_BPR: isize = 104536;
pub const REG_FEAT_TTCNP_IMPLEMENTED: isize = 104544;
pub const REG_FEAT_LRCPC2_IMPLEMENTED: isize = 104552;
pub const REG_U_DBGDCCINT: isize = 104560;
pub const REG_SPESAMPLECONTEXTEL1: isize = 104568;
pub const REG_U__CNTCONTROLBASE: isize = 104576;
pub const REG_GICD_IIDR: isize = 104584;
pub const REG_PMPIDR4: isize = 104592;
pub const REG_CTIDEVID2: isize = 104600;
pub const REG_FEAT_AA32BF16_IMPLEMENTED: isize = 104608;
pub const REG_FEAT_BRBE_IMPLEMENTED: isize = 104616;
pub const REG_FEAT_AA32I8MM_IMPLEMENTED: isize = 104624;
pub const REG_PIR_EL1: isize = 104632;
pub const REG_PMOVSSET_EL0: isize = 104640;
pub const REG_MDSCR_EL1: isize = 104648;
pub const REG_FEAT_ETMV4P3_IMPLEMENTED: isize = 104656;
pub const REG_ID_AA64ZFR0_EL1: isize = 104664;
pub const REG_U__GICD_TYPER: isize = 1368;
pub const REG_U__G1_ACTIVITY_MONITOR_OFFSET_IMPLEMENTED: isize = 104672;
pub const REG_ACTLR_EL1: isize = 104680;
pub const REG_U_CLIDR: isize = 104688;
pub const REG_U__THISINSTR: isize = 104696;
pub const REG_U_CNTHVS_CTL: isize = 104704;
pub const REG_FEAT_S2POE_IMPLEMENTED: isize = 104712;
pub const REG_ID_DFR1_EL1: isize = 104720;
pub const REG_U__HAS_SPE_PSEUDO_CYCLES: isize = 104728;
pub const REG_PMU_EVENT_REMOTE_ACCESS: isize = 104;
pub const REG_FEAT_MTPMU_IMPLEMENTED: isize = 104736;
pub const REG_DBGOSLAR: isize = 104744;
pub const REG_U__EXTDEBUGBASE: isize = 104752;
pub const REG_TFSR_EL2: isize = 104760;
pub const REG_TFSR_EL3: isize = 104768;
pub const REG_PMCCNTR_EL0: isize = 104776;
pub const REG_U_DBGAUTHSTATUS: isize = 104784;
pub const REG_SHOULDADVANCEIT: isize = 104792;
pub const REG_ID_AA64DFR1_EL1: isize = 104800;
pub const REG_AMCNTENSET0_EL0: isize = 104808;
pub const REG_U_ICC_BPR1_S: isize = 104816;
pub const REG_U_ICC_PMR: isize = 104824;
pub const REG_GPTRANGE_4KB: isize = 912;
pub const REG_CTIDEVID1: isize = 104832;
pub const REG_HFGITR2_EL2: isize = 104840;
pub const REG_CYCLE_COUNTER_ID: isize = 0;
pub const REG_AMCG1IDR_EL0: isize = 104848;
pub const REG_SPESAMPLEEVENTS: isize = 104856;
pub const REG_FEAT_DOUBLEFAULT2_IMPLEMENTED: isize = 104864;
pub const REG_FAR_EL3: isize = 104872;
pub const REG_MDCR_EL2: isize = 104880;
pub const REG_PMOVSCLR_EL0: isize = 104888;
pub const REG_U__SYNCABORTONWRITENORMNONCACHE: isize = 104896;
pub const REG_MVFR1_EL1: isize = 104904;
pub const REG_TPIDR2_EL0: isize = 104912;
pub const REG_SPNIDEN: isize = 104920;
pub const REG_LST_64BV0: isize = 1216;
pub const REG_PMSCR_EL2: isize = 104928;
pub const REG_HSTR_EL2: isize = 104936;
pub const REG_CTICIDR2: isize = 104944;
pub const REG_GICV_ABPR: isize = 104952;
pub const REG_FEAT_JSCVT_IMPLEMENTED: isize = 104960;
pub const REG_FEAT_MPAMV1P1_IMPLEMENTED: isize = 104968;
pub const REG_PMU_EVENT_BR_MIS_PRED: isize = 56;
pub const REG_M32_IRQ: isize = 368;
pub const REG_FEAT_PMUV3P4_IMPLEMENTED: isize = 104976;
pub const REG_PMPIDR1: isize = 104984;
pub const REG_FEAT_GICV3_TDIR_IMPLEMENTED: isize = 104992;
pub const REG_R17: isize = 105000;
pub const REG_U_AMAIR0_NS: isize = 105008;
pub const REGISTER_NAME_MAP: &[(isize, &str)] = &[
    (0isize, "CYCLE_COUNTER_ID"),
    (8isize, "INSTRUCTION_COUNTER_ID"),
    (16isize, "PMU_EVENT_SW_INCR"),
    (24isize, "PMU_EVENT_L1D_CACHE_REFILL"),
    (32isize, "PMU_EVENT_L1D_CACHE"),
    (40isize, "PMU_EVENT_INST_RETIRED"),
    (48isize, "PMU_EVENT_EXC_TAKEN"),
    (56isize, "PMU_EVENT_BR_MIS_PRED"),
    (64isize, "PMU_EVENT_CPU_CYCLES"),
    (72isize, "PMU_EVENT_INST_SPEC"),
    (80isize, "PMU_EVENT_CHAIN"),
    (88isize, "PMU_EVENT_BR_MIS_PRED_RETIRED"),
    (96isize, "PMU_EVENT_L1D_TLB"),
    (104isize, "PMU_EVENT_REMOTE_ACCESS"),
    (112isize, "PMU_EVENT_LL_CACHE"),
    (120isize, "PMU_EVENT_LL_CACHE_MISS"),
    (128isize, "PMU_EVENT_DTLB_WALK"),
    (136isize, "PMU_EVENT_L1D_CACHE_LMISS_RD"),
    (144isize, "PMU_EVENT_L2D_CACHE_RD"),
    (152isize, "PMU_EVENT_SAMPLE_POP"),
    (160isize, "PMU_EVENT_SAMPLE_FEED"),
    (168isize, "PMU_EVENT_SAMPLE_FILTRATE"),
    (176isize, "PMU_EVENT_SAMPLE_COLLISION"),
    (184isize, "PMU_EVENT_L2D_CACHE_LMISS_RD"),
    (192isize, "PMU_EVENT_LDST_ALIGN_LAT"),
    (200isize, "PMU_EVENT_SVE_PRED_EMPTY_SPEC"),
    (208isize, "PMU_EVENT_SVE_PRED_PARTIAL_SPEC"),
    (216isize, "PMU_EVENT_BRB_FILTRATE"),
    (224isize, "PMU_EVENT_SAMPLE_WRAP"),
    (232isize, "PMU_EVENT_SAMPLE_FEED_BR"),
    (240isize, "PMU_EVENT_SAMPLE_FEED_LD"),
    (248isize, "PMU_EVENT_SAMPLE_FEED_ST"),
    (256isize, "PMU_EVENT_SAMPLE_FEED_OP"),
    (264isize, "PMU_EVENT_SAMPLE_FEED_EVENT"),
    (272isize, "PMU_EVENT_SAMPLE_FEED_LAT"),
    (280isize, "PMU_EVENT_DSNP_HIT_RD"),
    (288isize, "PMU_EVENT_L1D_CACHE_HITM_RD"),
    (296isize, "PMU_EVENT_L2D_CACHE_HITM_RD"),
    (304isize, "PMU_EVENT_L3D_CACHE_HITM_RD"),
    (312isize, "PMU_EVENT_LL_CACHE_HITM_RD"),
    (320isize, "PMU_EVENT_L1D_LFB_HIT_RD"),
    (328isize, "PMU_EVENT_L2D_LFB_HIT_RD"),
    (336isize, "PMU_EVENT_L3D_LFB_HIT_RD"),
    (344isize, "PMU_EVENT_LL_LFB_HIT_RD"),
    (352isize, "M32_User"),
    (360isize, "M32_FIQ"),
    (368isize, "M32_IRQ"),
    (376isize, "M32_Svc"),
    (384isize, "M32_Monitor"),
    (392isize, "M32_Abort"),
    (400isize, "M32_Hyp"),
    (408isize, "M32_Undef"),
    (416isize, "M32_System"),
    (424isize, "EL3"),
    (432isize, "EL2"),
    (440isize, "EL1"),
    (448isize, "EL0"),
    (456isize, "LOG2_TAG_GRANULE"),
    (464isize, "MemAttr_NC"),
    (472isize, "MemAttr_WT"),
    (480isize, "MemAttr_WB"),
    (488isize, "MemHint_No"),
    (496isize, "MemHint_WA"),
    (504isize, "MemHint_RA"),
    (512isize, "MemHint_RWA"),
    (520isize, "GPRs"),
    (768isize, "DefaultPARTID"),
    (776isize, "DefaultPMG"),
    (784isize, "Domain_NoAccess"),
    (792isize, "Domain_Client"),
    (800isize, "FINAL_LEVEL"),
    (808isize, "MAX_VL"),
    (816isize, "MAX_PL"),
    (824isize, "ZT0_LEN"),
    (832isize, "DEFAULT_MECID"),
    (840isize, "GPT_NoAccess"),
    (848isize, "GPT_Table"),
    (856isize, "GPT_Block"),
    (864isize, "GPT_Contig"),
    (872isize, "GPT_Secure"),
    (880isize, "GPT_NonSecure"),
    (888isize, "GPT_Root"),
    (896isize, "GPT_Realm"),
    (904isize, "GPT_Any"),
    (912isize, "GPTRange_4KB"),
    (920isize, "GPTRange_16KB"),
    (928isize, "GPTRange_64KB"),
    (936isize, "GPTRange_2MB"),
    (944isize, "GPTRange_32MB"),
    (952isize, "GPTRange_512MB"),
    (960isize, "GPTRange_1GB"),
    (968isize, "GPTRange_16GB"),
    (976isize, "GPTRange_64GB"),
    (984isize, "GPTRange_512GB"),
    (992isize, "SPEMaxAddrs"),
    (1000isize, "SPEMaxCounters"),
    (1008isize, "SPEMaxRecordSize"),
    (1016isize, "SPEAddrPosPCVirtual"),
    (1024isize, "SPEAddrPosBranchTarget"),
    (1032isize, "SPEAddrPosDataVirtual"),
    (1040isize, "SPEAddrPosDataPhysical"),
    (1048isize, "SPEAddrPosPrevBranchTarget"),
    (1056isize, "SPECounterPosTotalLatency"),
    (1064isize, "SPECounterPosIssueLatency"),
    (1072isize, "SPECounterPosTranslationLatency"),
    (1080isize, "VMID_NONE"),
    (1088isize, "MAX_ZERO_BLOCK_SIZE"),
    (1096isize, "DebugHalt_Breakpoint"),
    (1104isize, "DebugHalt_EDBGRQ"),
    (1112isize, "DebugHalt_Step_Normal"),
    (1120isize, "DebugHalt_Step_Exclusive"),
    (1128isize, "DebugHalt_OSUnlockCatch"),
    (1136isize, "DebugHalt_ResetCatch"),
    (1144isize, "DebugHalt_Watchpoint"),
    (1152isize, "DebugHalt_HaltInstruction"),
    (1160isize, "DebugHalt_SoftwareAccess"),
    (1168isize, "DebugHalt_ExceptionCatch"),
    (1176isize, "DebugHalt_Step_NoSyndrome"),
    (1184isize, "RCW64_PROTECTED_BIT"),
    (1192isize, "RCW128_PROTECTED_BIT"),
    (1200isize, "lst_64bv"),
    (1208isize, "lst_64b"),
    (1216isize, "lst_64bv0"),
    (1224isize, "CFG_ID_AA64PFR0_EL1_EL0"),
    (1232isize, "CFG_ID_AA64PFR0_EL1_EL1"),
    (1240isize, "CFG_ID_AA64PFR0_EL1_EL2"),
    (1248isize, "CFG_ID_AA64PFR0_EL1_EL3"),
    (1256isize, "CFG_PMCR_IDCODE"),
    (1264isize, "CFG_MPAM_none"),
    (1272isize, "CFG_MPAM_v0p1"),
    (1280isize, "CFG_MPAM_v1p1"),
    (1288isize, "CFG_MPAM_frac_none"),
    (1296isize, "CFG_MPAM_frac_v0p1"),
    (1304isize, "CFG_MPAM_frac_v1p1"),
    (1312isize, "DebugException_Breakpoint"),
    (1320isize, "DebugException_BKPT"),
    (1328isize, "DebugException_VectorCatch"),
    (1336isize, "DebugException_Watchpoint"),
    (1344isize, "TAG_GRANULE"),
    (1352isize, "UART_BASE"),
    (1360isize, "GIC_BASE"),
    (1368isize, "__GICD_TYPER"),
    (1376isize, "__GICC_IIDR"),
    (1384isize, "GIC_PENDING_NONE"),
    (1392isize, "COLD_RESET"),
    (1400isize, "ERXMISC1_EL1"),
    (1408isize, "FEAT_VMID16_IMPLEMENTED"),
    (1416isize, "v9Ap0_IMPLEMENTED"),
    (1424isize, "FEAT_SVE_PMULL128_IMPLEMENTED"),
    (1432isize, "__DBG_ROM_ADDR"),
    (1440isize, "_ERXMISC7"),
    (1448isize, "GICH_EISR"),
    (1456isize, "_VTCR"),
    (1464isize, "SCTLR2_EL3"),
    (1472isize, "ICC_CTLR_EL1_NS"),
    (1480isize, "ID_ISAR5_EL1"),
    (1488isize, "FEAT_EVT_IMPLEMENTED"),
    (1496isize, "_PMINTENSET"),
    (1504isize, "AMPIDR2"),
    (1512isize, "FEAT_EL3_IMPLEMENTED"),
    (1520isize, "PMSDSFR_EL1"),
    (1528isize, "_ICV_CTLR"),
    (1536isize, "PMVCIDSR"),
    (1544isize, "FEAT_FGT2_IMPLEMENTED"),
    (1552isize, "RLPIDEN"),
    (1560isize, "FEAT_ETEv1p2_IMPLEMENTED"),
    (1568isize, "FEAT_AES_IMPLEMENTED"),
    (1576isize, "__max_implemented_smeveclen"),
    (1592isize, "MECID_A0_EL2"),
    (1600isize, "ICC_AP1R_EL1_S"),
    (1632isize, "FEAT_SHA256_IMPLEMENTED"),
    (1640isize, "EDCIDR2"),
    (1648isize, "LORC_EL1"),
    (1656isize, "_PMEVCNTR"),
    (1784isize, "__exclusive_granule_size"),
    (1792isize, "FEAT_FGT_IMPLEMENTED"),
    (1800isize, "_Z"),
    (9992isize, "FEAT_GICv4_IMPLEMENTED"),
    (10000isize, "FEAT_SEL2_IMPLEMENTED"),
    (10008isize, "_ICH_AP1R"),
    (10024isize, "FEAT_SME2p1_IMPLEMENTED"),
    (10032isize, "__ETEBase"),
    (10040isize, "GICC_BPR"),
    (10048isize, "CONTEXTIDR_EL1"),
    (10056isize, "GICR_STATUSR"),
    (10064isize, "CNTHVS_CVAL_EL2"),
    (10072isize, "STACK_LIMIT"),
    (10080isize, "GICC_ABPR"),
    (10088isize, "_CTR"),
    (10096isize, "GICC_AIAR"),
    (10104isize, "FEAT_RASSAv1p1_IMPLEMENTED"),
    (10112isize, "ERXADDR_EL1"),
    (10120isize, "FEAT_PACQARMA5_IMPLEMENTED"),
    (10128isize, "OSLSR_EL1"),
    (10136isize, "GICR_SETLPIR"),
    (10144isize, "PMSLATFR_EL1"),
    (10152isize, "_DBGDTRRXext"),
    (10160isize, "_HDCR"),
    (10168isize, "BRBINFINJ_EL1"),
    (10176isize, "PMCEID1_EL0"),
    (10184isize, "SP_EL1"),
    (10192isize, "CP15SDISABLE"),
    (10200isize, "ICC_SRE_EL3"),
    (10208isize, "FEAT_HPMN0_IMPLEMENTED"),
    (10216isize, "GCSPR_EL2"),
    (10224isize, "_ERXMISC1"),
    (10232isize, "GICV_RPR"),
    (10240isize, "ICH_LR_EL2"),
    (10368isize, "__highest_el_aarch32"),
    (10376isize, "SMCR_EL2"),
    (10384isize, "SPERecordSize"),
    (10400isize, "FEAT_RNG_TRAP_IMPLEMENTED"),
    (10408isize, "FEAT_DoPD_IMPLEMENTED"),
    (10416isize, "ERXCTLR_EL1"),
    (10424isize, "__cycle_count"),
    (10440isize, "PMUACR_EL1"),
    (10448isize, "_CNTV_CTL"),
    (10456isize, "_ICC_CTLR_S"),
    (10464isize, "FEAT_HAFT_IMPLEMENTED"),
    (10472isize, "FEAT_PMUv3_EXT32_IMPLEMENTED"),
    (10480isize, "ACTLR2_S"),
    (10488isize, "_ID_MMFR1"),
    (10496isize, "GICD_CTLR"),
    (10504isize, "CNTHPS_CTL_EL2"),
    (10512isize, "AMCFGR_EL0"),
    (10520isize, "PMCIDR2"),
    (10528isize, "SPESampleInstIsNV2"),
    (10536isize, "VBAR_S"),
    (10544isize, "MAIR_EL2"),
    (10552isize, "FEAT_PACIMP_IMPLEMENTED"),
    (10560isize, "PMULastThresholdValue"),
    (10592isize, "R25"),
    (10600isize, "ICV_IGRPEN1_EL1"),
    (10608isize, "ID_AA64AFR0_EL1"),
    (10616isize, "ACTLR_EL2"),
    (10624isize, "FEAT_DGH_IMPLEMENTED"),
    (10632isize, "GITS_TYPER"),
    (10640isize, "__monomorphize_reads"),
    (10648isize, "MPAMVPM1_EL2"),
    (10656isize, "RNDRRS"),
    (10664isize, "SPERecordData"),
    (10728isize, "GICR_VSGIR"),
    (10736isize, "TCR_EL3"),
    (10744isize, "PMEVCNTR_EL0"),
    (11000isize, "_MAIR0_NS"),
    (11008isize, "EDRCR"),
    (11016isize, "IFSR_S"),
    (11024isize, "FEAT_FlagM2_IMPLEMENTED"),
    (11032isize, "MPAMIDR_EL1"),
    (11040isize, "ICH_MISR_EL2"),
    (11048isize, "_AIFSR_NS"),
    (11056isize, "GICC_AHPPIR"),
    (11064isize, "ZCR_EL3"),
    (11072isize, "_ERXFR"),
    (11080isize, "_ID_DFR0"),
    (11088isize, "CPTR_EL2"),
    (11096isize, "APIBKeyLo_EL1"),
    (11104isize, "NUM_PMU_COUNTERS"),
    (11120isize, "PMPCSCTL"),
    (11128isize, "__RD_base"),
    (11136isize, "SPESampleAddressValid"),
    (11168isize, "VSTCR_EL2"),
    (11176isize, "__max_implemented_sveveclen"),
    (11192isize, "_CNTHCTL"),
    (11200isize, "FEAT_ETMv4p1_IMPLEMENTED"),
    (11208isize, "PMEVTYPER_EL0"),
    (11464isize, "TRFCR_EL1"),
    (11472isize, "GICC_HPPIR"),
    (11480isize, "GCR_EL1"),
    (11488isize, "R23"),
    (11496isize, "FEAT_TIDCP1_IMPLEMENTED"),
    (11504isize, "DACR_S"),
    (11512isize, "EDPIDR1"),
    (11520isize, "_SDER32_EL3"),
    (11528isize, "SPESampleSubclassValid"),
    (11536isize, "ICC_AP1R_EL1_NS"),
    (11568isize, "_DBGDTR_EL0"),
    (11576isize, "FEAT_LSE128_IMPLEMENTED"),
    (11584isize, "__rme_l0gptsz"),
    (11592isize, "AMAIR_EL3"),
    (11600isize, "FEAT_AMUv1_IMPLEMENTED"),
    (11608isize, "FEAT_PMUv3_EDGE_IMPLEMENTED"),
    (11616isize, "TTBR0_NS"),
    (11624isize, "FEAT_AIE_IMPLEMENTED"),
    (11632isize, "ICC_CTLR_EL3"),
    (11640isize, "PMMIR"),
    (11648isize, "TRFCR_EL2"),
    (11656isize, "R28"),
    (11664isize, "FEAT_PCSRv8p2_IMPLEMENTED"),
    (11672isize, "TPIDRPRW_S"),
    (11680isize, "v8Ap0_IMPLEMENTED"),
    (11688isize, "FEAT_AA64EL2_IMPLEMENTED"),
    (11696isize, "LR_mon"),
    (11704isize, "GCSCR_EL3"),
    (11712isize, "_IFSR_NS"),
    (11720isize, "SCTLR2_EL1"),
    (11728isize, "ICC_NMIAR1_EL1"),
    (11736isize, "PMCNTENSET_EL0"),
    (11744isize, "ID_PFR2_EL1"),
    (11752isize, "_AMEVTYPER0"),
    (11768isize, "_ICH_LRC"),
    (11832isize, "EDDEVTYPE"),
    (11840isize, "FEAT_IDST_IMPLEMENTED"),
    (11848isize, "IsWFEsleep"),
    (11856isize, "_ICC_IAR1"),
    (11864isize, "FEAT_AA64EL3_IMPLEMENTED"),
    (11872isize, "_ICH_MISR"),
    (11880isize, "FEAT_PMUv3_ICNTR_IMPLEMENTED"),
    (11888isize, "HPFAR_EL2"),
    (11896isize, "APGAKeyLo_EL1"),
    (11904isize, "ICC_SRE_EL1_S"),
    (11912isize, "_ERXSTATUS"),
    (11920isize, "GICR_WAKER"),
    (11928isize, "FEAT_SVE_IMPLEMENTED"),
    (11936isize, "S2PIR_EL2"),
    (11944isize, "SPMACCESSR_EL1"),
    (11952isize, "_ICC_AP0R"),
    (11968isize, "CNTFID0"),
    (11976isize, "TPIDR_EL2"),
    (11984isize, "ICC_IGRPEN1_EL3"),
    (11992isize, "ESR_EL3"),
    (12000isize, "GICR_VSGIPENDR"),
    (12008isize, "FEAT_CSSC_IMPLEMENTED"),
    (12016isize, "R6"),
    (12024isize, "FEAT_SPEv1p1_IMPLEMENTED"),
    (12032isize, "FEAT_SCTLR2_IMPLEMENTED"),
    (12040isize, "FEAT_MTE_TAGGED_FAR_IMPLEMENTED"),
    (12048isize, "ICV_IGRPEN0_EL1"),
    (12056isize, "GICD_TYPER2"),
    (12064isize, "_CCSIDR"),
    (12072isize, "DBGCLAIMSET_EL1"),
    (12080isize, "SP_EL3"),
    (12088isize, "CPACR_EL1"),
    (12096isize, "_HVBAR"),
    (12104isize, "PMVIDSR"),
    (12112isize, "FEAT_TRBE_MPAM_IMPLEMENTED"),
    (12120isize, "ICV_IAR0_EL1"),
    (12128isize, "FEAT_BRBEv1p1_IMPLEMENTED"),
    (12136isize, "SPIDEN"),
    (12144isize, "FEAT_PMUv3p1_IMPLEMENTED"),
    (12152isize, "FEAT_SME_FA64_IMPLEMENTED"),
    (12160isize, "_HAMAIR0"),
    (12168isize, "FEAT_TWED_IMPLEMENTED"),
    (12176isize, "PIR_EL3"),
    (12184isize, "DBGBCR_EL1"),
    (12696isize, "STACK_BASE"),
    (12704isize, "_ICC_RPR"),
    (12712isize, "AMAIR0_S"),
    (12720isize, "GICV_STATUSR"),
    (12728isize, "PMITCTRL"),
    (12736isize, "PMSIRR_EL1"),
    (12744isize, "_PC"),
    (12752isize, "_ICC_ASGI1R"),
    (12760isize, "NUM_AMU_COUNTER_GROUPS"),
    (12776isize, "ICC_PMR_EL1"),
    (12784isize, "FEAT_RASSAv2_IMPLEMENTED"),
    (12792isize, "_MPAM3_EL3"),
    (12800isize, "FEAT_PAN3_IMPLEMENTED"),
    (12808isize, "CNTHCTL_EL2"),
    (12816isize, "TCR_EL2"),
    (12824isize, "ICV_CTLR_EL1"),
    (12832isize, "AMAIR_EL2"),
    (12840isize, "_MVFR1"),
    (12848isize, "_ICC_AP1R_NS"),
    (12864isize, "_CCSIDR2"),
    (12872isize, "_AMCGCR"),
    (12880isize, "TFSR_EL1"),
    (12888isize, "_HSR"),
    (12896isize, "FEAT_RASv2_IMPLEMENTED"),
    (12904isize, "PMSNEVFR_EL1"),
    (12912isize, "FEAT_CSV2_1p2_IMPLEMENTED"),
    (12920isize, "FPCR"),
    (12928isize, "_PMCCNTR"),
    (12936isize, "ERXMISC3_EL1"),
    (12944isize, "PMICNTR_EL0"),
    (12952isize, "__dczid_log2_block_size"),
    (12968isize, "EDPIDR2"),
    (12976isize, "_Dclone"),
    (13232isize, "CTIAUTHSTATUS"),
    (13240isize, "__syncAbortOnTTWNonCache"),
    (13248isize, "__syncAbortOnReadNormNonCache"),
    (13256isize, "_ICV_DIR"),
    (13264isize, "_AIDR"),
    (13272isize, "PMSSCR_EL1"),
    (13280isize, "_CNTP_CTL_NS"),
    (13288isize, "FEAT_AA32EL3_IMPLEMENTED"),
    (13296isize, "_AMEVTYPER1"),
    (13360isize, "DLR_EL0"),
    (13368isize, "AFSR0_EL2"),
    (13376isize, "_TTBCR2_NS"),
    (13384isize, "_ICV_BPR1"),
    (13392isize, "__mpam_pmg_max"),
    (13400isize, "FEAT_HPDS2_IMPLEMENTED"),
    (13408isize, "FEAT_PMUv3p9_IMPLEMENTED"),
    (13416isize, "_HADFSR"),
    (13424isize, "_ICH_ELRSR"),
    (13432isize, "APGAKeyHi_EL1"),
    (13440isize, "AMCNTENSET1_EL0"),
    (13448isize, "APDAKeyHi_EL1"),
    (13456isize, "PhysicalCount"),
    (13472isize, "__GICITSControlBase"),
    (13480isize, "ID_AA64PFR2_EL1"),
    (13488isize, "_AMCFGR"),
    (13496isize, "BRBIDR0_EL1"),
    (13504isize, "SPESampleTimestamp"),
    (13512isize, "GICR_SYNCR"),
    (13520isize, "_NMRR_NS"),
    (13528isize, "SPESampleSubclass"),
    (13536isize, "_MPAM1_EL1"),
    (13544isize, "_ID_MMFR5"),
    (13552isize, "ICV_EOIR0_EL1"),
    (13560isize, "FEAT_ExS_IMPLEMENTED"),
    (13568isize, "ICV_HPPIR0_EL1"),
    (13576isize, "FEAT_BBM_IMPLEMENTED"),
    (13584isize, "__sme_only"),
    (13592isize, "POR_EL1"),
    (13600isize, "__ThisInstrEnc"),
    (13608isize, "HFGITR_EL2"),
    (13616isize, "PMECR_EL1"),
    (13624isize, "EDAA32PFR"),
    (13632isize, "DISR_EL1"),
    (13640isize, "_ID_ISAR6"),
    (13648isize, "VNCR_EL2"),
    (13656isize, "FEAT_PFAR_IMPLEMENTED"),
    (13664isize, "ICC_EOIR0_EL1"),
    (13672isize, "GICR_IIDR"),
    (13680isize, "CTICIDR0"),
    (13688isize, "SPMACCESSR_EL3"),
    (13696isize, "CNTEL0ACR"),
    (13704isize, "PMBSR_EL1"),
    (13712isize, "_AMCR"),
    (13720isize, "_ICV_RPR"),
    (13728isize, "__impdef_TG1"),
    (13736isize, "CTIDEVTYPE"),
    (13744isize, "EDCIDR1"),
    (13752isize, "CTIDEVCTL"),
    (13760isize, "_HTRFCR"),
    (13768isize, "FEAT_RASv1p1_IMPLEMENTED"),
    (13776isize, "SPESampleAddress"),
    (14032isize, "__last_branch_valid"),
    (14040isize, "EDPRSR"),
    (14048isize, "CFG_MPIDR"),
    (14056isize, "FEAT_Debugv8p2_IMPLEMENTED"),
    (14064isize, "FEAT_LRCPC_IMPLEMENTED"),
    (14072isize, "PMPIDR2"),
    (14080isize, "_IFAR_NS"),
    (14088isize, "_HAIFSR"),
    (14096isize, "_DBGWCR"),
    (14160isize, "CNTPS_CVAL_EL1"),
    (14168isize, "_TTBR1_EL1"),
    (14184isize, "SPESampleDataSourceValid"),
    (14192isize, "AMDEVTYPE"),
    (14200isize, "POR_EL3"),
    (14208isize, "_EDSCR2"),
    (14216isize, "__supported_va_size"),
    (14232isize, "FEAT_HCX_IMPLEMENTED"),
    (14240isize, "__CNTbase_frequency"),
    (14248isize, "GITS_CBASER"),
    (14256isize, "__mpam_frac"),
    (14264isize, "FEAT_ADERR_IMPLEMENTED"),
    (14272isize, "_PMCNTEN"),
    (14280isize, "TPIDR_EL1"),
    (14288isize, "_TPIDRURW_NS"),
    (14296isize, "FEAT_AMUv1p1_IMPLEMENTED"),
    (14304isize, "FEAT_CSV2_1p1_IMPLEMENTED"),
    (14312isize, "FEAT_ANERR_IMPLEMENTED"),
    (14320isize, "APDBKeyHi_EL1"),
    (14328isize, "NUM_GIC_PREEMPTION_BITS"),
    (14344isize, "__set_mops_option_a_supported"),
    (14352isize, "FEAT_LS64_V_IMPLEMENTED"),
    (14360isize, "HEAP_LIMIT"),
    (14368isize, "_PMCEID0"),
    (14376isize, "sp_rel_access_pc"),
    (14384isize, "ID_ISAR1_EL1"),
    (14392isize, "_ERRIDR"),
    (14400isize, "__has_sme_priority_control"),
    (14408isize, "GICR_CLRLPIR"),
    (14416isize, "ERXGSR_EL1"),
    (14424isize, "FEAT_TRC_SR_IMPLEMENTED"),
    (14432isize, "FEAT_RNG_IMPLEMENTED"),
    (14440isize, "GITS_MPIDR"),
    (14448isize, "FEAT_PMUv3p5_IMPLEMENTED"),
    (14456isize, "FEAT_LVA3_IMPLEMENTED"),
    (14464isize, "FEAT_MTE_STORE_ONLY_IMPLEMENTED"),
    (14472isize, "FEAT_PCSRv8p9_IMPLEMENTED"),
    (14480isize, "FEAT_SPE_FDS_IMPLEMENTED"),
    (14488isize, "_AMAIR1_NS"),
    (14496isize, "ICC_IGRPEN0_EL1"),
    (14504isize, "_PMINTEN"),
    (14512isize, "GICR_CTLR"),
    (14520isize, "DBGDEVID"),
    (14528isize, "_TTBR0_EL1"),
    (14544isize, "__CNTBaseN"),
    (14552isize, "_FFR"),
    (14584isize, "CNTPOFF_EL2"),
    (14592isize, "APDAKeyLo_EL1"),
    (14600isize, "ID_AA64ISAR1_EL1"),
    (14608isize, "AFSR1_EL3"),
    (14616isize, "FEAT_SHA512_IMPLEMENTED"),
    (14624isize, "AMEVCNTR0"),
    (14656isize, "AMCGCR_EL0"),
    (14664isize, "FEAT_EL1_IMPLEMENTED"),
    (14672isize, "_ID_ISAR3"),
    (14680isize, "_PMSWINC"),
    (14688isize, "FEAT_IVIPT_IMPLEMENTED"),
    (14696isize, "SEE"),
    (14712isize, "EDESR"),
    (14720isize, "_IFAR_S"),
    (14728isize, "_ID_PFR0"),
    (14736isize, "PMSIDR_EL1"),
    (14744isize, "FEAT_SB_IMPLEMENTED"),
    (14752isize, "_CNTHP_CVAL"),
    (14760isize, "FEAT_PCSRv8_IMPLEMENTED"),
    (14768isize, "R29"),
    (14776isize, "TCR2_EL1"),
    (14784isize, "FEAT_LSE_IMPLEMENTED"),
    (14792isize, "APIAKeyHi_EL1"),
    (14800isize, "ZCR_EL3_LEN_VALUE"),
    (14816isize, "FEAT_SVE_BitPerm_IMPLEMENTED"),
    (14824isize, "HTTBR"),
    (14832isize, "ICH_AP0R_EL2"),
    (14864isize, "ID_AA64ISAR2_EL1"),
    (14872isize, "CNTHVS_CTL_EL2"),
    (14880isize, "SPESampleContextEL2Valid"),
    (14888isize, "ICC_ASGI1R_EL1"),
    (14896isize, "ID_AA64MMFR0_EL1"),
    (14904isize, "HACR_EL2"),
    (14912isize, "FEAT_CONSTPACFIELD_IMPLEMENTED"),
    (14920isize, "FEAT_GICv3_IMPLEMENTED"),
    (14928isize, "FEAT_CHK_IMPLEMENTED"),
    (14936isize, "FEAT_ETEv1p1_IMPLEMENTED"),
    (14944isize, "__BranchTaken"),
    (14952isize, "TFSRE0_EL1"),
    (14960isize, "MDRAR_EL1"),
    (14968isize, "PMCEID0_EL0"),
    (14976isize, "GITS_CREADR"),
    (14984isize, "PMIIDR"),
    (14992isize, "_ID_ISAR4"),
    (15000isize, "__CNTCTLBase"),
    (15008isize, "_ERXMISC4"),
    (15016isize, "GITS_CTLR"),
    (15024isize, "GICM_CLRSPI_NSR"),
    (15032isize, "RVBAR"),
    (15040isize, "_EDSCR"),
    (15048isize, "SDCR"),
    (15056isize, "IFSR32_EL2"),
    (15064isize, "ICV_PMR_EL1"),
    (15072isize, "ZCR_EL2"),
    (15080isize, "_AMEVCNTR1"),
    (15208isize, "FEAT_FRINTTS_IMPLEMENTED"),
    (15216isize, "_SPSR_svc"),
    (15224isize, "__empam_tidr_implemented"),
    (15232isize, "DBGDEVID1"),
    (15240isize, "FEAT_TRC_EXT_IMPLEMENTED"),
    (15248isize, "_ERXMISC0"),
    (15256isize, "FEAT_F32MM_IMPLEMENTED"),
    (15264isize, "v8Ap3_IMPLEMENTED"),
    (15272isize, "ERRIDR_EL1"),
    (15280isize, "GICC_AEOIR"),
    (15288isize, "GICC_DIR"),
    (15296isize, "FEAT_ECV_IMPLEMENTED"),
    (15304isize, "_CPACR"),
    (15312isize, "FEAT_SPEv1p2_IMPLEMENTED"),
    (15320isize, "__syncAbortOnPrefetch"),
    (15328isize, "VTCR_EL2"),
    (15336isize, "POR_EL2"),
    (15344isize, "PMCCNTSVR_EL1"),
    (15352isize, "PMXEVCNTR_EL0"),
    (15360isize, "SP_mon"),
    (15368isize, "TTBCR_S"),
    (15376isize, "ICH_VMCR_EL2"),
    (15384isize, "_FPSCR"),
    (15392isize, "ICV_RPR_EL1"),
    (15400isize, "AFSR1_EL2"),
    (15408isize, "ACTLR_S"),
    (15416isize, "FEAT_LPA_IMPLEMENTED"),
    (15424isize, "EDPFR"),
    (15432isize, "FEAT_ETMv4p4_IMPLEMENTED"),
    (15440isize, "SPESamplePreviousBranchAddress"),
    (15448isize, "PMINTENCLR_EL1"),
    (15456isize, "EDLSR"),
    (15464isize, "MPAMVPM2_EL2"),
    (15472isize, "AMPIDR1"),
    (15480isize, "RTPIDEN"),
    (15488isize, "FEAT_DotProd_IMPLEMENTED"),
    (15496isize, "GICR_PENDBASER"),
    (15504isize, "_ID_ISAR2"),
    (15512isize, "GICC_IAR"),
    (15520isize, "_MAIR1_S"),
    (15528isize, "_ICC_BPR0"),
    (15536isize, "SPSR_fiq"),
    (15544isize, "AMCR_EL0"),
    (15552isize, "FEAT_DPB_IMPLEMENTED"),
    (15560isize, "_SCTLR_NS"),
    (15568isize, "ICC_IAR0_EL1"),
    (15576isize, "FPSID"),
    (15584isize, "FEAT_CSV3_IMPLEMENTED"),
    (15592isize, "FEAT_S1POE_IMPLEMENTED"),
    (15600isize, "FEAT_LSMAOC_IMPLEMENTED"),
    (15608isize, "GCSCRE0_EL1"),
    (15616isize, "AMIIDR"),
    (15624isize, "__block_bbm_implemented"),
    (15640isize, "_ERXCTLR"),
    (15648isize, "GICC_CTLR"),
    (15656isize, "CPTR_EL3_EZ_VALUE"),
    (15672isize, "R2"),
    (15680isize, "ACTLR_EL3"),
    (15688isize, "FEAT_VPIPT_IMPLEMENTED"),
    (15696isize, "_ICC_HPPIR0"),
    (15704isize, "PMBIDR_EL1"),
    (15712isize, "CTIITCTRL"),
    (15720isize, "VMECID_A_EL2"),
    (15728isize, "_HAMAIR1"),
    (15736isize, "SPSR_EL2"),
    (15744isize, "LORSA_EL1"),
    (15752isize, "TCR2_EL2"),
    (15760isize, "APDBKeyLo_EL1"),
    (15768isize, "RVBAR_EL3"),
    (15776isize, "PMPIDR0"),
    (15784isize, "_ICH_LR"),
    (15848isize, "__clock_divider"),
    (15864isize, "PMCCFILTR_EL0"),
    (15872isize, "OSDTRRX_EL1"),
    (15880isize, "DBGDSAR"),
    (15888isize, "_VPIDR"),
    (15896isize, "CNTID"),
    (15904isize, "FEAT_SVE2_IMPLEMENTED"),
    (15912isize, "FEAT_SME2_IMPLEMENTED"),
    (15920isize, "HEAP_BASE"),
    (15928isize, "FEAT_ETMv4p2_IMPLEMENTED"),
    (15936isize, "__mecid_width"),
    (15944isize, "BRBTGT_EL1"),
    (16200isize, "GICV_HPPIR"),
    (16208isize, "FEAT_PMUv3_IMPLEMENTED"),
    (16216isize, "FEAT_SSBS_IMPLEMENTED"),
    (16224isize, "_HDFAR"),
    (16232isize, "_ICV_IAR1"),
    (16240isize, "ISR_EL1"),
    (16248isize, "FEAT_nTLBPA_IMPLEMENTED"),
    (16256isize, "FAR_EL1"),
    (16264isize, "RVBAR_EL1"),
    (16272isize, "_CNTKCTL"),
    (16280isize, "TPIDR_EL3"),
    (16288isize, "ID_PFR0_EL1"),
    (16296isize, "FEAT_RPRES_IMPLEMENTED"),
    (16304isize, "_PRRR_NS"),
    (16312isize, "FEAT_TCR2_IMPLEMENTED"),
    (16320isize, "_ICC_IAR0"),
    (16328isize, "FEAT_SHA1_IMPLEMENTED"),
    (16336isize, "FEAT_AA32HPD_IMPLEMENTED"),
    (16344isize, "FEAT_LSE2_IMPLEMENTED"),
    (16352isize, "CFG_RMR_AA64"),
    (16360isize, "_PMCNTENSET"),
    (16368isize, "ICC_SRE_EL2"),
    (16376isize, "HFGWTR2_EL2"),
    (16384isize, "PMPIDR3"),
    (16392isize, "_DBGBVR"),
    (16456isize, "SCTLR_S"),
    (16464isize, "FEAT_FHM_IMPLEMENTED"),
    (16472isize, "EDWAR"),
    (16480isize, "R1"),
    (16488isize, "_CONTEXTIDR_NS"),
    (16496isize, "AFSR0_EL1"),
    (16504isize, "RCWSMASK_EL1"),
    (16520isize, "SCXTNUM_EL2"),
    (16528isize, "ERXPFGCDN_EL1"),
    (16536isize, "BRBFCR_EL1"),
    (16544isize, "__impdef_TG0"),
    (16552isize, "SPMSELR_EL0"),
    (16560isize, "_PMUSERENR"),
    (16568isize, "FCSEIDR"),
    (16576isize, "GICD_SETSPI_SR"),
    (16584isize, "DACR32_EL2"),
    (16592isize, "HFGRTR_EL2"),
    (16600isize, "TPIDRURO_S"),
    (16608isize, "FEAT_Debugv8p9_IMPLEMENTED"),
    (16616isize, "FEAT_MEC_IMPLEMENTED"),
    (16624isize, "MPAM0_EL1"),
    (16632isize, "FEAT_TLBIOS_IMPLEMENTED"),
    (16640isize, "CNTHP_CVAL_EL2"),
    (16648isize, "GPCCR_EL3"),
    (16656isize, "AFSR0_EL3"),
    (16664isize, "AMEVCNTVOFF1_EL2"),
    (16792isize, "_AMUSERENR"),
    (16800isize, "_ICC_EOIR1"),
    (16808isize, "EDCIDR3"),
    (16816isize, "DBGDIDR"),
    (16824isize, "FEAT_LVA_IMPLEMENTED"),
    (16832isize, "MDCCSR_EL0"),
    (16840isize, "CPTR_EL3"),
    (16848isize, "CNTP_CVAL_S"),
    (16856isize, "AIDR_EL1"),
    (16864isize, "_AMCNTENSET0"),
    (16872isize, "_DACR_NS"),
    (16880isize, "EDLAR"),
    (16888isize, "FEAT_AA64EL1_IMPLEMENTED"),
    (16896isize, "_ICH_AP0R"),
    (16912isize, "ERRnFR"),
    (16944isize, "R15"),
    (16952isize, "_PMCCFILTR"),
    (16960isize, "PMCFGR"),
    (16968isize, "PSTATE"),
    (17000isize, "EDDEVARCH"),
    (17008isize, "_ID_ISAR1"),
    (17016isize, "TCMTR"),
    (17024isize, "EDHSR"),
    (17032isize, "__CNTReadBase"),
    (17040isize, "ICC_IGRPEN1_EL1_NS"),
    (17048isize, "GICH_VTR"),
    (17056isize, "GICD_SGIR"),
    (17064isize, "FEAT_AdvSIMD_IMPLEMENTED"),
    (17072isize, "SCTLR_EL3"),
    (17080isize, "_ERXMISC3"),
    (17088isize, "_ELR_hyp"),
    (17096isize, "_PMSELR"),
    (17104isize, "R19"),
    (17112isize, "CNTHVS_TVAL_EL2"),
    (17120isize, "AIFSR_S"),
    (17128isize, "_PMCEID2"),
    (17136isize, "SPESampleClass"),
    (17144isize, "NIDEN"),
    (17152isize, "VBAR_EL1"),
    (17160isize, "FEAT_ECBHB_IMPLEMENTED"),
    (17168isize, "ICC_HPPIR1_EL1"),
    (17176isize, "ICH_ELRSR_EL2"),
    (17184isize, "FEAT_MOPS_IMPLEMENTED"),
    (17192isize, "CLIDR_EL1"),
    (17200isize, "CNTV_CTL_EL0"),
    (17208isize, "_MAIR1_NS"),
    (17216isize, "FEAT_SPE_IMPLEMENTED"),
    (17224isize, "ELR_EL2"),
    (17232isize, "DBGDTRTX_EL0"),
    (17240isize, "TPIDRRO_EL0"),
    (17248isize, "ICC_EOIR1_EL1"),
    (17256isize, "PMCIDR0"),
    (17264isize, "FEAT_SME_I16I64_IMPLEMENTED"),
    (17272isize, "FEAT_FP_IMPLEMENTED"),
    (17280isize, "FEAT_MTE_ASYM_FAULT_IMPLEMENTED"),
    (17288isize, "FEAT_SPE_CRR_IMPLEMENTED"),
    (17296isize, "FEAT_TRBE_IMPLEMENTED"),
    (17304isize, "SMCR_EL1"),
    (17312isize, "MPAMVPMV_EL2"),
    (17320isize, "_VDISR"),
    (17328isize, "ICC_BPR0_EL1"),
    (17336isize, "ID_ISAR0_EL1"),
    (17344isize, "ICC_BPR1_EL1_NS"),
    (17352isize, "ICH_VTR_EL2"),
    (17360isize, "HDFGWTR_EL2"),
    (17368isize, "FEAT_MTE_PERM_IMPLEMENTED"),
    (17376isize, "MPIDR_EL1"),
    (17384isize, "PMPCSR"),
    (17392isize, "_ICC_SGI0R"),
    (17400isize, "AMEVCNTVOFF0_EL2"),
    (17528isize, "ERXFR_EL1"),
    (17536isize, "GICR_VPENDBASER"),
    (17544isize, "_ICC_BPR1_NS"),
    (17552isize, "SPESampleDataSource"),
    (17560isize, "GICM_SETSPI_NSR"),
    (17568isize, "NUM_GIC_LIST_REGS"),
    (17584isize, "_PMINTENCLR"),
    (17592isize, "GICM_TYPER"),
    (17600isize, "FEAT_Debugv8p8_IMPLEMENTED"),
    (17608isize, "MPAMHCR_EL2"),
    (17616isize, "SPESampleTimestampValid"),
    (17624isize, "FEAT_CMOW_IMPLEMENTED"),
    (17632isize, "FEAT_ETEv1p3_IMPLEMENTED"),
    (17640isize, "v8Ap1_IMPLEMENTED"),
    (17648isize, "_DBGDSCRext"),
    (17656isize, "MAIR_EL3"),
    (17664isize, "HDFGWTR2_EL2"),
    (17672isize, "FEAT_ABLE_IMPLEMENTED"),
    (17680isize, "GICV_IAR"),
    (17688isize, "_PMOVS"),
    (17696isize, "CTIPIDR2"),
    (17704isize, "v8Ap8_IMPLEMENTED"),
    (17712isize, "FEAT_RME_IMPLEMENTED"),
    (17720isize, "_DBGDRAR"),
    (17728isize, "GITS_PARTIDR"),
    (17736isize, "_P"),
    (18248isize, "GCSPR_EL3"),
    (18256isize, "FEAT_ASMv8p2_IMPLEMENTED"),
    (18264isize, "__VLPI_base"),
    (18272isize, "BRBCR_EL2"),
    (18280isize, "__unpred_tsize_aborts"),
    (18288isize, "CNTCR"),
    (18296isize, "CNTHP_TVAL_EL2"),
    (18304isize, "_ICV_HPPIR1"),
    (18312isize, "ELR_EL1"),
    (18320isize, "R4"),
    (18328isize, "__ICACHE_CCSIDR_RESET"),
    (18384isize, "_HSCTLR"),
    (18392isize, "ICC_CTLR_EL1_S"),
    (18400isize, "_TPIDRURO_NS"),
    (18408isize, "_ERXADDR2"),
    (18416isize, "MDSELR_EL1"),
    (18424isize, "SPSR_und"),
    (18432isize, "TTBR1_EL2"),
    (18448isize, "_VTTBR_EL2"),
    (18464isize, "SPESampleCounter"),
    (18976isize, "GICH_VMCR"),
    (18984isize, "CTILAR"),
    (18992isize, "PMDEVTYPE"),
    (19000isize, "GICC_EOIR"),
    (19008isize, "ID_ISAR4_EL1"),
    (19016isize, "FEAT_CSV2_2_IMPLEMENTED"),
    (19024isize, "FEAT_SYSREG128_IMPLEMENTED"),
    (19032isize, "R9"),
    (19040isize, "SPESampleOpType"),
    (19048isize, "CTIPIDR0"),
    (19056isize, "CTR_EL0"),
    (19064isize, "SPMACCESSR_EL2"),
    (19072isize, "FEAT_CSV2_3_IMPLEMENTED"),
    (19080isize, "FEAT_SPMU_IMPLEMENTED"),
    (19088isize, "__tlb_enabled"),
    (19096isize, "_VBAR_NS"),
    (19104isize, "MAIR2_EL3"),
    (19112isize, "R14"),
    (19120isize, "TTBR1_S"),
    (19128isize, "v8Ap5_IMPLEMENTED"),
    (19136isize, "PMSELR_EL0"),
    (19144isize, "HDFGRTR_EL2"),
    (19152isize, "AMEVTYPER1_EL0"),
    (19280isize, "CNTHV_CTL_EL2"),
    (19288isize, "ICC_RPR_EL1"),
    (19296isize, "AMDEVARCH"),
    (19304isize, "GCSCR_EL2"),
    (19312isize, "EDPCSR"),
    (19320isize, "_ERXFR2"),
    (19328isize, "VDISR_EL2"),
    (19336isize, "FEAT_MTE_ASYNC_IMPLEMENTED"),
    (19344isize, "_CNTP_CVAL_NS"),
    (19352isize, "DBGDEVID2"),
    (19360isize, "NUM_WATCHPOINTS"),
    (19376isize, "CNTSR"),
    (19384isize, "AMCIDR1"),
    (19392isize, "DBGWVR_EL1"),
    (19904isize, "ICH_AP1R_EL2"),
    (19936isize, "FEAT_FCMA_IMPLEMENTED"),
    (19944isize, "FEAT_GICv3p1_IMPLEMENTED"),
    (19952isize, "__syncAbortOnTTWCache"),
    (19960isize, "FEAT_S1PIE_IMPLEMENTED"),
    (19968isize, "OSECCR_EL1"),
    (19976isize, "FEAT_ETMv4p5_IMPLEMENTED"),
    (19984isize, "PRRR_S"),
    (19992isize, "ICC_MSRE"),
    (20000isize, "_ERXMISC5"),
    (20008isize, "PFAR_EL2"),
    (20016isize, "CTICIDR1"),
    (20024isize, "TTBR1_NS"),
    (20032isize, "SPSR_abt"),
    (20040isize, "_ICV_IAR0"),
    (20048isize, "MAIR2_EL1"),
    (20056isize, "FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED"),
    (20064isize, "R21"),
    (20072isize, "MDCCINT_EL1"),
    (20080isize, "AMCIDR2"),
    (20088isize, "_ICH_HCR"),
    (20096isize, "RGSR_EL1"),
    (20104isize, "_MIDR"),
    (20112isize, "ID_AA64DFR0_EL1"),
    (20120isize, "_ID_PFR1"),
    (20128isize, "ELR_EL3"),
    (20136isize, "__syncAbortOnSoRead"),
    (20144isize, "ID_AA64AFR1_EL1"),
    (20152isize, "FEAT_AA64EL0_IMPLEMENTED"),
    (20160isize, "SPESampleContextEL1Valid"),
    (20168isize, "FEAT_EBEP_IMPLEMENTED"),
    (20176isize, "EDECR"),
    (20184isize, "GICR_VPROPBASER"),
    (20192isize, "_CSSELR_NS"),
    (20200isize, "_MVFR0"),
    (20208isize, "AMAIR1_S"),
    (20216isize, "ID_MMFR5_EL1"),
    (20224isize, "PMCIDR3"),
    (20232isize, "_DBGCLAIMCLR"),
    (20240isize, "_ADFSR_NS"),
    (20248isize, "v8Ap6_IMPLEMENTED"),
    (20256isize, "_HPFAR"),
    (20264isize, "EDPIDR0"),
    (20272isize, "_DBGOSLSR"),
    (20280isize, "PIRE0_EL1"),
    (20288isize, "FEAT_LRCPC3_IMPLEMENTED"),
    (20296isize, "FEAT_SVE_AES_IMPLEMENTED"),
    (20304isize, "SPSR_EL3"),
    (20312isize, "GICM_CLRSPI_SR"),
    (20320isize, "__syncAbortOnWriteNormCache"),
    (20328isize, "CP15SDISABLE2"),
    (20336isize, "FEAT_CRC32_IMPLEMENTED"),
    (20344isize, "FEAT_TTST_IMPLEMENTED"),
    (20352isize, "TTBCR2_S"),
    (20360isize, "_ICC_IGRPEN0"),
    (20368isize, "R20"),
    (20376isize, "CNTPS_CTL_EL1"),
    (20384isize, "_HTPIDR"),
    (20392isize, "GICR_PARTIDR"),
    (20400isize, "FEAT_PMUv3_EXT_IMPLEMENTED"),
    (20408isize, "R13"),
    (20416isize, "ID_DFR0_EL1"),
    (20424isize, "GICD_CLRSPI_SR"),
    (20432isize, "PMMIR_EL1"),
    (20440isize, "DBGEN"),
    (20448isize, "FEAT_IESB_IMPLEMENTED"),
    (20456isize, "FEAT_BTI_IMPLEMENTED"),
    (20464isize, "ICC_SGI1R_EL1"),
    (20472isize, "R30"),
    (20480isize, "PMBLIMITR_EL1"),
    (20488isize, "_TPIDRPRW_NS"),
    (20496isize, "FEAT_GTG_IMPLEMENTED"),
    (20504isize, "_CNTHV_CTL"),
    (20512isize, "GITS_MPAMIDR"),
    (20520isize, "_DBGDTRRXint"),
    (20528isize, "FEAT_AA32EL0_IMPLEMENTED"),
    (20536isize, "FEAT_DoubleFault_IMPLEMENTED"),
    (20544isize, "__isla_vector_gpr"),
    (20552isize, "__GICCPUInterfaceBase"),
    (20560isize, "RC"),
    (20600isize, "VMECID_P_EL2"),
    (20608isize, "__GIC_Pending"),
    (20616isize, "ICC_DIR_EL1"),
    (20624isize, "GPTBR_EL3"),
    (20632isize, "_ICC_EOIR0"),
    (20640isize, "_MAIR0_S"),
    (20648isize, "_ICC_SRE_S"),
    (20656isize, "FEAT_SPECRES2_IMPLEMENTED"),
    (20664isize, "__mops_forward_copy"),
    (20672isize, "VMPIDR_EL2"),
    (20680isize, "_ICV_BPR0"),
    (20688isize, "FEAT_PMUv3_SS_IMPLEMENTED"),
    (20696isize, "FPSR"),
    (20704isize, "_HIFAR"),
    (20712isize, "_ICV_EOIR1"),
    (20720isize, "_HMAIR1"),
    (20728isize, "SPESamplePreviousBranchAddressValid"),
    (20736isize, "Branchtypetaken"),
    (20744isize, "ICV_AP1R_EL1"),
    (20776isize, "AMAIR2_EL3"),
    (20784isize, "SCTLR_EL2"),
    (20792isize, "VPIDR_EL2"),
    (20800isize, "CNTP_CVAL_EL0"),
    (20808isize, "_ICV_AP0R"),
    (20824isize, "NUM_BRBE_RECORDS"),
    (20840isize, "GCSPR_EL0"),
    (20848isize, "__has_sve_extended_bf16"),
    (20864isize, "v8Ap2_IMPLEMENTED"),
    (20872isize, "_ACTLR2_NS"),
    (20880isize, "SPESampleContextEL2"),
    (20888isize, "AMEVTYPER0_EL0"),
    (20920isize, "SCR"),
    (20928isize, "MAIR2_EL2"),
    (20936isize, "GICC_STATUSR"),
    (20944isize, "ID_AA64MMFR4_EL1"),
    (20952isize, "BTypeCompatible"),
    (20960isize, "FEAT_S2PIE_IMPLEMENTED"),
    (20968isize, "_DBGOSDLR"),
    (20976isize, "DBGAUTHSTATUS_EL1"),
    (20984isize, "MPAMVPM7_EL2"),
    (20992isize, "ICH_HCR_EL2"),
    (21000isize, "GICV_DIR"),
    (21008isize, "FEAT_EBF16_IMPLEMENTED"),
    (21016isize, "PMCR_EL0"),
    (21024isize, "FPEXC32_EL2"),
    (21032isize, "ICV_HPPIR1_EL1"),
    (21040isize, "FEAT_FP16_IMPLEMENTED"),
    (21048isize, "_TRFCR"),
    (21056isize, "__empam_sdeflt_implemented"),
    (21064isize, "CNTHV_TVAL_EL2"),
    (21072isize, "PMSCR_EL1"),
    (21080isize, "ID_AFR0_EL1"),
    (21088isize, "DBGCLAIMCLR_EL1"),
    (21096isize, "APIAKeyLo_EL1"),
    (21104isize, "FEAT_UAO_IMPLEMENTED"),
    (21112isize, "SDER32_EL2"),
    (21120isize, "EDDFR1"),
    (21128isize, "FEAT_GICv3_NMI_IMPLEMENTED"),
    (21136isize, "SPSR_mon"),
    (21144isize, "__mpam_has_altsp"),
    (21152isize, "ICV_AP0R_EL1"),
    (21184isize, "SCXTNUM_EL3"),
    (21192isize, "__mpam_vpmr_max"),
    (21200isize, "R18"),
    (21208isize, "__SGI_base"),
    (21216isize, "R0"),
    (21224isize, "v9Ap3_IMPLEMENTED"),
    (21232isize, "__apply_effective_shareability"),
    (21240isize, "Records_SRC"),
    (21752isize, "_DFAR_S"),
    (21760isize, "HAFGRTR_EL2"),
    (21768isize, "__syncAbortOnReadNormCache"),
    (21776isize, "LOREA_EL1"),
    (21784isize, "AMAIR2_EL1"),
    (21792isize, "ERRSELR_EL1"),
    (21800isize, "ICC_MCTLR"),
    (21808isize, "__mpam_partid_max"),
    (21816isize, "FEAT_RDM_IMPLEMENTED"),
    (21824isize, "__syncAbortOnDeviceWrite"),
    (21832isize, "FEAT_ETMv4p6_IMPLEMENTED"),
    (21840isize, "R27"),
    (21848isize, "_DormantCtlReg"),
    (21856isize, "_ID_MMFR0"),
    (21864isize, "_ERXADDR"),
    (21872isize, "EDITCTRL"),
    (21880isize, "__ignore_rvbar_in_aarch32"),
    (21888isize, "CNTP_CTL_S"),
    (21896isize, "FEAT_EL2_IMPLEMENTED"),
    (21904isize, "CTICONTROL"),
    (21912isize, "GCSPR_EL1"),
    (21920isize, "__currentCond"),
    (21928isize, "BRBSRCINJ_EL1"),
    (21936isize, "CONTEXTIDR_S"),
    (21944isize, "GITS_STATUSR"),
    (21952isize, "_HCR2"),
    (21960isize, "AMCIDR0"),
    (21968isize, "EventRegister"),
    (21976isize, "FEAT_ETS2_IMPLEMENTED"),
    (21984isize, "_DBGPRCR"),
    (21992isize, "_DLR"),
    (22000isize, "FEAT_SME_IMPLEMENTED"),
    (22008isize, "__SPE_LFSR"),
    (22016isize, "CNTSCR"),
    (22024isize, "_AMEVCNTR0_EL0"),
    (22056isize, "CNTKCTL_EL1"),
    (22064isize, "__isb_is_branch"),
    (22072isize, "GICR_MPAMIDR"),
    (22080isize, "LORID_EL1"),
    (22088isize, "_ICC_SRE_NS"),
    (22096isize, "_ICV_IGRPEN0"),
    (22104isize, "FEAT_DPB2_IMPLEMENTED"),
    (22112isize, "ID_AA64MMFR3_EL1"),
    (22120isize, "BRBINF_EL1"),
    (22376isize, "GICH_ELRSR"),
    (22384isize, "GICH_MISR"),
    (22392isize, "TCR_EL1"),
    (22400isize, "CNTVOFF_EL2"),
    (22408isize, "VTTBR"),
    (22416isize, "SPESampleInFlight"),
    (22424isize, "REVIDR_EL1"),
    (22432isize, "_DBGBXVR"),
    (22496isize, "TPIDRURW_S"),
    (22504isize, "AMCIDR3"),
    (22512isize, "FEAT_XS_IMPLEMENTED"),
    (22520isize, "MPAMVPM4_EL2"),
    (22528isize, "HCRX_EL2"),
    (22536isize, "OSDTRTX_EL1"),
    (22544isize, "MPAMVPM6_EL2"),
    (22552isize, "ID_AA64PFR1_EL1"),
    (22560isize, "ERXPFGF_EL1"),
    (22568isize, "FEAT_NV2_IMPLEMENTED"),
    (22576isize, "FEAT_HAFDBS_IMPLEMENTED"),
    (22584isize, "FEAT_PAuth_IMPLEMENTED"),
    (22592isize, "ICH_EISR_EL2"),
    (22600isize, "ERXMISC0_EL1"),
    (22608isize, "JOSCR"),
    (22616isize, "AMAIR2_EL2"),
    (22624isize, "PMAUTHSTATUS"),
    (22632isize, "PMCNTENCLR_EL0"),
    (22640isize, "__last_cycle_count"),
    (22656isize, "FEAT_F64MM_IMPLEMENTED"),
    (22664isize, "FEAT_PAuth2_IMPLEMENTED"),
    (22672isize, "CNTHPS_CVAL_EL2"),
    (22680isize, "__trcclaim_tags"),
    (22688isize, "AFSR1_EL1"),
    (22696isize, "_AMCNTENCLR1"),
    (22704isize, "GICD_SETSPI_NSR"),
    (22712isize, "MDCR_EL3"),
    (22720isize, "_VMPIDR"),
    (22728isize, "GICV_AHPPIR"),
    (22736isize, "AMPIDR0"),
    (22744isize, "PMSEVFR_EL1"),
    (22752isize, "v8Ap7_IMPLEMENTED"),
    (22760isize, "__InstructionStep"),
    (22768isize, "FEAT_SVE2p1_IMPLEMENTED"),
    (22776isize, "NUM_BREAKPOINTS"),
    (22792isize, "AMCNTENCLR0_EL0"),
    (22800isize, "EDDFR"),
    (22808isize, "__SPE_LFSR_initialized"),
    (22816isize, "VBAR_EL2"),
    (22824isize, "VSTTBR_EL2"),
    (22832isize, "EDVIDSR"),
    (22840isize, "PMZR_EL0"),
    (22848isize, "ADFSR_S"),
    (22856isize, "_ID_PFR2"),
    (22864isize, "_ICC_AP1R_S"),
    (22880isize, "_ICC_SGI1R"),
    (22888isize, "_CNTFRQ"),
    (22896isize, "CSSELR_EL1"),
    (22904isize, "MECID_P0_EL2"),
    (22912isize, "CNTFRQ_EL0"),
    (22920isize, "MAIR_EL1"),
    (22928isize, "R5"),
    (22936isize, "_HRMR"),
    (22944isize, "_HACTLR2"),
    (22952isize, "ESR_EL1"),
    (22960isize, "ICC_SRE_EL1_NS"),
    (22968isize, "_PAR_EL1"),
    (22984isize, "R3"),
    (22992isize, "ShouldAdvanceSS"),
    (23000isize, "FEAT_SME_F64F64_IMPLEMENTED"),
    (23008isize, "BRBTS_EL1"),
    (23016isize, "_ICV_AP1R"),
    (23032isize, "FEAT_MTE4_IMPLEMENTED"),
    (23040isize, "_DBGDSCRint"),
    (23048isize, "_DSPSR2"),
    (23056isize, "SPESampleCounterValid"),
    (23088isize, "_DISR"),
    (23096isize, "R26"),
    (23104isize, "VBAR_EL3"),
    (23112isize, "MECID_A1_EL2"),
    (23120isize, "RMR_EL2"),
    (23128isize, "_ID_DFR1"),
    (23136isize, "_ICV_PMR"),
    (23144isize, "_CNTV_CVAL"),
    (23152isize, "R10"),
    (23160isize, "FEAT_BF16_IMPLEMENTED"),
    (23168isize, "FEAT_THE_IMPLEMENTED"),
    (23176isize, "TTBR0_EL3"),
    (23184isize, "ICC_IAR1_EL1"),
    (23192isize, "R16"),
    (23200isize, "_PMOVSSET"),
    (23208isize, "_DBGDTRTXext"),
    (23216isize, "CTICIDR3"),
    (23224isize, "FEAT_PMUv3_EXT64_IMPLEMENTED"),
    (23232isize, "FEAT_SEBEP_IMPLEMENTED"),
    (23240isize, "_REVIDR"),
    (23248isize, "FEAT_I8MM_IMPLEMENTED"),
    (23256isize, "__CNTEL0BaseN"),
    (23264isize, "FEAT_ETE_IMPLEMENTED"),
    (23272isize, "__GICDistBase"),
    (23280isize, "CCSIDR_EL1"),
    (23288isize, "FEAT_EPAC_IMPLEMENTED"),
    (23296isize, "_DBGWVR"),
    (23360isize, "__feat_rpres"),
    (23368isize, "ID_ISAR3_EL1"),
    (23376isize, "__gmid_log2_block_size"),
    (23392isize, "GICM_SETSPI_SR"),
    (23400isize, "GITS_SGIR"),
    (23408isize, "__PMUBase"),
    (23416isize, "_VDFSR"),
    (23424isize, "TPIDR_EL0"),
    (23432isize, "EDDEVID"),
    (23440isize, "GICV_EOIR"),
    (23448isize, "ICV_DIR_EL1"),
    (23456isize, "_HTCR"),
    (23464isize, "_PMEVTYPER"),
    (23592isize, "ERXPFGCTL_EL1"),
    (23600isize, "_PMCEID1"),
    (23608isize, "_AMCNTENCLR0"),
    (23616isize, "RCWMASK_EL1"),
    (23632isize, "CNTV_CVAL_EL0"),
    (23640isize, "__cpy_mops_option_a_supported"),
    (23648isize, "BRBSRC_EL1"),
    (23904isize, "GITS_IIDR"),
    (23912isize, "R24"),
    (23920isize, "FEAT_CSV2_IMPLEMENTED"),
    (23928isize, "RNDR"),
    (23936isize, "__syncAbortOnSoWrite"),
    (23944isize, "GICM_IIDR"),
    (23952isize, "_ZA"),
    (89488isize, "GICD_TYPER"),
    (89496isize, "RMR_EL1"),
    (89504isize, "GICC_PMR"),
    (89512isize, "FEAT_MTE_IMPLEMENTED"),
    (89520isize, "FEAT_MPAMv0p1_IMPLEMENTED"),
    (89528isize, "__cpyf_mops_option_a_supported"),
    (89536isize, "ICV_EOIR1_EL1"),
    (89544isize, "ICC_MGRPEN1"),
    (89552isize, "_ERXCTLR2"),
    (89560isize, "PIR_EL2"),
    (89568isize, "FEAT_SPECRES_IMPLEMENTED"),
    (89576isize, "_CNTHP_CTL"),
    (89584isize, "FEAT_TRBE_EXT_IMPLEMENTED"),
    (89592isize, "RVBAR_EL2"),
    (89600isize, "_ID_MMFR2"),
    (89608isize, "ID_MMFR0_EL1"),
    (89616isize, "FEAT_XNX_IMPLEMENTED"),
    (89624isize, "AMAIR_EL1"),
    (89632isize, "PMUEventAccumulator"),
    (90128isize, "SP_EL0"),
    (90136isize, "_ICH_VMCR"),
    (90144isize, "__mpam_major"),
    (90152isize, "FEAT_E0PD_IMPLEMENTED"),
    (90160isize, "EDPIDR4"),
    (90168isize, "MECID_P1_EL2"),
    (90176isize, "_DBGBCR"),
    (90240isize, "FEAT_GICv3_LEGACY_IMPLEMENTED"),
    (90248isize, "SMPRIMAP_EL2"),
    (90256isize, "__supported_pa_size"),
    (90272isize, "SCTLR_EL1"),
    (90280isize, "__syncAbortOnDeviceRead"),
    (90288isize, "FEAT_Debugv8p1_IMPLEMENTED"),
    (90296isize, "FEAT_TME_IMPLEMENTED"),
    (90304isize, "DBGPRCR_EL1"),
    (90312isize, "ID_MMFR4_EL1"),
    (90320isize, "PMINTENSET_EL1"),
    (90328isize, "v8Ap4_IMPLEMENTED"),
    (90336isize, "_CNTHPS_CTL"),
    (90344isize, "ICV_BPR0_EL1"),
    (90352isize, "CPTR_EL3_ESM_VALUE"),
    (90368isize, "FEAT_AFP_IMPLEMENTED"),
    (90376isize, "GITS_CWRITER"),
    (90384isize, "_ICC_IGRPEN1_NS"),
    (90392isize, "__mpam_has_hcr"),
    (90400isize, "__empam_force_ns_RAO"),
    (90408isize, "ID_ISAR6_EL1"),
    (90416isize, "SCXTNUM_EL0"),
    (90424isize, "MVFR0_EL1"),
    (90432isize, "_DFAR_NS"),
    (90440isize, "_HACR"),
    (90448isize, "FEAT_PMUv3p8_IMPLEMENTED"),
    (90456isize, "_DBGCLAIMSET"),
    (90464isize, "GICR_INMIR0"),
    (90472isize, "NUM_AMU_CG0_MONITORS"),
    (90488isize, "CTIPIDR4"),
    (90496isize, "AMUSERENR_EL0"),
    (90504isize, "MPAM2_EL2"),
    (90512isize, "PMBPTR_EL1"),
    (90520isize, "_ZT0"),
    (90584isize, "FEAT_SVE_SHA3_IMPLEMENTED"),
    (90592isize, "_HSTR"),
    (90600isize, "ID_AA64MMFR2_EL1"),
    (90608isize, "ID_AA64ISAR0_EL1"),
    (90616isize, "_DBGOSECCR"),
    (90624isize, "AMPIDR4"),
    (90632isize, "ICC_SGI0R_EL1"),
    (90640isize, "BRBCR_EL1"),
    (90648isize, "SPSR_EL1"),
    (90656isize, "_PMCR"),
    (90664isize, "_ICC_IGRPEN1_S"),
    (90672isize, "_ICH_EISR"),
    (90680isize, "__GIC_Active"),
    (90688isize, "ESR_EL2"),
    (90696isize, "FEAT_PAN2_IMPLEMENTED"),
    (90704isize, "SCR_EL3"),
    (90712isize, "PAR_S"),
    (90720isize, "FEAT_WFxT_IMPLEMENTED"),
    (90728isize, "ID_MMFR3_EL1"),
    (90736isize, "CSSELR_S"),
    (90744isize, "_ICC_HSRE"),
    (90752isize, "CNTNSAR"),
    (90760isize, "FEAT_PMUv3_TH_IMPLEMENTED"),
    (90768isize, "FEAT_HBC_IMPLEMENTED"),
    (90776isize, "FEAT_SME_F16F16_IMPLEMENTED"),
    (90784isize, "NUM_AMU_CG1_MONITORS"),
    (90800isize, "OSLAR_EL1"),
    (90808isize, "MECIDR_EL2"),
    (90816isize, "MVFR2_EL1"),
    (90824isize, "_PMCEID3"),
    (90832isize, "CNTP_CTL_EL0"),
    (90840isize, "FEAT_CLRBHB_IMPLEMENTED"),
    (90848isize, "FEAT_MTE2_IMPLEMENTED"),
    (90856isize, "_PMCNTENCLR"),
    (90864isize, "MPAMVPM3_EL2"),
    (90872isize, "ID_MMFR1_EL1"),
    (90880isize, "ICV_NMIAR1_EL1"),
    (90888isize, "FEAT_SVE_B16B16_IMPLEMENTED"),
    (90896isize, "v9Ap2_IMPLEMENTED"),
    (90904isize, "FEAT_FPACCOMBINE_IMPLEMENTED"),
    (90912isize, "BTypeNext"),
    (90920isize, "FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED"),
    (90928isize, "SMCR_EL3_LEN_VALUE"),
    (90944isize, "ID_PFR1_EL1"),
    (90952isize, "_ERXMISC6"),
    (90960isize, "SMCR_EL3"),
    (90968isize, "SP_EL2"),
    (90976isize, "_ICV_EOIR0"),
    (90984isize, "FEAT_SVE_SM4_IMPLEMENTED"),
    (90992isize, "_CNTVOFF"),
    (91000isize, "__mte_implemented"),
    (91008isize, "CONTEXTIDR_EL2"),
    (91016isize, "SPSR_irq"),
    (91024isize, "_TTBR0_EL2"),
    (91040isize, "JMCR"),
    (91048isize, "ICV_IAR1_EL1"),
    (91056isize, "__empam_force_ns_implemented"),
    (91064isize, "_SPSR_hyp"),
    (91072isize, "ICC_AP0R_EL1"),
    (91104isize, "GICC_RPR"),
    (91112isize, "_HACTLR"),
    (91120isize, "GICR_ISENABLER0"),
    (91128isize, "SMPRI_EL1"),
    (91136isize, "TSTATE"),
    (100208isize, "MVBAR"),
    (100216isize, "CNTV_TVAL_EL0"),
    (100224isize, "MPAMVPM0_EL2"),
    (100232isize, "VariantImplemented"),
    (100248isize, "_DBGVCR"),
    (100256isize, "ID_AA64SMFR0_EL1"),
    (100264isize, "FEAT_PMULL_IMPLEMENTED"),
    (100272isize, "FEAT_PAN_IMPLEMENTED"),
    (100280isize, "MFAR_EL3"),
    (100288isize, "Records_INF"),
    (100800isize, "CTIPIDR3"),
    (100808isize, "FEAT_FPAC_IMPLEMENTED"),
    (100816isize, "GMID_EL1"),
    (100824isize, "VSESR_EL2"),
    (100832isize, "CNTHPS_TVAL_EL2"),
    (100840isize, "NMRR_S"),
    (100848isize, "_ID_MMFR4"),
    (100856isize, "_ICH_VTR"),
    (100864isize, "EDDEVID1"),
    (100872isize, "PMCIDR1"),
    (100880isize, "GICR_INVALLR"),
    (100888isize, "FEAT_EDHSR_IMPLEMENTED"),
    (100896isize, "FEAT_NV_IMPLEMENTED"),
    (100904isize, "FEAT_SYSINSTR128_IMPLEMENTED"),
    (100912isize, "CNTHP_CTL_EL2"),
    (100920isize, "APIBKeyHi_EL1"),
    (100928isize, "CNTP_TVAL_EL0"),
    (100936isize, "FEAT_S2FWB_IMPLEMENTED"),
    (100944isize, "FEAT_AA32EL2_IMPLEMENTED"),
    (100952isize, "R8"),
    (100960isize, "_ICC_CTLR_NS"),
    (100968isize, "_EDECCR"),
    (100976isize, "CCSIDR2_EL1"),
    (100984isize, "MPAMVPM5_EL2"),
    (100992isize, "HFGWTR_EL2"),
    (101000isize, "SMIDR_EL1"),
    (101008isize, "_ERXMISC2"),
    (101016isize, "FEAT_LS64_ACCDATA_IMPLEMENTED"),
    (101024isize, "FEAT_ITE_IMPLEMENTED"),
    (101032isize, "CTIDEVARCH"),
    (101040isize, "S2POR_EL1"),
    (101048isize, "GICD_CLRSPI_NSR"),
    (101056isize, "GCSCR_EL1"),
    (101064isize, "FEAT_GCS_IMPLEMENTED"),
    (101072isize, "FEAT_Debugv8p4_IMPLEMENTED"),
    (101080isize, "_TTBCR_NS"),
    (101088isize, "LORN_EL1"),
    (101096isize, "FEAT_PACQARMA3_IMPLEMENTED"),
    (101104isize, "_RMR"),
    (101112isize, "FEAT_PMUv3p7_IMPLEMENTED"),
    (101120isize, "R7"),
    (101128isize, "__emulator_termination_opcode"),
    (101144isize, "_PMOVSR"),
    (101152isize, "__monomorphize_writes"),
    (101160isize, "__ExclusiveMonitorSet"),
    (101168isize, "FEAT_FlagM_IMPLEMENTED"),
    (101176isize, "TLBTR"),
    (101184isize, "FEAT_SHA3_IMPLEMENTED"),
    (101192isize, "FEAT_TLBIRANGE_IMPLEMENTED"),
    (101200isize, "IsWFIsleep"),
    (101208isize, "PMSFCR_EL1"),
    (101216isize, "ICC_IGRPEN1_EL1_S"),
    (101224isize, "HDFGRTR2_EL2"),
    (101232isize, "CTIPIDR1"),
    (101240isize, "_MPIDR"),
    (101248isize, "Records_TGT"),
    (101760isize, "EDPIDR3"),
    (101768isize, "EDDEVID2"),
    (101776isize, "PMIAR_EL1"),
    (101784isize, "GICR_PROPBASER"),
    (101792isize, "v9Ap4_IMPLEMENTED"),
    (101800isize, "TTBR0_S"),
    (101808isize, "GICV_CTLR"),
    (101816isize, "PMSICR_EL1"),
    (101824isize, "ID_AA64PFR0_EL1"),
    (101832isize, "FEAT_TTL_IMPLEMENTED"),
    (101840isize, "FEAT_LS64_IMPLEMENTED"),
    (101848isize, "FEAT_HPDS_IMPLEMENTED"),
    (101856isize, "v8Ap9_IMPLEMENTED"),
    (101864isize, "_DBGDTRTXint"),
    (101872isize, "JIDR"),
    (101880isize, "DBGWFAR"),
    (101888isize, "GICV_AIAR"),
    (101896isize, "ZCR_EL1"),
    (101904isize, "FEAT_ETMv4_IMPLEMENTED"),
    (101912isize, "RMR_EL3"),
    (101920isize, "AMCNTENCLR1_EL0"),
    (101928isize, "PMEVCNTSVR_EL1"),
    (102176isize, "NUM_GIC_PRIORITY_BITS"),
    (102192isize, "_ICV_HPPIR0"),
    (102200isize, "PMLSR"),
    (102208isize, "DCZID_EL0"),
    (102216isize, "_ICV_IGRPEN1"),
    (102224isize, "__DCACHE_CCSIDR_RESET"),
    (102280isize, "FEAT_RPRFM_IMPLEMENTED"),
    (102288isize, "DBGVCR32_EL2"),
    (102296isize, "CTIDEVID"),
    (102304isize, "BRBTGTINJ_EL1"),
    (102312isize, "FEAT_DoubleLock_IMPLEMENTED"),
    (102320isize, "_ID_MMFR3"),
    (102328isize, "_SDER"),
    (102336isize, "FEAT_SM4_IMPLEMENTED"),
    (102344isize, "MPAMSM_EL1"),
    (102352isize, "FEAT_TRF_IMPLEMENTED"),
    (102360isize, "PIRE0_EL2"),
    (102368isize, "_ICC_HPPIR1"),
    (102376isize, "EDCIDR0"),
    (102384isize, "FEAT_CNTSC_IMPLEMENTED"),
    (102392isize, "__trickbox_enabled"),
    (102400isize, "AMPIDR3"),
    (102408isize, "FEAT_CCIDX_IMPLEMENTED"),
    (102416isize, "_ICC_DIR"),
    (102424isize, "PMLAR"),
    (102432isize, "FEAT_SM3_IMPLEMENTED"),
    (102440isize, "CFG_RVBAR"),
    (102448isize, "_FPEXC"),
    (102456isize, "ICV_BPR1_EL1"),
    (102464isize, "ACCDATA_EL1"),
    (102472isize, "ERXMISC2_EL1"),
    (102480isize, "FEAT_VHE_IMPLEMENTED"),
    (102488isize, "NSACR"),
    (102496isize, "__CTIBase"),
    (102504isize, "CTILSR"),
    (102512isize, "_ISR"),
    (102520isize, "InGuardedPage"),
    (102528isize, "ICC_BPR1_EL1_S"),
    (102536isize, "_ERRSELR"),
    (102544isize, "GICV_AEOIR"),
    (102552isize, "HCR_EL2"),
    (102560isize, "ID_ISAR2_EL1"),
    (102568isize, "MECID_RL_A_EL3"),
    (102576isize, "FEAT_EL0_IMPLEMENTED"),
    (102584isize, "DSPSR_EL0"),
    (102592isize, "FEAT_D128_IMPLEMENTED"),
    (102600isize, "_DFSR_NS"),
    (102608isize, "GICD_STATUSR"),
    (102616isize, "FAR_EL2"),
    (102624isize, "PMUSERENR_EL0"),
    (102632isize, "FEAT_SSBS2_IMPLEMENTED"),
    (102640isize, "_ID_ISAR5"),
    (102648isize, "SPESampleCounterPending"),
    (102680isize, "SCTLR2_EL2"),
    (102688isize, "POR_EL0"),
    (102696isize, "R12"),
    (102704isize, "FEAT_PRFMSLC_IMPLEMENTED"),
    (102712isize, "R22"),
    (102720isize, "_MVFR2"),
    (102728isize, "GICV_PMR"),
    (102736isize, "GICR_INVLPIR"),
    (102744isize, "_ACTLR_NS"),
    (102752isize, "FEAT_LOR_IMPLEMENTED"),
    (102760isize, "v9Ap1_IMPLEMENTED"),
    (102768isize, "R11"),
    (102776isize, "PMICNTSVR_EL1"),
    (102784isize, "HFGRTR2_EL2"),
    (102792isize, "__num_ctx_breakpoints"),
    (102808isize, "_ID_AFR0"),
    (102816isize, "_ConfigReg"),
    (102824isize, "PAR_NS"),
    (102832isize, "PMDEVID"),
    (102840isize, "PFAR_EL1"),
    (102848isize, "_HCR"),
    (102856isize, "OSDLR_EL1"),
    (102864isize, "FEAT_SPEv1p4_IMPLEMENTED"),
    (102872isize, "FeatureImpl"),
    (103136isize, "ICC_HPPIR0_EL1"),
    (103144isize, "ID_AA64MMFR1_EL1"),
    (103152isize, "CNTHV_CVAL_EL2"),
    (103160isize, "ID_MMFR2_EL1"),
    (103168isize, "_HCPTR"),
    (103176isize, "SCXTNUM_EL1"),
    (103184isize, "DBGDTRRX_EL0"),
    (103192isize, "__setg_mops_option_a_supported"),
    (103200isize, "_DSPSR"),
    (103208isize, "EDPRCR"),
    (103216isize, "FEAT_DIT_IMPLEMENTED"),
    (103224isize, "FEAT_MPAM_IMPLEMENTED"),
    (103232isize, "_ID_ISAR0"),
    (103240isize, "AMEVCNTR1_EL0"),
    (103368isize, "_HMAIR0"),
    (103376isize, "FEAT_AA32EL1_IMPLEMENTED"),
    (103384isize, "ERXSTATUS_EL1"),
    (103392isize, "GICH_HCR"),
    (103400isize, "DFSR_S"),
    (103408isize, "FEAT_GICv4p1_IMPLEMENTED"),
    (103416isize, "MIDR_EL1"),
    (103424isize, "DBGBVR_EL1"),
    (103936isize, "FEAT_RAS_IMPLEMENTED"),
    (103944isize, "PMSWINC_EL0"),
    (103952isize, "CNTPS_TVAL_EL1"),
    (103960isize, "PMCGCR0"),
    (103968isize, "FEAT_NMI_IMPLEMENTED"),
    (103976isize, "FEAT_LPA2_IMPLEMENTED"),
    (103984isize, "DBGWCR_EL1"),
    (104496isize, "PMICFILTR_EL0"),
    (104504isize, "FEAT_MTE3_IMPLEMENTED"),
    (104512isize, "_AMCNTENSET1"),
    (104520isize, "__g1_activity_monitor_implemented"),
    (104528isize, "FEAT_SPEv1p3_IMPLEMENTED"),
    (104536isize, "GICV_BPR"),
    (104544isize, "FEAT_TTCNP_IMPLEMENTED"),
    (104552isize, "FEAT_LRCPC2_IMPLEMENTED"),
    (104560isize, "_DBGDCCINT"),
    (104568isize, "SPESampleContextEL1"),
    (104576isize, "__CNTControlBase"),
    (104584isize, "GICD_IIDR"),
    (104592isize, "PMPIDR4"),
    (104600isize, "CTIDEVID2"),
    (104608isize, "FEAT_AA32BF16_IMPLEMENTED"),
    (104616isize, "FEAT_BRBE_IMPLEMENTED"),
    (104624isize, "FEAT_AA32I8MM_IMPLEMENTED"),
    (104632isize, "PIR_EL1"),
    (104640isize, "PMOVSSET_EL0"),
    (104648isize, "MDSCR_EL1"),
    (104656isize, "FEAT_ETMv4p3_IMPLEMENTED"),
    (104664isize, "ID_AA64ZFR0_EL1"),
    (104672isize, "__g1_activity_monitor_offset_implemented"),
    (104680isize, "ACTLR_EL1"),
    (104688isize, "_CLIDR"),
    (104696isize, "__ThisInstr"),
    (104704isize, "_CNTHVS_CTL"),
    (104712isize, "FEAT_S2POE_IMPLEMENTED"),
    (104720isize, "ID_DFR1_EL1"),
    (104728isize, "__has_spe_pseudo_cycles"),
    (104736isize, "FEAT_MTPMU_IMPLEMENTED"),
    (104744isize, "DBGOSLAR"),
    (104752isize, "__ExtDebugBase"),
    (104760isize, "TFSR_EL2"),
    (104768isize, "TFSR_EL3"),
    (104776isize, "PMCCNTR_EL0"),
    (104784isize, "_DBGAUTHSTATUS"),
    (104792isize, "ShouldAdvanceIT"),
    (104800isize, "ID_AA64DFR1_EL1"),
    (104808isize, "AMCNTENSET0_EL0"),
    (104816isize, "_ICC_BPR1_S"),
    (104824isize, "_ICC_PMR"),
    (104832isize, "CTIDEVID1"),
    (104840isize, "HFGITR2_EL2"),
    (104848isize, "AMCG1IDR_EL0"),
    (104856isize, "SPESampleEvents"),
    (104864isize, "FEAT_DoubleFault2_IMPLEMENTED"),
    (104872isize, "FAR_EL3"),
    (104880isize, "MDCR_EL2"),
    (104888isize, "PMOVSCLR_EL0"),
    (104896isize, "__syncAbortOnWriteNormNonCache"),
    (104904isize, "MVFR1_EL1"),
    (104912isize, "TPIDR2_EL0"),
    (104920isize, "SPNIDEN"),
    (104928isize, "PMSCR_EL2"),
    (104936isize, "HSTR_EL2"),
    (104944isize, "CTICIDR2"),
    (104952isize, "GICV_ABPR"),
    (104960isize, "FEAT_JSCVT_IMPLEMENTED"),
    (104968isize, "FEAT_MPAMv1p1_IMPLEMENTED"),
    (104976isize, "FEAT_PMUv3p4_IMPLEMENTED"),
    (104984isize, "PMPIDR1"),
    (104992isize, "FEAT_GICv3_TDIR_IMPLEMENTED"),
    (105000isize, "R17"),
    (105008isize, "_AMAIR0_NS"),
];
// Variable length bitvector implementation
//
// Operations must zero unused bits before returning
#[derive(Default, Clone, Copy, Debug)]
pub struct Bits {
    value: u128,
    length: u16,
}
impl Bits {
    pub fn new(value: u128, length: u16) -> Self {
        Self { value, length }.normalize()
    }
    pub fn value(&self) -> u128 {
        self.value
    }
    pub fn length(&self) -> u16 {
        self.length
    }
    fn normalize(self) -> Self {
        let mask = 1u128
            .checked_shl(u32::from(self.length()))
            .map(|i| i - 1)
            .unwrap_or(!0);
        Self {
            value: self.value() & mask,
            length: self.length(),
        }
    }
    pub fn zero_extend(&self, i: i128) -> Self {
        let length = u16::try_from(i).unwrap();
        Self {
            value: self.value(),
            length,
        }
            .normalize()
    }
    pub fn sign_extend(&self, i: i128) -> Self {
        let length = u16::try_from(i).unwrap();
        let shift_amount = 128 - self.length();
        Self {
            value: (((self.value() as i128) << shift_amount) >> shift_amount) as u128,
            length,
        }
            .normalize()
    }
    pub fn truncate(&self, i: i128) -> Self {
        Self {
            value: self.value(),
            length: u16::try_from(i).unwrap(),
        }
            .normalize()
    }
}
impl core::ops::Shl<i128> for Bits {
    type Output = Self;
    fn shl(self, rhs: i128) -> Self::Output {
        Self {
            value: self.value().checked_shl(u32::try_from(rhs).unwrap()).unwrap_or(0),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Shr<i128> for Bits {
    type Output = Self;
    fn shr(self, rhs: i128) -> Self::Output {
        Self {
            value: self.value().checked_shr(u32::try_from(rhs).unwrap()).unwrap_or(0),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Shl for Bits {
    type Output = Self;
    fn shl(self, rhs: Bits) -> Self::Output {
        Self {
            value: self
                .value()
                .checked_shl(u32::try_from(rhs.value()).unwrap())
                .unwrap_or(0),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::BitAnd for Bits {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value() & rhs.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::BitOr for Bits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value() | rhs.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::BitXor for Bits {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value() ^ rhs.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Add for Bits {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value().wrapping_add(rhs.value()),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Sub for Bits {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value().wrapping_sub(rhs.value()),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::ops::Not for Bits {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self {
            value: !self.value(),
            length: self.length(),
        }
            .normalize()
    }
}
impl core::cmp::PartialEq for Bits {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl core::cmp::Eq for Bits {}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType396b95aa74979079 {
    pub _0: u32,
    pub _1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeb78df3ce1505b121 {
    pub _0: u64,
    pub _1: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypec716851b6df2cc69 {
    pub _0: i128,
    pub _1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType2fc9d3588999ac79 {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
    pub _6: bool,
    pub _7: bool,
    pub _8: bool,
    pub _9: bool,
    pub _10: bool,
    pub _11: bool,
    pub _12: bool,
    pub _13: bool,
    pub _14: bool,
    pub _15: bool,
    pub _16: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTyped54bc449dd09e5bd {
    pub _0: Bits,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType5c38c56b0a400358 {
    pub _0: SumType74cedc916365e5ab,
    pub _1: Bits,
    pub _2: i128,
    pub _3: SumType4e7ec44c61baf335,
    pub _4: SumTypeb20592b6489a79bd,
    pub _5: SumType755586eec3e2b646,
    pub _6: SumType755586eec3e2b646,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType6ae80b81f3d514ff {
    pub _0: u32,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypefb7b2cabacce34a2 {
    pub _0: u64,
    pub _1: u16,
    pub _2: u32,
    pub _3: bool,
    pub _4: bool,
    pub _5: u64,
    pub _6: bool,
    pub _7: u32,
    pub _8: u32,
    pub _9: u32,
    pub _10: u32,
    pub _11: u32,
    pub _12: u8,
    pub _13: u8,
    pub _14: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType1d757adad216cdef {
    pub _0: ProductType9878976b5bcce9c9,
    pub _1: bool,
    pub _2: u8,
    pub _3: bool,
    pub _4: u8,
    pub _5: bool,
    pub _6: ProductType396b95aa74979079,
    pub _7: bool,
    pub _8: ProductTypeda0231e9dc169f81,
    pub _9: i128,
    pub _10: u32,
    pub _11: bool,
    pub _12: ProductTypeda0231e9dc169f81,
    pub _13: bool,
    pub _14: bool,
    pub _15: bool,
    pub _16: u32,
    pub _17: bool,
    pub _18: bool,
    pub _19: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType201519a0f62623dc {
    pub _0: ProductType1d757adad216cdef,
    pub _1: ProductType96e7acababe246a1,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType7b8639ca40b2f578 {
    pub _0: i128,
    pub _1: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType5c790c8ef59cc8b2 {
    pub _0: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypebc91b195b0b2a883 {
    pub _0: Bits,
    pub _1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType3b8bd97143a1dd5c {
    pub _0: ProductType1d757adad216cdef,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType8ae001a7cc8b5154 {
    pub _0: u32,
    pub _1: u16,
    pub _2: u32,
    pub _3: u32,
    pub _4: u32,
    pub _5: bool,
    pub _6: bool,
    pub _7: i128,
    pub _8: u32,
    pub _9: ProductTypeda0231e9dc169f81,
    pub _10: u64,
    pub _11: u32,
    pub _12: i128,
    pub _13: u32,
    pub _14: bool,
    pub _15: u64,
    pub _16: u16,
    pub _17: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypef8f4cc6b86c26dd9 {
    pub _0: ProductType1d757adad216cdef,
    pub _1: ProductTypece7c66ccb2cab13e,
    pub _2: ProductType96e7acababe246a1,
    pub _3: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType63dc1b957c45bf6b {
    pub _0: bool,
    pub _1: bool,
    pub _2: u16,
    pub _3: bool,
    pub _4: bool,
    pub _5: u32,
    pub _6: u32,
    pub _7: u8,
    pub _8: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType148d093dc0428500 {
    pub _0: u64,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTyped8f896a024a4e2cb {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType887a02170738ab3b {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: u64,
    pub _4: u64,
    pub _5: u64,
    pub _6: u64,
    pub _7: bool,
    pub _8: u64,
    pub _9: [u64; 16usize],
    pub _10: i128,
    pub _11: u64,
    pub _12: [u64; 31usize],
    pub _13: [u64; 32usize],
    pub _14: i128,
    pub _15: u64,
    pub _16: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType183e6678e5239c85 {
    pub _0: u8,
    pub _1: u8,
    pub _2: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType6910a69d11d0fc51 {
    pub _0: ProductType1d757adad216cdef,
    pub _1: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeef284266e139aee2 {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
    pub _6: bool,
    pub _7: bool,
    pub _8: bool,
    pub _9: bool,
    pub _10: bool,
    pub _11: bool,
    pub _12: bool,
    pub _13: bool,
    pub _14: bool,
    pub _15: bool,
    pub _16: u8,
    pub _17: ProductType5c790c8ef59cc8b2,
    pub _18: ProductType5c790c8ef59cc8b2,
    pub _19: bool,
    pub _20: bool,
    pub _21: bool,
    pub _22: bool,
    pub _23: u8,
    pub _24: bool,
    pub _25: ProductType5c790c8ef59cc8b2,
    pub _26: ProductType5c790c8ef59cc8b2,
    pub _27: bool,
    pub _28: u8,
    pub _29: u8,
    pub _30: bool,
    pub _31: u8,
    pub _32: u8,
    pub _33: u8,
    pub _34: bool,
    pub _35: bool,
    pub _36: u32,
    pub _37: u8,
    pub _38: bool,
    pub _39: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeda0231e9dc169f81 {
    pub _0: u64,
    pub _1: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType9d47af446174e9f7 {
    pub _0: bool,
    pub _1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypefe4abf226459da13 {
    pub _0: u32,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType8b847afc727d5818 {
    pub _0: bool,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType9799615a3dcac2c0 {
    pub _0: i128,
    pub _1: u8,
    pub _2: i128,
    pub _3: u64,
    pub _4: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType81b3b4c60b2f37ac {
    pub _0: u32,
    pub _1: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType47719ee8854de2a0 {
    pub _0: u32,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeb05ce25a107f0c5e {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
    pub _6: bool,
    pub _7: bool,
    pub _8: bool,
    pub _9: bool,
    pub _10: u8,
    pub _11: bool,
    pub _12: bool,
    pub _13: u8,
    pub _14: u8,
    pub _15: bool,
    pub _16: bool,
    pub _17: bool,
    pub _18: ProductType5c790c8ef59cc8b2,
    pub _19: bool,
    pub _20: u8,
    pub _21: u8,
    pub _22: u8,
    pub _23: bool,
    pub _24: bool,
    pub _25: u8,
    pub _26: u32,
    pub _27: bool,
    pub _28: bool,
    pub _29: u8,
    pub _30: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType7b38a52e3b2f4e94 {
    pub _0: ProductTypef8c3639b88223255,
    pub _1: ProductTypece7c66ccb2cab13e,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType782ac6922b48c20d {
    pub _0: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeba129578e5d1bd1b {
    pub _0: u32,
    pub _1: u8,
    pub _2: bool,
    pub _3: bool,
    pub _4: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypef506aa96a892fe52 {
    pub _0: Bits,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType145f0f6db15bbddf {
    pub _0: u32,
    pub _1: bool,
    pub _2: f32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType74f83332f678823c {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType5b104d2f9e197511 {
    pub _0: u32,
    pub _1: ProductType9799615a3dcac2c0,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType9fbf57605f07e214 {
    pub _0: u8,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeb525737120e184b3 {
    pub _0: SumType755586eec3e2b646,
    pub _1: ProductTypef170cab34335b70c,
    pub _2: u32,
    pub _3: SumTypebf36e919d71ba1d6,
    pub _4: SumTypefc0aa8a49e605a17,
    pub _5: SumType3cca557f9e907281,
    pub _6: SumType3436044442b382d9,
    pub _7: u64,
    pub _8: SumType755586eec3e2b646,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType2743ddd4af418639 {
    pub _0: ProductTypef8c3639b88223255,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeb7f99f96751e17c4 {
    pub _0: bool,
    pub _1: u32,
    pub _2: u64,
    pub _3: bool,
    pub _4: ProductTypeda0231e9dc169f81,
    pub _5: bool,
    pub _6: u32,
    pub _7: u32,
    pub _8: bool,
    pub _9: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType7f884a90ebc7c1c1 {
    pub _0: u32,
    pub _1: bool,
    pub _2: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType2cbba2b6f5a5e45a {
    pub _0: u32,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeee12e330a5f80ce {
    pub _0: SumType74cedc916365e5ab,
    pub _1: Bits,
    pub _2: i128,
    pub _3: bool,
    pub _4: SumTypeb20592b6489a79bd,
    pub _5: SumType755586eec3e2b646,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypebf05c51f33174538 {
    pub _0: u8,
    pub _1: u8,
    pub _2: bool,
    pub _3: u8,
    pub _4: u8,
    pub _5: bool,
    pub _6: bool,
    pub _7: u8,
    pub _8: bool,
    pub _9: u8,
    pub _10: u8,
    pub _11: bool,
    pub _12: bool,
    pub _13: bool,
    pub _14: u8,
    pub _15: bool,
    pub _16: bool,
    pub _17: bool,
    pub _18: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType37abbcb1894e7c56 {
    pub _0: bool,
    pub _1: u8,
    pub _2: u64,
    pub _3: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType2b2aba4822138824 {
    pub _0: ProductTypef8c3639b88223255,
    pub _1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType90c39552810120fd {
    pub _0: u32,
    pub _1: i64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypec1bd230b943b3b8c {
    pub _0: i128,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeb4cea7287e2eb9d6 {
    pub _0: ProductType1d757adad216cdef,
    pub _1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypead6b611358cb4242 {
    pub _0: Bits,
    pub _1: SumType4e7ec44c61baf335,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeddb1e26c564d218c {
    pub _0: u8,
    pub _1: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType3121c658f1e84c22 {
    pub _0: u32,
    pub _1: ProductTypeecb3a6c821d7caab,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType963c597a88a9ddbc {
    pub _0: u64,
    pub _1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType9878976b5bcce9c9 {
    pub _0: bool,
    pub _1: u32,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
    pub _5: u32,
    pub _6: u32,
    pub _7: bool,
    pub _8: u8,
    pub _9: bool,
    pub _10: bool,
    pub _11: bool,
    pub _12: bool,
    pub _13: bool,
    pub _14: u32,
    pub _15: bool,
    pub _16: ProductTypee79b4310dbe79c8c,
    pub _17: bool,
    pub _18: bool,
    pub _19: u32,
    pub _20: bool,
    pub _21: bool,
    pub _22: bool,
    pub _23: bool,
    pub _24: bool,
    pub _25: u32,
    pub _26: bool,
    pub _27: bool,
    pub _28: bool,
    pub _29: bool,
    pub _30: bool,
    pub _31: u32,
    pub _32: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTyped9cc76446c2fc207 {
    pub _0: i128,
    pub _1: i128,
    pub _2: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypea5cc8de4daab131c {
    pub _0: bool,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeaf9a543d074bfcd1 {
    pub _0: ProductType1d757adad216cdef,
    pub _1: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType690b94b58c91cec7 {
    pub _0: u32,
    pub _1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypec31788cf6e435e32 {
    pub _0: u8,
    pub _1: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType700c18a878c5601b {
    pub _0: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypedc31059ca7e2391c {
    pub _0: ProductType1d757adad216cdef,
    pub _1: ProductTypece7c66ccb2cab13e,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType1bf0d680bd42e964 {
    pub _0: ProductType1d757adad216cdef,
    pub _1: ProductTypece7c66ccb2cab13e,
    pub _2: ProductType96e7acababe246a1,
    pub _3: u128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType42c31f3d0ab6eb17 {
    pub _0: u8,
    pub _1: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType72d61775f103f7e0 {
    pub _0: u32,
    pub _1: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypedd6beaf8dda484f5 {
    pub _0: bool,
    pub _1: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypea17960981c63889e {
    pub _0: i128,
    pub _1: i128,
    pub _2: i64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeb265ba323b463df {
    pub _0: i128,
    pub _1: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType4813027798de1e98 {
    pub _0: u32,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypece7c66ccb2cab13e {
    pub _0: ProductType1d757adad216cdef,
    pub _1: u16,
    pub _2: ProductTypef170cab34335b70c,
    pub _3: ProductTypeda0231e9dc169f81,
    pub _4: bool,
    pub _5: bool,
    pub _6: ProductTypec0d0fb0603850c4c,
    pub _7: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType4b99944cd5e0b59d {
    pub _0: ProductType1d757adad216cdef,
    pub _1: ProductTypece7c66ccb2cab13e,
    pub _2: ProductType96e7acababe246a1,
    pub _3: Bits,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypef8c3639b88223255 {
    pub _0: bool,
    pub _1: u32,
    pub _2: u32,
    pub _3: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypea231b9ca5c98dc3c {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
    pub _6: bool,
    pub _7: bool,
    pub _8: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType3a4c1fd3c81de471 {
    pub _0: ProductTypefb7b2cabacce34a2,
    pub _1: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType1a93b8c16f53fb84 {
    pub _0: bool,
    pub _1: u16,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypef4504d47691e9b88 {
    pub _0: ProductType396b95aa74979079,
    pub _1: ProductType9799615a3dcac2c0,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypec09a9362abf87ee {
    pub _0: ProductType1d757adad216cdef,
    pub _1: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType9741cd95b8c1706c {
    pub _0: ProductTypef8c3639b88223255,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypec98939056e929b9c {
    pub _0: bool,
    pub _1: bool,
    pub _2: u8,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
    pub _6: bool,
    pub _7: u8,
    pub _8: bool,
    pub _9: bool,
    pub _10: u8,
    pub _11: bool,
    pub _12: bool,
    pub _13: u8,
    pub _14: bool,
    pub _15: u8,
    pub _16: bool,
    pub _17: bool,
    pub _18: bool,
    pub _19: bool,
    pub _20: bool,
    pub _21: bool,
    pub _22: bool,
    pub _23: bool,
    pub _24: bool,
    pub _25: bool,
    pub _26: bool,
    pub _27: bool,
    pub _28: bool,
    pub _29: bool,
    pub _30: bool,
    pub _31: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeeb828c17bbe5e68 {
    pub _0: ProductTypee47dd77b186df56e,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType1f0c48777d4d25a0 {
    pub _0: Bits,
    pub _1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypec0d0fb0603850c4c {
    pub _0: u16,
    pub _1: bool,
    pub _2: u64,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
    pub _6: u32,
    pub _7: bool,
    pub _8: i128,
    pub _9: bool,
    pub _10: u32,
    pub _11: u32,
    pub _12: u32,
    pub _13: u16,
    pub _14: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType3b1541e73da39111 {
    pub _0: u64,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeea264718a40d3f4a {
    pub _0: u32,
    pub _1: bool,
    pub _2: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType7a7b8e220c467737 {
    pub _0: ProductType9799615a3dcac2c0,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType6e608f0222d797fa {
    pub _0: bool,
    pub _1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypee501372d76ef32c0 {
    pub _0: &'static str,
    pub _1: i128,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType44ac89053e6d35a9 {
    pub _0: u32,
    pub _1: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypee79b4310dbe79c8c {
    pub _0: u32,
    pub _1: u16,
    pub _2: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypeecb3a6c821d7caab {
    pub _0: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType234df14d4fab6c9d {
    pub _0: ProductType1d757adad216cdef,
    pub _1: ProductTypece7c66ccb2cab13e,
    pub _2: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypef170cab34335b70c {
    pub _0: u32,
    pub _1: ProductType183e6678e5239c85,
    pub _2: u32,
    pub _3: bool,
    pub _4: ProductType183e6678e5239c85,
    pub _5: u32,
    pub _6: u32,
    pub _7: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType96e7acababe246a1 {
    pub _0: ProductTypeda0231e9dc169f81,
    pub _1: bool,
    pub _2: bool,
    pub _3: u8,
    pub _4: bool,
    pub _5: bool,
    pub _6: i128,
    pub _7: ProductTypef170cab34335b70c,
    pub _8: bool,
    pub _9: ProductTypebf05c51f33174538,
    pub _10: bool,
    pub _11: bool,
    pub _12: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType8d21682dd93cdf6d {
    pub _0: ProductTypef8c3639b88223255,
    pub _1: u64,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType4d3ef3a5cd661176 {
    pub _0: u16,
    pub _1: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypec8897aad3eb4a29e {
    pub _0: u32,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypee47dd77b186df56e {
    pub _0: i128,
    pub _1: ProductTypec0d0fb0603850c4c,
    pub _2: i128,
    pub _3: u128,
    pub _4: u128,
    pub _5: ProductType96e7acababe246a1,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductType54c7e1b9151093d0 {
    pub _0: u32,
    pub _1: u8,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypefe062afb059b3bbc {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
    pub _5: bool,
    pub _6: bool,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ProductTypede60d0d1f6e7c94c {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumTyped75edc9926641b8a {
    _0(bool),
    _1(()),
    _2(()),
    _3(()),
    _4(()),
    _5(()),
    _6(()),
    _7(()),
    _8(bool),
    _9(()),
    _10(bool),
}
impl Default for SumTyped75edc9926641b8a {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType38e4b53a8482180f {
    _0(()),
    _1(()),
    _2(&'static str),
    _3(()),
    _4(bool),
    _5(&'static str),
    _6(()),
    _7(()),
}
impl Default for SumType38e4b53a8482180f {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumTypef8de2b264306e832 {
    _0(()),
    _1(u32),
}
impl Default for SumTypef8de2b264306e832 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumTypebfdf2f926abd32c5 {
    _0(u32),
    _1(ProductTypead6b611358cb4242),
}
impl Default for SumTypebfdf2f926abd32c5 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumTypeb20592b6489a79bd {
    _0(()),
    _1(ProductTypeb525737120e184b3),
}
impl Default for SumTypeb20592b6489a79bd {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType7151e9c01acfacea {
    _0(u32),
    _1(SumType4e7ec44c61baf335),
}
impl Default for SumType7151e9c01acfacea {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType4e7ec44c61baf335 {
    _0(()),
    _1(bool),
}
impl Default for SumType4e7ec44c61baf335 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType74cedc916365e5ab {
    _0(SumTyped75edc9926641b8a),
    _1(ProductType72d61775f103f7e0),
    _2(()),
    _3(()),
}
impl Default for SumType74cedc916365e5ab {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType632af2c8e0e1944c {
    _0(ProductType7f884a90ebc7c1c1),
    _1(ProductType7f884a90ebc7c1c1),
    _2(()),
    _3(()),
    _4(()),
    _5(()),
}
impl Default for SumType632af2c8e0e1944c {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumTypea0f5ebb1a394e20b {
    _0(()),
    _1(ProductType1d757adad216cdef),
}
impl Default for SumTypea0f5ebb1a394e20b {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType3436044442b382d9 {
    _0(()),
    _1(ProductTypeb05ce25a107f0c5e),
}
impl Default for SumType3436044442b382d9 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumTypefc0aa8a49e605a17 {
    _0(()),
    _1(ProductTypeef284266e139aee2),
}
impl Default for SumTypefc0aa8a49e605a17 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumTypebf36e919d71ba1d6 {
    _0(()),
    _1(i128),
}
impl Default for SumTypebf36e919d71ba1d6 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType3cca557f9e907281 {
    _0(()),
    _1(ProductType1f0c48777d4d25a0),
}
impl Default for SumType3cca557f9e907281 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SumType755586eec3e2b646 {
    _0(()),
    _1(Bits),
}
impl Default for SumType755586eec3e2b646 {
    fn default() -> Self {
        Self::_0(Default::default())
    }
}
pub trait Tracer {
    fn begin(&self, instruction: u32, pc: u64);
    fn end(&self);
    fn read_register<T: core::fmt::Debug>(&self, offset: isize, value: T);
    fn write_register<T: core::fmt::Debug>(&self, offset: isize, value: T);
    fn read_memory<T: core::fmt::Debug>(&self, address: usize, value: T);
    fn write_memory<T: core::fmt::Debug>(&self, address: usize, value: T);
}
#[derive(Debug)]
pub enum ExecuteResult {
    Ok,
    EndOfBlock,
    UndefinedInstruction,
}
