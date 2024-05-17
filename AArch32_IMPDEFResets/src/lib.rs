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
extern crate alloc;
use u_update_ID_MMFR1_Type_L1UniVA::*;
use u_update_ID_ISAR6_Type_SPECRES::*;
use u_update_ID_ISAR3_Type_SVC::*;
use u_update_ID_ISAR0_Type_Coproc::*;
use u_update_ID_ISAR2_Type_MemHint::*;
use ID_MMFR3_write::*;
use ID_PFR0_read::*;
use u_update_ID_ISAR2_Type_MultiAccessInt::*;
use u_update_ID_PFR0_Type_State2::*;
use Zeros::*;
use u_update_CTR_Type_DIC::*;
use ID_ISAR1_write::*;
use u_update_ID_ISAR1_Type_Immediate::*;
use u_update_ID_MMFR2_Type_HWAccFlg::*;
use u_update_ID_PFR1_Type_Virtualization::*;
use u_update_ID_MMFR2_Type_L1HvdBG::*;
use u_update_ID_ISAR1_Type_Endian::*;
use MVFR0_read::*;
use u_update_MVFR1_Type_SIMDLS::*;
use u_update_CTR_Type_IDC::*;
use u_update_CLIDR_Type_LoUU::*;
use u_update_ID_ISAR4_Type_PSR_M::*;
use u_update_MVFR1_Type_SIMDInt::*;
use u_update_CTR_Type_IminLine::*;
use u_update_MIDR_Type_Revision::*;
use ID_DFR0_read::*;
use u_update_ID_ISAR0_Type_Swap::*;
use u_update_ID_MMFR3_Type_MaintBcst::*;
use u_update_ID_ISAR1_Type_Except_AR::*;
use ID_ISAR2_write::*;
use PMCEID3_write::*;
use integer_subrange::*;
use u_update_ID_ISAR3_Type_SIMD::*;
use u_update_ID_PFR1_Type_ProgMod::*;
use u_update_ID_PFR0_Type_State1::*;
use VPIDR_read::*;
use u_update_ID_MMFR2_Type_HvdTLB::*;
use MPIDR_read::*;
use u_update_SCTLR_Type_UNK::*;
use u_update_ID_PFR0_Type_CSV2::*;
use u_update_CTR_Type_CWG::*;
use u_update_ID_PFR0_Type_DIT::*;
use MIDR_write::*;
use HSCTLR_write::*;
use Mk_NMRR_Type::*;
use MIDR_read::*;
use HasArchVersion::*;
use NMRR_write::*;
use u_update_ID_ISAR3_Type_TabBranch::*;
use HCPTR_write::*;
use ID_ISAR2_read::*;
use u_update_PMCR_Type_P::*;
use u_update_ID_MMFR0_Type_ShareLvl::*;
use ID_ISAR5_write::*;
use u_update_ID_ISAR6_Type_BF16::*;
use u_update_FPEXC_Type_EX::*;
use u_update_HCPTR_Type_TCP10::*;
use u_update_ID_DFR0_Type_TraceFilt::*;
use CTR_read::*;
use u_update_SCTLR_Type_SPAN::*;
use HTCR_read::*;
use u_update_HTCR_Type_HWU60::*;
use u_update_ID_PFR1_Type_Security::*;
use Mk_PMCEID3_Type::*;
use u_update_ID_MMFR0_Type_VMSA::*;
use u_update_VPIDR_Type_PartNum::*;
use CCSIDR_read::*;
use u_get_ID_DFR0_Type_CopDbg::*;
use ID_ISAR4_write::*;
use u_update_ID_ISAR2_Type_Reversal::*;
use u_update_ID_ISAR1_Type_Jazelle::*;
use PMCR_read::*;
use u_update_MVFR1_Type_SIMDFMAC::*;
use Mk_ID_ISAR6_Type::*;
use u_update_ID_ISAR6_Type_FHM::*;
use DBGOSLSR_write::*;
use u_update_ID_ISAR5_Type_RDM::*;
use u_update_ID_MMFR2_Type_MemBarr::*;
use u_update_MVFR0_Type_FPShVec::*;
use u_update_HSCTLR_Type_EE::*;
use u_update_HCPTR_Type_TCP11::*;
use u_get_PMCR_Type_IMP::*;
use u_update_ID_ISAR0_Type_CmpBranch::*;
use u_update_VPIDR_Type_Revision::*;
use u_update_ID_MMFR3_Type_CMaintVA::*;
use u_get_Configuration_Type_ExceptInit::*;
use IsZero::*;
use u_update_MVFR1_Type_SIMDSP::*;
use u_update_ID_ISAR4_Type_SynchPrim_frac::*;
use u_update_ID_MMFR2_Type_UniTLB::*;
use u_update_MPIDR_Type_MT::*;
use u_update_MPIDR_Type_M::*;
use u_update_PRRR_Type_DS1::*;
use u_update_PRRR_Type_NS1::*;
use u_update_HTCR_Type_SH0::*;
use u_update_ID_PFR1_Type_Sec_frac::*;
use CLIDR_read::*;
use u_update_ID_PFR0_Type_State0::*;
use u_update_FPEXC_Type_VV::*;
use u_update_MVFR0_Type_FPDP::*;
use Mk_PMCEID2_Type::*;
use u_update_FPEXC_Type_FP2V::*;
use u_update_FPEXC_Type_VECITR::*;
use u_update_MVFR0_Type_FPSqrt::*;
use MVFR2_write::*;
use ID_DFR1_write::*;
use u_update_ID_MMFR3_Type_CMemSz::*;
use Mk_PMCEID0_Type::*;
use u_update_ID_MMFR2_Type_L1HvdFG::*;
use u_update_VPIDR_Type_Architecture::*;
use u_update_HCPTR_Type_TCPAC::*;
use ID_ISAR1_read::*;
use u_update_HTCR_Type_HWU59::*;
use ID_MMFR5_write::*;
use u_update_ID_ISAR5_Type_SEVL::*;
use ICC_HSRE_read::*;
use u_update_ID_ISAR0_Type_Debug::*;
use u_update_ID_MMFR1_Type_BPred::*;
use u_update_CLIDR_Type_LoUIS::*;
use u_update_MVFR0_Type_SIMDReg::*;
use SCTLR_read__2::*;
use PMCEID1_write::*;
use ID_ISAR0_read::*;
use u_update_ID_PFR0_Type_AMU::*;
use u_update_ID_PFR0_Type_RAS::*;
use u_update_PMCR_Type_C::*;
use PRRR_write::*;
use u_update_ID_PFR0_Type_State3::*;
use PMCEID0_write::*;
use u_update_MVFR0_Type_FPRound::*;
use ID_MMFR1_write::*;
use u_update_MVFR1_Type_FPFtZ::*;
use u_update_VPIDR_Type_Variant::*;
use u_update_ID_DFR0_Type_CopTrc::*;
use u_update_MVFR0_Type_FPSP::*;
use NMRR_read::*;
use ID_MMFR4_read::*;
use u_get_Configuration_Type_CFGEND::*;
use MVFR1_read::*;
use u_update_MVFR0_Type_FPTrap::*;
use u_update_ID_MMFR0_Type_AuxReg::*;
use u_update_CCSIDR_Type_LineSize::*;
use ID_ISAR5_read::*;
use Mk_CLIDR_Type::*;
use u_update_HTCR_Type_IRGN0::*;
use u_update_PRRR_Type_DS0::*;
use u_update_ID_ISAR5_Type_SHA1::*;
use u_update_ID_ISAR4_Type_SWP_frac::*;
use u_update_HDCR_Type_HPMN::*;
use u_update_PMCR_Type_IDCODE::*;
use u_update_ID_ISAR1_Type_Extend::*;
use u_update_ID_ISAR2_Type_MultS::*;
use ID_MMFR3_read::*;
use u_update_ID_ISAR3_Type_SynchPrim::*;
use u_update_MIDR_Type_Variant::*;
use u_update_CCSIDR_Type_Associativity::*;
use u_update_ID_MMFR0_Type_OuterShr::*;
use u_update_ID_ISAR3_Type_TrueNOP::*;
use u_update_HCPTR_Type_TTA::*;
use u_update_ID_DFR0_Type_MProfDbg::*;
use u_update_ID_ISAR2_Type_Mult::*;
use Mk_PMCEID1_Type::*;
use u_update_ID_ISAR2_Type_LoadStore::*;
use u_update_FPEXC_Type_TFV::*;
use u_update_HTCR_Type_HWU61::*;
use ID_MMFR5_read::*;
use u_update_ID_ISAR1_Type_Interwork::*;
use IsFeatureImplemented::*;
use u_update_ID_DFR0_Type_CopSDbg::*;
use ID_ISAR3_write::*;
use ID_ISAR3_read::*;
use u_update_ID_MMFR1_Type_L1HvdVA::*;
use ID_MMFR2_read::*;
use u_update_ID_MMFR5_Type_ETS::*;
use ID_ISAR6_write::*;
use u_update_ID_MMFR1_Type_L1TstCln::*;
use u_update_ID_ISAR0_Type_BitField::*;
use u_update_ID_ISAR1_Type_IfThen::*;
use u_update_PRRR_Type_NS0::*;
use ID_ISAR4_read::*;
use u_update_DBGOSLSR_Type_OSLM::*;
use u_update_ID_MMFR0_Type_InnerShr::*;
use u_update_ID_MMFR3_Type_Supersec::*;
use VPIDR_write::*;
use CCSIDR_write::*;
use u_update_MVFR2_Type_SIMDMisc::*;
use Mk_PRRR_Type::*;
use u_update_ID_ISAR4_Type_Writeback::*;
use u_update_ID_ISAR4_Type_SMC::*;
use u_update_ID_ISAR5_Type_SHA2::*;
use ID_PFR1_write::*;
use u_update_MPIDR_Type_Aff2::*;
use u_get_PMCR_Type_N::*;
use u_update_ID_DFR0_Type_PerfMon::*;
use ID_MMFR1_read::*;
use u_update_ID_MMFR0_Type_FCSE::*;
use u_update_ID_MMFR0_Type_TCM::*;
use u_update_PMCR_Type_N::*;
use PMCEID2_write::*;
use ID_MMFR0_read::*;
use u_update_ID_ISAR2_Type_MultU::*;
use u_update_SCTLR_Type_TE::*;
use DBGOSLSR_read::*;
use u_update_SCTLR_Type_EE::*;
use u_update_SCTLR_Type_V::*;
use u_update_MIDR_Type_Implementer::*;
use u_update_CLIDR_Type_ICB::*;
use HDCR_read::*;
use u_update_VPIDR_Type_Implementer::*;
use u_update_HTCR_Type_T0SZ::*;
use u_update_ID_DFR0_Type_CopDbg::*;
use u_update_ID_ISAR0_Type_Divide::*;
use MVFR2_read::*;
use u_update_MVFR1_Type_FPHP::*;
use u_update_MIDR_Type_PartNum::*;
use ID_MMFR4_write::*;
use CTR_write::*;
use u_update_ID_MMFR2_Type_L1HvdRng::*;
use u_update_ID_ISAR5_Type_AES::*;
use u_update_PMCR_Type_IMP::*;
use u_update_ID_DFR1_Type_MTPMU::*;
use u_update_HCPTR_Type_TASE::*;
use u_update_MPIDR_Type_U::*;
use u_update_HTCR_Type_ORGN0::*;
use u_update_MPIDR_Type_Aff1::*;
use FPEXC_read::*;
use u_update_MVFR1_Type_FPDNaN::*;
use u_update_ID_MMFR2_Type_WFIStall::*;
use SCTLR_write::*;
use u_update_ID_MMFR3_Type_CohWalk::*;
use set_subrange_zeros::*;
use u_update_MIDR_Type_Architecture::*;
use HDCR_write::*;
use u_update_CTR_Type_DminLine::*;
use u_update_ID_MMFR0_Type_PMSA::*;
use HCPTR_read::*;
use u_get_ID_DFR0_EL1_Type_CopDbg::*;
use u_update_ID_MMFR4_Type_CCIDX::*;
use u_update_ID_ISAR6_Type_JSCVT::*;
use u_update_ID_MMFR3_Type_CMaintSW::*;
use u_update_DBGOSLSR_Type_OSLK::*;
use u_update_ID_ISAR0_Type_BitCount::*;
use u_update_ID_MMFR1_Type_L1HvdSW::*;
use HSCTLR_read::*;
use u_update_HTCR_Type_HPD::*;
use u_update_ID_PFR1_Type_GenTimer::*;
use u_update_ID_MMFR3_Type_PAN::*;
use u_update_MVFR0_Type_FPDivide::*;
use u_update_MVFR2_Type_FPMisc::*;
use u_update_ID_ISAR4_Type_Barrier::*;
use u_update_ID_ISAR3_Type_Saturate::*;
use u_update_ID_DFR0_Type_MMapDbg::*;
use ID_MMFR0_write::*;
use ID_PFR0_write::*;
use ID_DFR0_write::*;
use u_update_ID_ISAR1_Type_Except::*;
use u_update_MVFR1_Type_SIMDHP::*;
use u_update_ID_MMFR1_Type_L1UniSW::*;
use u_update_ID_ISAR4_Type_WithShifts::*;
use MPIDR_write::*;
use MVFR1_write::*;
use u_update_ICC_HSRE_Type_SRE::*;
use u_update_ID_MMFR1_Type_L1Hvd::*;
use CLIDR_write::*;
use u_update_ID_ISAR6_Type_I8MM::*;
use ID_MMFR2_write::*;
use u_update_ID_ISAR6_Type_DP::*;
use u_update_HTCR_Type_HWU62::*;
use ICC_HSRE_write::*;
use ID_ISAR6_read::*;
use u_update_ID_ISAR3_Type_T32Copy::*;
use ID_PFR1_read::*;
use PRRR_read::*;
use HTCR_write::*;
use u_update_ID_DFR0_Type_MMapTrc::*;
use u_update_ID_ISAR2_Type_PSR_AR::*;
use u_update_ID_MMFR1_Type_L1Uni::*;
use u_update_ID_MMFR3_Type_BPMaint::*;
use u_update_ID_PFR1_Type_MProgMod::*;
use MVFR0_write::*;
use u_update_ID_ISAR5_Type_VCMA::*;
use u_get_SCR_Type_NS::*;
use u_update_MPIDR_Type_Aff0::*;
use u_update_ID_PFR1_Type_Virt_frac::*;
use ID_DFR1_read::*;
use u_update_CTR_Type_ERG::*;
use u_update_ID_ISAR4_Type_Unpriv::*;
use u_update_ID_ISAR5_Type_CRC32::*;
use u_update_CTR_Type_L1Ip::*;
use u_update_ID_PFR1_Type_GIC::*;
use PMCR_write::*;
use ID_ISAR0_write::*;
use u_update_ID_ISAR3_Type_T32EE::*;
use FPEXC_write::*;
use common::*;
pub fn AArch32_IMPDEFResets<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_327667: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_368422: ProductType700c18a878c5601b,
        gs_328017: bool,
        ga_368418: ProductType700c18a878c5601b,
        ga_367925: u8,
        gs_328028: bool,
        ga_368515: ProductType700c18a878c5601b,
        ga_367924: ProductType700c18a878c5601b,
        ga_368008: ProductType700c18a878c5601b,
        ga_368443: ProductType700c18a878c5601b,
        gs_327782: bool,
        ga_368430: ProductType700c18a878c5601b,
        gs_327667: (),
    }
    let fn_state = FunctionState {
        gs_327667,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call PMCR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_0_0);
        // C s_0_2: const #8s : i
        let s_0_2: i128 = 8;
        // S s_0_3: call Zeros(s_0_2)
        let s_0_3: Bits = Zeros(state, tracer, s_0_2);
        // S s_0_4: cast reint s_0_3 -> u8
        let s_0_4: u8 = (s_0_3.value() as u8);
        // S s_0_5: call _update_PMCR_Type_IMP(s_0_1, s_0_4)
        let s_0_5: ProductType700c18a878c5601b = u_update_PMCR_Type_IMP(
            state,
            tracer,
            s_0_1,
            s_0_4,
        );
        // S s_0_6: call PMCR_write(s_0_5)
        let s_0_6: () = PMCR_write(state, tracer, s_0_5);
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call PMCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_0_7);
        // D s_0_9: write-var ga#367924 <= s_0_8
        fn_state.ga_367924 = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call PMCR_read(s_0_10)
        let s_0_11: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_0_10);
        // S s_0_12: call _get_PMCR_Type_IMP(s_0_11)
        let s_0_12: u8 = u_get_PMCR_Type_IMP(state, tracer, s_0_11);
        // S s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 8u16);
        // S s_0_14: call IsZero(s_0_13)
        let s_0_14: bool = IsZero(state, tracer, s_0_13);
        // N s_0_15: branch s_0_14 b51 b1
        if s_0_14 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1256u : u32
        let s_1_0: u32 = 1256;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var ga#367925 <= s_1_1
        fn_state.ga_367925 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#367924:struct
        let s_2_0: ProductType700c18a878c5601b = fn_state.ga_367924;
        // D s_2_1: read-var ga#367925:u8
        let s_2_1: u8 = fn_state.ga_367925;
        // D s_2_2: call _update_PMCR_Type_IDCODE(s_2_0, s_2_1)
        let s_2_2: ProductType700c18a878c5601b = u_update_PMCR_Type_IDCODE(
            state,
            tracer,
            s_2_0,
            s_2_1,
        );
        // D s_2_3: call PMCR_write(s_2_2)
        let s_2_3: () = PMCR_write(state, tracer, s_2_2);
        // C s_2_4: const #() : ()
        let s_2_4: () = ();
        // S s_2_5: call PMCR_read(s_2_4)
        let s_2_5: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_2_4);
        // C s_2_6: const #4s : i
        let s_2_6: i128 = 4;
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // C s_2_8: const #11104u : u32
        let s_2_8: u32 = 11104;
        // D s_2_9: read-reg s_2_8:i
        let s_2_9: i128 = {
            let value = state.read_register::<i128>(s_2_8 as isize);
            tracer.read_register(s_2_8 as isize, value);
            value
        };
        // D s_2_10: call integer_subrange(s_2_9, s_2_6, s_2_7)
        let s_2_10: Bits = integer_subrange(state, tracer, s_2_9, s_2_6, s_2_7);
        // D s_2_11: cast reint s_2_10 -> u8
        let s_2_11: u8 = (s_2_10.value() as u8);
        // D s_2_12: call _update_PMCR_Type_N(s_2_5, s_2_11)
        let s_2_12: ProductType700c18a878c5601b = u_update_PMCR_Type_N(
            state,
            tracer,
            s_2_5,
            s_2_11,
        );
        // D s_2_13: call PMCR_write(s_2_12)
        let s_2_13: () = PMCR_write(state, tracer, s_2_12);
        // C s_2_14: const #() : ()
        let s_2_14: () = ();
        // S s_2_15: call PMCR_read(s_2_14)
        let s_2_15: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_2_14);
        // C s_2_16: const #0u : u8
        let s_2_16: bool = false;
        // S s_2_17: call _update_PMCR_Type_C(s_2_15, s_2_16)
        let s_2_17: ProductType700c18a878c5601b = u_update_PMCR_Type_C(
            state,
            tracer,
            s_2_15,
            s_2_16,
        );
        // S s_2_18: call PMCR_write(s_2_17)
        let s_2_18: () = PMCR_write(state, tracer, s_2_17);
        // C s_2_19: const #() : ()
        let s_2_19: () = ();
        // S s_2_20: call PMCR_read(s_2_19)
        let s_2_20: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_2_19);
        // C s_2_21: const #0u : u8
        let s_2_21: bool = false;
        // S s_2_22: call _update_PMCR_Type_P(s_2_20, s_2_21)
        let s_2_22: ProductType700c18a878c5601b = u_update_PMCR_Type_P(
            state,
            tracer,
            s_2_20,
            s_2_21,
        );
        // S s_2_23: call PMCR_write(s_2_22)
        let s_2_23: () = PMCR_write(state, tracer, s_2_22);
        // C s_2_24: const #1208090881u : u32
        let s_2_24: u32 = 1208090881;
        // S s_2_25: call Mk_PMCEID0_Type(s_2_24)
        let s_2_25: ProductType700c18a878c5601b = Mk_PMCEID0_Type(state, tracer, s_2_24);
        // S s_2_26: call PMCEID0_write(s_2_25)
        let s_2_26: () = PMCEID0_write(state, tracer, s_2_25);
        // C s_2_27: const #32s : i
        let s_2_27: i128 = 32;
        // C s_2_28: const #15u : u8
        let s_2_28: u8 = 15;
        // C s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 4u16);
        // D s_2_30: bits-cast zx s_2_29 -> bv length s_2_27
        let s_2_30: Bits = s_2_29.zero_extend(s_2_27);
        // D s_2_31: cast reint s_2_30 -> u32
        let s_2_31: u32 = (s_2_30.value() as u32);
        // D s_2_32: call Mk_PMCEID2_Type(s_2_31)
        let s_2_32: ProductType700c18a878c5601b = Mk_PMCEID2_Type(state, tracer, s_2_31);
        // D s_2_33: call PMCEID2_write(s_2_32)
        let s_2_33: () = PMCEID2_write(state, tracer, s_2_32);
        // C s_2_34: const #32s : i
        let s_2_34: i128 = 32;
        // S s_2_35: call Zeros(s_2_34)
        let s_2_35: Bits = Zeros(state, tracer, s_2_34);
        // S s_2_36: cast reint s_2_35 -> u32
        let s_2_36: u32 = (s_2_35.value() as u32);
        // S s_2_37: call Mk_PMCEID1_Type(s_2_36)
        let s_2_37: ProductType700c18a878c5601b = Mk_PMCEID1_Type(state, tracer, s_2_36);
        // S s_2_38: call PMCEID1_write(s_2_37)
        let s_2_38: () = PMCEID1_write(state, tracer, s_2_37);
        // C s_2_39: const #32s : i
        let s_2_39: i128 = 32;
        // S s_2_40: call Zeros(s_2_39)
        let s_2_40: Bits = Zeros(state, tracer, s_2_39);
        // S s_2_41: cast reint s_2_40 -> u32
        let s_2_41: u32 = (s_2_40.value() as u32);
        // S s_2_42: call Mk_PMCEID3_Type(s_2_41)
        let s_2_42: ProductType700c18a878c5601b = Mk_PMCEID3_Type(state, tracer, s_2_41);
        // S s_2_43: call PMCEID3_write(s_2_42)
        let s_2_43: () = PMCEID3_write(state, tracer, s_2_42);
        // C s_2_44: const #16960u : u32
        let s_2_44: u32 = 16960;
        // D s_2_45: read-reg s_2_44:struct
        let s_2_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_44 as isize);
            tracer.read_register(s_2_44 as isize, value);
            value
        };
        // C s_2_46: const #16960u : u32
        let s_2_46: u32 = 16960;
        // N s_2_47: write-reg s_2_46 <= s_2_45
        let s_2_47: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_46 as isize, s_2_45);
            tracer.write_register(s_2_46 as isize, s_2_45);
        };
        // C s_2_48: const #16960u : u32
        let s_2_48: u32 = 16960;
        // D s_2_49: read-reg s_2_48:struct
        let s_2_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_48 as isize);
            tracer.read_register(s_2_48 as isize, value);
            value
        };
        // C s_2_50: const #16960u : u32
        let s_2_50: u32 = 16960;
        // N s_2_51: write-reg s_2_50 <= s_2_49
        let s_2_51: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_50 as isize, s_2_49);
            tracer.write_register(s_2_50 as isize, s_2_49);
        };
        // C s_2_52: const #16960u : u32
        let s_2_52: u32 = 16960;
        // D s_2_53: read-reg s_2_52:struct
        let s_2_53: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_52 as isize);
            tracer.read_register(s_2_52 as isize, value);
            value
        };
        // C s_2_54: const #16960u : u32
        let s_2_54: u32 = 16960;
        // N s_2_55: write-reg s_2_54 <= s_2_53
        let s_2_55: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_54 as isize, s_2_53);
            tracer.write_register(s_2_54 as isize, s_2_53);
        };
        // C s_2_56: const #16960u : u32
        let s_2_56: u32 = 16960;
        // D s_2_57: read-reg s_2_56:struct
        let s_2_57: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_56 as isize);
            tracer.read_register(s_2_56 as isize, value);
            value
        };
        // C s_2_58: const #16960u : u32
        let s_2_58: u32 = 16960;
        // N s_2_59: write-reg s_2_58 <= s_2_57
        let s_2_59: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_58 as isize, s_2_57);
            tracer.write_register(s_2_58 as isize, s_2_57);
        };
        // C s_2_60: const #16960u : u32
        let s_2_60: u32 = 16960;
        // D s_2_61: read-reg s_2_60:struct
        let s_2_61: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_60 as isize);
            tracer.read_register(s_2_60 as isize, value);
            value
        };
        // C s_2_62: const #16960u : u32
        let s_2_62: u32 = 16960;
        // N s_2_63: write-reg s_2_62 <= s_2_61
        let s_2_63: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_62 as isize, s_2_61);
            tracer.write_register(s_2_62 as isize, s_2_61);
        };
        // C s_2_64: const #16960u : u32
        let s_2_64: u32 = 16960;
        // D s_2_65: read-reg s_2_64:struct
        let s_2_65: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_64 as isize);
            tracer.read_register(s_2_64 as isize, value);
            value
        };
        // C s_2_66: const #16960u : u32
        let s_2_66: u32 = 16960;
        // N s_2_67: write-reg s_2_66 <= s_2_65
        let s_2_67: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_66 as isize, s_2_65);
            tracer.write_register(s_2_66 as isize, s_2_65);
        };
        // C s_2_68: const #16960u : u32
        let s_2_68: u32 = 16960;
        // D s_2_69: read-reg s_2_68:struct
        let s_2_69: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_68 as isize);
            tracer.read_register(s_2_68 as isize, value);
            value
        };
        // C s_2_70: const #16960u : u32
        let s_2_70: u32 = 16960;
        // N s_2_71: write-reg s_2_70 <= s_2_69
        let s_2_71: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_70 as isize, s_2_69);
            tracer.write_register(s_2_70 as isize, s_2_69);
        };
        // C s_2_72: const #16960u : u32
        let s_2_72: u32 = 16960;
        // D s_2_73: read-reg s_2_72:struct
        let s_2_73: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_72 as isize);
            tracer.read_register(s_2_72 as isize, value);
            value
        };
        // C s_2_74: const #16960u : u32
        let s_2_74: u32 = 16960;
        // N s_2_75: write-reg s_2_74 <= s_2_73
        let s_2_75: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_74 as isize, s_2_73);
            tracer.write_register(s_2_74 as isize, s_2_73);
        };
        // C s_2_76: const #16960u : u32
        let s_2_76: u32 = 16960;
        // D s_2_77: read-reg s_2_76:struct
        let s_2_77: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_76 as isize);
            tracer.read_register(s_2_76 as isize, value);
            value
        };
        // C s_2_78: const #16960u : u32
        let s_2_78: u32 = 16960;
        // N s_2_79: write-reg s_2_78 <= s_2_77
        let s_2_79: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_78 as isize, s_2_77);
            tracer.write_register(s_2_78 as isize, s_2_77);
        };
        // C s_2_80: const #4s : i
        let s_2_80: i128 = 4;
        // C s_2_81: const #0s : i
        let s_2_81: i128 = 0;
        // C s_2_82: const #11104u : u32
        let s_2_82: u32 = 11104;
        // D s_2_83: read-reg s_2_82:i
        let s_2_83: i128 = {
            let value = state.read_register::<i128>(s_2_82 as isize);
            tracer.read_register(s_2_82 as isize, value);
            value
        };
        // D s_2_84: call integer_subrange(s_2_83, s_2_80, s_2_81)
        let s_2_84: Bits = integer_subrange(state, tracer, s_2_83, s_2_80, s_2_81);
        // C s_2_85: const #16960u : u32
        let s_2_85: u32 = 16960;
        // D s_2_86: read-reg s_2_85:struct
        let s_2_86: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_85 as isize);
            tracer.read_register(s_2_85 as isize, value);
            value
        };
        // C s_2_87: const #16960u : u32
        let s_2_87: u32 = 16960;
        // N s_2_88: write-reg s_2_87 <= s_2_86
        let s_2_88: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_87 as isize, s_2_86);
            tracer.write_register(s_2_87 as isize, s_2_86);
        };
        // C s_2_89: const #18992u : u32
        let s_2_89: u32 = 18992;
        // D s_2_90: read-reg s_2_89:struct
        let s_2_90: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_89 as isize);
            tracer.read_register(s_2_89 as isize, value);
            value
        };
        // C s_2_91: const #18992u : u32
        let s_2_91: u32 = 18992;
        // N s_2_92: write-reg s_2_91 <= s_2_90
        let s_2_92: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_2_91 as isize, s_2_90);
            tracer.write_register(s_2_91 as isize, s_2_90);
        };
        // C s_2_93: const #18992u : u32
        let s_2_93: u32 = 18992;
        // D s_2_94: read-reg s_2_93:struct
        let s_2_94: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_93 as isize);
            tracer.read_register(s_2_93 as isize, value);
            value
        };
        // C s_2_95: const #18992u : u32
        let s_2_95: u32 = 18992;
        // N s_2_96: write-reg s_2_95 <= s_2_94
        let s_2_96: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_2_95 as isize, s_2_94);
            tracer.write_register(s_2_95 as isize, s_2_94);
        };
        // C s_2_97: const #15776u : u32
        let s_2_97: u32 = 15776;
        // D s_2_98: read-reg s_2_97:struct
        let s_2_98: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_97 as isize);
            tracer.read_register(s_2_97 as isize, value);
            value
        };
        // C s_2_99: const #15776u : u32
        let s_2_99: u32 = 15776;
        // N s_2_100: write-reg s_2_99 <= s_2_98
        let s_2_100: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_2_99 as isize, s_2_98);
            tracer.write_register(s_2_99 as isize, s_2_98);
        };
        // C s_2_101: const #104984u : u32
        let s_2_101: u32 = 104984;
        // D s_2_102: read-reg s_2_101:struct
        let s_2_102: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_101 as isize);
            tracer.read_register(s_2_101 as isize, value);
            value
        };
        // C s_2_103: const #104984u : u32
        let s_2_103: u32 = 104984;
        // N s_2_104: write-reg s_2_103 <= s_2_102
        let s_2_104: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_103 as isize, s_2_102);
            tracer.write_register(s_2_103 as isize, s_2_102);
        };
        // C s_2_105: const #104984u : u32
        let s_2_105: u32 = 104984;
        // D s_2_106: read-reg s_2_105:struct
        let s_2_106: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_105 as isize);
            tracer.read_register(s_2_105 as isize, value);
            value
        };
        // C s_2_107: const #104984u : u32
        let s_2_107: u32 = 104984;
        // N s_2_108: write-reg s_2_107 <= s_2_106
        let s_2_108: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_107 as isize, s_2_106);
            tracer.write_register(s_2_107 as isize, s_2_106);
        };
        // C s_2_109: const #14072u : u32
        let s_2_109: u32 = 14072;
        // D s_2_110: read-reg s_2_109:struct
        let s_2_110: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_109 as isize);
            tracer.read_register(s_2_109 as isize, value);
            value
        };
        // C s_2_111: const #14072u : u32
        let s_2_111: u32 = 14072;
        // N s_2_112: write-reg s_2_111 <= s_2_110
        let s_2_112: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_111 as isize, s_2_110);
            tracer.write_register(s_2_111 as isize, s_2_110);
        };
        // C s_2_113: const #14072u : u32
        let s_2_113: u32 = 14072;
        // D s_2_114: read-reg s_2_113:struct
        let s_2_114: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_113 as isize);
            tracer.read_register(s_2_113 as isize, value);
            value
        };
        // C s_2_115: const #14072u : u32
        let s_2_115: u32 = 14072;
        // N s_2_116: write-reg s_2_115 <= s_2_114
        let s_2_116: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_115 as isize, s_2_114);
            tracer.write_register(s_2_115 as isize, s_2_114);
        };
        // C s_2_117: const #14072u : u32
        let s_2_117: u32 = 14072;
        // D s_2_118: read-reg s_2_117:struct
        let s_2_118: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_117 as isize);
            tracer.read_register(s_2_117 as isize, value);
            value
        };
        // C s_2_119: const #14072u : u32
        let s_2_119: u32 = 14072;
        // N s_2_120: write-reg s_2_119 <= s_2_118
        let s_2_120: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_119 as isize, s_2_118);
            tracer.write_register(s_2_119 as isize, s_2_118);
        };
        // C s_2_121: const #16384u : u32
        let s_2_121: u32 = 16384;
        // D s_2_122: read-reg s_2_121:struct
        let s_2_122: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_121 as isize);
            tracer.read_register(s_2_121 as isize, value);
            value
        };
        // C s_2_123: const #16384u : u32
        let s_2_123: u32 = 16384;
        // N s_2_124: write-reg s_2_123 <= s_2_122
        let s_2_124: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_123 as isize, s_2_122);
            tracer.write_register(s_2_123 as isize, s_2_122);
        };
        // C s_2_125: const #16384u : u32
        let s_2_125: u32 = 16384;
        // D s_2_126: read-reg s_2_125:struct
        let s_2_126: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_125 as isize);
            tracer.read_register(s_2_125 as isize, value);
            value
        };
        // C s_2_127: const #16384u : u32
        let s_2_127: u32 = 16384;
        // N s_2_128: write-reg s_2_127 <= s_2_126
        let s_2_128: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_127 as isize, s_2_126);
            tracer.write_register(s_2_127 as isize, s_2_126);
        };
        // C s_2_129: const #104592u : u32
        let s_2_129: u32 = 104592;
        // D s_2_130: read-reg s_2_129:struct
        let s_2_130: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_129 as isize);
            tracer.read_register(s_2_129 as isize, value);
            value
        };
        // C s_2_131: const #104592u : u32
        let s_2_131: u32 = 104592;
        // N s_2_132: write-reg s_2_131 <= s_2_130
        let s_2_132: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_131 as isize, s_2_130);
            tracer.write_register(s_2_131 as isize, s_2_130);
        };
        // C s_2_133: const #104592u : u32
        let s_2_133: u32 = 104592;
        // D s_2_134: read-reg s_2_133:struct
        let s_2_134: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_133 as isize);
            tracer.read_register(s_2_133 as isize, value);
            value
        };
        // C s_2_135: const #104592u : u32
        let s_2_135: u32 = 104592;
        // N s_2_136: write-reg s_2_135 <= s_2_134
        let s_2_136: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_135 as isize, s_2_134);
            tracer.write_register(s_2_135 as isize, s_2_134);
        };
        // C s_2_137: const #17256u : u32
        let s_2_137: u32 = 17256;
        // D s_2_138: read-reg s_2_137:struct
        let s_2_138: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_137 as isize);
            tracer.read_register(s_2_137 as isize, value);
            value
        };
        // C s_2_139: const #17256u : u32
        let s_2_139: u32 = 17256;
        // N s_2_140: write-reg s_2_139 <= s_2_138
        let s_2_140: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_139 as isize, s_2_138);
            tracer.write_register(s_2_139 as isize, s_2_138);
        };
        // C s_2_141: const #100872u : u32
        let s_2_141: u32 = 100872;
        // D s_2_142: read-reg s_2_141:struct
        let s_2_142: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_141 as isize);
            tracer.read_register(s_2_141 as isize, value);
            value
        };
        // C s_2_143: const #100872u : u32
        let s_2_143: u32 = 100872;
        // N s_2_144: write-reg s_2_143 <= s_2_142
        let s_2_144: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_143 as isize, s_2_142);
            tracer.write_register(s_2_143 as isize, s_2_142);
        };
        // C s_2_145: const #100872u : u32
        let s_2_145: u32 = 100872;
        // D s_2_146: read-reg s_2_145:struct
        let s_2_146: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_145 as isize);
            tracer.read_register(s_2_145 as isize, value);
            value
        };
        // C s_2_147: const #100872u : u32
        let s_2_147: u32 = 100872;
        // N s_2_148: write-reg s_2_147 <= s_2_146
        let s_2_148: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_147 as isize, s_2_146);
            tracer.write_register(s_2_147 as isize, s_2_146);
        };
        // C s_2_149: const #10520u : u32
        let s_2_149: u32 = 10520;
        // D s_2_150: read-reg s_2_149:struct
        let s_2_150: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_149 as isize);
            tracer.read_register(s_2_149 as isize, value);
            value
        };
        // C s_2_151: const #10520u : u32
        let s_2_151: u32 = 10520;
        // N s_2_152: write-reg s_2_151 <= s_2_150
        let s_2_152: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_151 as isize, s_2_150);
            tracer.write_register(s_2_151 as isize, s_2_150);
        };
        // C s_2_153: const #20224u : u32
        let s_2_153: u32 = 20224;
        // D s_2_154: read-reg s_2_153:struct
        let s_2_154: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_153 as isize);
            tracer.read_register(s_2_153 as isize, value);
            value
        };
        // C s_2_155: const #20224u : u32
        let s_2_155: u32 = 20224;
        // N s_2_156: write-reg s_2_155 <= s_2_154
        let s_2_156: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_155 as isize, s_2_154);
            tracer.write_register(s_2_155 as isize, s_2_154);
        };
        // C s_2_157: const #32s : i
        let s_2_157: i128 = 32;
        // C s_2_158: const #31s : i
        let s_2_158: i128 = 31;
        // C s_2_159: const #5s : i
        let s_2_159: i128 = 5;
        // C s_2_160: const #10536u : u32
        let s_2_160: u32 = 10536;
        // D s_2_161: read-reg s_2_160:u32
        let s_2_161: u32 = {
            let value = state.read_register::<u32>(s_2_160 as isize);
            tracer.read_register(s_2_160 as isize, value);
            value
        };
        // D s_2_162: cast zx s_2_161 -> bv
        let s_2_162: Bits = Bits::new(s_2_161 as u128, 32u16);
        // D s_2_163: call set_subrange_zeros(s_2_157, s_2_162, s_2_158, s_2_159)
        let s_2_163: Bits = set_subrange_zeros(
            state,
            tracer,
            s_2_157,
            s_2_162,
            s_2_158,
            s_2_159,
        );
        // D s_2_164: cast reint s_2_163 -> u32
        let s_2_164: u32 = (s_2_163.value() as u32);
        // C s_2_165: const #10536u : u32
        let s_2_165: u32 = 10536;
        // N s_2_166: write-reg s_2_165 <= s_2_164
        let s_2_166: () = {
            state.write_register::<u32>(s_2_165 as isize, s_2_164);
            tracer.write_register(s_2_165 as isize, s_2_164);
        };
        // C s_2_167: const #() : ()
        let s_2_167: () = ();
        // S s_2_168: call CCSIDR_read(s_2_167)
        let s_2_168: ProductType700c18a878c5601b = CCSIDR_read(state, tracer, s_2_167);
        // C s_2_169: const #21s : i
        let s_2_169: i128 = 21;
        // C s_2_170: const #1u : u8
        let s_2_170: u8 = 1;
        // C s_2_171: cast zx s_2_170 -> bv
        let s_2_171: Bits = Bits::new(s_2_170 as u128, 4u16);
        // D s_2_172: bits-cast zx s_2_171 -> bv length s_2_169
        let s_2_172: Bits = s_2_171.zero_extend(s_2_169);
        // D s_2_173: cast reint s_2_172 -> u21
        let s_2_173: u32 = (s_2_172.value() as u32);
        // D s_2_174: call _update_CCSIDR_Type_Associativity(s_2_168, s_2_173)
        let s_2_174: ProductType700c18a878c5601b = u_update_CCSIDR_Type_Associativity(
            state,
            tracer,
            s_2_168,
            s_2_173,
        );
        // D s_2_175: call CCSIDR_write(s_2_174)
        let s_2_175: () = CCSIDR_write(state, tracer, s_2_174);
        // C s_2_176: const #() : ()
        let s_2_176: () = ();
        // S s_2_177: call CCSIDR_read(s_2_176)
        let s_2_177: ProductType700c18a878c5601b = CCSIDR_read(state, tracer, s_2_176);
        // C s_2_178: const #2u : u8
        let s_2_178: u8 = 2;
        // S s_2_179: call _update_CCSIDR_Type_LineSize(s_2_177, s_2_178)
        let s_2_179: ProductType700c18a878c5601b = u_update_CCSIDR_Type_LineSize(
            state,
            tracer,
            s_2_177,
            s_2_178,
        );
        // S s_2_180: call CCSIDR_write(s_2_179)
        let s_2_180: () = CCSIDR_write(state, tracer, s_2_179);
        // C s_2_181: const #() : ()
        let s_2_181: () = ();
        // S s_2_182: call CLIDR_read(s_2_181)
        let s_2_182: ProductType700c18a878c5601b = CLIDR_read(state, tracer, s_2_181);
        // C s_2_183: const #0u : u8
        let s_2_183: u8 = 0;
        // S s_2_184: call _update_CLIDR_Type_ICB(s_2_182, s_2_183)
        let s_2_184: ProductType700c18a878c5601b = u_update_CLIDR_Type_ICB(
            state,
            tracer,
            s_2_182,
            s_2_183,
        );
        // S s_2_185: call CLIDR_write(s_2_184)
        let s_2_185: () = CLIDR_write(state, tracer, s_2_184);
        // C s_2_186: const #() : ()
        let s_2_186: () = ();
        // S s_2_187: call CLIDR_read(s_2_186)
        let s_2_187: ProductType700c18a878c5601b = CLIDR_read(state, tracer, s_2_186);
        // C s_2_188: const #0u : u8
        let s_2_188: u8 = 0;
        // S s_2_189: call _update_CLIDR_Type_LoUU(s_2_187, s_2_188)
        let s_2_189: ProductType700c18a878c5601b = u_update_CLIDR_Type_LoUU(
            state,
            tracer,
            s_2_187,
            s_2_188,
        );
        // S s_2_190: call CLIDR_write(s_2_189)
        let s_2_190: () = CLIDR_write(state, tracer, s_2_189);
        // C s_2_191: const #() : ()
        let s_2_191: () = ();
        // S s_2_192: call CLIDR_read(s_2_191)
        let s_2_192: ProductType700c18a878c5601b = CLIDR_read(state, tracer, s_2_191);
        // C s_2_193: const #0u : u8
        let s_2_193: u8 = 0;
        // S s_2_194: call _update_CLIDR_Type_LoUIS(s_2_192, s_2_193)
        let s_2_194: ProductType700c18a878c5601b = u_update_CLIDR_Type_LoUIS(
            state,
            tracer,
            s_2_192,
            s_2_193,
        );
        // S s_2_195: call CLIDR_write(s_2_194)
        let s_2_195: () = CLIDR_write(state, tracer, s_2_194);
        // C s_2_196: const #() : ()
        let s_2_196: () = ();
        // S s_2_197: call CLIDR_read(s_2_196)
        let s_2_197: ProductType700c18a878c5601b = CLIDR_read(state, tracer, s_2_196);
        // D s_2_198: write-var ga#368008 <= s_2_197
        fn_state.ga_368008 = s_2_197;
        // D s_2_199: read-var ga#368008.0:struct
        let s_2_199: u32 = fn_state.ga_368008._0;
        // C s_2_200: const #0s : i
        let s_2_200: i128 = 0;
        // D s_2_201: cast zx s_2_199 -> bv
        let s_2_201: Bits = Bits::new(s_2_199 as u128, 32u16);
        // C s_2_202: const #35u : u21
        let s_2_202: u32 = 35;
        // C s_2_203: cast zx s_2_202 -> bv
        let s_2_203: Bits = Bits::new(s_2_202 as u128, 21u16);
        // C s_2_204: const #20s : i
        let s_2_204: i128 = 20;
        // C s_2_205: const #1u : u64
        let s_2_205: u64 = 1;
        // C s_2_206: cast zx s_2_205 -> bv
        let s_2_206: Bits = Bits::new(s_2_205 as u128, 64u16);
        // C s_2_207: lsl s_2_206 s_2_204
        let s_2_207: Bits = s_2_206 << s_2_204;
        // C s_2_208: sub s_2_207 s_2_206
        let s_2_208: Bits = ((s_2_207) - (s_2_206));
        // C s_2_209: and s_2_203 s_2_208
        let s_2_209: Bits = ((s_2_203) & (s_2_208));
        // C s_2_210: lsl s_2_209 s_2_200
        let s_2_210: Bits = s_2_209 << s_2_200;
        // C s_2_211: lsl s_2_208 s_2_200
        let s_2_211: Bits = s_2_208 << s_2_200;
        // C s_2_212: cmpl s_2_211
        let s_2_212: Bits = !s_2_211;
        // D s_2_213: and s_2_201 s_2_212
        let s_2_213: Bits = ((s_2_201) & (s_2_212));
        // D s_2_214: or s_2_213 s_2_210
        let s_2_214: Bits = ((s_2_213) | (s_2_210));
        // D s_2_215: cast reint s_2_214 -> u32
        let s_2_215: u32 = (s_2_214.value() as u32);
        // D s_2_216: call Mk_CLIDR_Type(s_2_215)
        let s_2_216: ProductType700c18a878c5601b = Mk_CLIDR_Type(state, tracer, s_2_215);
        // D s_2_217: call CLIDR_write(s_2_216)
        let s_2_217: () = CLIDR_write(state, tracer, s_2_216);
        // C s_2_218: const #() : ()
        let s_2_218: () = ();
        // S s_2_219: call CTR_read(s_2_218)
        let s_2_219: ProductType700c18a878c5601b = CTR_read(state, tracer, s_2_218);
        // C s_2_220: const #0u : u8
        let s_2_220: bool = false;
        // S s_2_221: call _update_CTR_Type_DIC(s_2_219, s_2_220)
        let s_2_221: ProductType700c18a878c5601b = u_update_CTR_Type_DIC(
            state,
            tracer,
            s_2_219,
            s_2_220,
        );
        // S s_2_222: call CTR_write(s_2_221)
        let s_2_222: () = CTR_write(state, tracer, s_2_221);
        // C s_2_223: const #() : ()
        let s_2_223: () = ();
        // S s_2_224: call CTR_read(s_2_223)
        let s_2_224: ProductType700c18a878c5601b = CTR_read(state, tracer, s_2_223);
        // C s_2_225: const #0u : u8
        let s_2_225: bool = false;
        // S s_2_226: call _update_CTR_Type_IDC(s_2_224, s_2_225)
        let s_2_226: ProductType700c18a878c5601b = u_update_CTR_Type_IDC(
            state,
            tracer,
            s_2_224,
            s_2_225,
        );
        // S s_2_227: call CTR_write(s_2_226)
        let s_2_227: () = CTR_write(state, tracer, s_2_226);
        // C s_2_228: const #() : ()
        let s_2_228: () = ();
        // S s_2_229: call CTR_read(s_2_228)
        let s_2_229: ProductType700c18a878c5601b = CTR_read(state, tracer, s_2_228);
        // C s_2_230: const #4u : u8
        let s_2_230: u8 = 4;
        // S s_2_231: call _update_CTR_Type_CWG(s_2_229, s_2_230)
        let s_2_231: ProductType700c18a878c5601b = u_update_CTR_Type_CWG(
            state,
            tracer,
            s_2_229,
            s_2_230,
        );
        // S s_2_232: call CTR_write(s_2_231)
        let s_2_232: () = CTR_write(state, tracer, s_2_231);
        // C s_2_233: const #() : ()
        let s_2_233: () = ();
        // S s_2_234: call CTR_read(s_2_233)
        let s_2_234: ProductType700c18a878c5601b = CTR_read(state, tracer, s_2_233);
        // C s_2_235: const #1784u : u32
        let s_2_235: u32 = 1784;
        // D s_2_236: read-reg s_2_235:u8
        let s_2_236: u8 = {
            let value = state.read_register::<u8>(s_2_235 as isize);
            tracer.read_register(s_2_235 as isize, value);
            value
        };
        // D s_2_237: call _update_CTR_Type_ERG(s_2_234, s_2_236)
        let s_2_237: ProductType700c18a878c5601b = u_update_CTR_Type_ERG(
            state,
            tracer,
            s_2_234,
            s_2_236,
        );
        // D s_2_238: call CTR_write(s_2_237)
        let s_2_238: () = CTR_write(state, tracer, s_2_237);
        // C s_2_239: const #() : ()
        let s_2_239: () = ();
        // S s_2_240: call CTR_read(s_2_239)
        let s_2_240: ProductType700c18a878c5601b = CTR_read(state, tracer, s_2_239);
        // C s_2_241: const #4u : u8
        let s_2_241: u8 = 4;
        // S s_2_242: call _update_CTR_Type_DminLine(s_2_240, s_2_241)
        let s_2_242: ProductType700c18a878c5601b = u_update_CTR_Type_DminLine(
            state,
            tracer,
            s_2_240,
            s_2_241,
        );
        // S s_2_243: call CTR_write(s_2_242)
        let s_2_243: () = CTR_write(state, tracer, s_2_242);
        // C s_2_244: const #() : ()
        let s_2_244: () = ();
        // S s_2_245: call CTR_read(s_2_244)
        let s_2_245: ProductType700c18a878c5601b = CTR_read(state, tracer, s_2_244);
        // C s_2_246: const #2u : u8
        let s_2_246: u8 = 2;
        // S s_2_247: call _update_CTR_Type_L1Ip(s_2_245, s_2_246)
        let s_2_247: ProductType700c18a878c5601b = u_update_CTR_Type_L1Ip(
            state,
            tracer,
            s_2_245,
            s_2_246,
        );
        // S s_2_248: call CTR_write(s_2_247)
        let s_2_248: () = CTR_write(state, tracer, s_2_247);
        // C s_2_249: const #() : ()
        let s_2_249: () = ();
        // S s_2_250: call CTR_read(s_2_249)
        let s_2_250: ProductType700c18a878c5601b = CTR_read(state, tracer, s_2_249);
        // C s_2_251: const #4u : u8
        let s_2_251: u8 = 4;
        // S s_2_252: call _update_CTR_Type_IminLine(s_2_250, s_2_251)
        let s_2_252: ProductType700c18a878c5601b = u_update_CTR_Type_IminLine(
            state,
            tracer,
            s_2_250,
            s_2_251,
        );
        // S s_2_253: call CTR_write(s_2_252)
        let s_2_253: () = CTR_write(state, tracer, s_2_252);
        // C s_2_254: const #15576u : u32
        let s_2_254: u32 = 15576;
        // D s_2_255: read-reg s_2_254:struct
        let s_2_255: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_254 as isize);
            tracer.read_register(s_2_254 as isize, value);
            value
        };
        // C s_2_256: const #15576u : u32
        let s_2_256: u32 = 15576;
        // N s_2_257: write-reg s_2_256 <= s_2_255
        let s_2_257: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_256 as isize, s_2_255);
            tracer.write_register(s_2_256 as isize, s_2_255);
        };
        // C s_2_258: const #15576u : u32
        let s_2_258: u32 = 15576;
        // D s_2_259: read-reg s_2_258:struct
        let s_2_259: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_258 as isize);
            tracer.read_register(s_2_258 as isize, value);
            value
        };
        // C s_2_260: const #15576u : u32
        let s_2_260: u32 = 15576;
        // N s_2_261: write-reg s_2_260 <= s_2_259
        let s_2_261: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_260 as isize, s_2_259);
            tracer.write_register(s_2_260 as isize, s_2_259);
        };
        // C s_2_262: const #15576u : u32
        let s_2_262: u32 = 15576;
        // D s_2_263: read-reg s_2_262:struct
        let s_2_263: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_262 as isize);
            tracer.read_register(s_2_262 as isize, value);
            value
        };
        // C s_2_264: const #15576u : u32
        let s_2_264: u32 = 15576;
        // N s_2_265: write-reg s_2_264 <= s_2_263
        let s_2_265: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_264 as isize, s_2_263);
            tracer.write_register(s_2_264 as isize, s_2_263);
        };
        // C s_2_266: const #15576u : u32
        let s_2_266: u32 = 15576;
        // D s_2_267: read-reg s_2_266:struct
        let s_2_267: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_266 as isize);
            tracer.read_register(s_2_266 as isize, value);
            value
        };
        // C s_2_268: const #15576u : u32
        let s_2_268: u32 = 15576;
        // N s_2_269: write-reg s_2_268 <= s_2_267
        let s_2_269: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_268 as isize, s_2_267);
            tracer.write_register(s_2_268 as isize, s_2_267);
        };
        // C s_2_270: const #15576u : u32
        let s_2_270: u32 = 15576;
        // D s_2_271: read-reg s_2_270:struct
        let s_2_271: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_270 as isize);
            tracer.read_register(s_2_270 as isize, value);
            value
        };
        // C s_2_272: const #15576u : u32
        let s_2_272: u32 = 15576;
        // N s_2_273: write-reg s_2_272 <= s_2_271
        let s_2_273: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_272 as isize, s_2_271);
            tracer.write_register(s_2_272 as isize, s_2_271);
        };
        // C s_2_274: const #15576u : u32
        let s_2_274: u32 = 15576;
        // D s_2_275: read-reg s_2_274:struct
        let s_2_275: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_274 as isize);
            tracer.read_register(s_2_274 as isize, value);
            value
        };
        // C s_2_276: const #15576u : u32
        let s_2_276: u32 = 15576;
        // N s_2_277: write-reg s_2_276 <= s_2_275
        let s_2_277: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_2_276 as isize, s_2_275);
            tracer.write_register(s_2_276 as isize, s_2_275);
        };
        // C s_2_278: const #() : ()
        let s_2_278: () = ();
        // S s_2_279: call ID_DFR0_read(s_2_278)
        let s_2_279: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_2_278);
        // C s_2_280: const #0u : u8
        let s_2_280: u8 = 0;
        // S s_2_281: call _update_ID_DFR0_Type_MProfDbg(s_2_279, s_2_280)
        let s_2_281: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_MProfDbg(
            state,
            tracer,
            s_2_279,
            s_2_280,
        );
        // S s_2_282: call ID_DFR0_write(s_2_281)
        let s_2_282: () = ID_DFR0_write(state, tracer, s_2_281);
        // C s_2_283: const #() : ()
        let s_2_283: () = ();
        // S s_2_284: call ID_DFR0_read(s_2_283)
        let s_2_284: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_2_283);
        // C s_2_285: const #0u : u8
        let s_2_285: u8 = 0;
        // S s_2_286: call _update_ID_DFR0_Type_MMapDbg(s_2_284, s_2_285)
        let s_2_286: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_MMapDbg(
            state,
            tracer,
            s_2_284,
            s_2_285,
        );
        // S s_2_287: call ID_DFR0_write(s_2_286)
        let s_2_287: () = ID_DFR0_write(state, tracer, s_2_286);
        // C s_2_288: const #424u : u32
        let s_2_288: u32 = 424;
        // D s_2_289: read-reg s_2_288:u8
        let s_2_289: u8 = {
            let value = state.read_register::<u8>(s_2_288 as isize);
            tracer.read_register(s_2_288 as isize, value);
            value
        };
        // C s_2_290: const #2u : u8
        let s_2_290: u8 = 2;
        // D s_2_291: cmp-lt s_2_289 s_2_290
        let s_2_291: bool = ((s_2_289) < (s_2_290));
        // D s_2_292: not s_2_291
        let s_2_292: bool = !s_2_291;
        // N s_2_293: branch s_2_292 b50 b3
        if s_2_292 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#327782 <= s_3_0
        fn_state.gs_327782 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#327782:u8
        let s_4_0: bool = fn_state.gs_327782;
        // N s_4_1: branch s_4_0 b49 b5
        if s_4_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call ID_DFR0_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_5_0);
        // C s_5_2: const #() : ()
        let s_5_2: () = ();
        // S s_5_3: call ID_DFR0_read(s_5_2)
        let s_5_3: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_5_2);
        // S s_5_4: call _get_ID_DFR0_Type_CopDbg(s_5_3)
        let s_5_4: u8 = u_get_ID_DFR0_Type_CopDbg(state, tracer, s_5_3);
        // S s_5_5: call _update_ID_DFR0_Type_CopSDbg(s_5_1, s_5_4)
        let s_5_5: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopSDbg(
            state,
            tracer,
            s_5_1,
            s_5_4,
        );
        // S s_5_6: call ID_DFR0_write(s_5_5)
        let s_5_6: () = ID_DFR0_write(state, tracer, s_5_5);
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call ID_ISAR0_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = ID_ISAR0_read(state, tracer, s_6_0);
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // S s_6_3: call _update_ID_ISAR0_Type_Divide(s_6_1, s_6_2)
        let s_6_3: ProductType700c18a878c5601b = u_update_ID_ISAR0_Type_Divide(
            state,
            tracer,
            s_6_1,
            s_6_2,
        );
        // S s_6_4: call ID_ISAR0_write(s_6_3)
        let s_6_4: () = ID_ISAR0_write(state, tracer, s_6_3);
        // C s_6_5: const #() : ()
        let s_6_5: () = ();
        // S s_6_6: call ID_ISAR0_read(s_6_5)
        let s_6_6: ProductType700c18a878c5601b = ID_ISAR0_read(state, tracer, s_6_5);
        // C s_6_7: const #1u : u8
        let s_6_7: u8 = 1;
        // S s_6_8: call _update_ID_ISAR0_Type_Debug(s_6_6, s_6_7)
        let s_6_8: ProductType700c18a878c5601b = u_update_ID_ISAR0_Type_Debug(
            state,
            tracer,
            s_6_6,
            s_6_7,
        );
        // S s_6_9: call ID_ISAR0_write(s_6_8)
        let s_6_9: () = ID_ISAR0_write(state, tracer, s_6_8);
        // C s_6_10: const #() : ()
        let s_6_10: () = ();
        // S s_6_11: call ID_ISAR0_read(s_6_10)
        let s_6_11: ProductType700c18a878c5601b = ID_ISAR0_read(state, tracer, s_6_10);
        // C s_6_12: const #0u : u8
        let s_6_12: u8 = 0;
        // S s_6_13: call _update_ID_ISAR0_Type_Coproc(s_6_11, s_6_12)
        let s_6_13: ProductType700c18a878c5601b = u_update_ID_ISAR0_Type_Coproc(
            state,
            tracer,
            s_6_11,
            s_6_12,
        );
        // S s_6_14: call ID_ISAR0_write(s_6_13)
        let s_6_14: () = ID_ISAR0_write(state, tracer, s_6_13);
        // C s_6_15: const #() : ()
        let s_6_15: () = ();
        // S s_6_16: call ID_ISAR0_read(s_6_15)
        let s_6_16: ProductType700c18a878c5601b = ID_ISAR0_read(state, tracer, s_6_15);
        // C s_6_17: const #1u : u8
        let s_6_17: u8 = 1;
        // S s_6_18: call _update_ID_ISAR0_Type_CmpBranch(s_6_16, s_6_17)
        let s_6_18: ProductType700c18a878c5601b = u_update_ID_ISAR0_Type_CmpBranch(
            state,
            tracer,
            s_6_16,
            s_6_17,
        );
        // S s_6_19: call ID_ISAR0_write(s_6_18)
        let s_6_19: () = ID_ISAR0_write(state, tracer, s_6_18);
        // C s_6_20: const #() : ()
        let s_6_20: () = ();
        // S s_6_21: call ID_ISAR0_read(s_6_20)
        let s_6_21: ProductType700c18a878c5601b = ID_ISAR0_read(state, tracer, s_6_20);
        // C s_6_22: const #1u : u8
        let s_6_22: u8 = 1;
        // S s_6_23: call _update_ID_ISAR0_Type_BitField(s_6_21, s_6_22)
        let s_6_23: ProductType700c18a878c5601b = u_update_ID_ISAR0_Type_BitField(
            state,
            tracer,
            s_6_21,
            s_6_22,
        );
        // S s_6_24: call ID_ISAR0_write(s_6_23)
        let s_6_24: () = ID_ISAR0_write(state, tracer, s_6_23);
        // C s_6_25: const #() : ()
        let s_6_25: () = ();
        // S s_6_26: call ID_ISAR0_read(s_6_25)
        let s_6_26: ProductType700c18a878c5601b = ID_ISAR0_read(state, tracer, s_6_25);
        // C s_6_27: const #1u : u8
        let s_6_27: u8 = 1;
        // S s_6_28: call _update_ID_ISAR0_Type_BitCount(s_6_26, s_6_27)
        let s_6_28: ProductType700c18a878c5601b = u_update_ID_ISAR0_Type_BitCount(
            state,
            tracer,
            s_6_26,
            s_6_27,
        );
        // S s_6_29: call ID_ISAR0_write(s_6_28)
        let s_6_29: () = ID_ISAR0_write(state, tracer, s_6_28);
        // C s_6_30: const #() : ()
        let s_6_30: () = ();
        // S s_6_31: call ID_ISAR0_read(s_6_30)
        let s_6_31: ProductType700c18a878c5601b = ID_ISAR0_read(state, tracer, s_6_30);
        // C s_6_32: const #0u : u8
        let s_6_32: u8 = 0;
        // S s_6_33: call _update_ID_ISAR0_Type_Swap(s_6_31, s_6_32)
        let s_6_33: ProductType700c18a878c5601b = u_update_ID_ISAR0_Type_Swap(
            state,
            tracer,
            s_6_31,
            s_6_32,
        );
        // S s_6_34: call ID_ISAR0_write(s_6_33)
        let s_6_34: () = ID_ISAR0_write(state, tracer, s_6_33);
        // C s_6_35: const #() : ()
        let s_6_35: () = ();
        // S s_6_36: call ID_ISAR1_read(s_6_35)
        let s_6_36: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_35);
        // C s_6_37: const #1u : u8
        let s_6_37: u8 = 1;
        // S s_6_38: call _update_ID_ISAR1_Type_Jazelle(s_6_36, s_6_37)
        let s_6_38: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_Jazelle(
            state,
            tracer,
            s_6_36,
            s_6_37,
        );
        // S s_6_39: call ID_ISAR1_write(s_6_38)
        let s_6_39: () = ID_ISAR1_write(state, tracer, s_6_38);
        // C s_6_40: const #() : ()
        let s_6_40: () = ();
        // S s_6_41: call ID_ISAR1_read(s_6_40)
        let s_6_41: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_40);
        // C s_6_42: const #3u : u8
        let s_6_42: u8 = 3;
        // S s_6_43: call _update_ID_ISAR1_Type_Interwork(s_6_41, s_6_42)
        let s_6_43: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_Interwork(
            state,
            tracer,
            s_6_41,
            s_6_42,
        );
        // S s_6_44: call ID_ISAR1_write(s_6_43)
        let s_6_44: () = ID_ISAR1_write(state, tracer, s_6_43);
        // C s_6_45: const #() : ()
        let s_6_45: () = ();
        // S s_6_46: call ID_ISAR1_read(s_6_45)
        let s_6_46: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_45);
        // C s_6_47: const #1u : u8
        let s_6_47: u8 = 1;
        // S s_6_48: call _update_ID_ISAR1_Type_Immediate(s_6_46, s_6_47)
        let s_6_48: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_Immediate(
            state,
            tracer,
            s_6_46,
            s_6_47,
        );
        // S s_6_49: call ID_ISAR1_write(s_6_48)
        let s_6_49: () = ID_ISAR1_write(state, tracer, s_6_48);
        // C s_6_50: const #() : ()
        let s_6_50: () = ();
        // S s_6_51: call ID_ISAR1_read(s_6_50)
        let s_6_51: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_50);
        // C s_6_52: const #1u : u8
        let s_6_52: u8 = 1;
        // S s_6_53: call _update_ID_ISAR1_Type_IfThen(s_6_51, s_6_52)
        let s_6_53: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_IfThen(
            state,
            tracer,
            s_6_51,
            s_6_52,
        );
        // S s_6_54: call ID_ISAR1_write(s_6_53)
        let s_6_54: () = ID_ISAR1_write(state, tracer, s_6_53);
        // C s_6_55: const #() : ()
        let s_6_55: () = ();
        // S s_6_56: call ID_ISAR1_read(s_6_55)
        let s_6_56: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_55);
        // C s_6_57: const #2u : u8
        let s_6_57: u8 = 2;
        // S s_6_58: call _update_ID_ISAR1_Type_Extend(s_6_56, s_6_57)
        let s_6_58: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_Extend(
            state,
            tracer,
            s_6_56,
            s_6_57,
        );
        // S s_6_59: call ID_ISAR1_write(s_6_58)
        let s_6_59: () = ID_ISAR1_write(state, tracer, s_6_58);
        // C s_6_60: const #() : ()
        let s_6_60: () = ();
        // S s_6_61: call ID_ISAR1_read(s_6_60)
        let s_6_61: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_60);
        // C s_6_62: const #1u : u8
        let s_6_62: u8 = 1;
        // S s_6_63: call _update_ID_ISAR1_Type_Except_AR(s_6_61, s_6_62)
        let s_6_63: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_Except_AR(
            state,
            tracer,
            s_6_61,
            s_6_62,
        );
        // S s_6_64: call ID_ISAR1_write(s_6_63)
        let s_6_64: () = ID_ISAR1_write(state, tracer, s_6_63);
        // C s_6_65: const #() : ()
        let s_6_65: () = ();
        // S s_6_66: call ID_ISAR1_read(s_6_65)
        let s_6_66: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_65);
        // C s_6_67: const #1u : u8
        let s_6_67: u8 = 1;
        // S s_6_68: call _update_ID_ISAR1_Type_Except(s_6_66, s_6_67)
        let s_6_68: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_Except(
            state,
            tracer,
            s_6_66,
            s_6_67,
        );
        // S s_6_69: call ID_ISAR1_write(s_6_68)
        let s_6_69: () = ID_ISAR1_write(state, tracer, s_6_68);
        // C s_6_70: const #() : ()
        let s_6_70: () = ();
        // S s_6_71: call ID_ISAR1_read(s_6_70)
        let s_6_71: ProductType700c18a878c5601b = ID_ISAR1_read(state, tracer, s_6_70);
        // C s_6_72: const #1u : u8
        let s_6_72: u8 = 1;
        // S s_6_73: call _update_ID_ISAR1_Type_Endian(s_6_71, s_6_72)
        let s_6_73: ProductType700c18a878c5601b = u_update_ID_ISAR1_Type_Endian(
            state,
            tracer,
            s_6_71,
            s_6_72,
        );
        // S s_6_74: call ID_ISAR1_write(s_6_73)
        let s_6_74: () = ID_ISAR1_write(state, tracer, s_6_73);
        // C s_6_75: const #() : ()
        let s_6_75: () = ();
        // S s_6_76: call ID_ISAR2_read(s_6_75)
        let s_6_76: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_75);
        // C s_6_77: const #2u : u8
        let s_6_77: u8 = 2;
        // S s_6_78: call _update_ID_ISAR2_Type_Reversal(s_6_76, s_6_77)
        let s_6_78: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_Reversal(
            state,
            tracer,
            s_6_76,
            s_6_77,
        );
        // S s_6_79: call ID_ISAR2_write(s_6_78)
        let s_6_79: () = ID_ISAR2_write(state, tracer, s_6_78);
        // C s_6_80: const #() : ()
        let s_6_80: () = ();
        // S s_6_81: call ID_ISAR2_read(s_6_80)
        let s_6_81: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_80);
        // C s_6_82: const #1u : u8
        let s_6_82: u8 = 1;
        // S s_6_83: call _update_ID_ISAR2_Type_PSR_AR(s_6_81, s_6_82)
        let s_6_83: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_PSR_AR(
            state,
            tracer,
            s_6_81,
            s_6_82,
        );
        // S s_6_84: call ID_ISAR2_write(s_6_83)
        let s_6_84: () = ID_ISAR2_write(state, tracer, s_6_83);
        // C s_6_85: const #() : ()
        let s_6_85: () = ();
        // S s_6_86: call ID_ISAR2_read(s_6_85)
        let s_6_86: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_85);
        // C s_6_87: const #2u : u8
        let s_6_87: u8 = 2;
        // S s_6_88: call _update_ID_ISAR2_Type_MultU(s_6_86, s_6_87)
        let s_6_88: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_MultU(
            state,
            tracer,
            s_6_86,
            s_6_87,
        );
        // S s_6_89: call ID_ISAR2_write(s_6_88)
        let s_6_89: () = ID_ISAR2_write(state, tracer, s_6_88);
        // C s_6_90: const #() : ()
        let s_6_90: () = ();
        // S s_6_91: call ID_ISAR2_read(s_6_90)
        let s_6_91: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_90);
        // C s_6_92: const #3u : u8
        let s_6_92: u8 = 3;
        // S s_6_93: call _update_ID_ISAR2_Type_MultS(s_6_91, s_6_92)
        let s_6_93: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_MultS(
            state,
            tracer,
            s_6_91,
            s_6_92,
        );
        // S s_6_94: call ID_ISAR2_write(s_6_93)
        let s_6_94: () = ID_ISAR2_write(state, tracer, s_6_93);
        // C s_6_95: const #() : ()
        let s_6_95: () = ();
        // S s_6_96: call ID_ISAR2_read(s_6_95)
        let s_6_96: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_95);
        // C s_6_97: const #2u : u8
        let s_6_97: u8 = 2;
        // S s_6_98: call _update_ID_ISAR2_Type_Mult(s_6_96, s_6_97)
        let s_6_98: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_Mult(
            state,
            tracer,
            s_6_96,
            s_6_97,
        );
        // S s_6_99: call ID_ISAR2_write(s_6_98)
        let s_6_99: () = ID_ISAR2_write(state, tracer, s_6_98);
        // C s_6_100: const #() : ()
        let s_6_100: () = ();
        // S s_6_101: call ID_ISAR2_read(s_6_100)
        let s_6_101: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_100);
        // C s_6_102: const #0u : u8
        let s_6_102: u8 = 0;
        // S s_6_103: call _update_ID_ISAR2_Type_MultiAccessInt(s_6_101, s_6_102)
        let s_6_103: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_MultiAccessInt(
            state,
            tracer,
            s_6_101,
            s_6_102,
        );
        // S s_6_104: call ID_ISAR2_write(s_6_103)
        let s_6_104: () = ID_ISAR2_write(state, tracer, s_6_103);
        // C s_6_105: const #() : ()
        let s_6_105: () = ();
        // S s_6_106: call ID_ISAR2_read(s_6_105)
        let s_6_106: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_105);
        // C s_6_107: const #4u : u8
        let s_6_107: u8 = 4;
        // S s_6_108: call _update_ID_ISAR2_Type_MemHint(s_6_106, s_6_107)
        let s_6_108: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_MemHint(
            state,
            tracer,
            s_6_106,
            s_6_107,
        );
        // S s_6_109: call ID_ISAR2_write(s_6_108)
        let s_6_109: () = ID_ISAR2_write(state, tracer, s_6_108);
        // C s_6_110: const #() : ()
        let s_6_110: () = ();
        // S s_6_111: call ID_ISAR2_read(s_6_110)
        let s_6_111: ProductType700c18a878c5601b = ID_ISAR2_read(state, tracer, s_6_110);
        // C s_6_112: const #2u : u8
        let s_6_112: u8 = 2;
        // S s_6_113: call _update_ID_ISAR2_Type_LoadStore(s_6_111, s_6_112)
        let s_6_113: ProductType700c18a878c5601b = u_update_ID_ISAR2_Type_LoadStore(
            state,
            tracer,
            s_6_111,
            s_6_112,
        );
        // S s_6_114: call ID_ISAR2_write(s_6_113)
        let s_6_114: () = ID_ISAR2_write(state, tracer, s_6_113);
        // C s_6_115: const #() : ()
        let s_6_115: () = ();
        // S s_6_116: call ID_ISAR3_read(s_6_115)
        let s_6_116: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_115);
        // C s_6_117: const #0u : u8
        let s_6_117: u8 = 0;
        // S s_6_118: call _update_ID_ISAR3_Type_T32EE(s_6_116, s_6_117)
        let s_6_118: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_T32EE(
            state,
            tracer,
            s_6_116,
            s_6_117,
        );
        // S s_6_119: call ID_ISAR3_write(s_6_118)
        let s_6_119: () = ID_ISAR3_write(state, tracer, s_6_118);
        // C s_6_120: const #() : ()
        let s_6_120: () = ();
        // S s_6_121: call ID_ISAR3_read(s_6_120)
        let s_6_121: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_120);
        // C s_6_122: const #1u : u8
        let s_6_122: u8 = 1;
        // S s_6_123: call _update_ID_ISAR3_Type_TrueNOP(s_6_121, s_6_122)
        let s_6_123: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_TrueNOP(
            state,
            tracer,
            s_6_121,
            s_6_122,
        );
        // S s_6_124: call ID_ISAR3_write(s_6_123)
        let s_6_124: () = ID_ISAR3_write(state, tracer, s_6_123);
        // C s_6_125: const #() : ()
        let s_6_125: () = ();
        // S s_6_126: call ID_ISAR3_read(s_6_125)
        let s_6_126: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_125);
        // C s_6_127: const #1u : u8
        let s_6_127: u8 = 1;
        // S s_6_128: call _update_ID_ISAR3_Type_T32Copy(s_6_126, s_6_127)
        let s_6_128: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_T32Copy(
            state,
            tracer,
            s_6_126,
            s_6_127,
        );
        // S s_6_129: call ID_ISAR3_write(s_6_128)
        let s_6_129: () = ID_ISAR3_write(state, tracer, s_6_128);
        // C s_6_130: const #() : ()
        let s_6_130: () = ();
        // S s_6_131: call ID_ISAR3_read(s_6_130)
        let s_6_131: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_130);
        // C s_6_132: const #1u : u8
        let s_6_132: u8 = 1;
        // S s_6_133: call _update_ID_ISAR3_Type_TabBranch(s_6_131, s_6_132)
        let s_6_133: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_TabBranch(
            state,
            tracer,
            s_6_131,
            s_6_132,
        );
        // S s_6_134: call ID_ISAR3_write(s_6_133)
        let s_6_134: () = ID_ISAR3_write(state, tracer, s_6_133);
        // C s_6_135: const #() : ()
        let s_6_135: () = ();
        // S s_6_136: call ID_ISAR3_read(s_6_135)
        let s_6_136: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_135);
        // C s_6_137: const #2u : u8
        let s_6_137: u8 = 2;
        // S s_6_138: call _update_ID_ISAR3_Type_SynchPrim(s_6_136, s_6_137)
        let s_6_138: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_SynchPrim(
            state,
            tracer,
            s_6_136,
            s_6_137,
        );
        // S s_6_139: call ID_ISAR3_write(s_6_138)
        let s_6_139: () = ID_ISAR3_write(state, tracer, s_6_138);
        // C s_6_140: const #() : ()
        let s_6_140: () = ();
        // S s_6_141: call ID_ISAR3_read(s_6_140)
        let s_6_141: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_140);
        // C s_6_142: const #1u : u8
        let s_6_142: u8 = 1;
        // S s_6_143: call _update_ID_ISAR3_Type_SVC(s_6_141, s_6_142)
        let s_6_143: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_SVC(
            state,
            tracer,
            s_6_141,
            s_6_142,
        );
        // S s_6_144: call ID_ISAR3_write(s_6_143)
        let s_6_144: () = ID_ISAR3_write(state, tracer, s_6_143);
        // C s_6_145: const #() : ()
        let s_6_145: () = ();
        // S s_6_146: call ID_ISAR3_read(s_6_145)
        let s_6_146: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_145);
        // C s_6_147: const #3u : u8
        let s_6_147: u8 = 3;
        // S s_6_148: call _update_ID_ISAR3_Type_SIMD(s_6_146, s_6_147)
        let s_6_148: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_SIMD(
            state,
            tracer,
            s_6_146,
            s_6_147,
        );
        // S s_6_149: call ID_ISAR3_write(s_6_148)
        let s_6_149: () = ID_ISAR3_write(state, tracer, s_6_148);
        // C s_6_150: const #() : ()
        let s_6_150: () = ();
        // S s_6_151: call ID_ISAR3_read(s_6_150)
        let s_6_151: ProductType700c18a878c5601b = ID_ISAR3_read(state, tracer, s_6_150);
        // C s_6_152: const #1u : u8
        let s_6_152: u8 = 1;
        // S s_6_153: call _update_ID_ISAR3_Type_Saturate(s_6_151, s_6_152)
        let s_6_153: ProductType700c18a878c5601b = u_update_ID_ISAR3_Type_Saturate(
            state,
            tracer,
            s_6_151,
            s_6_152,
        );
        // S s_6_154: call ID_ISAR3_write(s_6_153)
        let s_6_154: () = ID_ISAR3_write(state, tracer, s_6_153);
        // C s_6_155: const #() : ()
        let s_6_155: () = ();
        // S s_6_156: call ID_ISAR4_read(s_6_155)
        let s_6_156: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_155);
        // C s_6_157: const #0u : u8
        let s_6_157: u8 = 0;
        // S s_6_158: call _update_ID_ISAR4_Type_SWP_frac(s_6_156, s_6_157)
        let s_6_158: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_SWP_frac(
            state,
            tracer,
            s_6_156,
            s_6_157,
        );
        // S s_6_159: call ID_ISAR4_write(s_6_158)
        let s_6_159: () = ID_ISAR4_write(state, tracer, s_6_158);
        // C s_6_160: const #() : ()
        let s_6_160: () = ();
        // S s_6_161: call ID_ISAR4_read(s_6_160)
        let s_6_161: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_160);
        // C s_6_162: const #0u : u8
        let s_6_162: u8 = 0;
        // S s_6_163: call _update_ID_ISAR4_Type_PSR_M(s_6_161, s_6_162)
        let s_6_163: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_PSR_M(
            state,
            tracer,
            s_6_161,
            s_6_162,
        );
        // S s_6_164: call ID_ISAR4_write(s_6_163)
        let s_6_164: () = ID_ISAR4_write(state, tracer, s_6_163);
        // C s_6_165: const #() : ()
        let s_6_165: () = ();
        // S s_6_166: call ID_ISAR4_read(s_6_165)
        let s_6_166: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_165);
        // C s_6_167: const #0u : u8
        let s_6_167: u8 = 0;
        // S s_6_168: call _update_ID_ISAR4_Type_SynchPrim_frac(s_6_166, s_6_167)
        let s_6_168: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_SynchPrim_frac(
            state,
            tracer,
            s_6_166,
            s_6_167,
        );
        // S s_6_169: call ID_ISAR4_write(s_6_168)
        let s_6_169: () = ID_ISAR4_write(state, tracer, s_6_168);
        // C s_6_170: const #() : ()
        let s_6_170: () = ();
        // S s_6_171: call ID_ISAR4_read(s_6_170)
        let s_6_171: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_170);
        // C s_6_172: const #1u : u8
        let s_6_172: u8 = 1;
        // S s_6_173: call _update_ID_ISAR4_Type_Barrier(s_6_171, s_6_172)
        let s_6_173: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_Barrier(
            state,
            tracer,
            s_6_171,
            s_6_172,
        );
        // S s_6_174: call ID_ISAR4_write(s_6_173)
        let s_6_174: () = ID_ISAR4_write(state, tracer, s_6_173);
        // C s_6_175: const #() : ()
        let s_6_175: () = ();
        // S s_6_176: call ID_ISAR4_read(s_6_175)
        let s_6_176: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_175);
        // C s_6_177: const #0u : u8
        let s_6_177: u8 = 0;
        // S s_6_178: call _update_ID_ISAR4_Type_SMC(s_6_176, s_6_177)
        let s_6_178: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_SMC(
            state,
            tracer,
            s_6_176,
            s_6_177,
        );
        // S s_6_179: call ID_ISAR4_write(s_6_178)
        let s_6_179: () = ID_ISAR4_write(state, tracer, s_6_178);
        // C s_6_180: const #() : ()
        let s_6_180: () = ();
        // S s_6_181: call ID_ISAR4_read(s_6_180)
        let s_6_181: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_180);
        // C s_6_182: const #1u : u8
        let s_6_182: u8 = 1;
        // S s_6_183: call _update_ID_ISAR4_Type_Writeback(s_6_181, s_6_182)
        let s_6_183: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_Writeback(
            state,
            tracer,
            s_6_181,
            s_6_182,
        );
        // S s_6_184: call ID_ISAR4_write(s_6_183)
        let s_6_184: () = ID_ISAR4_write(state, tracer, s_6_183);
        // C s_6_185: const #() : ()
        let s_6_185: () = ();
        // S s_6_186: call ID_ISAR4_read(s_6_185)
        let s_6_186: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_185);
        // C s_6_187: const #4u : u8
        let s_6_187: u8 = 4;
        // S s_6_188: call _update_ID_ISAR4_Type_WithShifts(s_6_186, s_6_187)
        let s_6_188: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_WithShifts(
            state,
            tracer,
            s_6_186,
            s_6_187,
        );
        // S s_6_189: call ID_ISAR4_write(s_6_188)
        let s_6_189: () = ID_ISAR4_write(state, tracer, s_6_188);
        // C s_6_190: const #() : ()
        let s_6_190: () = ();
        // S s_6_191: call ID_ISAR4_read(s_6_190)
        let s_6_191: ProductType700c18a878c5601b = ID_ISAR4_read(state, tracer, s_6_190);
        // C s_6_192: const #2u : u8
        let s_6_192: u8 = 2;
        // S s_6_193: call _update_ID_ISAR4_Type_Unpriv(s_6_191, s_6_192)
        let s_6_193: ProductType700c18a878c5601b = u_update_ID_ISAR4_Type_Unpriv(
            state,
            tracer,
            s_6_191,
            s_6_192,
        );
        // S s_6_194: call ID_ISAR4_write(s_6_193)
        let s_6_194: () = ID_ISAR4_write(state, tracer, s_6_193);
        // C s_6_195: const #20216u : u32
        let s_6_195: u32 = 20216;
        // D s_6_196: read-reg s_6_195:struct
        let s_6_196: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_195 as isize);
            tracer.read_register(s_6_195 as isize, value);
            value
        };
        // C s_6_197: const #20216u : u32
        let s_6_197: u32 = 20216;
        // N s_6_198: write-reg s_6_197 <= s_6_196
        let s_6_198: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_6_197 as isize, s_6_196);
            tracer.write_register(s_6_197 as isize, s_6_196);
        };
        // C s_6_199: const #() : ()
        let s_6_199: () = ();
        // S s_6_200: call ID_ISAR5_read(s_6_199)
        let s_6_200: ProductType700c18a878c5601b = ID_ISAR5_read(state, tracer, s_6_199);
        // C s_6_201: const #1u : u8
        let s_6_201: u8 = 1;
        // S s_6_202: call _update_ID_ISAR5_Type_VCMA(s_6_200, s_6_201)
        let s_6_202: ProductType700c18a878c5601b = u_update_ID_ISAR5_Type_VCMA(
            state,
            tracer,
            s_6_200,
            s_6_201,
        );
        // S s_6_203: call ID_ISAR5_write(s_6_202)
        let s_6_203: () = ID_ISAR5_write(state, tracer, s_6_202);
        // C s_6_204: const #() : ()
        let s_6_204: () = ();
        // S s_6_205: call ID_ISAR5_read(s_6_204)
        let s_6_205: ProductType700c18a878c5601b = ID_ISAR5_read(state, tracer, s_6_204);
        // C s_6_206: const #1u : u8
        let s_6_206: u8 = 1;
        // S s_6_207: call _update_ID_ISAR5_Type_RDM(s_6_205, s_6_206)
        let s_6_207: ProductType700c18a878c5601b = u_update_ID_ISAR5_Type_RDM(
            state,
            tracer,
            s_6_205,
            s_6_206,
        );
        // S s_6_208: call ID_ISAR5_write(s_6_207)
        let s_6_208: () = ID_ISAR5_write(state, tracer, s_6_207);
        // C s_6_209: const #() : ()
        let s_6_209: () = ();
        // S s_6_210: call ID_ISAR5_read(s_6_209)
        let s_6_210: ProductType700c18a878c5601b = ID_ISAR5_read(state, tracer, s_6_209);
        // C s_6_211: const #0u : u8
        let s_6_211: u8 = 0;
        // S s_6_212: call _update_ID_ISAR5_Type_CRC32(s_6_210, s_6_211)
        let s_6_212: ProductType700c18a878c5601b = u_update_ID_ISAR5_Type_CRC32(
            state,
            tracer,
            s_6_210,
            s_6_211,
        );
        // S s_6_213: call ID_ISAR5_write(s_6_212)
        let s_6_213: () = ID_ISAR5_write(state, tracer, s_6_212);
        // C s_6_214: const #() : ()
        let s_6_214: () = ();
        // S s_6_215: call ID_ISAR5_read(s_6_214)
        let s_6_215: ProductType700c18a878c5601b = ID_ISAR5_read(state, tracer, s_6_214);
        // C s_6_216: const #1u : u8
        let s_6_216: u8 = 1;
        // S s_6_217: call _update_ID_ISAR5_Type_SHA2(s_6_215, s_6_216)
        let s_6_217: ProductType700c18a878c5601b = u_update_ID_ISAR5_Type_SHA2(
            state,
            tracer,
            s_6_215,
            s_6_216,
        );
        // S s_6_218: call ID_ISAR5_write(s_6_217)
        let s_6_218: () = ID_ISAR5_write(state, tracer, s_6_217);
        // C s_6_219: const #() : ()
        let s_6_219: () = ();
        // S s_6_220: call ID_ISAR5_read(s_6_219)
        let s_6_220: ProductType700c18a878c5601b = ID_ISAR5_read(state, tracer, s_6_219);
        // C s_6_221: const #1u : u8
        let s_6_221: u8 = 1;
        // S s_6_222: call _update_ID_ISAR5_Type_SHA1(s_6_220, s_6_221)
        let s_6_222: ProductType700c18a878c5601b = u_update_ID_ISAR5_Type_SHA1(
            state,
            tracer,
            s_6_220,
            s_6_221,
        );
        // S s_6_223: call ID_ISAR5_write(s_6_222)
        let s_6_223: () = ID_ISAR5_write(state, tracer, s_6_222);
        // C s_6_224: const #() : ()
        let s_6_224: () = ();
        // S s_6_225: call ID_ISAR5_read(s_6_224)
        let s_6_225: ProductType700c18a878c5601b = ID_ISAR5_read(state, tracer, s_6_224);
        // C s_6_226: const #2u : u8
        let s_6_226: u8 = 2;
        // S s_6_227: call _update_ID_ISAR5_Type_AES(s_6_225, s_6_226)
        let s_6_227: ProductType700c18a878c5601b = u_update_ID_ISAR5_Type_AES(
            state,
            tracer,
            s_6_225,
            s_6_226,
        );
        // S s_6_228: call ID_ISAR5_write(s_6_227)
        let s_6_228: () = ID_ISAR5_write(state, tracer, s_6_227);
        // C s_6_229: const #() : ()
        let s_6_229: () = ();
        // S s_6_230: call ID_ISAR5_read(s_6_229)
        let s_6_230: ProductType700c18a878c5601b = ID_ISAR5_read(state, tracer, s_6_229);
        // C s_6_231: const #1u : u8
        let s_6_231: u8 = 1;
        // S s_6_232: call _update_ID_ISAR5_Type_SEVL(s_6_230, s_6_231)
        let s_6_232: ProductType700c18a878c5601b = u_update_ID_ISAR5_Type_SEVL(
            state,
            tracer,
            s_6_230,
            s_6_231,
        );
        // S s_6_233: call ID_ISAR5_write(s_6_232)
        let s_6_233: () = ID_ISAR5_write(state, tracer, s_6_232);
        // C s_6_234: const #() : ()
        let s_6_234: () = ();
        // S s_6_235: call ID_ISAR6_read(s_6_234)
        let s_6_235: ProductType700c18a878c5601b = ID_ISAR6_read(state, tracer, s_6_234);
        // C s_6_236: const #1u : u8
        let s_6_236: u8 = 1;
        // S s_6_237: call _update_ID_ISAR6_Type_DP(s_6_235, s_6_236)
        let s_6_237: ProductType700c18a878c5601b = u_update_ID_ISAR6_Type_DP(
            state,
            tracer,
            s_6_235,
            s_6_236,
        );
        // S s_6_238: call ID_ISAR6_write(s_6_237)
        let s_6_238: () = ID_ISAR6_write(state, tracer, s_6_237);
        // C s_6_239: const #() : ()
        let s_6_239: () = ();
        // S s_6_240: call ID_ISAR6_read(s_6_239)
        let s_6_240: ProductType700c18a878c5601b = ID_ISAR6_read(state, tracer, s_6_239);
        // C s_6_241: const #1u : u8
        let s_6_241: u8 = 1;
        // S s_6_242: call _update_ID_ISAR6_Type_BF16(s_6_240, s_6_241)
        let s_6_242: ProductType700c18a878c5601b = u_update_ID_ISAR6_Type_BF16(
            state,
            tracer,
            s_6_240,
            s_6_241,
        );
        // S s_6_243: call ID_ISAR6_write(s_6_242)
        let s_6_243: () = ID_ISAR6_write(state, tracer, s_6_242);
        // C s_6_244: const #() : ()
        let s_6_244: () = ();
        // S s_6_245: call ID_ISAR6_read(s_6_244)
        let s_6_245: ProductType700c18a878c5601b = ID_ISAR6_read(state, tracer, s_6_244);
        // C s_6_246: const #1u : u8
        let s_6_246: u8 = 1;
        // S s_6_247: call _update_ID_ISAR6_Type_I8MM(s_6_245, s_6_246)
        let s_6_247: ProductType700c18a878c5601b = u_update_ID_ISAR6_Type_I8MM(
            state,
            tracer,
            s_6_245,
            s_6_246,
        );
        // S s_6_248: call ID_ISAR6_write(s_6_247)
        let s_6_248: () = ID_ISAR6_write(state, tracer, s_6_247);
        // C s_6_249: const #65u : u32
        let s_6_249: u32 = 65;
        // S s_6_250: call IsFeatureImplemented(s_6_249)
        let s_6_250: bool = IsFeatureImplemented(state, tracer, s_6_249);
        // N s_6_251: branch s_6_250 b48 b7
        if s_6_250 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call MVFR1_read(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_7_0);
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // S s_7_3: call _update_MVFR1_Type_FPHP(s_7_1, s_7_2)
        let s_7_3: ProductType700c18a878c5601b = u_update_MVFR1_Type_FPHP(
            state,
            tracer,
            s_7_1,
            s_7_2,
        );
        // S s_7_4: call MVFR1_write(s_7_3)
        let s_7_4: () = MVFR1_write(state, tracer, s_7_3);
        // C s_7_5: const #() : ()
        let s_7_5: () = ();
        // S s_7_6: call MVFR1_read(s_7_5)
        let s_7_6: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_7_5);
        // C s_7_7: const #1u : u8
        let s_7_7: u8 = 1;
        // S s_7_8: call _update_MVFR1_Type_SIMDHP(s_7_6, s_7_7)
        let s_7_8: ProductType700c18a878c5601b = u_update_MVFR1_Type_SIMDHP(
            state,
            tracer,
            s_7_6,
            s_7_7,
        );
        // S s_7_9: call MVFR1_write(s_7_8)
        let s_7_9: () = MVFR1_write(state, tracer, s_7_8);
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // S s_8_1: call HasArchVersion(s_8_0)
        let s_8_1: bool = HasArchVersion(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b47 b9
        if s_8_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #4u : u32
        let s_10_0: u32 = 4;
        // S s_10_1: call HasArchVersion(s_10_0)
        let s_10_1: bool = HasArchVersion(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b41 b11
        if s_10_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #7u : u32
        let s_12_0: u32 = 7;
        // S s_12_1: call HasArchVersion(s_12_0)
        let s_12_1: bool = HasArchVersion(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b40 b13
        if s_12_1 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #8u : u32
        let s_14_0: u32 = 8;
        // S s_14_1: call HasArchVersion(s_14_0)
        let s_14_1: bool = HasArchVersion(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b34 b15
        if s_14_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #6u : u32
        let s_16_0: u32 = 6;
        // S s_16_1: call HasArchVersion(s_16_0)
        let s_16_1: bool = HasArchVersion(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b33 b17
        if s_16_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call ID_MMFR0_read(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_0);
        // C s_18_2: const #1u : u8
        let s_18_2: u8 = 1;
        // S s_18_3: call _update_ID_MMFR0_Type_InnerShr(s_18_1, s_18_2)
        let s_18_3: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_InnerShr(
            state,
            tracer,
            s_18_1,
            s_18_2,
        );
        // S s_18_4: call ID_MMFR0_write(s_18_3)
        let s_18_4: () = ID_MMFR0_write(state, tracer, s_18_3);
        // C s_18_5: const #() : ()
        let s_18_5: () = ();
        // S s_18_6: call ID_MMFR0_read(s_18_5)
        let s_18_6: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_5);
        // C s_18_7: const #0u : u8
        let s_18_7: u8 = 0;
        // S s_18_8: call _update_ID_MMFR0_Type_FCSE(s_18_6, s_18_7)
        let s_18_8: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_FCSE(
            state,
            tracer,
            s_18_6,
            s_18_7,
        );
        // S s_18_9: call ID_MMFR0_write(s_18_8)
        let s_18_9: () = ID_MMFR0_write(state, tracer, s_18_8);
        // C s_18_10: const #() : ()
        let s_18_10: () = ();
        // S s_18_11: call ID_MMFR0_read(s_18_10)
        let s_18_11: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_10);
        // C s_18_12: const #2u : u8
        let s_18_12: u8 = 2;
        // S s_18_13: call _update_ID_MMFR0_Type_AuxReg(s_18_11, s_18_12)
        let s_18_13: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_AuxReg(
            state,
            tracer,
            s_18_11,
            s_18_12,
        );
        // S s_18_14: call ID_MMFR0_write(s_18_13)
        let s_18_14: () = ID_MMFR0_write(state, tracer, s_18_13);
        // C s_18_15: const #() : ()
        let s_18_15: () = ();
        // S s_18_16: call ID_MMFR0_read(s_18_15)
        let s_18_16: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_15);
        // C s_18_17: const #0u : u8
        let s_18_17: u8 = 0;
        // S s_18_18: call _update_ID_MMFR0_Type_TCM(s_18_16, s_18_17)
        let s_18_18: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_TCM(
            state,
            tracer,
            s_18_16,
            s_18_17,
        );
        // S s_18_19: call ID_MMFR0_write(s_18_18)
        let s_18_19: () = ID_MMFR0_write(state, tracer, s_18_18);
        // C s_18_20: const #() : ()
        let s_18_20: () = ();
        // S s_18_21: call ID_MMFR0_read(s_18_20)
        let s_18_21: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_20);
        // C s_18_22: const #1u : u8
        let s_18_22: u8 = 1;
        // S s_18_23: call _update_ID_MMFR0_Type_ShareLvl(s_18_21, s_18_22)
        let s_18_23: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_ShareLvl(
            state,
            tracer,
            s_18_21,
            s_18_22,
        );
        // S s_18_24: call ID_MMFR0_write(s_18_23)
        let s_18_24: () = ID_MMFR0_write(state, tracer, s_18_23);
        // C s_18_25: const #() : ()
        let s_18_25: () = ();
        // S s_18_26: call ID_MMFR0_read(s_18_25)
        let s_18_26: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_25);
        // C s_18_27: const #1u : u8
        let s_18_27: u8 = 1;
        // S s_18_28: call _update_ID_MMFR0_Type_OuterShr(s_18_26, s_18_27)
        let s_18_28: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_OuterShr(
            state,
            tracer,
            s_18_26,
            s_18_27,
        );
        // S s_18_29: call ID_MMFR0_write(s_18_28)
        let s_18_29: () = ID_MMFR0_write(state, tracer, s_18_28);
        // C s_18_30: const #() : ()
        let s_18_30: () = ();
        // S s_18_31: call ID_MMFR0_read(s_18_30)
        let s_18_31: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_30);
        // C s_18_32: const #0u : u8
        let s_18_32: u8 = 0;
        // S s_18_33: call _update_ID_MMFR0_Type_PMSA(s_18_31, s_18_32)
        let s_18_33: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_PMSA(
            state,
            tracer,
            s_18_31,
            s_18_32,
        );
        // S s_18_34: call ID_MMFR0_write(s_18_33)
        let s_18_34: () = ID_MMFR0_write(state, tracer, s_18_33);
        // C s_18_35: const #() : ()
        let s_18_35: () = ();
        // S s_18_36: call ID_MMFR0_read(s_18_35)
        let s_18_36: ProductType700c18a878c5601b = ID_MMFR0_read(state, tracer, s_18_35);
        // C s_18_37: const #5u : u8
        let s_18_37: u8 = 5;
        // S s_18_38: call _update_ID_MMFR0_Type_VMSA(s_18_36, s_18_37)
        let s_18_38: ProductType700c18a878c5601b = u_update_ID_MMFR0_Type_VMSA(
            state,
            tracer,
            s_18_36,
            s_18_37,
        );
        // S s_18_39: call ID_MMFR0_write(s_18_38)
        let s_18_39: () = ID_MMFR0_write(state, tracer, s_18_38);
        // C s_18_40: const #() : ()
        let s_18_40: () = ();
        // S s_18_41: call ID_MMFR1_read(s_18_40)
        let s_18_41: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_40);
        // C s_18_42: const #2u : u8
        let s_18_42: u8 = 2;
        // S s_18_43: call _update_ID_MMFR1_Type_BPred(s_18_41, s_18_42)
        let s_18_43: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_BPred(
            state,
            tracer,
            s_18_41,
            s_18_42,
        );
        // S s_18_44: call ID_MMFR1_write(s_18_43)
        let s_18_44: () = ID_MMFR1_write(state, tracer, s_18_43);
        // C s_18_45: const #() : ()
        let s_18_45: () = ();
        // S s_18_46: call ID_MMFR1_read(s_18_45)
        let s_18_46: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_45);
        // C s_18_47: const #0u : u8
        let s_18_47: u8 = 0;
        // S s_18_48: call _update_ID_MMFR1_Type_L1TstCln(s_18_46, s_18_47)
        let s_18_48: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_L1TstCln(
            state,
            tracer,
            s_18_46,
            s_18_47,
        );
        // S s_18_49: call ID_MMFR1_write(s_18_48)
        let s_18_49: () = ID_MMFR1_write(state, tracer, s_18_48);
        // C s_18_50: const #() : ()
        let s_18_50: () = ();
        // S s_18_51: call ID_MMFR1_read(s_18_50)
        let s_18_51: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_50);
        // C s_18_52: const #0u : u8
        let s_18_52: u8 = 0;
        // S s_18_53: call _update_ID_MMFR1_Type_L1Uni(s_18_51, s_18_52)
        let s_18_53: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_L1Uni(
            state,
            tracer,
            s_18_51,
            s_18_52,
        );
        // S s_18_54: call ID_MMFR1_write(s_18_53)
        let s_18_54: () = ID_MMFR1_write(state, tracer, s_18_53);
        // C s_18_55: const #() : ()
        let s_18_55: () = ();
        // S s_18_56: call ID_MMFR1_read(s_18_55)
        let s_18_56: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_55);
        // C s_18_57: const #0u : u8
        let s_18_57: u8 = 0;
        // S s_18_58: call _update_ID_MMFR1_Type_L1Hvd(s_18_56, s_18_57)
        let s_18_58: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_L1Hvd(
            state,
            tracer,
            s_18_56,
            s_18_57,
        );
        // S s_18_59: call ID_MMFR1_write(s_18_58)
        let s_18_59: () = ID_MMFR1_write(state, tracer, s_18_58);
        // C s_18_60: const #() : ()
        let s_18_60: () = ();
        // S s_18_61: call ID_MMFR1_read(s_18_60)
        let s_18_61: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_60);
        // C s_18_62: const #0u : u8
        let s_18_62: u8 = 0;
        // S s_18_63: call _update_ID_MMFR1_Type_L1UniSW(s_18_61, s_18_62)
        let s_18_63: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_L1UniSW(
            state,
            tracer,
            s_18_61,
            s_18_62,
        );
        // S s_18_64: call ID_MMFR1_write(s_18_63)
        let s_18_64: () = ID_MMFR1_write(state, tracer, s_18_63);
        // C s_18_65: const #() : ()
        let s_18_65: () = ();
        // S s_18_66: call ID_MMFR1_read(s_18_65)
        let s_18_66: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_65);
        // C s_18_67: const #0u : u8
        let s_18_67: u8 = 0;
        // S s_18_68: call _update_ID_MMFR1_Type_L1HvdSW(s_18_66, s_18_67)
        let s_18_68: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_L1HvdSW(
            state,
            tracer,
            s_18_66,
            s_18_67,
        );
        // S s_18_69: call ID_MMFR1_write(s_18_68)
        let s_18_69: () = ID_MMFR1_write(state, tracer, s_18_68);
        // C s_18_70: const #() : ()
        let s_18_70: () = ();
        // S s_18_71: call ID_MMFR1_read(s_18_70)
        let s_18_71: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_70);
        // C s_18_72: const #0u : u8
        let s_18_72: u8 = 0;
        // S s_18_73: call _update_ID_MMFR1_Type_L1UniVA(s_18_71, s_18_72)
        let s_18_73: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_L1UniVA(
            state,
            tracer,
            s_18_71,
            s_18_72,
        );
        // S s_18_74: call ID_MMFR1_write(s_18_73)
        let s_18_74: () = ID_MMFR1_write(state, tracer, s_18_73);
        // C s_18_75: const #() : ()
        let s_18_75: () = ();
        // S s_18_76: call ID_MMFR1_read(s_18_75)
        let s_18_76: ProductType700c18a878c5601b = ID_MMFR1_read(state, tracer, s_18_75);
        // C s_18_77: const #0u : u8
        let s_18_77: u8 = 0;
        // S s_18_78: call _update_ID_MMFR1_Type_L1HvdVA(s_18_76, s_18_77)
        let s_18_78: ProductType700c18a878c5601b = u_update_ID_MMFR1_Type_L1HvdVA(
            state,
            tracer,
            s_18_76,
            s_18_77,
        );
        // S s_18_79: call ID_MMFR1_write(s_18_78)
        let s_18_79: () = ID_MMFR1_write(state, tracer, s_18_78);
        // C s_18_80: const #() : ()
        let s_18_80: () = ();
        // S s_18_81: call ID_MMFR2_read(s_18_80)
        let s_18_81: ProductType700c18a878c5601b = ID_MMFR2_read(state, tracer, s_18_80);
        // C s_18_82: const #0u : u8
        let s_18_82: u8 = 0;
        // S s_18_83: call _update_ID_MMFR2_Type_HWAccFlg(s_18_81, s_18_82)
        let s_18_83: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_HWAccFlg(
            state,
            tracer,
            s_18_81,
            s_18_82,
        );
        // S s_18_84: call ID_MMFR2_write(s_18_83)
        let s_18_84: () = ID_MMFR2_write(state, tracer, s_18_83);
        // C s_18_85: const #() : ()
        let s_18_85: () = ();
        // S s_18_86: call ID_MMFR2_read(s_18_85)
        let s_18_86: ProductType700c18a878c5601b = ID_MMFR2_read(state, tracer, s_18_85);
        // C s_18_87: const #1u : u8
        let s_18_87: u8 = 1;
        // S s_18_88: call _update_ID_MMFR2_Type_WFIStall(s_18_86, s_18_87)
        let s_18_88: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_WFIStall(
            state,
            tracer,
            s_18_86,
            s_18_87,
        );
        // S s_18_89: call ID_MMFR2_write(s_18_88)
        let s_18_89: () = ID_MMFR2_write(state, tracer, s_18_88);
        // C s_18_90: const #() : ()
        let s_18_90: () = ();
        // S s_18_91: call ID_MMFR2_read(s_18_90)
        let s_18_91: ProductType700c18a878c5601b = ID_MMFR2_read(state, tracer, s_18_90);
        // C s_18_92: const #2u : u8
        let s_18_92: u8 = 2;
        // S s_18_93: call _update_ID_MMFR2_Type_MemBarr(s_18_91, s_18_92)
        let s_18_93: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_MemBarr(
            state,
            tracer,
            s_18_91,
            s_18_92,
        );
        // S s_18_94: call ID_MMFR2_write(s_18_93)
        let s_18_94: () = ID_MMFR2_write(state, tracer, s_18_93);
        // C s_18_95: const #() : ()
        let s_18_95: () = ();
        // S s_18_96: call ID_MMFR2_read(s_18_95)
        let s_18_96: ProductType700c18a878c5601b = ID_MMFR2_read(state, tracer, s_18_95);
        // C s_18_97: const #6u : u8
        let s_18_97: u8 = 6;
        // S s_18_98: call _update_ID_MMFR2_Type_UniTLB(s_18_96, s_18_97)
        let s_18_98: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_UniTLB(
            state,
            tracer,
            s_18_96,
            s_18_97,
        );
        // S s_18_99: call ID_MMFR2_write(s_18_98)
        let s_18_99: () = ID_MMFR2_write(state, tracer, s_18_98);
        // C s_18_100: const #() : ()
        let s_18_100: () = ();
        // S s_18_101: call ID_MMFR2_read(s_18_100)
        let s_18_101: ProductType700c18a878c5601b = ID_MMFR2_read(
            state,
            tracer,
            s_18_100,
        );
        // C s_18_102: const #0u : u8
        let s_18_102: u8 = 0;
        // S s_18_103: call _update_ID_MMFR2_Type_HvdTLB(s_18_101, s_18_102)
        let s_18_103: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_HvdTLB(
            state,
            tracer,
            s_18_101,
            s_18_102,
        );
        // S s_18_104: call ID_MMFR2_write(s_18_103)
        let s_18_104: () = ID_MMFR2_write(state, tracer, s_18_103);
        // C s_18_105: const #() : ()
        let s_18_105: () = ();
        // S s_18_106: call ID_MMFR2_read(s_18_105)
        let s_18_106: ProductType700c18a878c5601b = ID_MMFR2_read(
            state,
            tracer,
            s_18_105,
        );
        // C s_18_107: const #0u : u8
        let s_18_107: u8 = 0;
        // S s_18_108: call _update_ID_MMFR2_Type_L1HvdRng(s_18_106, s_18_107)
        let s_18_108: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_L1HvdRng(
            state,
            tracer,
            s_18_106,
            s_18_107,
        );
        // S s_18_109: call ID_MMFR2_write(s_18_108)
        let s_18_109: () = ID_MMFR2_write(state, tracer, s_18_108);
        // C s_18_110: const #() : ()
        let s_18_110: () = ();
        // S s_18_111: call ID_MMFR2_read(s_18_110)
        let s_18_111: ProductType700c18a878c5601b = ID_MMFR2_read(
            state,
            tracer,
            s_18_110,
        );
        // C s_18_112: const #0u : u8
        let s_18_112: u8 = 0;
        // S s_18_113: call _update_ID_MMFR2_Type_L1HvdBG(s_18_111, s_18_112)
        let s_18_113: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_L1HvdBG(
            state,
            tracer,
            s_18_111,
            s_18_112,
        );
        // S s_18_114: call ID_MMFR2_write(s_18_113)
        let s_18_114: () = ID_MMFR2_write(state, tracer, s_18_113);
        // C s_18_115: const #() : ()
        let s_18_115: () = ();
        // S s_18_116: call ID_MMFR2_read(s_18_115)
        let s_18_116: ProductType700c18a878c5601b = ID_MMFR2_read(
            state,
            tracer,
            s_18_115,
        );
        // C s_18_117: const #0u : u8
        let s_18_117: u8 = 0;
        // S s_18_118: call _update_ID_MMFR2_Type_L1HvdFG(s_18_116, s_18_117)
        let s_18_118: ProductType700c18a878c5601b = u_update_ID_MMFR2_Type_L1HvdFG(
            state,
            tracer,
            s_18_116,
            s_18_117,
        );
        // S s_18_119: call ID_MMFR2_write(s_18_118)
        let s_18_119: () = ID_MMFR2_write(state, tracer, s_18_118);
        // C s_18_120: const #() : ()
        let s_18_120: () = ();
        // S s_18_121: call ID_MMFR3_read(s_18_120)
        let s_18_121: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_120,
        );
        // C s_18_122: const #0u : u8
        let s_18_122: u8 = 0;
        // S s_18_123: call _update_ID_MMFR3_Type_Supersec(s_18_121, s_18_122)
        let s_18_123: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_Supersec(
            state,
            tracer,
            s_18_121,
            s_18_122,
        );
        // S s_18_124: call ID_MMFR3_write(s_18_123)
        let s_18_124: () = ID_MMFR3_write(state, tracer, s_18_123);
        // C s_18_125: const #() : ()
        let s_18_125: () = ();
        // S s_18_126: call ID_MMFR3_read(s_18_125)
        let s_18_126: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_125,
        );
        // C s_18_127: const #2u : u8
        let s_18_127: u8 = 2;
        // S s_18_128: call _update_ID_MMFR3_Type_CMemSz(s_18_126, s_18_127)
        let s_18_128: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_CMemSz(
            state,
            tracer,
            s_18_126,
            s_18_127,
        );
        // S s_18_129: call ID_MMFR3_write(s_18_128)
        let s_18_129: () = ID_MMFR3_write(state, tracer, s_18_128);
        // C s_18_130: const #() : ()
        let s_18_130: () = ();
        // S s_18_131: call ID_MMFR3_read(s_18_130)
        let s_18_131: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_130,
        );
        // C s_18_132: const #1u : u8
        let s_18_132: u8 = 1;
        // S s_18_133: call _update_ID_MMFR3_Type_CohWalk(s_18_131, s_18_132)
        let s_18_133: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_CohWalk(
            state,
            tracer,
            s_18_131,
            s_18_132,
        );
        // S s_18_134: call ID_MMFR3_write(s_18_133)
        let s_18_134: () = ID_MMFR3_write(state, tracer, s_18_133);
        // C s_18_135: const #() : ()
        let s_18_135: () = ();
        // S s_18_136: call ID_MMFR3_read(s_18_135)
        let s_18_136: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_135,
        );
        // C s_18_137: const #1u : u8
        let s_18_137: u8 = 1;
        // S s_18_138: call _update_ID_MMFR3_Type_PAN(s_18_136, s_18_137)
        let s_18_138: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_PAN(
            state,
            tracer,
            s_18_136,
            s_18_137,
        );
        // S s_18_139: call ID_MMFR3_write(s_18_138)
        let s_18_139: () = ID_MMFR3_write(state, tracer, s_18_138);
        // C s_18_140: const #() : ()
        let s_18_140: () = ();
        // S s_18_141: call ID_MMFR3_read(s_18_140)
        let s_18_141: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_140,
        );
        // C s_18_142: const #2u : u8
        let s_18_142: u8 = 2;
        // S s_18_143: call _update_ID_MMFR3_Type_MaintBcst(s_18_141, s_18_142)
        let s_18_143: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_MaintBcst(
            state,
            tracer,
            s_18_141,
            s_18_142,
        );
        // S s_18_144: call ID_MMFR3_write(s_18_143)
        let s_18_144: () = ID_MMFR3_write(state, tracer, s_18_143);
        // C s_18_145: const #() : ()
        let s_18_145: () = ();
        // S s_18_146: call ID_MMFR3_read(s_18_145)
        let s_18_146: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_145,
        );
        // C s_18_147: const #2u : u8
        let s_18_147: u8 = 2;
        // S s_18_148: call _update_ID_MMFR3_Type_BPMaint(s_18_146, s_18_147)
        let s_18_148: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_BPMaint(
            state,
            tracer,
            s_18_146,
            s_18_147,
        );
        // S s_18_149: call ID_MMFR3_write(s_18_148)
        let s_18_149: () = ID_MMFR3_write(state, tracer, s_18_148);
        // C s_18_150: const #() : ()
        let s_18_150: () = ();
        // S s_18_151: call ID_MMFR3_read(s_18_150)
        let s_18_151: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_150,
        );
        // C s_18_152: const #1u : u8
        let s_18_152: u8 = 1;
        // S s_18_153: call _update_ID_MMFR3_Type_CMaintSW(s_18_151, s_18_152)
        let s_18_153: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_CMaintSW(
            state,
            tracer,
            s_18_151,
            s_18_152,
        );
        // S s_18_154: call ID_MMFR3_write(s_18_153)
        let s_18_154: () = ID_MMFR3_write(state, tracer, s_18_153);
        // C s_18_155: const #() : ()
        let s_18_155: () = ();
        // S s_18_156: call ID_MMFR3_read(s_18_155)
        let s_18_156: ProductType700c18a878c5601b = ID_MMFR3_read(
            state,
            tracer,
            s_18_155,
        );
        // C s_18_157: const #1u : u8
        let s_18_157: u8 = 1;
        // S s_18_158: call _update_ID_MMFR3_Type_CMaintVA(s_18_156, s_18_157)
        let s_18_158: ProductType700c18a878c5601b = u_update_ID_MMFR3_Type_CMaintVA(
            state,
            tracer,
            s_18_156,
            s_18_157,
        );
        // S s_18_159: call ID_MMFR3_write(s_18_158)
        let s_18_159: () = ID_MMFR3_write(state, tracer, s_18_158);
        // C s_18_160: const #() : ()
        let s_18_160: () = ();
        // S s_18_161: call ID_PFR0_read(s_18_160)
        let s_18_161: ProductType700c18a878c5601b = ID_PFR0_read(
            state,
            tracer,
            s_18_160,
        );
        // C s_18_162: const #2u : u8
        let s_18_162: u8 = 2;
        // S s_18_163: call _update_ID_PFR0_Type_RAS(s_18_161, s_18_162)
        let s_18_163: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_RAS(
            state,
            tracer,
            s_18_161,
            s_18_162,
        );
        // S s_18_164: call ID_PFR0_write(s_18_163)
        let s_18_164: () = ID_PFR0_write(state, tracer, s_18_163);
        // C s_18_165: const #() : ()
        let s_18_165: () = ();
        // S s_18_166: call ID_PFR0_read(s_18_165)
        let s_18_166: ProductType700c18a878c5601b = ID_PFR0_read(
            state,
            tracer,
            s_18_165,
        );
        // C s_18_167: const #0u : u8
        let s_18_167: u8 = 0;
        // S s_18_168: call _update_ID_PFR0_Type_State3(s_18_166, s_18_167)
        let s_18_168: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_State3(
            state,
            tracer,
            s_18_166,
            s_18_167,
        );
        // S s_18_169: call ID_PFR0_write(s_18_168)
        let s_18_169: () = ID_PFR0_write(state, tracer, s_18_168);
        // C s_18_170: const #() : ()
        let s_18_170: () = ();
        // S s_18_171: call ID_PFR0_read(s_18_170)
        let s_18_171: ProductType700c18a878c5601b = ID_PFR0_read(
            state,
            tracer,
            s_18_170,
        );
        // C s_18_172: const #1u : u8
        let s_18_172: u8 = 1;
        // S s_18_173: call _update_ID_PFR0_Type_State2(s_18_171, s_18_172)
        let s_18_173: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_State2(
            state,
            tracer,
            s_18_171,
            s_18_172,
        );
        // S s_18_174: call ID_PFR0_write(s_18_173)
        let s_18_174: () = ID_PFR0_write(state, tracer, s_18_173);
        // C s_18_175: const #() : ()
        let s_18_175: () = ();
        // S s_18_176: call ID_PFR0_read(s_18_175)
        let s_18_176: ProductType700c18a878c5601b = ID_PFR0_read(
            state,
            tracer,
            s_18_175,
        );
        // C s_18_177: const #3u : u8
        let s_18_177: u8 = 3;
        // S s_18_178: call _update_ID_PFR0_Type_State1(s_18_176, s_18_177)
        let s_18_178: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_State1(
            state,
            tracer,
            s_18_176,
            s_18_177,
        );
        // S s_18_179: call ID_PFR0_write(s_18_178)
        let s_18_179: () = ID_PFR0_write(state, tracer, s_18_178);
        // C s_18_180: const #() : ()
        let s_18_180: () = ();
        // S s_18_181: call ID_PFR0_read(s_18_180)
        let s_18_181: ProductType700c18a878c5601b = ID_PFR0_read(
            state,
            tracer,
            s_18_180,
        );
        // C s_18_182: const #1u : u8
        let s_18_182: u8 = 1;
        // S s_18_183: call _update_ID_PFR0_Type_State0(s_18_181, s_18_182)
        let s_18_183: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_State0(
            state,
            tracer,
            s_18_181,
            s_18_182,
        );
        // S s_18_184: call ID_PFR0_write(s_18_183)
        let s_18_184: () = ID_PFR0_write(state, tracer, s_18_183);
        // C s_18_185: const #() : ()
        let s_18_185: () = ();
        // S s_18_186: call ID_DFR1_read(s_18_185)
        let s_18_186: ProductType700c18a878c5601b = ID_DFR1_read(
            state,
            tracer,
            s_18_185,
        );
        // C s_18_187: const #1u : u8
        let s_18_187: u8 = 1;
        // S s_18_188: call _update_ID_DFR1_Type_MTPMU(s_18_186, s_18_187)
        let s_18_188: ProductType700c18a878c5601b = u_update_ID_DFR1_Type_MTPMU(
            state,
            tracer,
            s_18_186,
            s_18_187,
        );
        // S s_18_189: call ID_DFR1_write(s_18_188)
        let s_18_189: () = ID_DFR1_write(state, tracer, s_18_188);
        // C s_18_190: const #5u : u32
        let s_18_190: u32 = 5;
        // S s_18_191: call HasArchVersion(s_18_190)
        let s_18_191: bool = HasArchVersion(state, tracer, s_18_190);
        // N s_18_192: branch s_18_191 b32 b19
        if s_18_191 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call ID_DFR0_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_20_0);
        // C s_20_2: const #1u : u8
        let s_20_2: u8 = 1;
        // S s_20_3: call _update_ID_DFR0_Type_MMapTrc(s_20_1, s_20_2)
        let s_20_3: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_MMapTrc(
            state,
            tracer,
            s_20_1,
            s_20_2,
        );
        // S s_20_4: call ID_DFR0_write(s_20_3)
        let s_20_4: () = ID_DFR0_write(state, tracer, s_20_3);
        // C s_20_5: const #() : ()
        let s_20_5: () = ();
        // S s_20_6: call ID_DFR0_read(s_20_5)
        let s_20_6: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_20_5);
        // C s_20_7: const #1u : u8
        let s_20_7: u8 = 1;
        // S s_20_8: call _update_ID_DFR0_Type_CopTrc(s_20_6, s_20_7)
        let s_20_8: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopTrc(
            state,
            tracer,
            s_20_6,
            s_20_7,
        );
        // S s_20_9: call ID_DFR0_write(s_20_8)
        let s_20_9: () = ID_DFR0_write(state, tracer, s_20_8);
        // C s_20_10: const #() : ()
        let s_20_10: () = ();
        // S s_20_11: call ID_PFR1_read(s_20_10)
        let s_20_11: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_20_10);
        // C s_20_12: const #0u : u8
        let s_20_12: u8 = 0;
        // S s_20_13: call _update_ID_PFR1_Type_GIC(s_20_11, s_20_12)
        let s_20_13: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_GIC(
            state,
            tracer,
            s_20_11,
            s_20_12,
        );
        // S s_20_14: call ID_PFR1_write(s_20_13)
        let s_20_14: () = ID_PFR1_write(state, tracer, s_20_13);
        // C s_20_15: const #() : ()
        let s_20_15: () = ();
        // S s_20_16: call ID_PFR1_read(s_20_15)
        let s_20_16: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_20_15);
        // C s_20_17: const #0u : u8
        let s_20_17: u8 = 0;
        // S s_20_18: call _update_ID_PFR1_Type_Virt_frac(s_20_16, s_20_17)
        let s_20_18: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_Virt_frac(
            state,
            tracer,
            s_20_16,
            s_20_17,
        );
        // S s_20_19: call ID_PFR1_write(s_20_18)
        let s_20_19: () = ID_PFR1_write(state, tracer, s_20_18);
        // C s_20_20: const #() : ()
        let s_20_20: () = ();
        // S s_20_21: call ID_PFR1_read(s_20_20)
        let s_20_21: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_20_20);
        // C s_20_22: const #0u : u8
        let s_20_22: u8 = 0;
        // S s_20_23: call _update_ID_PFR1_Type_Sec_frac(s_20_21, s_20_22)
        let s_20_23: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_Sec_frac(
            state,
            tracer,
            s_20_21,
            s_20_22,
        );
        // S s_20_24: call ID_PFR1_write(s_20_23)
        let s_20_24: () = ID_PFR1_write(state, tracer, s_20_23);
        // C s_20_25: const #() : ()
        let s_20_25: () = ();
        // S s_20_26: call ID_PFR1_read(s_20_25)
        let s_20_26: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_20_25);
        // C s_20_27: const #0u : u8
        let s_20_27: u8 = 0;
        // S s_20_28: call _update_ID_PFR1_Type_MProgMod(s_20_26, s_20_27)
        let s_20_28: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_MProgMod(
            state,
            tracer,
            s_20_26,
            s_20_27,
        );
        // S s_20_29: call ID_PFR1_write(s_20_28)
        let s_20_29: () = ID_PFR1_write(state, tracer, s_20_28);
        // C s_20_30: const #() : ()
        let s_20_30: () = ();
        // S s_20_31: call ID_PFR1_read(s_20_30)
        let s_20_31: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_20_30);
        // C s_20_32: const #0u : u8
        let s_20_32: u8 = 0;
        // S s_20_33: call _update_ID_PFR1_Type_ProgMod(s_20_31, s_20_32)
        let s_20_33: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_ProgMod(
            state,
            tracer,
            s_20_31,
            s_20_32,
        );
        // S s_20_34: call ID_PFR1_write(s_20_33)
        let s_20_34: () = ID_PFR1_write(state, tracer, s_20_33);
        // C s_20_35: const #() : ()
        let s_20_35: () = ();
        // S s_20_36: call ID_PFR1_read(s_20_35)
        let s_20_36: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_20_35);
        // C s_20_37: const #0u : u8
        let s_20_37: u8 = 0;
        // S s_20_38: call _update_ID_PFR1_Type_Security(s_20_36, s_20_37)
        let s_20_38: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_Security(
            state,
            tracer,
            s_20_36,
            s_20_37,
        );
        // S s_20_39: call ID_PFR1_write(s_20_38)
        let s_20_39: () = ID_PFR1_write(state, tracer, s_20_38);
        // C s_20_40: const #() : ()
        let s_20_40: () = ();
        // S s_20_41: call ID_PFR1_read(s_20_40)
        let s_20_41: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_20_40);
        // C s_20_42: const #0u : u8
        let s_20_42: u8 = 0;
        // S s_20_43: call _update_ID_PFR1_Type_Virtualization(s_20_41, s_20_42)
        let s_20_43: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_Virtualization(
            state,
            tracer,
            s_20_41,
            s_20_42,
        );
        // S s_20_44: call ID_PFR1_write(s_20_43)
        let s_20_44: () = ID_PFR1_write(state, tracer, s_20_43);
        // C s_20_45: const #() : ()
        let s_20_45: () = ();
        // S s_20_46: call MIDR_read(s_20_45)
        let s_20_46: ProductType700c18a878c5601b = MIDR_read(state, tracer, s_20_45);
        // C s_20_47: const #65u : u8
        let s_20_47: u8 = 65;
        // S s_20_48: call _update_MIDR_Type_Implementer(s_20_46, s_20_47)
        let s_20_48: ProductType700c18a878c5601b = u_update_MIDR_Type_Implementer(
            state,
            tracer,
            s_20_46,
            s_20_47,
        );
        // S s_20_49: call MIDR_write(s_20_48)
        let s_20_49: () = MIDR_write(state, tracer, s_20_48);
        // C s_20_50: const #() : ()
        let s_20_50: () = ();
        // S s_20_51: call MIDR_read(s_20_50)
        let s_20_51: ProductType700c18a878c5601b = MIDR_read(state, tracer, s_20_50);
        // C s_20_52: const #0u : u8
        let s_20_52: u8 = 0;
        // S s_20_53: call _update_MIDR_Type_Variant(s_20_51, s_20_52)
        let s_20_53: ProductType700c18a878c5601b = u_update_MIDR_Type_Variant(
            state,
            tracer,
            s_20_51,
            s_20_52,
        );
        // S s_20_54: call MIDR_write(s_20_53)
        let s_20_54: () = MIDR_write(state, tracer, s_20_53);
        // C s_20_55: const #() : ()
        let s_20_55: () = ();
        // S s_20_56: call MIDR_read(s_20_55)
        let s_20_56: ProductType700c18a878c5601b = MIDR_read(state, tracer, s_20_55);
        // C s_20_57: const #15u : u8
        let s_20_57: u8 = 15;
        // S s_20_58: call _update_MIDR_Type_Architecture(s_20_56, s_20_57)
        let s_20_58: ProductType700c18a878c5601b = u_update_MIDR_Type_Architecture(
            state,
            tracer,
            s_20_56,
            s_20_57,
        );
        // S s_20_59: call MIDR_write(s_20_58)
        let s_20_59: () = MIDR_write(state, tracer, s_20_58);
        // C s_20_60: const #() : ()
        let s_20_60: () = ();
        // S s_20_61: call MIDR_read(s_20_60)
        let s_20_61: ProductType700c18a878c5601b = MIDR_read(state, tracer, s_20_60);
        // C s_20_62: const #3343u : u12
        let s_20_62: u16 = 3343;
        // S s_20_63: call _update_MIDR_Type_PartNum(s_20_61, s_20_62)
        let s_20_63: ProductType700c18a878c5601b = u_update_MIDR_Type_PartNum(
            state,
            tracer,
            s_20_61,
            s_20_62,
        );
        // S s_20_64: call MIDR_write(s_20_63)
        let s_20_64: () = MIDR_write(state, tracer, s_20_63);
        // C s_20_65: const #() : ()
        let s_20_65: () = ();
        // S s_20_66: call MIDR_read(s_20_65)
        let s_20_66: ProductType700c18a878c5601b = MIDR_read(state, tracer, s_20_65);
        // C s_20_67: const #0u : u8
        let s_20_67: u8 = 0;
        // S s_20_68: call _update_MIDR_Type_Revision(s_20_66, s_20_67)
        let s_20_68: ProductType700c18a878c5601b = u_update_MIDR_Type_Revision(
            state,
            tracer,
            s_20_66,
            s_20_67,
        );
        // S s_20_69: call MIDR_write(s_20_68)
        let s_20_69: () = MIDR_write(state, tracer, s_20_68);
        // C s_20_70: const #() : ()
        let s_20_70: () = ();
        // S s_20_71: call VPIDR_read(s_20_70)
        let s_20_71: ProductType700c18a878c5601b = VPIDR_read(state, tracer, s_20_70);
        // C s_20_72: const #65u : u8
        let s_20_72: u8 = 65;
        // S s_20_73: call _update_VPIDR_Type_Implementer(s_20_71, s_20_72)
        let s_20_73: ProductType700c18a878c5601b = u_update_VPIDR_Type_Implementer(
            state,
            tracer,
            s_20_71,
            s_20_72,
        );
        // S s_20_74: call VPIDR_write(s_20_73)
        let s_20_74: () = VPIDR_write(state, tracer, s_20_73);
        // C s_20_75: const #() : ()
        let s_20_75: () = ();
        // S s_20_76: call VPIDR_read(s_20_75)
        let s_20_76: ProductType700c18a878c5601b = VPIDR_read(state, tracer, s_20_75);
        // C s_20_77: const #0u : u8
        let s_20_77: u8 = 0;
        // S s_20_78: call _update_VPIDR_Type_Variant(s_20_76, s_20_77)
        let s_20_78: ProductType700c18a878c5601b = u_update_VPIDR_Type_Variant(
            state,
            tracer,
            s_20_76,
            s_20_77,
        );
        // S s_20_79: call VPIDR_write(s_20_78)
        let s_20_79: () = VPIDR_write(state, tracer, s_20_78);
        // C s_20_80: const #() : ()
        let s_20_80: () = ();
        // S s_20_81: call VPIDR_read(s_20_80)
        let s_20_81: ProductType700c18a878c5601b = VPIDR_read(state, tracer, s_20_80);
        // C s_20_82: const #15u : u8
        let s_20_82: u8 = 15;
        // S s_20_83: call _update_VPIDR_Type_Architecture(s_20_81, s_20_82)
        let s_20_83: ProductType700c18a878c5601b = u_update_VPIDR_Type_Architecture(
            state,
            tracer,
            s_20_81,
            s_20_82,
        );
        // S s_20_84: call VPIDR_write(s_20_83)
        let s_20_84: () = VPIDR_write(state, tracer, s_20_83);
        // C s_20_85: const #() : ()
        let s_20_85: () = ();
        // S s_20_86: call VPIDR_read(s_20_85)
        let s_20_86: ProductType700c18a878c5601b = VPIDR_read(state, tracer, s_20_85);
        // C s_20_87: const #3343u : u12
        let s_20_87: u16 = 3343;
        // S s_20_88: call _update_VPIDR_Type_PartNum(s_20_86, s_20_87)
        let s_20_88: ProductType700c18a878c5601b = u_update_VPIDR_Type_PartNum(
            state,
            tracer,
            s_20_86,
            s_20_87,
        );
        // S s_20_89: call VPIDR_write(s_20_88)
        let s_20_89: () = VPIDR_write(state, tracer, s_20_88);
        // C s_20_90: const #() : ()
        let s_20_90: () = ();
        // S s_20_91: call VPIDR_read(s_20_90)
        let s_20_91: ProductType700c18a878c5601b = VPIDR_read(state, tracer, s_20_90);
        // C s_20_92: const #0u : u8
        let s_20_92: u8 = 0;
        // S s_20_93: call _update_VPIDR_Type_Revision(s_20_91, s_20_92)
        let s_20_93: ProductType700c18a878c5601b = u_update_VPIDR_Type_Revision(
            state,
            tracer,
            s_20_91,
            s_20_92,
        );
        // S s_20_94: call VPIDR_write(s_20_93)
        let s_20_94: () = VPIDR_write(state, tracer, s_20_93);
        // C s_20_95: const #() : ()
        let s_20_95: () = ();
        // S s_20_96: call MPIDR_read(s_20_95)
        let s_20_96: ProductType700c18a878c5601b = MPIDR_read(state, tracer, s_20_95);
        // C s_20_97: const #1u : u8
        let s_20_97: bool = true;
        // S s_20_98: call _update_MPIDR_Type_M(s_20_96, s_20_97)
        let s_20_98: ProductType700c18a878c5601b = u_update_MPIDR_Type_M(
            state,
            tracer,
            s_20_96,
            s_20_97,
        );
        // S s_20_99: call MPIDR_write(s_20_98)
        let s_20_99: () = MPIDR_write(state, tracer, s_20_98);
        // C s_20_100: const #() : ()
        let s_20_100: () = ();
        // S s_20_101: call MPIDR_read(s_20_100)
        let s_20_101: ProductType700c18a878c5601b = MPIDR_read(state, tracer, s_20_100);
        // C s_20_102: const #0u : u8
        let s_20_102: bool = false;
        // S s_20_103: call _update_MPIDR_Type_U(s_20_101, s_20_102)
        let s_20_103: ProductType700c18a878c5601b = u_update_MPIDR_Type_U(
            state,
            tracer,
            s_20_101,
            s_20_102,
        );
        // S s_20_104: call MPIDR_write(s_20_103)
        let s_20_104: () = MPIDR_write(state, tracer, s_20_103);
        // C s_20_105: const #() : ()
        let s_20_105: () = ();
        // S s_20_106: call MPIDR_read(s_20_105)
        let s_20_106: ProductType700c18a878c5601b = MPIDR_read(state, tracer, s_20_105);
        // C s_20_107: const #0u : u8
        let s_20_107: bool = false;
        // S s_20_108: call _update_MPIDR_Type_MT(s_20_106, s_20_107)
        let s_20_108: ProductType700c18a878c5601b = u_update_MPIDR_Type_MT(
            state,
            tracer,
            s_20_106,
            s_20_107,
        );
        // S s_20_109: call MPIDR_write(s_20_108)
        let s_20_109: () = MPIDR_write(state, tracer, s_20_108);
        // C s_20_110: const #() : ()
        let s_20_110: () = ();
        // S s_20_111: call MPIDR_read(s_20_110)
        let s_20_111: ProductType700c18a878c5601b = MPIDR_read(state, tracer, s_20_110);
        // C s_20_112: const #0u : u8
        let s_20_112: u8 = 0;
        // S s_20_113: call _update_MPIDR_Type_Aff2(s_20_111, s_20_112)
        let s_20_113: ProductType700c18a878c5601b = u_update_MPIDR_Type_Aff2(
            state,
            tracer,
            s_20_111,
            s_20_112,
        );
        // S s_20_114: call MPIDR_write(s_20_113)
        let s_20_114: () = MPIDR_write(state, tracer, s_20_113);
        // C s_20_115: const #() : ()
        let s_20_115: () = ();
        // S s_20_116: call MPIDR_read(s_20_115)
        let s_20_116: ProductType700c18a878c5601b = MPIDR_read(state, tracer, s_20_115);
        // C s_20_117: const #0u : u8
        let s_20_117: u8 = 0;
        // S s_20_118: call _update_MPIDR_Type_Aff1(s_20_116, s_20_117)
        let s_20_118: ProductType700c18a878c5601b = u_update_MPIDR_Type_Aff1(
            state,
            tracer,
            s_20_116,
            s_20_117,
        );
        // S s_20_119: call MPIDR_write(s_20_118)
        let s_20_119: () = MPIDR_write(state, tracer, s_20_118);
        // C s_20_120: const #() : ()
        let s_20_120: () = ();
        // S s_20_121: call MPIDR_read(s_20_120)
        let s_20_121: ProductType700c18a878c5601b = MPIDR_read(state, tracer, s_20_120);
        // C s_20_122: const #0u : u8
        let s_20_122: u8 = 0;
        // S s_20_123: call _update_MPIDR_Type_Aff0(s_20_121, s_20_122)
        let s_20_123: ProductType700c18a878c5601b = u_update_MPIDR_Type_Aff0(
            state,
            tracer,
            s_20_121,
            s_20_122,
        );
        // S s_20_124: call MPIDR_write(s_20_123)
        let s_20_124: () = MPIDR_write(state, tracer, s_20_123);
        // C s_20_125: const #() : ()
        let s_20_125: () = ();
        // S s_20_126: call MVFR0_read(s_20_125)
        let s_20_126: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_125);
        // C s_20_127: const #1u : u8
        let s_20_127: u8 = 1;
        // S s_20_128: call _update_MVFR0_Type_FPRound(s_20_126, s_20_127)
        let s_20_128: ProductType700c18a878c5601b = u_update_MVFR0_Type_FPRound(
            state,
            tracer,
            s_20_126,
            s_20_127,
        );
        // S s_20_129: call MVFR0_write(s_20_128)
        let s_20_129: () = MVFR0_write(state, tracer, s_20_128);
        // C s_20_130: const #() : ()
        let s_20_130: () = ();
        // S s_20_131: call MVFR0_read(s_20_130)
        let s_20_131: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_130);
        // C s_20_132: const #0u : u8
        let s_20_132: u8 = 0;
        // S s_20_133: call _update_MVFR0_Type_FPShVec(s_20_131, s_20_132)
        let s_20_133: ProductType700c18a878c5601b = u_update_MVFR0_Type_FPShVec(
            state,
            tracer,
            s_20_131,
            s_20_132,
        );
        // S s_20_134: call MVFR0_write(s_20_133)
        let s_20_134: () = MVFR0_write(state, tracer, s_20_133);
        // C s_20_135: const #() : ()
        let s_20_135: () = ();
        // S s_20_136: call MVFR0_read(s_20_135)
        let s_20_136: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_135);
        // C s_20_137: const #1u : u8
        let s_20_137: u8 = 1;
        // S s_20_138: call _update_MVFR0_Type_FPSqrt(s_20_136, s_20_137)
        let s_20_138: ProductType700c18a878c5601b = u_update_MVFR0_Type_FPSqrt(
            state,
            tracer,
            s_20_136,
            s_20_137,
        );
        // S s_20_139: call MVFR0_write(s_20_138)
        let s_20_139: () = MVFR0_write(state, tracer, s_20_138);
        // C s_20_140: const #() : ()
        let s_20_140: () = ();
        // S s_20_141: call MVFR0_read(s_20_140)
        let s_20_141: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_140);
        // C s_20_142: const #1u : u8
        let s_20_142: u8 = 1;
        // S s_20_143: call _update_MVFR0_Type_FPDivide(s_20_141, s_20_142)
        let s_20_143: ProductType700c18a878c5601b = u_update_MVFR0_Type_FPDivide(
            state,
            tracer,
            s_20_141,
            s_20_142,
        );
        // S s_20_144: call MVFR0_write(s_20_143)
        let s_20_144: () = MVFR0_write(state, tracer, s_20_143);
        // C s_20_145: const #() : ()
        let s_20_145: () = ();
        // S s_20_146: call MVFR0_read(s_20_145)
        let s_20_146: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_145);
        // C s_20_147: const #1u : u8
        let s_20_147: u8 = 1;
        // S s_20_148: call _update_MVFR0_Type_FPTrap(s_20_146, s_20_147)
        let s_20_148: ProductType700c18a878c5601b = u_update_MVFR0_Type_FPTrap(
            state,
            tracer,
            s_20_146,
            s_20_147,
        );
        // S s_20_149: call MVFR0_write(s_20_148)
        let s_20_149: () = MVFR0_write(state, tracer, s_20_148);
        // C s_20_150: const #() : ()
        let s_20_150: () = ();
        // S s_20_151: call MVFR0_read(s_20_150)
        let s_20_151: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_150);
        // C s_20_152: const #2u : u8
        let s_20_152: u8 = 2;
        // S s_20_153: call _update_MVFR0_Type_FPDP(s_20_151, s_20_152)
        let s_20_153: ProductType700c18a878c5601b = u_update_MVFR0_Type_FPDP(
            state,
            tracer,
            s_20_151,
            s_20_152,
        );
        // S s_20_154: call MVFR0_write(s_20_153)
        let s_20_154: () = MVFR0_write(state, tracer, s_20_153);
        // C s_20_155: const #() : ()
        let s_20_155: () = ();
        // S s_20_156: call MVFR0_read(s_20_155)
        let s_20_156: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_155);
        // C s_20_157: const #2u : u8
        let s_20_157: u8 = 2;
        // S s_20_158: call _update_MVFR0_Type_FPSP(s_20_156, s_20_157)
        let s_20_158: ProductType700c18a878c5601b = u_update_MVFR0_Type_FPSP(
            state,
            tracer,
            s_20_156,
            s_20_157,
        );
        // S s_20_159: call MVFR0_write(s_20_158)
        let s_20_159: () = MVFR0_write(state, tracer, s_20_158);
        // C s_20_160: const #() : ()
        let s_20_160: () = ();
        // S s_20_161: call MVFR0_read(s_20_160)
        let s_20_161: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_20_160);
        // C s_20_162: const #2u : u8
        let s_20_162: u8 = 2;
        // S s_20_163: call _update_MVFR0_Type_SIMDReg(s_20_161, s_20_162)
        let s_20_163: ProductType700c18a878c5601b = u_update_MVFR0_Type_SIMDReg(
            state,
            tracer,
            s_20_161,
            s_20_162,
        );
        // S s_20_164: call MVFR0_write(s_20_163)
        let s_20_164: () = MVFR0_write(state, tracer, s_20_163);
        // C s_20_165: const #() : ()
        let s_20_165: () = ();
        // S s_20_166: call MVFR1_read(s_20_165)
        let s_20_166: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_20_165);
        // C s_20_167: const #1u : u8
        let s_20_167: u8 = 1;
        // S s_20_168: call _update_MVFR1_Type_SIMDFMAC(s_20_166, s_20_167)
        let s_20_168: ProductType700c18a878c5601b = u_update_MVFR1_Type_SIMDFMAC(
            state,
            tracer,
            s_20_166,
            s_20_167,
        );
        // S s_20_169: call MVFR1_write(s_20_168)
        let s_20_169: () = MVFR1_write(state, tracer, s_20_168);
        // C s_20_170: const #() : ()
        let s_20_170: () = ();
        // S s_20_171: call MVFR1_read(s_20_170)
        let s_20_171: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_20_170);
        // C s_20_172: const #1u : u8
        let s_20_172: u8 = 1;
        // S s_20_173: call _update_MVFR1_Type_SIMDSP(s_20_171, s_20_172)
        let s_20_173: ProductType700c18a878c5601b = u_update_MVFR1_Type_SIMDSP(
            state,
            tracer,
            s_20_171,
            s_20_172,
        );
        // S s_20_174: call MVFR1_write(s_20_173)
        let s_20_174: () = MVFR1_write(state, tracer, s_20_173);
        // C s_20_175: const #() : ()
        let s_20_175: () = ();
        // S s_20_176: call MVFR1_read(s_20_175)
        let s_20_176: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_20_175);
        // C s_20_177: const #1u : u8
        let s_20_177: u8 = 1;
        // S s_20_178: call _update_MVFR1_Type_SIMDInt(s_20_176, s_20_177)
        let s_20_178: ProductType700c18a878c5601b = u_update_MVFR1_Type_SIMDInt(
            state,
            tracer,
            s_20_176,
            s_20_177,
        );
        // S s_20_179: call MVFR1_write(s_20_178)
        let s_20_179: () = MVFR1_write(state, tracer, s_20_178);
        // C s_20_180: const #() : ()
        let s_20_180: () = ();
        // S s_20_181: call MVFR1_read(s_20_180)
        let s_20_181: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_20_180);
        // C s_20_182: const #1u : u8
        let s_20_182: u8 = 1;
        // S s_20_183: call _update_MVFR1_Type_SIMDLS(s_20_181, s_20_182)
        let s_20_183: ProductType700c18a878c5601b = u_update_MVFR1_Type_SIMDLS(
            state,
            tracer,
            s_20_181,
            s_20_182,
        );
        // S s_20_184: call MVFR1_write(s_20_183)
        let s_20_184: () = MVFR1_write(state, tracer, s_20_183);
        // C s_20_185: const #() : ()
        let s_20_185: () = ();
        // S s_20_186: call MVFR1_read(s_20_185)
        let s_20_186: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_20_185);
        // C s_20_187: const #1u : u8
        let s_20_187: u8 = 1;
        // S s_20_188: call _update_MVFR1_Type_FPDNaN(s_20_186, s_20_187)
        let s_20_188: ProductType700c18a878c5601b = u_update_MVFR1_Type_FPDNaN(
            state,
            tracer,
            s_20_186,
            s_20_187,
        );
        // S s_20_189: call MVFR1_write(s_20_188)
        let s_20_189: () = MVFR1_write(state, tracer, s_20_188);
        // C s_20_190: const #() : ()
        let s_20_190: () = ();
        // S s_20_191: call MVFR1_read(s_20_190)
        let s_20_191: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_20_190);
        // C s_20_192: const #1u : u8
        let s_20_192: u8 = 1;
        // S s_20_193: call _update_MVFR1_Type_FPFtZ(s_20_191, s_20_192)
        let s_20_193: ProductType700c18a878c5601b = u_update_MVFR1_Type_FPFtZ(
            state,
            tracer,
            s_20_191,
            s_20_192,
        );
        // S s_20_194: call MVFR1_write(s_20_193)
        let s_20_194: () = MVFR1_write(state, tracer, s_20_193);
        // C s_20_195: const #() : ()
        let s_20_195: () = ();
        // S s_20_196: call MVFR2_read(s_20_195)
        let s_20_196: ProductType700c18a878c5601b = MVFR2_read(state, tracer, s_20_195);
        // C s_20_197: const #4u : u8
        let s_20_197: u8 = 4;
        // S s_20_198: call _update_MVFR2_Type_FPMisc(s_20_196, s_20_197)
        let s_20_198: ProductType700c18a878c5601b = u_update_MVFR2_Type_FPMisc(
            state,
            tracer,
            s_20_196,
            s_20_197,
        );
        // S s_20_199: call MVFR2_write(s_20_198)
        let s_20_199: () = MVFR2_write(state, tracer, s_20_198);
        // C s_20_200: const #() : ()
        let s_20_200: () = ();
        // S s_20_201: call MVFR2_read(s_20_200)
        let s_20_201: ProductType700c18a878c5601b = MVFR2_read(state, tracer, s_20_200);
        // C s_20_202: const #3u : u8
        let s_20_202: u8 = 3;
        // S s_20_203: call _update_MVFR2_Type_SIMDMisc(s_20_201, s_20_202)
        let s_20_203: ProductType700c18a878c5601b = u_update_MVFR2_Type_SIMDMisc(
            state,
            tracer,
            s_20_201,
            s_20_202,
        );
        // S s_20_204: call MVFR2_write(s_20_203)
        let s_20_204: () = MVFR2_write(state, tracer, s_20_203);
        // C s_20_205: const #101176u : u32
        let s_20_205: u32 = 101176;
        // D s_20_206: read-reg s_20_205:struct
        let s_20_206: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_205 as isize);
            tracer.read_register(s_20_205 as isize, value);
            value
        };
        // C s_20_207: const #101176u : u32
        let s_20_207: u32 = 101176;
        // N s_20_208: write-reg s_20_207 <= s_20_206
        let s_20_208: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_20_207 as isize, s_20_206);
            tracer.write_register(s_20_207 as isize, s_20_206);
        };
        // C s_20_209: const #() : ()
        let s_20_209: () = ();
        // S s_20_210: call HSCTLR_read(s_20_209)
        let s_20_210: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_20_209);
        // C s_20_211: const #0u : u8
        let s_20_211: bool = false;
        // S s_20_212: call _update_HSCTLR_Type_EE(s_20_210, s_20_211)
        let s_20_212: ProductType700c18a878c5601b = u_update_HSCTLR_Type_EE(
            state,
            tracer,
            s_20_210,
            s_20_211,
        );
        // S s_20_213: call HSCTLR_write(s_20_212)
        let s_20_213: () = HSCTLR_write(state, tracer, s_20_212);
        // C s_20_214: const #() : ()
        let s_20_214: () = ();
        // S s_20_215: call SCTLR_read__2(s_20_214)
        let s_20_215: ProductType700c18a878c5601b = SCTLR_read__2(
            state,
            tracer,
            s_20_214,
        );
        // C s_20_216: const #102816u : u32
        let s_20_216: u32 = 102816;
        // D s_20_217: read-reg s_20_216:struct
        let s_20_217: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_216 as isize);
            tracer.read_register(s_20_216 as isize, value);
            value
        };
        // D s_20_218: call _get_Configuration_Type_ExceptInit(s_20_217)
        let s_20_218: bool = u_get_Configuration_Type_ExceptInit(
            state,
            tracer,
            s_20_217,
        );
        // D s_20_219: call _update_SCTLR_Type_TE(s_20_215, s_20_218)
        let s_20_219: ProductType700c18a878c5601b = u_update_SCTLR_Type_TE(
            state,
            tracer,
            s_20_215,
            s_20_218,
        );
        // D s_20_220: call SCTLR_write(s_20_219)
        let s_20_220: () = SCTLR_write(state, tracer, s_20_219);
        // C s_20_221: const #() : ()
        let s_20_221: () = ();
        // S s_20_222: call SCTLR_read__2(s_20_221)
        let s_20_222: ProductType700c18a878c5601b = SCTLR_read__2(
            state,
            tracer,
            s_20_221,
        );
        // C s_20_223: const #102816u : u32
        let s_20_223: u32 = 102816;
        // D s_20_224: read-reg s_20_223:struct
        let s_20_224: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_223 as isize);
            tracer.read_register(s_20_223 as isize, value);
            value
        };
        // D s_20_225: call _get_Configuration_Type_CFGEND(s_20_224)
        let s_20_225: bool = u_get_Configuration_Type_CFGEND(state, tracer, s_20_224);
        // D s_20_226: call _update_SCTLR_Type_EE(s_20_222, s_20_225)
        let s_20_226: ProductType700c18a878c5601b = u_update_SCTLR_Type_EE(
            state,
            tracer,
            s_20_222,
            s_20_225,
        );
        // D s_20_227: call SCTLR_write(s_20_226)
        let s_20_227: () = SCTLR_write(state, tracer, s_20_226);
        // C s_20_228: const #() : ()
        let s_20_228: () = ();
        // S s_20_229: call SCTLR_read__2(s_20_228)
        let s_20_229: ProductType700c18a878c5601b = SCTLR_read__2(
            state,
            tracer,
            s_20_228,
        );
        // C s_20_230: const #0u : u8
        let s_20_230: bool = false;
        // S s_20_231: call _update_SCTLR_Type_SPAN(s_20_229, s_20_230)
        let s_20_231: ProductType700c18a878c5601b = u_update_SCTLR_Type_SPAN(
            state,
            tracer,
            s_20_229,
            s_20_230,
        );
        // S s_20_232: call SCTLR_write(s_20_231)
        let s_20_232: () = SCTLR_write(state, tracer, s_20_231);
        // C s_20_233: const #() : ()
        let s_20_233: () = ();
        // S s_20_234: call SCTLR_read__2(s_20_233)
        let s_20_234: ProductType700c18a878c5601b = SCTLR_read__2(
            state,
            tracer,
            s_20_233,
        );
        // C s_20_235: const #0u : u8
        let s_20_235: bool = false;
        // S s_20_236: call _update_SCTLR_Type_V(s_20_234, s_20_235)
        let s_20_236: ProductType700c18a878c5601b = u_update_SCTLR_Type_V(
            state,
            tracer,
            s_20_234,
            s_20_235,
        );
        // S s_20_237: call SCTLR_write(s_20_236)
        let s_20_237: () = SCTLR_write(state, tracer, s_20_236);
        // C s_20_238: const #() : ()
        let s_20_238: () = ();
        // S s_20_239: call SCTLR_read__2(s_20_238)
        let s_20_239: ProductType700c18a878c5601b = SCTLR_read__2(
            state,
            tracer,
            s_20_238,
        );
        // C s_20_240: const #0u : u8
        let s_20_240: bool = false;
        // S s_20_241: call _update_SCTLR_Type_UNK(s_20_239, s_20_240)
        let s_20_241: ProductType700c18a878c5601b = u_update_SCTLR_Type_UNK(
            state,
            tracer,
            s_20_239,
            s_20_240,
        );
        // S s_20_242: call SCTLR_write(s_20_241)
        let s_20_242: () = SCTLR_write(state, tracer, s_20_241);
        // C s_20_243: const #() : ()
        let s_20_243: () = ();
        // S s_20_244: call FPEXC_read(s_20_243)
        let s_20_244: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_20_243);
        // C s_20_245: const #0u : u8
        let s_20_245: bool = false;
        // S s_20_246: call _update_FPEXC_Type_EX(s_20_244, s_20_245)
        let s_20_246: ProductType700c18a878c5601b = u_update_FPEXC_Type_EX(
            state,
            tracer,
            s_20_244,
            s_20_245,
        );
        // S s_20_247: call FPEXC_write(s_20_246)
        let s_20_247: () = FPEXC_write(state, tracer, s_20_246);
        // C s_20_248: const #() : ()
        let s_20_248: () = ();
        // S s_20_249: call FPEXC_read(s_20_248)
        let s_20_249: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_20_248);
        // C s_20_250: const #0u : u8
        let s_20_250: bool = false;
        // S s_20_251: call _update_FPEXC_Type_FP2V(s_20_249, s_20_250)
        let s_20_251: ProductType700c18a878c5601b = u_update_FPEXC_Type_FP2V(
            state,
            tracer,
            s_20_249,
            s_20_250,
        );
        // S s_20_252: call FPEXC_write(s_20_251)
        let s_20_252: () = FPEXC_write(state, tracer, s_20_251);
        // C s_20_253: const #() : ()
        let s_20_253: () = ();
        // S s_20_254: call FPEXC_read(s_20_253)
        let s_20_254: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_20_253);
        // C s_20_255: const #0u : u8
        let s_20_255: bool = false;
        // S s_20_256: call _update_FPEXC_Type_VV(s_20_254, s_20_255)
        let s_20_256: ProductType700c18a878c5601b = u_update_FPEXC_Type_VV(
            state,
            tracer,
            s_20_254,
            s_20_255,
        );
        // S s_20_257: call FPEXC_write(s_20_256)
        let s_20_257: () = FPEXC_write(state, tracer, s_20_256);
        // C s_20_258: const #() : ()
        let s_20_258: () = ();
        // S s_20_259: call FPEXC_read(s_20_258)
        let s_20_259: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_20_258);
        // C s_20_260: const #0u : u8
        let s_20_260: bool = false;
        // S s_20_261: call _update_FPEXC_Type_TFV(s_20_259, s_20_260)
        let s_20_261: ProductType700c18a878c5601b = u_update_FPEXC_Type_TFV(
            state,
            tracer,
            s_20_259,
            s_20_260,
        );
        // S s_20_262: call FPEXC_write(s_20_261)
        let s_20_262: () = FPEXC_write(state, tracer, s_20_261);
        // C s_20_263: const #() : ()
        let s_20_263: () = ();
        // S s_20_264: call FPEXC_read(s_20_263)
        let s_20_264: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_20_263);
        // C s_20_265: const #7u : u8
        let s_20_265: u8 = 7;
        // S s_20_266: call _update_FPEXC_Type_VECITR(s_20_264, s_20_265)
        let s_20_266: ProductType700c18a878c5601b = u_update_FPEXC_Type_VECITR(
            state,
            tracer,
            s_20_264,
            s_20_265,
        );
        // S s_20_267: call FPEXC_write(s_20_266)
        let s_20_267: () = FPEXC_write(state, tracer, s_20_266);
        // C s_20_268: const #() : ()
        let s_20_268: () = ();
        // S s_20_269: call HTCR_read(s_20_268)
        let s_20_269: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_268);
        // C s_20_270: const #0u : u8
        let s_20_270: bool = false;
        // S s_20_271: call _update_HTCR_Type_HWU62(s_20_269, s_20_270)
        let s_20_271: ProductType700c18a878c5601b = u_update_HTCR_Type_HWU62(
            state,
            tracer,
            s_20_269,
            s_20_270,
        );
        // S s_20_272: call HTCR_write(s_20_271)
        let s_20_272: () = HTCR_write(state, tracer, s_20_271);
        // C s_20_273: const #() : ()
        let s_20_273: () = ();
        // S s_20_274: call HTCR_read(s_20_273)
        let s_20_274: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_273);
        // C s_20_275: const #0u : u8
        let s_20_275: bool = false;
        // S s_20_276: call _update_HTCR_Type_HWU61(s_20_274, s_20_275)
        let s_20_276: ProductType700c18a878c5601b = u_update_HTCR_Type_HWU61(
            state,
            tracer,
            s_20_274,
            s_20_275,
        );
        // S s_20_277: call HTCR_write(s_20_276)
        let s_20_277: () = HTCR_write(state, tracer, s_20_276);
        // C s_20_278: const #() : ()
        let s_20_278: () = ();
        // S s_20_279: call HTCR_read(s_20_278)
        let s_20_279: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_278);
        // C s_20_280: const #0u : u8
        let s_20_280: bool = false;
        // S s_20_281: call _update_HTCR_Type_HWU60(s_20_279, s_20_280)
        let s_20_281: ProductType700c18a878c5601b = u_update_HTCR_Type_HWU60(
            state,
            tracer,
            s_20_279,
            s_20_280,
        );
        // S s_20_282: call HTCR_write(s_20_281)
        let s_20_282: () = HTCR_write(state, tracer, s_20_281);
        // C s_20_283: const #() : ()
        let s_20_283: () = ();
        // S s_20_284: call HTCR_read(s_20_283)
        let s_20_284: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_283);
        // C s_20_285: const #0u : u8
        let s_20_285: bool = false;
        // S s_20_286: call _update_HTCR_Type_HWU59(s_20_284, s_20_285)
        let s_20_286: ProductType700c18a878c5601b = u_update_HTCR_Type_HWU59(
            state,
            tracer,
            s_20_284,
            s_20_285,
        );
        // S s_20_287: call HTCR_write(s_20_286)
        let s_20_287: () = HTCR_write(state, tracer, s_20_286);
        // C s_20_288: const #() : ()
        let s_20_288: () = ();
        // S s_20_289: call HTCR_read(s_20_288)
        let s_20_289: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_288);
        // C s_20_290: const #0u : u8
        let s_20_290: bool = false;
        // S s_20_291: call _update_HTCR_Type_HPD(s_20_289, s_20_290)
        let s_20_291: ProductType700c18a878c5601b = u_update_HTCR_Type_HPD(
            state,
            tracer,
            s_20_289,
            s_20_290,
        );
        // S s_20_292: call HTCR_write(s_20_291)
        let s_20_292: () = HTCR_write(state, tracer, s_20_291);
        // C s_20_293: const #() : ()
        let s_20_293: () = ();
        // S s_20_294: call HTCR_read(s_20_293)
        let s_20_294: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_293);
        // C s_20_295: const #0u : u8
        let s_20_295: u8 = 0;
        // S s_20_296: call _update_HTCR_Type_SH0(s_20_294, s_20_295)
        let s_20_296: ProductType700c18a878c5601b = u_update_HTCR_Type_SH0(
            state,
            tracer,
            s_20_294,
            s_20_295,
        );
        // S s_20_297: call HTCR_write(s_20_296)
        let s_20_297: () = HTCR_write(state, tracer, s_20_296);
        // C s_20_298: const #() : ()
        let s_20_298: () = ();
        // S s_20_299: call HTCR_read(s_20_298)
        let s_20_299: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_298);
        // C s_20_300: const #0u : u8
        let s_20_300: u8 = 0;
        // S s_20_301: call _update_HTCR_Type_ORGN0(s_20_299, s_20_300)
        let s_20_301: ProductType700c18a878c5601b = u_update_HTCR_Type_ORGN0(
            state,
            tracer,
            s_20_299,
            s_20_300,
        );
        // S s_20_302: call HTCR_write(s_20_301)
        let s_20_302: () = HTCR_write(state, tracer, s_20_301);
        // C s_20_303: const #() : ()
        let s_20_303: () = ();
        // S s_20_304: call HTCR_read(s_20_303)
        let s_20_304: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_303);
        // C s_20_305: const #0u : u8
        let s_20_305: u8 = 0;
        // S s_20_306: call _update_HTCR_Type_IRGN0(s_20_304, s_20_305)
        let s_20_306: ProductType700c18a878c5601b = u_update_HTCR_Type_IRGN0(
            state,
            tracer,
            s_20_304,
            s_20_305,
        );
        // S s_20_307: call HTCR_write(s_20_306)
        let s_20_307: () = HTCR_write(state, tracer, s_20_306);
        // C s_20_308: const #() : ()
        let s_20_308: () = ();
        // S s_20_309: call HTCR_read(s_20_308)
        let s_20_309: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_20_308);
        // C s_20_310: const #0u : u8
        let s_20_310: u8 = 0;
        // S s_20_311: call _update_HTCR_Type_T0SZ(s_20_309, s_20_310)
        let s_20_311: ProductType700c18a878c5601b = u_update_HTCR_Type_T0SZ(
            state,
            tracer,
            s_20_309,
            s_20_310,
        );
        // S s_20_312: call HTCR_write(s_20_311)
        let s_20_312: () = HTCR_write(state, tracer, s_20_311);
        // C s_20_313: const #() : ()
        let s_20_313: () = ();
        // S s_20_314: call NMRR_read(s_20_313)
        let s_20_314: ProductType700c18a878c5601b = NMRR_read(state, tracer, s_20_313);
        // D s_20_315: write-var ga#368418 <= s_20_314
        fn_state.ga_368418 = s_20_314;
        // D s_20_316: read-var ga#368418.0:struct
        let s_20_316: u32 = fn_state.ga_368418._0;
        // C s_20_317: const #16s : i
        let s_20_317: i128 = 16;
        // D s_20_318: cast zx s_20_316 -> bv
        let s_20_318: Bits = Bits::new(s_20_316 as u128, 32u16);
        // C s_20_319: const #17632u : u16
        let s_20_319: u16 = 17632;
        // C s_20_320: cast zx s_20_319 -> bv
        let s_20_320: Bits = Bits::new(s_20_319 as u128, 16u16);
        // C s_20_321: const #15s : i
        let s_20_321: i128 = 15;
        // C s_20_322: const #1u : u64
        let s_20_322: u64 = 1;
        // C s_20_323: cast zx s_20_322 -> bv
        let s_20_323: Bits = Bits::new(s_20_322 as u128, 64u16);
        // C s_20_324: lsl s_20_323 s_20_321
        let s_20_324: Bits = s_20_323 << s_20_321;
        // C s_20_325: sub s_20_324 s_20_323
        let s_20_325: Bits = ((s_20_324) - (s_20_323));
        // C s_20_326: and s_20_320 s_20_325
        let s_20_326: Bits = ((s_20_320) & (s_20_325));
        // C s_20_327: lsl s_20_326 s_20_317
        let s_20_327: Bits = s_20_326 << s_20_317;
        // C s_20_328: lsl s_20_325 s_20_317
        let s_20_328: Bits = s_20_325 << s_20_317;
        // C s_20_329: cmpl s_20_328
        let s_20_329: Bits = !s_20_328;
        // D s_20_330: and s_20_318 s_20_329
        let s_20_330: Bits = ((s_20_318) & (s_20_329));
        // D s_20_331: or s_20_330 s_20_327
        let s_20_331: Bits = ((s_20_330) | (s_20_327));
        // D s_20_332: cast reint s_20_331 -> u32
        let s_20_332: u32 = (s_20_331.value() as u32);
        // D s_20_333: call Mk_NMRR_Type(s_20_332)
        let s_20_333: ProductType700c18a878c5601b = Mk_NMRR_Type(
            state,
            tracer,
            s_20_332,
        );
        // D s_20_334: call NMRR_write(s_20_333)
        let s_20_334: () = NMRR_write(state, tracer, s_20_333);
        // C s_20_335: const #() : ()
        let s_20_335: () = ();
        // S s_20_336: call NMRR_read(s_20_335)
        let s_20_336: ProductType700c18a878c5601b = NMRR_read(state, tracer, s_20_335);
        // D s_20_337: write-var ga#368422 <= s_20_336
        fn_state.ga_368422 = s_20_336;
        // D s_20_338: read-var ga#368422.0:struct
        let s_20_338: u32 = fn_state.ga_368422._0;
        // C s_20_339: const #0s : i
        let s_20_339: i128 = 0;
        // D s_20_340: cast zx s_20_338 -> bv
        let s_20_340: Bits = Bits::new(s_20_338 as u128, 32u16);
        // C s_20_341: const #18656u : u16
        let s_20_341: u16 = 18656;
        // C s_20_342: cast zx s_20_341 -> bv
        let s_20_342: Bits = Bits::new(s_20_341 as u128, 16u16);
        // C s_20_343: const #15s : i
        let s_20_343: i128 = 15;
        // C s_20_344: const #1u : u64
        let s_20_344: u64 = 1;
        // C s_20_345: cast zx s_20_344 -> bv
        let s_20_345: Bits = Bits::new(s_20_344 as u128, 64u16);
        // C s_20_346: lsl s_20_345 s_20_343
        let s_20_346: Bits = s_20_345 << s_20_343;
        // C s_20_347: sub s_20_346 s_20_345
        let s_20_347: Bits = ((s_20_346) - (s_20_345));
        // C s_20_348: and s_20_342 s_20_347
        let s_20_348: Bits = ((s_20_342) & (s_20_347));
        // C s_20_349: lsl s_20_348 s_20_339
        let s_20_349: Bits = s_20_348 << s_20_339;
        // C s_20_350: lsl s_20_347 s_20_339
        let s_20_350: Bits = s_20_347 << s_20_339;
        // C s_20_351: cmpl s_20_350
        let s_20_351: Bits = !s_20_350;
        // D s_20_352: and s_20_340 s_20_351
        let s_20_352: Bits = ((s_20_340) & (s_20_351));
        // D s_20_353: or s_20_352 s_20_349
        let s_20_353: Bits = ((s_20_352) | (s_20_349));
        // D s_20_354: cast reint s_20_353 -> u32
        let s_20_354: u32 = (s_20_353.value() as u32);
        // D s_20_355: call Mk_NMRR_Type(s_20_354)
        let s_20_355: ProductType700c18a878c5601b = Mk_NMRR_Type(
            state,
            tracer,
            s_20_354,
        );
        // D s_20_356: call NMRR_write(s_20_355)
        let s_20_356: () = NMRR_write(state, tracer, s_20_355);
        // C s_20_357: const #102488u : u32
        let s_20_357: u32 = 102488;
        // D s_20_358: read-reg s_20_357:struct
        let s_20_358: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_357 as isize);
            tracer.read_register(s_20_357 as isize, value);
            value
        };
        // C s_20_359: const #102488u : u32
        let s_20_359: u32 = 102488;
        // N s_20_360: write-reg s_20_359 <= s_20_358
        let s_20_360: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_20_359 as isize, s_20_358);
            tracer.write_register(s_20_359 as isize, s_20_358);
        };
        // C s_20_361: const #102488u : u32
        let s_20_361: u32 = 102488;
        // D s_20_362: read-reg s_20_361:struct
        let s_20_362: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_361 as isize);
            tracer.read_register(s_20_361 as isize, value);
            value
        };
        // C s_20_363: const #102488u : u32
        let s_20_363: u32 = 102488;
        // N s_20_364: write-reg s_20_363 <= s_20_362
        let s_20_364: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_20_363 as isize, s_20_362);
            tracer.write_register(s_20_363 as isize, s_20_362);
        };
        // C s_20_365: const #() : ()
        let s_20_365: () = ();
        // S s_20_366: call PRRR_read(s_20_365)
        let s_20_366: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_20_365);
        // D s_20_367: write-var ga#368430 <= s_20_366
        fn_state.ga_368430 = s_20_366;
        // D s_20_368: read-var ga#368430.0:struct
        let s_20_368: u32 = fn_state.ga_368430._0;
        // C s_20_369: const #8s : i
        let s_20_369: i128 = 8;
        // S s_20_370: call Zeros(s_20_369)
        let s_20_370: Bits = Zeros(state, tracer, s_20_369);
        // S s_20_371: cast reint s_20_370 -> u8
        let s_20_371: u8 = (s_20_370.value() as u8);
        // C s_20_372: const #24s : i
        let s_20_372: i128 = 24;
        // D s_20_373: cast zx s_20_368 -> bv
        let s_20_373: Bits = Bits::new(s_20_368 as u128, 32u16);
        // S s_20_374: cast zx s_20_371 -> bv
        let s_20_374: Bits = Bits::new(s_20_371 as u128, 8u16);
        // C s_20_375: const #7s : i
        let s_20_375: i128 = 7;
        // C s_20_376: const #1u : u64
        let s_20_376: u64 = 1;
        // C s_20_377: cast zx s_20_376 -> bv
        let s_20_377: Bits = Bits::new(s_20_376 as u128, 64u16);
        // C s_20_378: lsl s_20_377 s_20_375
        let s_20_378: Bits = s_20_377 << s_20_375;
        // C s_20_379: sub s_20_378 s_20_377
        let s_20_379: Bits = ((s_20_378) - (s_20_377));
        // S s_20_380: and s_20_374 s_20_379
        let s_20_380: Bits = ((s_20_374) & (s_20_379));
        // S s_20_381: lsl s_20_380 s_20_372
        let s_20_381: Bits = s_20_380 << s_20_372;
        // C s_20_382: lsl s_20_379 s_20_372
        let s_20_382: Bits = s_20_379 << s_20_372;
        // C s_20_383: cmpl s_20_382
        let s_20_383: Bits = !s_20_382;
        // D s_20_384: and s_20_373 s_20_383
        let s_20_384: Bits = ((s_20_373) & (s_20_383));
        // D s_20_385: or s_20_384 s_20_381
        let s_20_385: Bits = ((s_20_384) | (s_20_381));
        // D s_20_386: cast reint s_20_385 -> u32
        let s_20_386: u32 = (s_20_385.value() as u32);
        // D s_20_387: call Mk_PRRR_Type(s_20_386)
        let s_20_387: ProductType700c18a878c5601b = Mk_PRRR_Type(
            state,
            tracer,
            s_20_386,
        );
        // D s_20_388: call PRRR_write(s_20_387)
        let s_20_388: () = PRRR_write(state, tracer, s_20_387);
        // C s_20_389: const #() : ()
        let s_20_389: () = ();
        // S s_20_390: call PRRR_read(s_20_389)
        let s_20_390: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_20_389);
        // C s_20_391: const #1u : u8
        let s_20_391: bool = true;
        // S s_20_392: call _update_PRRR_Type_NS1(s_20_390, s_20_391)
        let s_20_392: ProductType700c18a878c5601b = u_update_PRRR_Type_NS1(
            state,
            tracer,
            s_20_390,
            s_20_391,
        );
        // S s_20_393: call PRRR_write(s_20_392)
        let s_20_393: () = PRRR_write(state, tracer, s_20_392);
        // C s_20_394: const #() : ()
        let s_20_394: () = ();
        // S s_20_395: call PRRR_read(s_20_394)
        let s_20_395: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_20_394);
        // C s_20_396: const #0u : u8
        let s_20_396: bool = false;
        // S s_20_397: call _update_PRRR_Type_NS0(s_20_395, s_20_396)
        let s_20_397: ProductType700c18a878c5601b = u_update_PRRR_Type_NS0(
            state,
            tracer,
            s_20_395,
            s_20_396,
        );
        // S s_20_398: call PRRR_write(s_20_397)
        let s_20_398: () = PRRR_write(state, tracer, s_20_397);
        // C s_20_399: const #() : ()
        let s_20_399: () = ();
        // S s_20_400: call PRRR_read(s_20_399)
        let s_20_400: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_20_399);
        // C s_20_401: const #0u : u8
        let s_20_401: bool = false;
        // S s_20_402: call _update_PRRR_Type_DS1(s_20_400, s_20_401)
        let s_20_402: ProductType700c18a878c5601b = u_update_PRRR_Type_DS1(
            state,
            tracer,
            s_20_400,
            s_20_401,
        );
        // S s_20_403: call PRRR_write(s_20_402)
        let s_20_403: () = PRRR_write(state, tracer, s_20_402);
        // C s_20_404: const #() : ()
        let s_20_404: () = ();
        // S s_20_405: call PRRR_read(s_20_404)
        let s_20_405: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_20_404);
        // C s_20_406: const #1u : u8
        let s_20_406: bool = true;
        // S s_20_407: call _update_PRRR_Type_DS0(s_20_405, s_20_406)
        let s_20_407: ProductType700c18a878c5601b = u_update_PRRR_Type_DS0(
            state,
            tracer,
            s_20_405,
            s_20_406,
        );
        // S s_20_408: call PRRR_write(s_20_407)
        let s_20_408: () = PRRR_write(state, tracer, s_20_407);
        // C s_20_409: const #() : ()
        let s_20_409: () = ();
        // S s_20_410: call PRRR_read(s_20_409)
        let s_20_410: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_20_409);
        // D s_20_411: write-var ga#368443 <= s_20_410
        fn_state.ga_368443 = s_20_410;
        // D s_20_412: read-var ga#368443.0:struct
        let s_20_412: u32 = fn_state.ga_368443._0;
        // C s_20_413: const #0s : i
        let s_20_413: i128 = 0;
        // D s_20_414: cast zx s_20_412 -> bv
        let s_20_414: Bits = Bits::new(s_20_412 as u128, 32u16);
        // C s_20_415: const #35492u : u16
        let s_20_415: u16 = 35492;
        // C s_20_416: cast zx s_20_415 -> bv
        let s_20_416: Bits = Bits::new(s_20_415 as u128, 16u16);
        // C s_20_417: const #15s : i
        let s_20_417: i128 = 15;
        // C s_20_418: const #1u : u64
        let s_20_418: u64 = 1;
        // C s_20_419: cast zx s_20_418 -> bv
        let s_20_419: Bits = Bits::new(s_20_418 as u128, 64u16);
        // C s_20_420: lsl s_20_419 s_20_417
        let s_20_420: Bits = s_20_419 << s_20_417;
        // C s_20_421: sub s_20_420 s_20_419
        let s_20_421: Bits = ((s_20_420) - (s_20_419));
        // C s_20_422: and s_20_416 s_20_421
        let s_20_422: Bits = ((s_20_416) & (s_20_421));
        // C s_20_423: lsl s_20_422 s_20_413
        let s_20_423: Bits = s_20_422 << s_20_413;
        // C s_20_424: lsl s_20_421 s_20_413
        let s_20_424: Bits = s_20_421 << s_20_413;
        // C s_20_425: cmpl s_20_424
        let s_20_425: Bits = !s_20_424;
        // D s_20_426: and s_20_414 s_20_425
        let s_20_426: Bits = ((s_20_414) & (s_20_425));
        // D s_20_427: or s_20_426 s_20_423
        let s_20_427: Bits = ((s_20_426) | (s_20_423));
        // D s_20_428: cast reint s_20_427 -> u32
        let s_20_428: u32 = (s_20_427.value() as u32);
        // D s_20_429: call Mk_PRRR_Type(s_20_428)
        let s_20_429: ProductType700c18a878c5601b = Mk_PRRR_Type(
            state,
            tracer,
            s_20_428,
        );
        // D s_20_430: call PRRR_write(s_20_429)
        let s_20_430: () = PRRR_write(state, tracer, s_20_429);
        // C s_20_431: const #1s : i
        let s_20_431: i128 = 1;
        // C s_20_432: const #19360u : u32
        let s_20_432: u32 = 19360;
        // D s_20_433: read-reg s_20_432:i
        let s_20_433: i128 = {
            let value = state.read_register::<i128>(s_20_432 as isize);
            tracer.read_register(s_20_432 as isize, value);
            value
        };
        // D s_20_434: sub s_20_433 s_20_431
        let s_20_434: i128 = ((s_20_433) - (s_20_431));
        // C s_20_435: const #3s : i
        let s_20_435: i128 = 3;
        // C s_20_436: const #0s : i
        let s_20_436: i128 = 0;
        // D s_20_437: call integer_subrange(s_20_434, s_20_435, s_20_436)
        let s_20_437: Bits = integer_subrange(
            state,
            tracer,
            s_20_434,
            s_20_435,
            s_20_436,
        );
        // C s_20_438: const #16816u : u32
        let s_20_438: u32 = 16816;
        // D s_20_439: read-reg s_20_438:struct
        let s_20_439: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_438 as isize);
            tracer.read_register(s_20_438 as isize, value);
            value
        };
        // C s_20_440: const #16816u : u32
        let s_20_440: u32 = 16816;
        // N s_20_441: write-reg s_20_440 <= s_20_439
        let s_20_441: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_20_440 as isize, s_20_439);
            tracer.write_register(s_20_440 as isize, s_20_439);
        };
        // C s_20_442: const #1s : i
        let s_20_442: i128 = 1;
        // C s_20_443: const #22776u : u32
        let s_20_443: u32 = 22776;
        // D s_20_444: read-reg s_20_443:i
        let s_20_444: i128 = {
            let value = state.read_register::<i128>(s_20_443 as isize);
            tracer.read_register(s_20_443 as isize, value);
            value
        };
        // D s_20_445: sub s_20_444 s_20_442
        let s_20_445: i128 = ((s_20_444) - (s_20_442));
        // C s_20_446: const #3s : i
        let s_20_446: i128 = 3;
        // C s_20_447: const #0s : i
        let s_20_447: i128 = 0;
        // D s_20_448: call integer_subrange(s_20_445, s_20_446, s_20_447)
        let s_20_448: Bits = integer_subrange(
            state,
            tracer,
            s_20_445,
            s_20_446,
            s_20_447,
        );
        // C s_20_449: const #16816u : u32
        let s_20_449: u32 = 16816;
        // D s_20_450: read-reg s_20_449:struct
        let s_20_450: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_449 as isize);
            tracer.read_register(s_20_449 as isize, value);
            value
        };
        // C s_20_451: const #16816u : u32
        let s_20_451: u32 = 16816;
        // N s_20_452: write-reg s_20_451 <= s_20_450
        let s_20_452: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_20_451 as isize, s_20_450);
            tracer.write_register(s_20_451 as isize, s_20_450);
        };
        // C s_20_453: const #1s : i
        let s_20_453: i128 = 1;
        // C s_20_454: const #102792u : u32
        let s_20_454: u32 = 102792;
        // D s_20_455: read-reg s_20_454:i
        let s_20_455: i128 = {
            let value = state.read_register::<i128>(s_20_454 as isize);
            tracer.read_register(s_20_454 as isize, value);
            value
        };
        // D s_20_456: sub s_20_455 s_20_453
        let s_20_456: i128 = ((s_20_455) - (s_20_453));
        // C s_20_457: const #3s : i
        let s_20_457: i128 = 3;
        // C s_20_458: const #0s : i
        let s_20_458: i128 = 0;
        // D s_20_459: call integer_subrange(s_20_456, s_20_457, s_20_458)
        let s_20_459: Bits = integer_subrange(
            state,
            tracer,
            s_20_456,
            s_20_457,
            s_20_458,
        );
        // C s_20_460: const #16816u : u32
        let s_20_460: u32 = 16816;
        // D s_20_461: read-reg s_20_460:struct
        let s_20_461: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_460 as isize);
            tracer.read_register(s_20_460 as isize, value);
            value
        };
        // C s_20_462: const #16816u : u32
        let s_20_462: u32 = 16816;
        // N s_20_463: write-reg s_20_462 <= s_20_461
        let s_20_463: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_20_462 as isize, s_20_461);
            tracer.write_register(s_20_462 as isize, s_20_461);
        };
        // C s_20_464: const #2u : u32
        let s_20_464: u32 = 2;
        // S s_20_465: call HasArchVersion(s_20_464)
        let s_20_465: bool = HasArchVersion(state, tracer, s_20_464);
        // N s_20_466: branch s_20_465 b31 b21
        if s_20_465 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #4u : u32
        let s_22_0: u32 = 4;
        // S s_22_1: call HasArchVersion(s_22_0)
        let s_22_1: bool = HasArchVersion(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b30 b23
        if s_22_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #8u : u32
        let s_24_0: u32 = 8;
        // S s_24_1: call HasArchVersion(s_24_0)
        let s_24_1: bool = HasArchVersion(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b29 b25
        if s_24_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #16816u : u32
        let s_26_0: u32 = 16816;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // C s_26_2: const #16816u : u32
        let s_26_2: u32 = 16816;
        // N s_26_3: write-reg s_26_2 <= s_26_1
        let s_26_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_26_2 as isize, s_26_1);
            tracer.write_register(s_26_2 as isize, s_26_1);
        };
        // C s_26_4: const #16816u : u32
        let s_26_4: u32 = 16816;
        // D s_26_5: read-reg s_26_4:struct
        let s_26_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_4 as isize);
            tracer.read_register(s_26_4 as isize, value);
            value
        };
        // C s_26_6: const #16816u : u32
        let s_26_6: u32 = 16816;
        // N s_26_7: write-reg s_26_6 <= s_26_5
        let s_26_7: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_26_6 as isize, s_26_5);
            tracer.write_register(s_26_6 as isize, s_26_5);
        };
        // C s_26_8: const #14520u : u32
        let s_26_8: u32 = 14520;
        // D s_26_9: read-reg s_26_8:struct
        let s_26_9: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_8 as isize);
            tracer.read_register(s_26_8 as isize, value);
            value
        };
        // C s_26_10: const #14520u : u32
        let s_26_10: u32 = 14520;
        // N s_26_11: write-reg s_26_10 <= s_26_9
        let s_26_11: () = {
            state
                .write_register::<ProductType700c18a878c5601b>(s_26_10 as isize, s_26_9);
            tracer.write_register(s_26_10 as isize, s_26_9);
        };
        // C s_26_12: const #14520u : u32
        let s_26_12: u32 = 14520;
        // D s_26_13: read-reg s_26_12:struct
        let s_26_13: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_12 as isize);
            tracer.read_register(s_26_12 as isize, value);
            value
        };
        // C s_26_14: const #14520u : u32
        let s_26_14: u32 = 14520;
        // N s_26_15: write-reg s_26_14 <= s_26_13
        let s_26_15: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_26_14 as isize, s_26_13);
            tracer.write_register(s_26_14 as isize, s_26_13);
        };
        // C s_26_16: const #14520u : u32
        let s_26_16: u32 = 14520;
        // D s_26_17: read-reg s_26_16:struct
        let s_26_17: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_16 as isize);
            tracer.read_register(s_26_16 as isize, value);
            value
        };
        // C s_26_18: const #14520u : u32
        let s_26_18: u32 = 14520;
        // N s_26_19: write-reg s_26_18 <= s_26_17
        let s_26_19: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_26_18 as isize, s_26_17);
            tracer.write_register(s_26_18 as isize, s_26_17);
        };
        // C s_26_20: const #14520u : u32
        let s_26_20: u32 = 14520;
        // D s_26_21: read-reg s_26_20:struct
        let s_26_21: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_20 as isize);
            tracer.read_register(s_26_20 as isize, value);
            value
        };
        // C s_26_22: const #14520u : u32
        let s_26_22: u32 = 14520;
        // N s_26_23: write-reg s_26_22 <= s_26_21
        let s_26_23: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_26_22 as isize, s_26_21);
            tracer.write_register(s_26_22 as isize, s_26_21);
        };
        // C s_26_24: const #14520u : u32
        let s_26_24: u32 = 14520;
        // D s_26_25: read-reg s_26_24:struct
        let s_26_25: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_24 as isize);
            tracer.read_register(s_26_24 as isize, value);
            value
        };
        // C s_26_26: const #14520u : u32
        let s_26_26: u32 = 14520;
        // N s_26_27: write-reg s_26_26 <= s_26_25
        let s_26_27: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_26_26 as isize, s_26_25);
            tracer.write_register(s_26_26 as isize, s_26_25);
        };
        // C s_26_28: const #() : ()
        let s_26_28: () = ();
        // S s_26_29: call HDCR_read(s_26_28)
        let s_26_29: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_26_28);
        // C s_26_30: const #() : ()
        let s_26_30: () = ();
        // S s_26_31: call PMCR_read(s_26_30)
        let s_26_31: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_26_30);
        // S s_26_32: call _get_PMCR_Type_N(s_26_31)
        let s_26_32: u8 = u_get_PMCR_Type_N(state, tracer, s_26_31);
        // S s_26_33: call _update_HDCR_Type_HPMN(s_26_29, s_26_32)
        let s_26_33: ProductType700c18a878c5601b = u_update_HDCR_Type_HPMN(
            state,
            tracer,
            s_26_29,
            s_26_32,
        );
        // S s_26_34: call HDCR_write(s_26_33)
        let s_26_34: () = HDCR_write(state, tracer, s_26_33);
        // C s_26_35: const #19992u : u32
        let s_26_35: u32 = 19992;
        // D s_26_36: read-reg s_26_35:struct
        let s_26_36: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_35 as isize);
            tracer.read_register(s_26_35 as isize, value);
            value
        };
        // C s_26_37: const #19992u : u32
        let s_26_37: u32 = 19992;
        // N s_26_38: write-reg s_26_37 <= s_26_36
        let s_26_38: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_26_37 as isize, s_26_36);
            tracer.write_register(s_26_37 as isize, s_26_36);
        };
        // C s_26_39: const #() : ()
        let s_26_39: () = ();
        // S s_26_40: call ICC_HSRE_read(s_26_39)
        let s_26_40: ProductType700c18a878c5601b = ICC_HSRE_read(state, tracer, s_26_39);
        // C s_26_41: const #1u : u8
        let s_26_41: bool = true;
        // S s_26_42: call _update_ICC_HSRE_Type_SRE(s_26_40, s_26_41)
        let s_26_42: ProductType700c18a878c5601b = u_update_ICC_HSRE_Type_SRE(
            state,
            tracer,
            s_26_40,
            s_26_41,
        );
        // S s_26_43: call ICC_HSRE_write(s_26_42)
        let s_26_43: () = ICC_HSRE_write(state, tracer, s_26_42);
        // C s_26_44: const #32s : i
        let s_26_44: i128 = 32;
        // S s_26_45: call Zeros(s_26_44)
        let s_26_45: Bits = Zeros(state, tracer, s_26_44);
        // S s_26_46: cast reint s_26_45 -> u32
        let s_26_46: u32 = (s_26_45.value() as u32);
        // C s_26_47: const #91040u : u32
        let s_26_47: u32 = 91040;
        // N s_26_48: write-reg s_26_47 <= s_26_46
        let s_26_48: () = {
            state.write_register::<u32>(s_26_47 as isize, s_26_46);
            tracer.write_register(s_26_47 as isize, s_26_46);
        };
        // C s_26_49: const #32s : i
        let s_26_49: i128 = 32;
        // S s_26_50: call Zeros(s_26_49)
        let s_26_50: Bits = Zeros(state, tracer, s_26_49);
        // S s_26_51: cast reint s_26_50 -> u32
        let s_26_51: u32 = (s_26_50.value() as u32);
        // C s_26_52: const #22608u : u32
        let s_26_52: u32 = 22608;
        // N s_26_53: write-reg s_26_52 <= s_26_51
        let s_26_53: () = {
            state.write_register::<u32>(s_26_52 as isize, s_26_51);
            tracer.write_register(s_26_52 as isize, s_26_51);
        };
        // C s_26_54: const #32s : i
        let s_26_54: i128 = 32;
        // S s_26_55: call Zeros(s_26_54)
        let s_26_55: Bits = Zeros(state, tracer, s_26_54);
        // S s_26_56: cast reint s_26_55 -> u32
        let s_26_56: u32 = (s_26_55.value() as u32);
        // C s_26_57: const #101872u : u32
        let s_26_57: u32 = 101872;
        // N s_26_58: write-reg s_26_57 <= s_26_56
        let s_26_58: () = {
            state.write_register::<u32>(s_26_57 as isize, s_26_56);
            tracer.write_register(s_26_57 as isize, s_26_56);
        };
        // C s_26_59: const #() : ()
        let s_26_59: () = ();
        // S s_26_60: call DBGOSLSR_read(s_26_59)
        let s_26_60: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_26_59);
        // C s_26_61: const #2u : u8
        let s_26_61: u8 = 2;
        // S s_26_62: call _update_DBGOSLSR_Type_OSLM(s_26_60, s_26_61)
        let s_26_62: ProductType700c18a878c5601b = u_update_DBGOSLSR_Type_OSLM(
            state,
            tracer,
            s_26_60,
            s_26_61,
        );
        // S s_26_63: call DBGOSLSR_write(s_26_62)
        let s_26_63: () = DBGOSLSR_write(state, tracer, s_26_62);
        // C s_26_64: const #() : ()
        let s_26_64: () = ();
        // S s_26_65: call DBGOSLSR_read(s_26_64)
        let s_26_65: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_26_64);
        // C s_26_66: const #1u : u8
        let s_26_66: bool = true;
        // S s_26_67: call _update_DBGOSLSR_Type_OSLK(s_26_65, s_26_66)
        let s_26_67: ProductType700c18a878c5601b = u_update_DBGOSLSR_Type_OSLK(
            state,
            tracer,
            s_26_65,
            s_26_66,
        );
        // S s_26_68: call DBGOSLSR_write(s_26_67)
        let s_26_68: () = DBGOSLSR_write(state, tracer, s_26_67);
        // C s_26_69: const #() : ()
        let s_26_69: () = ();
        // S s_26_70: call HCPTR_read(s_26_69)
        let s_26_70: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_26_69);
        // C s_26_71: const #0u : u8
        let s_26_71: bool = false;
        // S s_26_72: call _update_HCPTR_Type_TCPAC(s_26_70, s_26_71)
        let s_26_72: ProductType700c18a878c5601b = u_update_HCPTR_Type_TCPAC(
            state,
            tracer,
            s_26_70,
            s_26_71,
        );
        // S s_26_73: call HCPTR_write(s_26_72)
        let s_26_73: () = HCPTR_write(state, tracer, s_26_72);
        // C s_26_74: const #() : ()
        let s_26_74: () = ();
        // S s_26_75: call HCPTR_read(s_26_74)
        let s_26_75: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_26_74);
        // C s_26_76: const #0u : u8
        let s_26_76: bool = false;
        // S s_26_77: call _update_HCPTR_Type_TTA(s_26_75, s_26_76)
        let s_26_77: ProductType700c18a878c5601b = u_update_HCPTR_Type_TTA(
            state,
            tracer,
            s_26_75,
            s_26_76,
        );
        // S s_26_78: call HCPTR_write(s_26_77)
        let s_26_78: () = HCPTR_write(state, tracer, s_26_77);
        // C s_26_79: const #() : ()
        let s_26_79: () = ();
        // S s_26_80: call HCPTR_read(s_26_79)
        let s_26_80: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_26_79);
        // C s_26_81: const #0u : u8
        let s_26_81: bool = false;
        // S s_26_82: call _update_HCPTR_Type_TASE(s_26_80, s_26_81)
        let s_26_82: ProductType700c18a878c5601b = u_update_HCPTR_Type_TASE(
            state,
            tracer,
            s_26_80,
            s_26_81,
        );
        // S s_26_83: call HCPTR_write(s_26_82)
        let s_26_83: () = HCPTR_write(state, tracer, s_26_82);
        // C s_26_84: const #() : ()
        let s_26_84: () = ();
        // S s_26_85: call HCPTR_read(s_26_84)
        let s_26_85: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_26_84);
        // C s_26_86: const #0u : u8
        let s_26_86: bool = false;
        // S s_26_87: call _update_HCPTR_Type_TCP11(s_26_85, s_26_86)
        let s_26_87: ProductType700c18a878c5601b = u_update_HCPTR_Type_TCP11(
            state,
            tracer,
            s_26_85,
            s_26_86,
        );
        // S s_26_88: call HCPTR_write(s_26_87)
        let s_26_88: () = HCPTR_write(state, tracer, s_26_87);
        // C s_26_89: const #() : ()
        let s_26_89: () = ();
        // S s_26_90: call HCPTR_read(s_26_89)
        let s_26_90: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_26_89);
        // C s_26_91: const #0u : u8
        let s_26_91: bool = false;
        // S s_26_92: call _update_HCPTR_Type_TCP10(s_26_90, s_26_91)
        let s_26_92: ProductType700c18a878c5601b = u_update_HCPTR_Type_TCP10(
            state,
            tracer,
            s_26_90,
            s_26_91,
        );
        // S s_26_93: call HCPTR_write(s_26_92)
        let s_26_93: () = HCPTR_write(state, tracer, s_26_92);
        // C s_26_94: const #14u : u32
        let s_26_94: u32 = 14;
        // S s_26_95: call HasArchVersion(s_26_94)
        let s_26_95: bool = HasArchVersion(state, tracer, s_26_94);
        // N s_26_96: branch s_26_95 b28 b27
        if s_26_95 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call ID_DFR0_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_28_0);
        // C s_28_2: const #11u : u8
        let s_28_2: u8 = 11;
        // S s_28_3: call _update_ID_DFR0_Type_CopSDbg(s_28_1, s_28_2)
        let s_28_3: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopSDbg(
            state,
            tracer,
            s_28_1,
            s_28_2,
        );
        // S s_28_4: call ID_DFR0_write(s_28_3)
        let s_28_4: () = ID_DFR0_write(state, tracer, s_28_3);
        // C s_28_5: const #() : ()
        let s_28_5: () = ();
        // S s_28_6: call ID_DFR0_read(s_28_5)
        let s_28_6: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_28_5);
        // C s_28_7: const #11u : u8
        let s_28_7: u8 = 11;
        // S s_28_8: call _update_ID_DFR0_Type_CopDbg(s_28_6, s_28_7)
        let s_28_8: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopDbg(
            state,
            tracer,
            s_28_6,
            s_28_7,
        );
        // S s_28_9: call ID_DFR0_write(s_28_8)
        let s_28_9: () = ID_DFR0_write(state, tracer, s_28_8);
        // C s_28_10: const #() : ()
        let s_28_10: () = ();
        // S s_28_11: call ID_DFR0_read(s_28_10)
        let s_28_11: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_28_10);
        // C s_28_12: const #9u : u8
        let s_28_12: u8 = 9;
        // S s_28_13: call _update_ID_DFR0_Type_PerfMon(s_28_11, s_28_12)
        let s_28_13: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_PerfMon(
            state,
            tracer,
            s_28_11,
            s_28_12,
        );
        // S s_28_14: call ID_DFR0_write(s_28_13)
        let s_28_14: () = ID_DFR0_write(state, tracer, s_28_13);
        // C s_28_15: const #() : ()
        let s_28_15: () = ();
        // S s_28_16: call ID_ISAR6_read(s_28_15)
        let s_28_16: ProductType700c18a878c5601b = ID_ISAR6_read(state, tracer, s_28_15);
        // C s_28_17: const #2u : u8
        let s_28_17: u8 = 2;
        // S s_28_18: call _update_ID_ISAR6_Type_SPECRES(s_28_16, s_28_17)
        let s_28_18: ProductType700c18a878c5601b = u_update_ID_ISAR6_Type_SPECRES(
            state,
            tracer,
            s_28_16,
            s_28_17,
        );
        // S s_28_19: call ID_ISAR6_write(s_28_18)
        let s_28_19: () = ID_ISAR6_write(state, tracer, s_28_18);
        // C s_28_20: const #() : ()
        let s_28_20: () = ();
        // S s_28_21: call ID_ISAR6_read(s_28_20)
        let s_28_21: ProductType700c18a878c5601b = ID_ISAR6_read(state, tracer, s_28_20);
        // D s_28_22: write-var ga#368515 <= s_28_21
        fn_state.ga_368515 = s_28_21;
        // D s_28_23: read-var ga#368515.0:struct
        let s_28_23: u32 = fn_state.ga_368515._0;
        // C s_28_24: const #28s : i
        let s_28_24: i128 = 28;
        // D s_28_25: cast zx s_28_23 -> bv
        let s_28_25: Bits = Bits::new(s_28_23 as u128, 32u16);
        // C s_28_26: const #1u : u8
        let s_28_26: u8 = 1;
        // C s_28_27: cast zx s_28_26 -> bv
        let s_28_27: Bits = Bits::new(s_28_26 as u128, 4u16);
        // C s_28_28: const #3s : i
        let s_28_28: i128 = 3;
        // C s_28_29: const #1u : u64
        let s_28_29: u64 = 1;
        // C s_28_30: cast zx s_28_29 -> bv
        let s_28_30: Bits = Bits::new(s_28_29 as u128, 64u16);
        // C s_28_31: lsl s_28_30 s_28_28
        let s_28_31: Bits = s_28_30 << s_28_28;
        // C s_28_32: sub s_28_31 s_28_30
        let s_28_32: Bits = ((s_28_31) - (s_28_30));
        // C s_28_33: and s_28_27 s_28_32
        let s_28_33: Bits = ((s_28_27) & (s_28_32));
        // C s_28_34: lsl s_28_33 s_28_24
        let s_28_34: Bits = s_28_33 << s_28_24;
        // C s_28_35: lsl s_28_32 s_28_24
        let s_28_35: Bits = s_28_32 << s_28_24;
        // C s_28_36: cmpl s_28_35
        let s_28_36: Bits = !s_28_35;
        // D s_28_37: and s_28_25 s_28_36
        let s_28_37: Bits = ((s_28_25) & (s_28_36));
        // D s_28_38: or s_28_37 s_28_34
        let s_28_38: Bits = ((s_28_37) | (s_28_34));
        // D s_28_39: cast reint s_28_38 -> u32
        let s_28_39: u32 = (s_28_38.value() as u32);
        // D s_28_40: call Mk_ID_ISAR6_Type(s_28_39)
        let s_28_40: ProductType700c18a878c5601b = Mk_ID_ISAR6_Type(
            state,
            tracer,
            s_28_39,
        );
        // D s_28_41: call ID_ISAR6_write(s_28_40)
        let s_28_41: () = ID_ISAR6_write(state, tracer, s_28_40);
        // C s_28_42: const #16816u : u32
        let s_28_42: u32 = 16816;
        // D s_28_43: read-reg s_28_42:struct
        let s_28_43: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_28_42 as isize);
            tracer.read_register(s_28_42 as isize, value);
            value
        };
        // C s_28_44: const #16816u : u32
        let s_28_44: u32 = 16816;
        // N s_28_45: write-reg s_28_44 <= s_28_43
        let s_28_45: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_28_44 as isize, s_28_43);
            tracer.write_register(s_28_44 as isize, s_28_43);
        };
        // C s_28_46: const #100864u : u32
        let s_28_46: u32 = 100864;
        // D s_28_47: read-reg s_28_46:struct
        let s_28_47: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_28_46 as isize);
            tracer.read_register(s_28_46 as isize, value);
            value
        };
        // C s_28_48: const #100864u : u32
        let s_28_48: u32 = 100864;
        // N s_28_49: write-reg s_28_48 <= s_28_47
        let s_28_49: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_28_48 as isize, s_28_47);
            tracer.write_register(s_28_48 as isize, s_28_47);
        };
        // N s_28_50: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #16816u : u32
        let s_29_0: u32 = 16816;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // C s_29_2: const #16816u : u32
        let s_29_2: u32 = 16816;
        // N s_29_3: write-reg s_29_2 <= s_29_1
        let s_29_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_29_2 as isize, s_29_1);
            tracer.write_register(s_29_2 as isize, s_29_1);
        };
        // C s_29_4: const #() : ()
        let s_29_4: () = ();
        // S s_29_5: call ID_DFR0_read(s_29_4)
        let s_29_5: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_29_4);
        // C s_29_6: const #8u : u8
        let s_29_6: u8 = 8;
        // S s_29_7: call _update_ID_DFR0_Type_PerfMon(s_29_5, s_29_6)
        let s_29_7: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_PerfMon(
            state,
            tracer,
            s_29_5,
            s_29_6,
        );
        // S s_29_8: call ID_DFR0_write(s_29_7)
        let s_29_8: () = ID_DFR0_write(state, tracer, s_29_7);
        // N s_29_9: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #16816u : u32
        let s_30_0: u32 = 16816;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #16816u : u32
        let s_30_2: u32 = 16816;
        // N s_30_3: write-reg s_30_2 <= s_30_1
        let s_30_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_30_2 as isize, s_30_1);
            tracer.write_register(s_30_2 as isize, s_30_1);
        };
        // N s_30_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #16816u : u32
        let s_31_0: u32 = 16816;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // C s_31_2: const #16816u : u32
        let s_31_2: u32 = 16816;
        // N s_31_3: write-reg s_31_2 <= s_31_1
        let s_31_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_31_2 as isize, s_31_1);
            tracer.write_register(s_31_2 as isize, s_31_1);
        };
        // N s_31_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call ID_PFR0_read(s_32_0)
        let s_32_1: ProductType700c18a878c5601b = ID_PFR0_read(state, tracer, s_32_0);
        // C s_32_2: const #1u : u8
        let s_32_2: u8 = 1;
        // S s_32_3: call _update_ID_PFR0_Type_CSV2(s_32_1, s_32_2)
        let s_32_3: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_CSV2(
            state,
            tracer,
            s_32_1,
            s_32_2,
        );
        // S s_32_4: call ID_PFR0_write(s_32_3)
        let s_32_4: () = ID_PFR0_write(state, tracer, s_32_3);
        // C s_32_5: const #() : ()
        let s_32_5: () = ();
        // S s_32_6: call ID_DFR0_read(s_32_5)
        let s_32_6: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_32_5);
        // C s_32_7: const #6u : u8
        let s_32_7: u8 = 6;
        // S s_32_8: call _update_ID_DFR0_Type_PerfMon(s_32_6, s_32_7)
        let s_32_8: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_PerfMon(
            state,
            tracer,
            s_32_6,
            s_32_7,
        );
        // S s_32_9: call ID_DFR0_write(s_32_8)
        let s_32_9: () = ID_DFR0_write(state, tracer, s_32_8);
        // N s_32_10: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call ID_PFR1_read(s_33_0)
        let s_33_1: ProductType700c18a878c5601b = ID_PFR1_read(state, tracer, s_33_0);
        // C s_33_2: const #2u : u8
        let s_33_2: u8 = 2;
        // S s_33_3: call _update_ID_PFR1_Type_GenTimer(s_33_1, s_33_2)
        let s_33_3: ProductType700c18a878c5601b = u_update_ID_PFR1_Type_GenTimer(
            state,
            tracer,
            s_33_1,
            s_33_2,
        );
        // S s_33_4: call ID_PFR1_write(s_33_3)
        let s_33_4: () = ID_PFR1_write(state, tracer, s_33_3);
        // C s_33_5: const #() : ()
        let s_33_5: () = ();
        // S s_33_6: call ID_PFR0_read(s_33_5)
        let s_33_6: ProductType700c18a878c5601b = ID_PFR0_read(state, tracer, s_33_5);
        // C s_33_7: const #2u : u8
        let s_33_7: u8 = 2;
        // S s_33_8: call _update_ID_PFR0_Type_AMU(s_33_6, s_33_7)
        let s_33_8: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_AMU(
            state,
            tracer,
            s_33_6,
            s_33_7,
        );
        // S s_33_9: call ID_PFR0_write(s_33_8)
        let s_33_9: () = ID_PFR0_write(state, tracer, s_33_8);
        // N s_33_10: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call ID_MMFR5_read(s_34_0)
        let s_34_1: ProductType700c18a878c5601b = ID_MMFR5_read(state, tracer, s_34_0);
        // C s_34_2: const #2u : u8
        let s_34_2: u8 = 2;
        // S s_34_3: call _update_ID_MMFR5_Type_ETS(s_34_1, s_34_2)
        let s_34_3: ProductType700c18a878c5601b = u_update_ID_MMFR5_Type_ETS(
            state,
            tracer,
            s_34_1,
            s_34_2,
        );
        // S s_34_4: call ID_MMFR5_write(s_34_3)
        let s_34_4: () = ID_MMFR5_write(state, tracer, s_34_3);
        // C s_34_5: const #() : ()
        let s_34_5: () = ();
        // S s_34_6: call ID_DFR0_read(s_34_5)
        let s_34_6: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_34_5);
        // C s_34_7: const #10u : u8
        let s_34_7: u8 = 10;
        // S s_34_8: call _update_ID_DFR0_Type_CopDbg(s_34_6, s_34_7)
        let s_34_8: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopDbg(
            state,
            tracer,
            s_34_6,
            s_34_7,
        );
        // S s_34_9: call ID_DFR0_write(s_34_8)
        let s_34_9: () = ID_DFR0_write(state, tracer, s_34_8);
        // C s_34_10: const #424u : u32
        let s_34_10: u32 = 424;
        // D s_34_11: read-reg s_34_10:u8
        let s_34_11: u8 = {
            let value = state.read_register::<u8>(s_34_10 as isize);
            tracer.read_register(s_34_10 as isize, value);
            value
        };
        // C s_34_12: const #2u : u8
        let s_34_12: u8 = 2;
        // D s_34_13: cmp-lt s_34_11 s_34_12
        let s_34_13: bool = ((s_34_11) < (s_34_12));
        // D s_34_14: not s_34_13
        let s_34_14: bool = !s_34_13;
        // N s_34_15: branch s_34_14 b39 b35
        if s_34_14 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#328017 <= s_35_0
        fn_state.gs_328017 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#328017:u8
        let s_36_0: bool = fn_state.gs_328017;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call ID_DFR0_read(s_37_0)
        let s_37_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_37_0);
        // C s_37_2: const #() : ()
        let s_37_2: () = ();
        // S s_37_3: call ID_DFR0_read(s_37_2)
        let s_37_3: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_37_2);
        // S s_37_4: call _get_ID_DFR0_Type_CopDbg(s_37_3)
        let s_37_4: u8 = u_get_ID_DFR0_Type_CopDbg(state, tracer, s_37_3);
        // S s_37_5: call _update_ID_DFR0_Type_CopSDbg(s_37_1, s_37_4)
        let s_37_5: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopSDbg(
            state,
            tracer,
            s_37_1,
            s_37_4,
        );
        // S s_37_6: call ID_DFR0_write(s_37_5)
        let s_37_6: () = ID_DFR0_write(state, tracer, s_37_5);
        // N s_37_7: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call ID_DFR0_read(s_38_0)
        let s_38_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_38_0);
        // C s_38_2: const #0u : u8
        let s_38_2: u8 = 0;
        // S s_38_3: call _update_ID_DFR0_Type_CopSDbg(s_38_1, s_38_2)
        let s_38_3: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopSDbg(
            state,
            tracer,
            s_38_1,
            s_38_2,
        );
        // S s_38_4: call ID_DFR0_write(s_38_3)
        let s_38_4: () = ID_DFR0_write(state, tracer, s_38_3);
        // N s_38_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #20920u : u32
        let s_39_0: u32 = 20920;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_SCR_Type_NS(s_39_1)
        let s_39_2: bool = u_get_SCR_Type_NS(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#328017 <= s_39_6
        fn_state.gs_328017 = s_39_6;
        // N s_39_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call ID_DFR0_read(s_40_0)
        let s_40_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_40_0);
        // C s_40_2: const #7u : u8
        let s_40_2: u8 = 7;
        // S s_40_3: call _update_ID_DFR0_Type_PerfMon(s_40_1, s_40_2)
        let s_40_3: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_PerfMon(
            state,
            tracer,
            s_40_1,
            s_40_2,
        );
        // S s_40_4: call ID_DFR0_write(s_40_3)
        let s_40_4: () = ID_DFR0_write(state, tracer, s_40_3);
        // N s_40_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #15424u : u32
        let s_41_0: u32 = 15424;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // C s_41_2: const #15424u : u32
        let s_41_2: u32 = 15424;
        // N s_41_3: write-reg s_41_2 <= s_41_1
        let s_41_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_41_2 as isize, s_41_1);
            tracer.write_register(s_41_2 as isize, s_41_1);
        };
        // C s_41_4: const #() : ()
        let s_41_4: () = ();
        // S s_41_5: call ID_ISAR6_read(s_41_4)
        let s_41_5: ProductType700c18a878c5601b = ID_ISAR6_read(state, tracer, s_41_4);
        // C s_41_6: const #1u : u8
        let s_41_6: u8 = 1;
        // S s_41_7: call _update_ID_ISAR6_Type_FHM(s_41_5, s_41_6)
        let s_41_7: ProductType700c18a878c5601b = u_update_ID_ISAR6_Type_FHM(
            state,
            tracer,
            s_41_5,
            s_41_6,
        );
        // S s_41_8: call ID_ISAR6_write(s_41_7)
        let s_41_8: () = ID_ISAR6_write(state, tracer, s_41_7);
        // C s_41_9: const #() : ()
        let s_41_9: () = ();
        // S s_41_10: call ID_PFR0_read(s_41_9)
        let s_41_10: ProductType700c18a878c5601b = ID_PFR0_read(state, tracer, s_41_9);
        // C s_41_11: const #1u : u8
        let s_41_11: u8 = 1;
        // S s_41_12: call _update_ID_PFR0_Type_AMU(s_41_10, s_41_11)
        let s_41_12: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_AMU(
            state,
            tracer,
            s_41_10,
            s_41_11,
        );
        // S s_41_13: call ID_PFR0_write(s_41_12)
        let s_41_13: () = ID_PFR0_write(state, tracer, s_41_12);
        // C s_41_14: const #() : ()
        let s_41_14: () = ();
        // S s_41_15: call ID_PFR0_read(s_41_14)
        let s_41_15: ProductType700c18a878c5601b = ID_PFR0_read(state, tracer, s_41_14);
        // C s_41_16: const #1u : u8
        let s_41_16: u8 = 1;
        // S s_41_17: call _update_ID_PFR0_Type_DIT(s_41_15, s_41_16)
        let s_41_17: ProductType700c18a878c5601b = u_update_ID_PFR0_Type_DIT(
            state,
            tracer,
            s_41_15,
            s_41_16,
        );
        // S s_41_18: call ID_PFR0_write(s_41_17)
        let s_41_18: () = ID_PFR0_write(state, tracer, s_41_17);
        // C s_41_19: const #() : ()
        let s_41_19: () = ();
        // S s_41_20: call ID_DFR0_read(s_41_19)
        let s_41_20: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_41_19);
        // C s_41_21: const #9u : u8
        let s_41_21: u8 = 9;
        // S s_41_22: call _update_ID_DFR0_Type_CopDbg(s_41_20, s_41_21)
        let s_41_22: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopDbg(
            state,
            tracer,
            s_41_20,
            s_41_21,
        );
        // S s_41_23: call ID_DFR0_write(s_41_22)
        let s_41_23: () = ID_DFR0_write(state, tracer, s_41_22);
        // C s_41_24: const #424u : u32
        let s_41_24: u32 = 424;
        // D s_41_25: read-reg s_41_24:u8
        let s_41_25: u8 = {
            let value = state.read_register::<u8>(s_41_24 as isize);
            tracer.read_register(s_41_24 as isize, value);
            value
        };
        // C s_41_26: const #2u : u8
        let s_41_26: u8 = 2;
        // D s_41_27: cmp-lt s_41_25 s_41_26
        let s_41_27: bool = ((s_41_25) < (s_41_26));
        // D s_41_28: not s_41_27
        let s_41_28: bool = !s_41_27;
        // N s_41_29: branch s_41_28 b46 b42
        if s_41_28 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#328028 <= s_42_0
        fn_state.gs_328028 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#328028:u8
        let s_43_0: bool = fn_state.gs_328028;
        // N s_43_1: branch s_43_0 b45 b44
        if s_43_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call ID_DFR0_read(s_44_0)
        let s_44_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_44_0);
        // C s_44_2: const #20416u : u32
        let s_44_2: u32 = 20416;
        // D s_44_3: read-reg s_44_2:struct
        let s_44_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_2 as isize);
            tracer.read_register(s_44_2 as isize, value);
            value
        };
        // D s_44_4: call _get_ID_DFR0_EL1_Type_CopDbg(s_44_3)
        let s_44_4: u8 = u_get_ID_DFR0_EL1_Type_CopDbg(state, tracer, s_44_3);
        // D s_44_5: call _update_ID_DFR0_Type_CopSDbg(s_44_1, s_44_4)
        let s_44_5: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopSDbg(
            state,
            tracer,
            s_44_1,
            s_44_4,
        );
        // D s_44_6: call ID_DFR0_write(s_44_5)
        let s_44_6: () = ID_DFR0_write(state, tracer, s_44_5);
        // N s_44_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call ID_DFR0_read(s_45_0)
        let s_45_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_45_0);
        // C s_45_2: const #0u : u8
        let s_45_2: u8 = 0;
        // S s_45_3: call _update_ID_DFR0_Type_CopSDbg(s_45_1, s_45_2)
        let s_45_3: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopSDbg(
            state,
            tracer,
            s_45_1,
            s_45_2,
        );
        // S s_45_4: call ID_DFR0_write(s_45_3)
        let s_45_4: () = ID_DFR0_write(state, tracer, s_45_3);
        // N s_45_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #20920u : u32
        let s_46_0: u32 = 20920;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_SCR_Type_NS(s_46_1)
        let s_46_2: bool = u_get_SCR_Type_NS(state, tracer, s_46_1);
        // D s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // C s_46_4: const #1u : u8
        let s_46_4: bool = true;
        // C s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 1u16);
        // D s_46_6: cmp-eq s_46_3 s_46_5
        let s_46_6: bool = ((s_46_3) == (s_46_5));
        // D s_46_7: write-var gs#328028 <= s_46_6
        fn_state.gs_328028 = s_46_6;
        // N s_46_8: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call ID_MMFR4_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = ID_MMFR4_read(state, tracer, s_47_0);
        // C s_47_2: const #1u : u8
        let s_47_2: u8 = 1;
        // S s_47_3: call _update_ID_MMFR4_Type_CCIDX(s_47_1, s_47_2)
        let s_47_3: ProductType700c18a878c5601b = u_update_ID_MMFR4_Type_CCIDX(
            state,
            tracer,
            s_47_1,
            s_47_2,
        );
        // S s_47_4: call ID_MMFR4_write(s_47_3)
        let s_47_4: () = ID_MMFR4_write(state, tracer, s_47_3);
        // C s_47_5: const #() : ()
        let s_47_5: () = ();
        // S s_47_6: call ID_ISAR6_read(s_47_5)
        let s_47_6: ProductType700c18a878c5601b = ID_ISAR6_read(state, tracer, s_47_5);
        // C s_47_7: const #1u : u8
        let s_47_7: u8 = 1;
        // S s_47_8: call _update_ID_ISAR6_Type_JSCVT(s_47_6, s_47_7)
        let s_47_8: ProductType700c18a878c5601b = u_update_ID_ISAR6_Type_JSCVT(
            state,
            tracer,
            s_47_6,
            s_47_7,
        );
        // S s_47_9: call ID_ISAR6_write(s_47_8)
        let s_47_9: () = ID_ISAR6_write(state, tracer, s_47_8);
        // C s_47_10: const #() : ()
        let s_47_10: () = ();
        // S s_47_11: call ID_DFR0_read(s_47_10)
        let s_47_11: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_47_10);
        // C s_47_12: const #1u : u8
        let s_47_12: u8 = 1;
        // S s_47_13: call _update_ID_DFR0_Type_TraceFilt(s_47_11, s_47_12)
        let s_47_13: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_TraceFilt(
            state,
            tracer,
            s_47_11,
            s_47_12,
        );
        // S s_47_14: call ID_DFR0_write(s_47_13)
        let s_47_14: () = ID_DFR0_write(state, tracer, s_47_13);
        // N s_47_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call MVFR1_read(s_48_0)
        let s_48_1: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_48_0);
        // C s_48_2: const #3u : u8
        let s_48_2: u8 = 3;
        // S s_48_3: call _update_MVFR1_Type_FPHP(s_48_1, s_48_2)
        let s_48_3: ProductType700c18a878c5601b = u_update_MVFR1_Type_FPHP(
            state,
            tracer,
            s_48_1,
            s_48_2,
        );
        // S s_48_4: call MVFR1_write(s_48_3)
        let s_48_4: () = MVFR1_write(state, tracer, s_48_3);
        // C s_48_5: const #() : ()
        let s_48_5: () = ();
        // S s_48_6: call MVFR1_read(s_48_5)
        let s_48_6: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_48_5);
        // C s_48_7: const #2u : u8
        let s_48_7: u8 = 2;
        // S s_48_8: call _update_MVFR1_Type_SIMDHP(s_48_6, s_48_7)
        let s_48_8: ProductType700c18a878c5601b = u_update_MVFR1_Type_SIMDHP(
            state,
            tracer,
            s_48_6,
            s_48_7,
        );
        // S s_48_9: call MVFR1_write(s_48_8)
        let s_48_9: () = MVFR1_write(state, tracer, s_48_8);
        // N s_48_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call ID_DFR0_read(s_49_0)
        let s_49_1: ProductType700c18a878c5601b = ID_DFR0_read(state, tracer, s_49_0);
        // C s_49_2: const #0u : u8
        let s_49_2: u8 = 0;
        // S s_49_3: call _update_ID_DFR0_Type_CopSDbg(s_49_1, s_49_2)
        let s_49_3: ProductType700c18a878c5601b = u_update_ID_DFR0_Type_CopSDbg(
            state,
            tracer,
            s_49_1,
            s_49_2,
        );
        // S s_49_4: call ID_DFR0_write(s_49_3)
        let s_49_4: () = ID_DFR0_write(state, tracer, s_49_3);
        // N s_49_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #20920u : u32
        let s_50_0: u32 = 20920;
        // D s_50_1: read-reg s_50_0:struct
        let s_50_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call _get_SCR_Type_NS(s_50_1)
        let s_50_2: bool = u_get_SCR_Type_NS(state, tracer, s_50_1);
        // D s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // C s_50_4: const #1u : u8
        let s_50_4: bool = true;
        // C s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 1u16);
        // D s_50_6: cmp-eq s_50_3 s_50_5
        let s_50_6: bool = ((s_50_3) == (s_50_5));
        // D s_50_7: write-var gs#327782 <= s_50_6
        fn_state.gs_327782 = s_50_6;
        // N s_50_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #8s : i
        let s_51_0: i128 = 8;
        // S s_51_1: call Zeros(s_51_0)
        let s_51_1: Bits = Zeros(state, tracer, s_51_0);
        // S s_51_2: cast reint s_51_1 -> u8
        let s_51_2: u8 = (s_51_1.value() as u8);
        // D s_51_3: write-var ga#367925 <= s_51_2
        fn_state.ga_367925 = s_51_2;
        // N s_51_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
