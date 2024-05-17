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
use u_get_ICH_VMCR_EL2_Type_VPMR::*;
use EDSCR_write::*;
use GetNumEventCounters::*;
use u_get_ICH_VMCR_EL2_Type_VEOIM::*;
use u_get_AMCGCR_EL0_Type_CG1NC::*;
use DBGDSCRext_write::*;
use u_get_ICV_CTLR_EL1_Type_IDbits::*;
use Zeros::*;
use u_get_ICH_VMCR_EL2_Type_VBPR0::*;
use u_get_ICV_BPR0_EL1_Type_BinaryPoint::*;
use u_get_ICV_CTLR_EL1_Type_CBPR::*;
use u_get_ICC_SRE_EL1_Type_SRE::*;
use u_get_ICC_CTLR_EL3_Type_nDS::*;
use u_get_ICV_IGRPEN0_EL1_Type_Enable::*;
use AArch64_AutoGen_SysRegWrite::*;
use u_get_OSLAR_EL1_Type_OSLK::*;
use u_get_ICC_CTLR_EL3_Type_IDbits::*;
use u_update_DBGOSLSR_Type_OSLK::*;
use u_get_ICC_CTLR_EL3_Type_PRIbits::*;
use u_get_ICV_CTLR_EL1_Type_ExtRange::*;
use u__get_PMCCFILTR_EL0::*;
use u_update_DBGDSCRext_Type_TXfull::*;
use u_get_ICV_CTLR_EL1_Type_SEIS::*;
use Mk_PMCCNTR_EL0_Type::*;
use u_get_ICC_CTLR_EL3_Type_A3V::*;
use X_read::*;
use Mk_PMCCFILTR_EL0_Type::*;
use u_update_DBGDSCRint_Type_TXfull::*;
use u_get_PMSELR_EL0_Type_SEL::*;
use AArch64_PMUSwIncrement::*;
use X_set::*;
use CheckOSUnlockCatch::*;
use DBGDSCRint_write::*;
use u_update_EDSCR_Type_TXfull::*;
use AArch64_GetNumEventCountersAccessible::*;
use integer_subrange::*;
use u_get_ICV_BPR1_EL1_Type_BinaryPoint::*;
use DBGDSCRint_read::*;
use u_get_ICH_VMCR_EL2_Type_VBPR1::*;
use u_get_ICH_VMCR_EL2_Type_VCBPR::*;
use DBGDTR_EL0_read__1::*;
use integer_access::*;
use u_get_ICH_VTR_EL2_Type_PREbits::*;
use Bit::*;
use u_get_ICV_CTLR_EL1_Type_PRIbits::*;
use DBGDSCRext_read::*;
use AArch64_SystemAccessTrap::*;
use Mk_PMCNTENCLR_EL0_Type::*;
use u_get_AMCGCR_EL0_Type_CG0NC::*;
use TakeReset::*;
use u_get_OSLSR_EL1_Type_OSLK::*;
use u_get_ICH_VMCR_EL2_Type_VENG0::*;
use Mk_PMOVSSET_EL0_Type::*;
use PMUCounterMask::*;
use Mk_PMEVTYPER_EL0_Type::*;
use DBGOSLSR_read::*;
use EL2Enabled::*;
use Mk_PMCNTENSET_EL0_Type::*;
use Mk_PMINTENCLR_EL1_Type::*;
use Mk_PMINTENSET_EL1_Type::*;
use u_get_ICV_CTLR_EL1_Type_EOImode::*;
use u_get_ICV_PMR_EL1_Type_Priority::*;
use CurrentSecurityState::*;
use u_get_ICC_CTLR_EL3_Type_SEIS::*;
use u__get_PMEVTYPER_EL0::*;
use u_get_ICC_CTLR_EL3_Type_ExtRange::*;
use u_get_ICV_CTLR_EL1_Type_A3V::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use u_get_ICV_IGRPEN1_EL1_Type_Enable::*;
use u_get_ICH_VMCR_EL2_Type_VENG1::*;
use Mk_PMOVSCLR_EL0_Type::*;
use DBGOSLSR_write::*;
use AArch64_ClearEventCounters::*;
use EDSCR_read::*;
use u_get_PMCR_EL0_Type_D::*;
use common::*;
pub fn AArch64_SysRegWrite<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: i128,
    op1: i128,
    crn: i128,
    crm: i128,
    op2: i128,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_141078: bool,
        tempxt2: u64,
        gs_141252: bool,
        gs_141155: bool,
        gs_141432: bool,
        gs_141320: bool,
        gs_141094: bool,
        gs_141572: bool,
        gs_141289: bool,
        gs_141467: bool,
        gs_141567: bool,
        gs_141087: bool,
        gs_141406: bool,
        mask: u64,
        gs_141418: bool,
        gs_141450: bool,
        gs_141472: bool,
        gs_141399: bool,
        gs_141323: bool,
        gs_141392: bool,
        gs_141403: bool,
        gs_141266: bool,
        gs_141298: bool,
        gs_141286: bool,
        gs_141259: bool,
        gs_141071: bool,
        gs_141558: bool,
        gs_141570: bool,
        gs_141458: bool,
        gs_141717: bool,
        gs_141065: bool,
        gs_141455: bool,
        gs_141120: bool,
        gs_141522: bool,
        gs_141489: bool,
        gs_141324: bool,
        gs_141085: bool,
        gs_141563: bool,
        gs_141254: bool,
        gs_141461: bool,
        gs_141226: bool,
        gs_141114: bool,
        gs_141538: bool,
        gs_141296: bool,
        gs_141430: bool,
        gs_141279: bool,
        gs_141401: bool,
        gs_141436: bool,
        gs_141224: bool,
        gs_141058: bool,
        gs_141525: bool,
        gs_141060: bool,
        gs_141495: bool,
        gs_141502: bool,
        gs_141388: bool,
        gs_141509: bool,
        gs_141549: bool,
        gs_141237: bool,
        gs_141394: bool,
        gs_141397: bool,
        gs_141474: bool,
        gs_141543: bool,
        gs_141561: bool,
        gs_141062: bool,
        gs_141470: bool,
        gs_141513: bool,
        gs_141415: bool,
        gs_141428: bool,
        gs_141282: bool,
        gs_141447: bool,
        gs_141452: bool,
        gs_141294: bool,
        gs_141235: bool,
        gs_141529: bool,
        gs_141444: bool,
        gs_141456: bool,
        gs_141422: bool,
        gs_141498: bool,
        gs_141465: bool,
        gs_141410: bool,
        gs_141290: bool,
        gs_141493: bool,
        gs_141435: bool,
        gs_141547: bool,
        ga_247801: ProductType5c790c8ef59cc8b2,
        gs_141076: bool,
        gs_141291: bool,
        gs_141278: bool,
        gs_141228: bool,
        gs_141518: bool,
        gs_141408: bool,
        gs_141080: bool,
        gs_141251: bool,
        gs_141576: bool,
        gs_141463: bool,
        gs_141527: bool,
        gs_141069: bool,
        gs_141417: bool,
        gs_141486: bool,
        ga_247804: ProductType5c790c8ef59cc8b2,
        gs_141545: bool,
        gs_141089: bool,
        gs_141520: bool,
        restore_xt: bool,
        gs_141412: bool,
        gs_141338: bool,
        gs_141243: bool,
        gs_141481: bool,
        gs_141347: bool,
        gs_141053: bool,
        gs_141357: bool,
        tempxt: u64,
        gs_141552: bool,
        gs_141261: bool,
        gs_141507: bool,
        gs_141477: bool,
        gs_141098: bool,
        gs_141074: bool,
        gs_141051: bool,
        gs_141478: bool,
        gs_141441: bool,
        gs_141554: bool,
        gs_141083: bool,
        gs_141491: bool,
        gs_141425: bool,
        gs_141321: bool,
        gs_141536: bool,
        gs_141240: bool,
        nshadow_1008: i64,
        gs_141241: bool,
        gs_141500: bool,
        gs_141263: bool,
        gs_141318: bool,
        gs_141268: bool,
        gs_141516: bool,
        gs_141531: bool,
        gs_141540: bool,
        gs_141319: bool,
        gs_141257: bool,
        gs_141101: bool,
        gs_141049: bool,
        gs_141574: bool,
        gs_141390: bool,
        gs_141284: bool,
        gs_141556: bool,
        gs_141504: bool,
        gs_141103: bool,
        index: i64,
        gs_141534: bool,
        gs_141339: bool,
        gs_141420: bool,
        gs_141232: bool,
        gs_141445: bool,
        gs_141246: bool,
        gs_141340: bool,
        gs_141231: bool,
        gs_141346: bool,
        gs_141277: bool,
        gs_141092: bool,
        gs_141056: bool,
        gs_141565: bool,
        gs_141096: bool,
        gs_141067: bool,
        gs_141248: bool,
        gs_141439: bool,
        gs_141511: bool,
        gs_141483: bool,
        gs_141337: bool,
        op0: i128,
        op1: i128,
        crn: i128,
        crm: i128,
        op2: i128,
        t: i128,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        crn,
        crm,
        op2,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // D s_0_1: read-var t:i
        let s_0_1: i128 = fn_state.t;
        // D s_0_2: call X_read(s_0_1, s_0_0)
        let s_0_2: Bits = X_read(state, tracer, s_0_1, s_0_0);
        // D s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // D s_0_4: write-var tempxt <= s_0_3
        fn_state.tempxt = s_0_3;
        // C s_0_5: const #2s : i
        let s_0_5: i128 = 2;
        // D s_0_6: read-var op0:i
        let s_0_6: i128 = fn_state.op0;
        // D s_0_7: cmp-eq s_0_6 s_0_5
        let s_0_7: bool = ((s_0_6) == (s_0_5));
        // N s_0_8: branch s_0_7 b707 b1
        if s_0_7 {
            return block_707(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#141049 <= s_1_0
        fn_state.gs_141049 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#141049:u8
        let s_2_0: bool = fn_state.gs_141049;
        // N s_2_1: branch s_2_0 b706 b3
        if s_2_0 {
            return block_706(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#141051 <= s_3_0
        fn_state.gs_141051 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#141051:u8
        let s_4_0: bool = fn_state.gs_141051;
        // N s_4_1: branch s_4_0 b705 b5
        if s_4_0 {
            return block_705(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#141053 <= s_5_0
        fn_state.gs_141053 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#141053:u8
        let s_6_0: bool = fn_state.gs_141053;
        // N s_6_1: branch s_6_0 b704 b7
        if s_6_0 {
            return block_704(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3s : i
        let s_8_0: i128 = 3;
        // D s_8_1: read-var op0:i
        let s_8_1: i128 = fn_state.op0;
        // D s_8_2: cmp-eq s_8_1 s_8_0
        let s_8_2: bool = ((s_8_1) == (s_8_0));
        // N s_8_3: branch s_8_2 b703 b9
        if s_8_2 {
            return block_703(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#141056 <= s_9_0
        fn_state.gs_141056 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#141056:u8
        let s_10_0: bool = fn_state.gs_141056;
        // N s_10_1: branch s_10_0 b702 b11
        if s_10_0 {
            return block_702(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#141058 <= s_11_0
        fn_state.gs_141058 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#141058:u8
        let s_12_0: bool = fn_state.gs_141058;
        // N s_12_1: branch s_12_0 b701 b13
        if s_12_0 {
            return block_701(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#141060 <= s_13_0
        fn_state.gs_141060 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#141060:u8
        let s_14_0: bool = fn_state.gs_141060;
        // N s_14_1: branch s_14_0 b700 b15
        if s_14_0 {
            return block_700(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#141062 <= s_15_0
        fn_state.gs_141062 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#141062:u8
        let s_16_0: bool = fn_state.gs_141062;
        // N s_16_1: branch s_16_0 b699 b17
        if s_16_0 {
            return block_699(state, tracer, fn_state);
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
        // C s_18_0: const #3s : i
        let s_18_0: i128 = 3;
        // D s_18_1: read-var op0:i
        let s_18_1: i128 = fn_state.op0;
        // D s_18_2: cmp-eq s_18_1 s_18_0
        let s_18_2: bool = ((s_18_1) == (s_18_0));
        // N s_18_3: branch s_18_2 b698 b19
        if s_18_2 {
            return block_698(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#141065 <= s_19_0
        fn_state.gs_141065 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#141065:u8
        let s_20_0: bool = fn_state.gs_141065;
        // N s_20_1: branch s_20_0 b697 b21
        if s_20_0 {
            return block_697(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#141067 <= s_21_0
        fn_state.gs_141067 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#141067:u8
        let s_22_0: bool = fn_state.gs_141067;
        // N s_22_1: branch s_22_0 b696 b23
        if s_22_0 {
            return block_696(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#141069 <= s_23_0
        fn_state.gs_141069 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#141069:u8
        let s_24_0: bool = fn_state.gs_141069;
        // N s_24_1: branch s_24_0 b695 b25
        if s_24_0 {
            return block_695(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#141071 <= s_25_0
        fn_state.gs_141071 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#141071:u8
        let s_26_0: bool = fn_state.gs_141071;
        // N s_26_1: branch s_26_0 b694 b27
        if s_26_0 {
            return block_694(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #3s : i
        let s_28_0: i128 = 3;
        // D s_28_1: read-var op0:i
        let s_28_1: i128 = fn_state.op0;
        // D s_28_2: cmp-eq s_28_1 s_28_0
        let s_28_2: bool = ((s_28_1) == (s_28_0));
        // N s_28_3: branch s_28_2 b693 b29
        if s_28_2 {
            return block_693(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#141074 <= s_29_0
        fn_state.gs_141074 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#141074:u8
        let s_30_0: bool = fn_state.gs_141074;
        // N s_30_1: branch s_30_0 b692 b31
        if s_30_0 {
            return block_692(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#141076 <= s_31_0
        fn_state.gs_141076 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#141076:u8
        let s_32_0: bool = fn_state.gs_141076;
        // N s_32_1: branch s_32_0 b691 b33
        if s_32_0 {
            return block_691(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#141078 <= s_33_0
        fn_state.gs_141078 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#141078:u8
        let s_34_0: bool = fn_state.gs_141078;
        // N s_34_1: branch s_34_0 b690 b35
        if s_34_0 {
            return block_690(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#141080 <= s_35_0
        fn_state.gs_141080 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#141080:u8
        let s_36_0: bool = fn_state.gs_141080;
        // N s_36_1: branch s_36_0 b668 b37
        if s_36_0 {
            return block_668(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #3s : i
        let s_38_0: i128 = 3;
        // D s_38_1: read-var op0:i
        let s_38_1: i128 = fn_state.op0;
        // D s_38_2: cmp-eq s_38_1 s_38_0
        let s_38_2: bool = ((s_38_1) == (s_38_0));
        // N s_38_3: branch s_38_2 b667 b39
        if s_38_2 {
            return block_667(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#141083 <= s_39_0
        fn_state.gs_141083 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#141083:u8
        let s_40_0: bool = fn_state.gs_141083;
        // N s_40_1: branch s_40_0 b666 b41
        if s_40_0 {
            return block_666(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#141085 <= s_41_0
        fn_state.gs_141085 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#141085:u8
        let s_42_0: bool = fn_state.gs_141085;
        // N s_42_1: branch s_42_0 b665 b43
        if s_42_0 {
            return block_665(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#141087 <= s_43_0
        fn_state.gs_141087 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#141087:u8
        let s_44_0: bool = fn_state.gs_141087;
        // N s_44_1: branch s_44_0 b664 b45
        if s_44_0 {
            return block_664(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#141089 <= s_45_0
        fn_state.gs_141089 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#141089:u8
        let s_46_0: bool = fn_state.gs_141089;
        // N s_46_1: branch s_46_0 b660 b47
        if s_46_0 {
            return block_660(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #3s : i
        let s_48_0: i128 = 3;
        // D s_48_1: read-var op0:i
        let s_48_1: i128 = fn_state.op0;
        // D s_48_2: cmp-eq s_48_1 s_48_0
        let s_48_2: bool = ((s_48_1) == (s_48_0));
        // N s_48_3: branch s_48_2 b659 b49
        if s_48_2 {
            return block_659(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#141092 <= s_49_0
        fn_state.gs_141092 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#141092:u8
        let s_50_0: bool = fn_state.gs_141092;
        // N s_50_1: branch s_50_0 b658 b51
        if s_50_0 {
            return block_658(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#141094 <= s_51_0
        fn_state.gs_141094 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#141094:u8
        let s_52_0: bool = fn_state.gs_141094;
        // N s_52_1: branch s_52_0 b657 b53
        if s_52_0 {
            return block_657(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#141096 <= s_53_0
        fn_state.gs_141096 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#141096:u8
        let s_54_0: bool = fn_state.gs_141096;
        // N s_54_1: branch s_54_0 b656 b55
        if s_54_0 {
            return block_656(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#141098 <= s_55_0
        fn_state.gs_141098 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#141098:u8
        let s_56_0: bool = fn_state.gs_141098;
        // N s_56_1: branch s_56_0 b645 b57
        if s_56_0 {
            return block_645(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #3s : i
        let s_58_0: i128 = 3;
        // D s_58_1: read-var op0:i
        let s_58_1: i128 = fn_state.op0;
        // D s_58_2: cmp-eq s_58_1 s_58_0
        let s_58_2: bool = ((s_58_1) == (s_58_0));
        // N s_58_3: branch s_58_2 b644 b59
        if s_58_2 {
            return block_644(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#141101 <= s_59_0
        fn_state.gs_141101 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#141101:u8
        let s_60_0: bool = fn_state.gs_141101;
        // N s_60_1: branch s_60_0 b643 b61
        if s_60_0 {
            return block_643(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#141103 <= s_61_0
        fn_state.gs_141103 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#141103:u8
        let s_62_0: bool = fn_state.gs_141103;
        // N s_62_1: branch s_62_0 b627 b63
        if s_62_0 {
            return block_627(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call PMUCounterMask(s_64_0)
        let s_64_1: u64 = PMUCounterMask(state, tracer, s_64_0);
        // D s_64_2: write-var mask <= s_64_1
        fn_state.mask = s_64_1;
        // C s_64_3: const #3s : i
        let s_64_3: i128 = 3;
        // D s_64_4: read-var op0:i
        let s_64_4: i128 = fn_state.op0;
        // D s_64_5: cmp-eq s_64_4 s_64_3
        let s_64_5: bool = ((s_64_4) == (s_64_3));
        // N s_64_6: branch s_64_5 b626 b65
        if s_64_5 {
            return block_626(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#141224 <= s_65_0
        fn_state.gs_141224 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#141224:u8
        let s_66_0: bool = fn_state.gs_141224;
        // N s_66_1: branch s_66_0 b625 b67
        if s_66_0 {
            return block_625(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#141226 <= s_67_0
        fn_state.gs_141226 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#141226:u8
        let s_68_0: bool = fn_state.gs_141226;
        // N s_68_1: branch s_68_0 b624 b69
        if s_68_0 {
            return block_624(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#141228 <= s_69_0
        fn_state.gs_141228 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#141228:u8
        let s_70_0: bool = fn_state.gs_141228;
        // N s_70_1: branch s_70_0 b620 b71
        if s_70_0 {
            return block_620(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#141232 <= s_71_0
        fn_state.gs_141232 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#141232:u8
        let s_72_0: bool = fn_state.gs_141232;
        // N s_72_1: branch s_72_0 b613 b73
        if s_72_0 {
            return block_613(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_73_0: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #3s : i
        let s_74_0: i128 = 3;
        // D s_74_1: read-var op0:i
        let s_74_1: i128 = fn_state.op0;
        // D s_74_2: cmp-eq s_74_1 s_74_0
        let s_74_2: bool = ((s_74_1) == (s_74_0));
        // N s_74_3: branch s_74_2 b612 b75
        if s_74_2 {
            return block_612(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#141235 <= s_75_0
        fn_state.gs_141235 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#141235:u8
        let s_76_0: bool = fn_state.gs_141235;
        // N s_76_1: branch s_76_0 b611 b77
        if s_76_0 {
            return block_611(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#141237 <= s_77_0
        fn_state.gs_141237 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#141237:u8
        let s_78_0: bool = fn_state.gs_141237;
        // N s_78_1: branch s_78_0 b607 b79
        if s_78_0 {
            return block_607(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#141241 <= s_79_0
        fn_state.gs_141241 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#141241:u8
        let s_80_0: bool = fn_state.gs_141241;
        // N s_80_1: branch s_80_0 b606 b81
        if s_80_0 {
            return block_606(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#141243 <= s_81_0
        fn_state.gs_141243 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#141243:u8
        let s_82_0: bool = fn_state.gs_141243;
        // N s_82_1: branch s_82_0 b599 b83
        if s_82_0 {
            return block_599(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_83_0: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #3s : i
        let s_84_0: i128 = 3;
        // D s_84_1: read-var op0:i
        let s_84_1: i128 = fn_state.op0;
        // D s_84_2: cmp-eq s_84_1 s_84_0
        let s_84_2: bool = ((s_84_1) == (s_84_0));
        // N s_84_3: branch s_84_2 b598 b85
        if s_84_2 {
            return block_598(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#141246 <= s_85_0
        fn_state.gs_141246 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#141246:u8
        let s_86_0: bool = fn_state.gs_141246;
        // N s_86_1: branch s_86_0 b597 b87
        if s_86_0 {
            return block_597(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#141248 <= s_87_0
        fn_state.gs_141248 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#141248:u8
        let s_88_0: bool = fn_state.gs_141248;
        // N s_88_1: branch s_88_0 b593 b89
        if s_88_0 {
            return block_593(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#141252 <= s_89_0
        fn_state.gs_141252 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#141252:u8
        let s_90_0: bool = fn_state.gs_141252;
        // N s_90_1: branch s_90_0 b592 b91
        if s_90_0 {
            return block_592(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#141254 <= s_91_0
        fn_state.gs_141254 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#141254:u8
        let s_92_0: bool = fn_state.gs_141254;
        // N s_92_1: branch s_92_0 b585 b93
        if s_92_0 {
            return block_585(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_93_0: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #3s : i
        let s_94_0: i128 = 3;
        // D s_94_1: read-var op0:i
        let s_94_1: i128 = fn_state.op0;
        // D s_94_2: cmp-eq s_94_1 s_94_0
        let s_94_2: bool = ((s_94_1) == (s_94_0));
        // N s_94_3: branch s_94_2 b584 b95
        if s_94_2 {
            return block_584(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#141257 <= s_95_0
        fn_state.gs_141257 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#141257:u8
        let s_96_0: bool = fn_state.gs_141257;
        // N s_96_1: branch s_96_0 b583 b97
        if s_96_0 {
            return block_583(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#141259 <= s_97_0
        fn_state.gs_141259 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#141259:u8
        let s_98_0: bool = fn_state.gs_141259;
        // N s_98_1: branch s_98_0 b582 b99
        if s_98_0 {
            return block_582(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#141261 <= s_99_0
        fn_state.gs_141261 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#141261:u8
        let s_100_0: bool = fn_state.gs_141261;
        // N s_100_1: branch s_100_0 b581 b101
        if s_100_0 {
            return block_581(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#141263 <= s_101_0
        fn_state.gs_141263 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#141263:u8
        let s_102_0: bool = fn_state.gs_141263;
        // N s_102_1: branch s_102_0 b574 b103
        if s_102_0 {
            return block_574(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #3s : i
        let s_104_0: i128 = 3;
        // D s_104_1: read-var op0:i
        let s_104_1: i128 = fn_state.op0;
        // D s_104_2: cmp-eq s_104_1 s_104_0
        let s_104_2: bool = ((s_104_1) == (s_104_0));
        // N s_104_3: branch s_104_2 b573 b105
        if s_104_2 {
            return block_573(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#141266 <= s_105_0
        fn_state.gs_141266 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#141266:u8
        let s_106_0: bool = fn_state.gs_141266;
        // N s_106_1: branch s_106_0 b572 b107
        if s_106_0 {
            return block_572(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#141268 <= s_107_0
        fn_state.gs_141268 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#141268:u8
        let s_108_0: bool = fn_state.gs_141268;
        // N s_108_1: branch s_108_0 b565 b109
        if s_108_0 {
            return block_565(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#141279 <= s_109_0
        fn_state.gs_141279 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#141279:u8
        let s_110_0: bool = fn_state.gs_141279;
        // N s_110_1: branch s_110_0 b541 b111
        if s_110_0 {
            return block_541(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_111_0: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #3s : i
        let s_112_0: i128 = 3;
        // D s_112_1: read-var op0:i
        let s_112_1: i128 = fn_state.op0;
        // D s_112_2: cmp-eq s_112_1 s_112_0
        let s_112_2: bool = ((s_112_1) == (s_112_0));
        // N s_112_3: branch s_112_2 b540 b113
        if s_112_2 {
            return block_540(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var gs#141282 <= s_113_0
        fn_state.gs_141282 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#141282:u8
        let s_114_0: bool = fn_state.gs_141282;
        // N s_114_1: branch s_114_0 b539 b115
        if s_114_0 {
            return block_539(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#141284 <= s_115_0
        fn_state.gs_141284 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#141284:u8
        let s_116_0: bool = fn_state.gs_141284;
        // N s_116_1: branch s_116_0 b538 b117
        if s_116_0 {
            return block_538(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#141286 <= s_117_0
        fn_state.gs_141286 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#141286:u8
        let s_118_0: bool = fn_state.gs_141286;
        // N s_118_1: branch s_118_0 b531 b119
        if s_118_0 {
            return block_531(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#141291 <= s_119_0
        fn_state.gs_141291 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#141291:u8
        let s_120_0: bool = fn_state.gs_141291;
        // N s_120_1: branch s_120_0 b507 b121
        if s_120_0 {
            return block_507(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_121_0: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #2s : i
        let s_122_0: i128 = 2;
        // D s_122_1: read-var op0:i
        let s_122_1: i128 = fn_state.op0;
        // D s_122_2: cmp-eq s_122_1 s_122_0
        let s_122_2: bool = ((s_122_1) == (s_122_0));
        // N s_122_3: branch s_122_2 b506 b123
        if s_122_2 {
            return block_506(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#141294 <= s_123_0
        fn_state.gs_141294 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#141294:u8
        let s_124_0: bool = fn_state.gs_141294;
        // N s_124_1: branch s_124_0 b505 b125
        if s_124_0 {
            return block_505(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#141296 <= s_125_0
        fn_state.gs_141296 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#141296:u8
        let s_126_0: bool = fn_state.gs_141296;
        // N s_126_1: branch s_126_0 b504 b127
        if s_126_0 {
            return block_504(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#141298 <= s_127_0
        fn_state.gs_141298 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#141298:u8
        let s_128_0: bool = fn_state.gs_141298;
        // N s_128_1: branch s_128_0 b497 b129
        if s_128_0 {
            return block_497(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_129_0: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #64s : i64
        let s_130_0: i64 = 64;
        // D s_130_1: read-var t:i
        let s_130_1: i128 = fn_state.t;
        // D s_130_2: call X_read(s_130_1, s_130_0)
        let s_130_2: Bits = X_read(state, tracer, s_130_1, s_130_0);
        // D s_130_3: cast reint s_130_2 -> u64
        let s_130_3: u64 = (s_130_2.value() as u64);
        // D s_130_4: read-var tempxt:u64
        let s_130_4: u64 = fn_state.tempxt;
        // D s_130_5: cast zx s_130_4 -> bv
        let s_130_5: Bits = Bits::new(s_130_4 as u128, 64u16);
        // D s_130_6: cast zx s_130_3 -> bv
        let s_130_6: Bits = Bits::new(s_130_3 as u128, 64u16);
        // D s_130_7: cmp-ne s_130_5 s_130_6
        let s_130_7: bool = ((s_130_5) != (s_130_6));
        // N s_130_8: branch s_130_7 b496 b131
        if s_130_7 {
            return block_496(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var restore_xt <= s_131_0
        fn_state.restore_xt = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #16975u : u32
        let s_132_0: u32 = 16975;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // C s_132_2: const #1s : i
        let s_132_2: i128 = 1;
        // C s_132_3: const #0s : i
        let s_132_3: i128 = 0;
        // D s_132_4: read-var op0:i
        let s_132_4: i128 = fn_state.op0;
        // D s_132_5: call integer_subrange(s_132_4, s_132_2, s_132_3)
        let s_132_5: Bits = integer_subrange(state, tracer, s_132_4, s_132_2, s_132_3);
        // D s_132_6: cast reint s_132_5 -> u8
        let s_132_6: u8 = (s_132_5.value() as u8);
        // C s_132_7: const #2s : i
        let s_132_7: i128 = 2;
        // C s_132_8: const #0s : i
        let s_132_8: i128 = 0;
        // D s_132_9: read-var op1:i
        let s_132_9: i128 = fn_state.op1;
        // D s_132_10: call integer_subrange(s_132_9, s_132_7, s_132_8)
        let s_132_10: Bits = integer_subrange(state, tracer, s_132_9, s_132_7, s_132_8);
        // D s_132_11: cast reint s_132_10 -> u8
        let s_132_11: u8 = (s_132_10.value() as u8);
        // C s_132_12: const #3s : i
        let s_132_12: i128 = 3;
        // C s_132_13: const #0s : i
        let s_132_13: i128 = 0;
        // D s_132_14: read-var crn:i
        let s_132_14: i128 = fn_state.crn;
        // D s_132_15: call integer_subrange(s_132_14, s_132_12, s_132_13)
        let s_132_15: Bits = integer_subrange(
            state,
            tracer,
            s_132_14,
            s_132_12,
            s_132_13,
        );
        // D s_132_16: cast reint s_132_15 -> u8
        let s_132_16: u8 = (s_132_15.value() as u8);
        // C s_132_17: const #2s : i
        let s_132_17: i128 = 2;
        // C s_132_18: const #0s : i
        let s_132_18: i128 = 0;
        // D s_132_19: read-var op2:i
        let s_132_19: i128 = fn_state.op2;
        // D s_132_20: call integer_subrange(s_132_19, s_132_17, s_132_18)
        let s_132_20: Bits = integer_subrange(
            state,
            tracer,
            s_132_19,
            s_132_17,
            s_132_18,
        );
        // D s_132_21: cast reint s_132_20 -> u8
        let s_132_21: u8 = (s_132_20.value() as u8);
        // C s_132_22: const #3s : i
        let s_132_22: i128 = 3;
        // C s_132_23: const #0s : i
        let s_132_23: i128 = 0;
        // D s_132_24: read-var crm:i
        let s_132_24: i128 = fn_state.crm;
        // D s_132_25: call integer_subrange(s_132_24, s_132_22, s_132_23)
        let s_132_25: Bits = integer_subrange(
            state,
            tracer,
            s_132_24,
            s_132_22,
            s_132_23,
        );
        // D s_132_26: cast reint s_132_25 -> u8
        let s_132_26: u8 = (s_132_25.value() as u8);
        // D s_132_27: read-var t:i
        let s_132_27: i128 = fn_state.t;
        // D s_132_28: call AArch64_AutoGen_SysRegWrite(s_132_1, s_132_6, s_132_11, s_132_16, s_132_21, s_132_26, s_132_27)
        let s_132_28: () = AArch64_AutoGen_SysRegWrite(
            state,
            tracer,
            s_132_1,
            s_132_6,
            s_132_11,
            s_132_16,
            s_132_21,
            s_132_26,
            s_132_27,
        );
        // N s_132_29: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var restore_xt:u8
        let s_133_0: bool = fn_state.restore_xt;
        // N s_133_1: branch s_133_0 b495 b134
        if s_133_0 {
            return block_495(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_134_0: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #2s : i
        let s_135_0: i128 = 2;
        // D s_135_1: read-var op0:i
        let s_135_1: i128 = fn_state.op0;
        // D s_135_2: cmp-eq s_135_1 s_135_0
        let s_135_2: bool = ((s_135_1) == (s_135_0));
        // N s_135_3: branch s_135_2 b494 b136
        if s_135_2 {
            return block_494(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#141388 <= s_136_0
        fn_state.gs_141388 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#141388:u8
        let s_137_0: bool = fn_state.gs_141388;
        // N s_137_1: branch s_137_0 b493 b138
        if s_137_0 {
            return block_493(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#141390 <= s_138_0
        fn_state.gs_141390 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#141390:u8
        let s_139_0: bool = fn_state.gs_141390;
        // N s_139_1: branch s_139_0 b492 b140
        if s_139_0 {
            return block_492(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#141392 <= s_140_0
        fn_state.gs_141392 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#141392:u8
        let s_141_0: bool = fn_state.gs_141392;
        // N s_141_1: branch s_141_0 b491 b142
        if s_141_0 {
            return block_491(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#141394 <= s_142_0
        fn_state.gs_141394 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#141394:u8
        let s_143_0: bool = fn_state.gs_141394;
        // N s_143_1: branch s_143_0 b490 b144
        if s_143_0 {
            return block_490(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_144_0: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0s : i
        let s_145_0: i128 = 0;
        // D s_145_1: read-var crm:i
        let s_145_1: i128 = fn_state.crm;
        // D s_145_2: cmp-eq s_145_1 s_145_0
        let s_145_2: bool = ((s_145_1) == (s_145_0));
        // N s_145_3: branch s_145_2 b489 b146
        if s_145_2 {
            return block_489(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0u : u8
        let s_146_0: bool = false;
        // D s_146_1: write-var gs#141397 <= s_146_0
        fn_state.gs_141397 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#141397:u8
        let s_147_0: bool = fn_state.gs_141397;
        // N s_147_1: branch s_147_0 b488 b148
        if s_147_0 {
            return block_488(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#141399 <= s_148_0
        fn_state.gs_141399 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#141399:u8
        let s_149_0: bool = fn_state.gs_141399;
        // N s_149_1: branch s_149_0 b487 b150
        if s_149_0 {
            return block_487(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#141401 <= s_150_0
        fn_state.gs_141401 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#141401:u8
        let s_151_0: bool = fn_state.gs_141401;
        // N s_151_1: branch s_151_0 b486 b152
        if s_151_0 {
            return block_486(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#141403 <= s_152_0
        fn_state.gs_141403 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#141403:u8
        let s_153_0: bool = fn_state.gs_141403;
        // N s_153_1: branch s_153_0 b479 b154
        if s_153_0 {
            return block_479(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_154_0: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #2s : i
        let s_155_0: i128 = 2;
        // D s_155_1: read-var op0:i
        let s_155_1: i128 = fn_state.op0;
        // D s_155_2: cmp-eq s_155_1 s_155_0
        let s_155_2: bool = ((s_155_1) == (s_155_0));
        // N s_155_3: branch s_155_2 b478 b156
        if s_155_2 {
            return block_478(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#141406 <= s_156_0
        fn_state.gs_141406 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#141406:u8
        let s_157_0: bool = fn_state.gs_141406;
        // N s_157_1: branch s_157_0 b477 b158
        if s_157_0 {
            return block_477(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#141408 <= s_158_0
        fn_state.gs_141408 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#141408:u8
        let s_159_0: bool = fn_state.gs_141408;
        // N s_159_1: branch s_159_0 b476 b160
        if s_159_0 {
            return block_476(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #0u : u8
        let s_160_0: bool = false;
        // D s_160_1: write-var gs#141410 <= s_160_0
        fn_state.gs_141410 = s_160_0;
        // N s_160_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var gs#141410:u8
        let s_161_0: bool = fn_state.gs_141410;
        // N s_161_1: branch s_161_0 b471 b162
        if s_161_0 {
            return block_471(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_162_0: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #3s : i
        let s_163_0: i128 = 3;
        // D s_163_1: read-var op0:i
        let s_163_1: i128 = fn_state.op0;
        // D s_163_2: cmp-eq s_163_1 s_163_0
        let s_163_2: bool = ((s_163_1) == (s_163_0));
        // N s_163_3: branch s_163_2 b470 b164
        if s_163_2 {
            return block_470(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#141412 <= s_164_0
        fn_state.gs_141412 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#141412:u8
        let s_165_0: bool = fn_state.gs_141412;
        // N s_165_1: branch s_165_0 b463 b166
        if s_165_0 {
            return block_463(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #0u : u8
        let s_166_0: bool = false;
        // D s_166_1: write-var gs#141418 <= s_166_0
        fn_state.gs_141418 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#141418:u8
        let s_167_0: bool = fn_state.gs_141418;
        // N s_167_1: branch s_167_0 b462 b168
        if s_167_0 {
            return block_462(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #0u : u8
        let s_168_0: bool = false;
        // D s_168_1: write-var gs#141420 <= s_168_0
        fn_state.gs_141420 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#141420:u8
        let s_169_0: bool = fn_state.gs_141420;
        // N s_169_1: branch s_169_0 b461 b170
        if s_169_0 {
            return block_461(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // D s_170_1: write-var gs#141422 <= s_170_0
        fn_state.gs_141422 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#141422:u8
        let s_171_0: bool = fn_state.gs_141422;
        // N s_171_1: branch s_171_0 b460 b172
        if s_171_0 {
            return block_460(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #0u : u8
        let s_172_0: bool = false;
        // D s_172_1: write-var gs#141425 <= s_172_0
        fn_state.gs_141425 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#141425:u8
        let s_173_0: bool = fn_state.gs_141425;
        // N s_173_1: branch s_173_0 b459 b174
        if s_173_0 {
            return block_459(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_174_0: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #3s : i
        let s_175_0: i128 = 3;
        // D s_175_1: read-var op0:i
        let s_175_1: i128 = fn_state.op0;
        // D s_175_2: cmp-eq s_175_1 s_175_0
        let s_175_2: bool = ((s_175_1) == (s_175_0));
        // N s_175_3: branch s_175_2 b458 b176
        if s_175_2 {
            return block_458(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #0u : u8
        let s_176_0: bool = false;
        // D s_176_1: write-var gs#141428 <= s_176_0
        fn_state.gs_141428 = s_176_0;
        // N s_176_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var gs#141428:u8
        let s_177_0: bool = fn_state.gs_141428;
        // N s_177_1: branch s_177_0 b457 b178
        if s_177_0 {
            return block_457(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #0u : u8
        let s_178_0: bool = false;
        // D s_178_1: write-var gs#141430 <= s_178_0
        fn_state.gs_141430 = s_178_0;
        // N s_178_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var gs#141430:u8
        let s_179_0: bool = fn_state.gs_141430;
        // N s_179_1: branch s_179_0 b456 b180
        if s_179_0 {
            return block_456(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #0u : u8
        let s_180_0: bool = false;
        // D s_180_1: write-var gs#141432 <= s_180_0
        fn_state.gs_141432 = s_180_0;
        // N s_180_2: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var gs#141432:u8
        let s_181_0: bool = fn_state.gs_141432;
        // N s_181_1: branch s_181_0 b452 b182
        if s_181_0 {
            return block_452(state, tracer, fn_state);
        } else {
            return block_182(state, tracer, fn_state);
        };
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #0u : u8
        let s_182_0: bool = false;
        // D s_182_1: write-var gs#141436 <= s_182_0
        fn_state.gs_141436 = s_182_0;
        // N s_182_2: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var gs#141436:u8
        let s_183_0: bool = fn_state.gs_141436;
        // N s_183_1: branch s_183_0 b445 b184
        if s_183_0 {
            return block_445(state, tracer, fn_state);
        } else {
            return block_184(state, tracer, fn_state);
        };
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_184_0: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #3s : i
        let s_185_0: i128 = 3;
        // D s_185_1: read-var op0:i
        let s_185_1: i128 = fn_state.op0;
        // D s_185_2: cmp-eq s_185_1 s_185_0
        let s_185_2: bool = ((s_185_1) == (s_185_0));
        // N s_185_3: branch s_185_2 b444 b186
        if s_185_2 {
            return block_444(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #0u : u8
        let s_186_0: bool = false;
        // D s_186_1: write-var gs#141439 <= s_186_0
        fn_state.gs_141439 = s_186_0;
        // N s_186_2: jump b187
        return block_187(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_187_0: read-var gs#141439:u8
        let s_187_0: bool = fn_state.gs_141439;
        // N s_187_1: branch s_187_0 b443 b188
        if s_187_0 {
            return block_443(state, tracer, fn_state);
        } else {
            return block_188(state, tracer, fn_state);
        };
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #0u : u8
        let s_188_0: bool = false;
        // D s_188_1: write-var gs#141441 <= s_188_0
        fn_state.gs_141441 = s_188_0;
        // N s_188_2: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var gs#141441:u8
        let s_189_0: bool = fn_state.gs_141441;
        // N s_189_1: branch s_189_0 b439 b190
        if s_189_0 {
            return block_439(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #0u : u8
        let s_190_0: bool = false;
        // D s_190_1: write-var gs#141445 <= s_190_0
        fn_state.gs_141445 = s_190_0;
        // N s_190_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var gs#141445:u8
        let s_191_0: bool = fn_state.gs_141445;
        // N s_191_1: branch s_191_0 b438 b192
        if s_191_0 {
            return block_438(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #0u : u8
        let s_192_0: bool = false;
        // D s_192_1: write-var gs#141447 <= s_192_0
        fn_state.gs_141447 = s_192_0;
        // N s_192_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var gs#141447:u8
        let s_193_0: bool = fn_state.gs_141447;
        // N s_193_1: branch s_193_0 b431 b194
        if s_193_0 {
            return block_431(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_194_0: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #3s : i
        let s_195_0: i128 = 3;
        // D s_195_1: read-var op0:i
        let s_195_1: i128 = fn_state.op0;
        // D s_195_2: cmp-eq s_195_1 s_195_0
        let s_195_2: bool = ((s_195_1) == (s_195_0));
        // N s_195_3: branch s_195_2 b430 b196
        if s_195_2 {
            return block_430(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #0u : u8
        let s_196_0: bool = false;
        // D s_196_1: write-var gs#141450 <= s_196_0
        fn_state.gs_141450 = s_196_0;
        // N s_196_2: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var gs#141450:u8
        let s_197_0: bool = fn_state.gs_141450;
        // N s_197_1: branch s_197_0 b429 b198
        if s_197_0 {
            return block_429(state, tracer, fn_state);
        } else {
            return block_198(state, tracer, fn_state);
        };
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #0u : u8
        let s_198_0: bool = false;
        // D s_198_1: write-var gs#141452 <= s_198_0
        fn_state.gs_141452 = s_198_0;
        // N s_198_2: jump b199
        return block_199(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_199_0: read-var gs#141452:u8
        let s_199_0: bool = fn_state.gs_141452;
        // N s_199_1: branch s_199_0 b425 b200
        if s_199_0 {
            return block_425(state, tracer, fn_state);
        } else {
            return block_200(state, tracer, fn_state);
        };
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #0u : u8
        let s_200_0: bool = false;
        // D s_200_1: write-var gs#141456 <= s_200_0
        fn_state.gs_141456 = s_200_0;
        // N s_200_2: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_201_0: read-var gs#141456:u8
        let s_201_0: bool = fn_state.gs_141456;
        // N s_201_1: branch s_201_0 b424 b202
        if s_201_0 {
            return block_424(state, tracer, fn_state);
        } else {
            return block_202(state, tracer, fn_state);
        };
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #0u : u8
        let s_202_0: bool = false;
        // D s_202_1: write-var gs#141458 <= s_202_0
        fn_state.gs_141458 = s_202_0;
        // N s_202_2: jump b203
        return block_203(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var gs#141458:u8
        let s_203_0: bool = fn_state.gs_141458;
        // N s_203_1: branch s_203_0 b417 b204
        if s_203_0 {
            return block_417(state, tracer, fn_state);
        } else {
            return block_204(state, tracer, fn_state);
        };
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_204_0: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #3s : i
        let s_205_0: i128 = 3;
        // D s_205_1: read-var op0:i
        let s_205_1: i128 = fn_state.op0;
        // D s_205_2: cmp-eq s_205_1 s_205_0
        let s_205_2: bool = ((s_205_1) == (s_205_0));
        // N s_205_3: branch s_205_2 b416 b206
        if s_205_2 {
            return block_416(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #0u : u8
        let s_206_0: bool = false;
        // D s_206_1: write-var gs#141461 <= s_206_0
        fn_state.gs_141461 = s_206_0;
        // N s_206_2: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var gs#141461:u8
        let s_207_0: bool = fn_state.gs_141461;
        // N s_207_1: branch s_207_0 b415 b208
        if s_207_0 {
            return block_415(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #0u : u8
        let s_208_0: bool = false;
        // D s_208_1: write-var gs#141463 <= s_208_0
        fn_state.gs_141463 = s_208_0;
        // N s_208_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var gs#141463:u8
        let s_209_0: bool = fn_state.gs_141463;
        // N s_209_1: branch s_209_0 b414 b210
        if s_209_0 {
            return block_414(state, tracer, fn_state);
        } else {
            return block_210(state, tracer, fn_state);
        };
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #0u : u8
        let s_210_0: bool = false;
        // D s_210_1: write-var gs#141465 <= s_210_0
        fn_state.gs_141465 = s_210_0;
        // N s_210_2: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var gs#141465:u8
        let s_211_0: bool = fn_state.gs_141465;
        // N s_211_1: branch s_211_0 b413 b212
        if s_211_0 {
            return block_413(state, tracer, fn_state);
        } else {
            return block_212(state, tracer, fn_state);
        };
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #0u : u8
        let s_212_0: bool = false;
        // D s_212_1: write-var gs#141467 <= s_212_0
        fn_state.gs_141467 = s_212_0;
        // N s_212_2: jump b213
        return block_213(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_213_0: read-var gs#141467:u8
        let s_213_0: bool = fn_state.gs_141467;
        // N s_213_1: branch s_213_0 b406 b214
        if s_213_0 {
            return block_406(state, tracer, fn_state);
        } else {
            return block_214(state, tracer, fn_state);
        };
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_214_0: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #3s : i
        let s_215_0: i128 = 3;
        // D s_215_1: read-var op0:i
        let s_215_1: i128 = fn_state.op0;
        // D s_215_2: cmp-eq s_215_1 s_215_0
        let s_215_2: bool = ((s_215_1) == (s_215_0));
        // N s_215_3: branch s_215_2 b405 b216
        if s_215_2 {
            return block_405(state, tracer, fn_state);
        } else {
            return block_216(state, tracer, fn_state);
        };
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #0u : u8
        let s_216_0: bool = false;
        // D s_216_1: write-var gs#141470 <= s_216_0
        fn_state.gs_141470 = s_216_0;
        // N s_216_2: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var gs#141470:u8
        let s_217_0: bool = fn_state.gs_141470;
        // N s_217_1: branch s_217_0 b404 b218
        if s_217_0 {
            return block_404(state, tracer, fn_state);
        } else {
            return block_218(state, tracer, fn_state);
        };
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #0u : u8
        let s_218_0: bool = false;
        // D s_218_1: write-var gs#141472 <= s_218_0
        fn_state.gs_141472 = s_218_0;
        // N s_218_2: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_219_0: read-var gs#141472:u8
        let s_219_0: bool = fn_state.gs_141472;
        // N s_219_1: branch s_219_0 b403 b220
        if s_219_0 {
            return block_403(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #0u : u8
        let s_220_0: bool = false;
        // D s_220_1: write-var gs#141474 <= s_220_0
        fn_state.gs_141474 = s_220_0;
        // N s_220_2: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_221_0: read-var gs#141474:u8
        let s_221_0: bool = fn_state.gs_141474;
        // N s_221_1: branch s_221_0 b399 b222
        if s_221_0 {
            return block_399(state, tracer, fn_state);
        } else {
            return block_222(state, tracer, fn_state);
        };
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #0u : u8
        let s_222_0: bool = false;
        // D s_222_1: write-var gs#141478 <= s_222_0
        fn_state.gs_141478 = s_222_0;
        // N s_222_2: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var gs#141478:u8
        let s_223_0: bool = fn_state.gs_141478;
        // N s_223_1: branch s_223_0 b390 b224
        if s_223_0 {
            return block_390(state, tracer, fn_state);
        } else {
            return block_224(state, tracer, fn_state);
        };
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_224_0: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #3s : i
        let s_225_0: i128 = 3;
        // D s_225_1: read-var op0:i
        let s_225_1: i128 = fn_state.op0;
        // D s_225_2: cmp-eq s_225_1 s_225_0
        let s_225_2: bool = ((s_225_1) == (s_225_0));
        // N s_225_3: branch s_225_2 b389 b226
        if s_225_2 {
            return block_389(state, tracer, fn_state);
        } else {
            return block_226(state, tracer, fn_state);
        };
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #0u : u8
        let s_226_0: bool = false;
        // D s_226_1: write-var gs#141481 <= s_226_0
        fn_state.gs_141481 = s_226_0;
        // N s_226_2: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var gs#141481:u8
        let s_227_0: bool = fn_state.gs_141481;
        // N s_227_1: branch s_227_0 b388 b228
        if s_227_0 {
            return block_388(state, tracer, fn_state);
        } else {
            return block_228(state, tracer, fn_state);
        };
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #0u : u8
        let s_228_0: bool = false;
        // D s_228_1: write-var gs#141483 <= s_228_0
        fn_state.gs_141483 = s_228_0;
        // N s_228_2: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_229_0: read-var gs#141483:u8
        let s_229_0: bool = fn_state.gs_141483;
        // N s_229_1: branch s_229_0 b387 b230
        if s_229_0 {
            return block_387(state, tracer, fn_state);
        } else {
            return block_230(state, tracer, fn_state);
        };
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_230_0: const #0u : u8
        let s_230_0: bool = false;
        // D s_230_1: write-var gs#141486 <= s_230_0
        fn_state.gs_141486 = s_230_0;
        // N s_230_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_231_0: read-var gs#141486:u8
        let s_231_0: bool = fn_state.gs_141486;
        // N s_231_1: branch s_231_0 b383 b232
        if s_231_0 {
            return block_383(state, tracer, fn_state);
        } else {
            return block_232(state, tracer, fn_state);
        };
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_232_0: jump b233
        return block_233(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #3s : i
        let s_233_0: i128 = 3;
        // D s_233_1: read-var op0:i
        let s_233_1: i128 = fn_state.op0;
        // D s_233_2: cmp-eq s_233_1 s_233_0
        let s_233_2: bool = ((s_233_1) == (s_233_0));
        // N s_233_3: branch s_233_2 b382 b234
        if s_233_2 {
            return block_382(state, tracer, fn_state);
        } else {
            return block_234(state, tracer, fn_state);
        };
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #0u : u8
        let s_234_0: bool = false;
        // D s_234_1: write-var gs#141489 <= s_234_0
        fn_state.gs_141489 = s_234_0;
        // N s_234_2: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_235_0: read-var gs#141489:u8
        let s_235_0: bool = fn_state.gs_141489;
        // N s_235_1: branch s_235_0 b381 b236
        if s_235_0 {
            return block_381(state, tracer, fn_state);
        } else {
            return block_236(state, tracer, fn_state);
        };
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_236_0: const #0u : u8
        let s_236_0: bool = false;
        // D s_236_1: write-var gs#141491 <= s_236_0
        fn_state.gs_141491 = s_236_0;
        // N s_236_2: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_237_0: read-var gs#141491:u8
        let s_237_0: bool = fn_state.gs_141491;
        // N s_237_1: branch s_237_0 b380 b238
        if s_237_0 {
            return block_380(state, tracer, fn_state);
        } else {
            return block_238(state, tracer, fn_state);
        };
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #0u : u8
        let s_238_0: bool = false;
        // D s_238_1: write-var gs#141493 <= s_238_0
        fn_state.gs_141493 = s_238_0;
        // N s_238_2: jump b239
        return block_239(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_239_0: read-var gs#141493:u8
        let s_239_0: bool = fn_state.gs_141493;
        // N s_239_1: branch s_239_0 b379 b240
        if s_239_0 {
            return block_379(state, tracer, fn_state);
        } else {
            return block_240(state, tracer, fn_state);
        };
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_240_0: const #0u : u8
        let s_240_0: bool = false;
        // D s_240_1: write-var gs#141495 <= s_240_0
        fn_state.gs_141495 = s_240_0;
        // N s_240_2: jump b241
        return block_241(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_241_0: read-var gs#141495:u8
        let s_241_0: bool = fn_state.gs_141495;
        // N s_241_1: branch s_241_0 b378 b242
        if s_241_0 {
            return block_378(state, tracer, fn_state);
        } else {
            return block_242(state, tracer, fn_state);
        };
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_242_0: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #2s : i
        let s_243_0: i128 = 2;
        // D s_243_1: read-var op0:i
        let s_243_1: i128 = fn_state.op0;
        // D s_243_2: cmp-eq s_243_1 s_243_0
        let s_243_2: bool = ((s_243_1) == (s_243_0));
        // N s_243_3: branch s_243_2 b377 b244
        if s_243_2 {
            return block_377(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #0u : u8
        let s_244_0: bool = false;
        // D s_244_1: write-var gs#141498 <= s_244_0
        fn_state.gs_141498 = s_244_0;
        // N s_244_2: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_245_0: read-var gs#141498:u8
        let s_245_0: bool = fn_state.gs_141498;
        // N s_245_1: branch s_245_0 b376 b246
        if s_245_0 {
            return block_376(state, tracer, fn_state);
        } else {
            return block_246(state, tracer, fn_state);
        };
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #0u : u8
        let s_246_0: bool = false;
        // D s_246_1: write-var gs#141500 <= s_246_0
        fn_state.gs_141500 = s_246_0;
        // N s_246_2: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_247_0: read-var gs#141500:u8
        let s_247_0: bool = fn_state.gs_141500;
        // N s_247_1: branch s_247_0 b375 b248
        if s_247_0 {
            return block_375(state, tracer, fn_state);
        } else {
            return block_248(state, tracer, fn_state);
        };
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_248_0: const #0u : u8
        let s_248_0: bool = false;
        // D s_248_1: write-var gs#141502 <= s_248_0
        fn_state.gs_141502 = s_248_0;
        // N s_248_2: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_249_0: read-var gs#141502:u8
        let s_249_0: bool = fn_state.gs_141502;
        // N s_249_1: branch s_249_0 b374 b250
        if s_249_0 {
            return block_374(state, tracer, fn_state);
        } else {
            return block_250(state, tracer, fn_state);
        };
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_250_0: const #0u : u8
        let s_250_0: bool = false;
        // D s_250_1: write-var gs#141504 <= s_250_0
        fn_state.gs_141504 = s_250_0;
        // N s_250_2: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_251_0: read-var gs#141504:u8
        let s_251_0: bool = fn_state.gs_141504;
        // N s_251_1: branch s_251_0 b373 b252
        if s_251_0 {
            return block_373(state, tracer, fn_state);
        } else {
            return block_252(state, tracer, fn_state);
        };
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_252_0: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #2s : i
        let s_253_0: i128 = 2;
        // D s_253_1: read-var op0:i
        let s_253_1: i128 = fn_state.op0;
        // D s_253_2: cmp-eq s_253_1 s_253_0
        let s_253_2: bool = ((s_253_1) == (s_253_0));
        // N s_253_3: branch s_253_2 b372 b254
        if s_253_2 {
            return block_372(state, tracer, fn_state);
        } else {
            return block_254(state, tracer, fn_state);
        };
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_254_0: const #0u : u8
        let s_254_0: bool = false;
        // D s_254_1: write-var gs#141507 <= s_254_0
        fn_state.gs_141507 = s_254_0;
        // N s_254_2: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_255_0: read-var gs#141507:u8
        let s_255_0: bool = fn_state.gs_141507;
        // N s_255_1: branch s_255_0 b371 b256
        if s_255_0 {
            return block_371(state, tracer, fn_state);
        } else {
            return block_256(state, tracer, fn_state);
        };
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #0u : u8
        let s_256_0: bool = false;
        // D s_256_1: write-var gs#141509 <= s_256_0
        fn_state.gs_141509 = s_256_0;
        // N s_256_2: jump b257
        return block_257(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_257_0: read-var gs#141509:u8
        let s_257_0: bool = fn_state.gs_141509;
        // N s_257_1: branch s_257_0 b370 b258
        if s_257_0 {
            return block_370(state, tracer, fn_state);
        } else {
            return block_258(state, tracer, fn_state);
        };
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_258_0: const #0u : u8
        let s_258_0: bool = false;
        // D s_258_1: write-var gs#141511 <= s_258_0
        fn_state.gs_141511 = s_258_0;
        // N s_258_2: jump b259
        return block_259(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_259_0: read-var gs#141511:u8
        let s_259_0: bool = fn_state.gs_141511;
        // N s_259_1: branch s_259_0 b369 b260
        if s_259_0 {
            return block_369(state, tracer, fn_state);
        } else {
            return block_260(state, tracer, fn_state);
        };
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_260_0: const #0u : u8
        let s_260_0: bool = false;
        // D s_260_1: write-var gs#141513 <= s_260_0
        fn_state.gs_141513 = s_260_0;
        // N s_260_2: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_261_0: read-var gs#141513:u8
        let s_261_0: bool = fn_state.gs_141513;
        // N s_261_1: branch s_261_0 b368 b262
        if s_261_0 {
            return block_368(state, tracer, fn_state);
        } else {
            return block_262(state, tracer, fn_state);
        };
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_262_0: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_263_0: const #3s : i
        let s_263_0: i128 = 3;
        // D s_263_1: read-var op0:i
        let s_263_1: i128 = fn_state.op0;
        // D s_263_2: cmp-eq s_263_1 s_263_0
        let s_263_2: bool = ((s_263_1) == (s_263_0));
        // N s_263_3: branch s_263_2 b367 b264
        if s_263_2 {
            return block_367(state, tracer, fn_state);
        } else {
            return block_264(state, tracer, fn_state);
        };
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_264_0: const #0u : u8
        let s_264_0: bool = false;
        // D s_264_1: write-var gs#141516 <= s_264_0
        fn_state.gs_141516 = s_264_0;
        // N s_264_2: jump b265
        return block_265(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_265_0: read-var gs#141516:u8
        let s_265_0: bool = fn_state.gs_141516;
        // N s_265_1: branch s_265_0 b366 b266
        if s_265_0 {
            return block_366(state, tracer, fn_state);
        } else {
            return block_266(state, tracer, fn_state);
        };
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_266_0: const #0u : u8
        let s_266_0: bool = false;
        // D s_266_1: write-var gs#141518 <= s_266_0
        fn_state.gs_141518 = s_266_0;
        // N s_266_2: jump b267
        return block_267(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_267_0: read-var gs#141518:u8
        let s_267_0: bool = fn_state.gs_141518;
        // N s_267_1: branch s_267_0 b365 b268
        if s_267_0 {
            return block_365(state, tracer, fn_state);
        } else {
            return block_268(state, tracer, fn_state);
        };
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_268_0: const #0u : u8
        let s_268_0: bool = false;
        // D s_268_1: write-var gs#141520 <= s_268_0
        fn_state.gs_141520 = s_268_0;
        // N s_268_2: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_269_0: read-var gs#141520:u8
        let s_269_0: bool = fn_state.gs_141520;
        // N s_269_1: branch s_269_0 b364 b270
        if s_269_0 {
            return block_364(state, tracer, fn_state);
        } else {
            return block_270(state, tracer, fn_state);
        };
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_270_0: const #0u : u8
        let s_270_0: bool = false;
        // D s_270_1: write-var gs#141522 <= s_270_0
        fn_state.gs_141522 = s_270_0;
        // N s_270_2: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_271_0: read-var gs#141522:u8
        let s_271_0: bool = fn_state.gs_141522;
        // N s_271_1: branch s_271_0 b363 b272
        if s_271_0 {
            return block_363(state, tracer, fn_state);
        } else {
            return block_272(state, tracer, fn_state);
        };
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_272_0: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #3s : i
        let s_273_0: i128 = 3;
        // D s_273_1: read-var op0:i
        let s_273_1: i128 = fn_state.op0;
        // D s_273_2: cmp-eq s_273_1 s_273_0
        let s_273_2: bool = ((s_273_1) == (s_273_0));
        // N s_273_3: branch s_273_2 b362 b274
        if s_273_2 {
            return block_362(state, tracer, fn_state);
        } else {
            return block_274(state, tracer, fn_state);
        };
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_274_0: const #0u : u8
        let s_274_0: bool = false;
        // D s_274_1: write-var gs#141525 <= s_274_0
        fn_state.gs_141525 = s_274_0;
        // N s_274_2: jump b275
        return block_275(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_275_0: read-var gs#141525:u8
        let s_275_0: bool = fn_state.gs_141525;
        // N s_275_1: branch s_275_0 b361 b276
        if s_275_0 {
            return block_361(state, tracer, fn_state);
        } else {
            return block_276(state, tracer, fn_state);
        };
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_276_0: const #0u : u8
        let s_276_0: bool = false;
        // D s_276_1: write-var gs#141527 <= s_276_0
        fn_state.gs_141527 = s_276_0;
        // N s_276_2: jump b277
        return block_277(state, tracer, fn_state);
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_277_0: read-var gs#141527:u8
        let s_277_0: bool = fn_state.gs_141527;
        // N s_277_1: branch s_277_0 b360 b278
        if s_277_0 {
            return block_360(state, tracer, fn_state);
        } else {
            return block_278(state, tracer, fn_state);
        };
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_278_0: const #0u : u8
        let s_278_0: bool = false;
        // D s_278_1: write-var gs#141529 <= s_278_0
        fn_state.gs_141529 = s_278_0;
        // N s_278_2: jump b279
        return block_279(state, tracer, fn_state);
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_279_0: read-var gs#141529:u8
        let s_279_0: bool = fn_state.gs_141529;
        // N s_279_1: branch s_279_0 b359 b280
        if s_279_0 {
            return block_359(state, tracer, fn_state);
        } else {
            return block_280(state, tracer, fn_state);
        };
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_280_0: const #0u : u8
        let s_280_0: bool = false;
        // D s_280_1: write-var gs#141531 <= s_280_0
        fn_state.gs_141531 = s_280_0;
        // N s_280_2: jump b281
        return block_281(state, tracer, fn_state);
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_281_0: read-var gs#141531:u8
        let s_281_0: bool = fn_state.gs_141531;
        // N s_281_1: branch s_281_0 b358 b282
        if s_281_0 {
            return block_358(state, tracer, fn_state);
        } else {
            return block_282(state, tracer, fn_state);
        };
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_282_0: jump b283
        return block_283(state, tracer, fn_state);
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_283_0: const #3s : i
        let s_283_0: i128 = 3;
        // D s_283_1: read-var op0:i
        let s_283_1: i128 = fn_state.op0;
        // D s_283_2: cmp-eq s_283_1 s_283_0
        let s_283_2: bool = ((s_283_1) == (s_283_0));
        // N s_283_3: branch s_283_2 b357 b284
        if s_283_2 {
            return block_357(state, tracer, fn_state);
        } else {
            return block_284(state, tracer, fn_state);
        };
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_284_0: const #0u : u8
        let s_284_0: bool = false;
        // D s_284_1: write-var gs#141534 <= s_284_0
        fn_state.gs_141534 = s_284_0;
        // N s_284_2: jump b285
        return block_285(state, tracer, fn_state);
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_285_0: read-var gs#141534:u8
        let s_285_0: bool = fn_state.gs_141534;
        // N s_285_1: branch s_285_0 b356 b286
        if s_285_0 {
            return block_356(state, tracer, fn_state);
        } else {
            return block_286(state, tracer, fn_state);
        };
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_286_0: const #0u : u8
        let s_286_0: bool = false;
        // D s_286_1: write-var gs#141536 <= s_286_0
        fn_state.gs_141536 = s_286_0;
        // N s_286_2: jump b287
        return block_287(state, tracer, fn_state);
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_287_0: read-var gs#141536:u8
        let s_287_0: bool = fn_state.gs_141536;
        // N s_287_1: branch s_287_0 b355 b288
        if s_287_0 {
            return block_355(state, tracer, fn_state);
        } else {
            return block_288(state, tracer, fn_state);
        };
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_288_0: const #0u : u8
        let s_288_0: bool = false;
        // D s_288_1: write-var gs#141538 <= s_288_0
        fn_state.gs_141538 = s_288_0;
        // N s_288_2: jump b289
        return block_289(state, tracer, fn_state);
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_289_0: read-var gs#141538:u8
        let s_289_0: bool = fn_state.gs_141538;
        // N s_289_1: branch s_289_0 b354 b290
        if s_289_0 {
            return block_354(state, tracer, fn_state);
        } else {
            return block_290(state, tracer, fn_state);
        };
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_290_0: const #0u : u8
        let s_290_0: bool = false;
        // D s_290_1: write-var gs#141540 <= s_290_0
        fn_state.gs_141540 = s_290_0;
        // N s_290_2: jump b291
        return block_291(state, tracer, fn_state);
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_291_0: read-var gs#141540:u8
        let s_291_0: bool = fn_state.gs_141540;
        // N s_291_1: branch s_291_0 b353 b292
        if s_291_0 {
            return block_353(state, tracer, fn_state);
        } else {
            return block_292(state, tracer, fn_state);
        };
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_292_0: jump b293
        return block_293(state, tracer, fn_state);
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_293_0: const #3s : i
        let s_293_0: i128 = 3;
        // D s_293_1: read-var op0:i
        let s_293_1: i128 = fn_state.op0;
        // D s_293_2: cmp-eq s_293_1 s_293_0
        let s_293_2: bool = ((s_293_1) == (s_293_0));
        // N s_293_3: branch s_293_2 b352 b294
        if s_293_2 {
            return block_352(state, tracer, fn_state);
        } else {
            return block_294(state, tracer, fn_state);
        };
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_294_0: const #0u : u8
        let s_294_0: bool = false;
        // D s_294_1: write-var gs#141543 <= s_294_0
        fn_state.gs_141543 = s_294_0;
        // N s_294_2: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_295_0: read-var gs#141543:u8
        let s_295_0: bool = fn_state.gs_141543;
        // N s_295_1: branch s_295_0 b351 b296
        if s_295_0 {
            return block_351(state, tracer, fn_state);
        } else {
            return block_296(state, tracer, fn_state);
        };
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_296_0: const #0u : u8
        let s_296_0: bool = false;
        // D s_296_1: write-var gs#141545 <= s_296_0
        fn_state.gs_141545 = s_296_0;
        // N s_296_2: jump b297
        return block_297(state, tracer, fn_state);
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_297_0: read-var gs#141545:u8
        let s_297_0: bool = fn_state.gs_141545;
        // N s_297_1: branch s_297_0 b350 b298
        if s_297_0 {
            return block_350(state, tracer, fn_state);
        } else {
            return block_298(state, tracer, fn_state);
        };
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_298_0: const #0u : u8
        let s_298_0: bool = false;
        // D s_298_1: write-var gs#141547 <= s_298_0
        fn_state.gs_141547 = s_298_0;
        // N s_298_2: jump b299
        return block_299(state, tracer, fn_state);
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_299_0: read-var gs#141547:u8
        let s_299_0: bool = fn_state.gs_141547;
        // N s_299_1: branch s_299_0 b349 b300
        if s_299_0 {
            return block_349(state, tracer, fn_state);
        } else {
            return block_300(state, tracer, fn_state);
        };
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_300_0: const #0u : u8
        let s_300_0: bool = false;
        // D s_300_1: write-var gs#141549 <= s_300_0
        fn_state.gs_141549 = s_300_0;
        // N s_300_2: jump b301
        return block_301(state, tracer, fn_state);
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_301_0: read-var gs#141549:u8
        let s_301_0: bool = fn_state.gs_141549;
        // N s_301_1: branch s_301_0 b348 b302
        if s_301_0 {
            return block_348(state, tracer, fn_state);
        } else {
            return block_302(state, tracer, fn_state);
        };
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_302_0: jump b303
        return block_303(state, tracer, fn_state);
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_303_0: const #3s : i
        let s_303_0: i128 = 3;
        // D s_303_1: read-var op0:i
        let s_303_1: i128 = fn_state.op0;
        // D s_303_2: cmp-eq s_303_1 s_303_0
        let s_303_2: bool = ((s_303_1) == (s_303_0));
        // N s_303_3: branch s_303_2 b347 b304
        if s_303_2 {
            return block_347(state, tracer, fn_state);
        } else {
            return block_304(state, tracer, fn_state);
        };
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_304_0: const #0u : u8
        let s_304_0: bool = false;
        // D s_304_1: write-var gs#141552 <= s_304_0
        fn_state.gs_141552 = s_304_0;
        // N s_304_2: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_305_0: read-var gs#141552:u8
        let s_305_0: bool = fn_state.gs_141552;
        // N s_305_1: branch s_305_0 b346 b306
        if s_305_0 {
            return block_346(state, tracer, fn_state);
        } else {
            return block_306(state, tracer, fn_state);
        };
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_306_0: const #0u : u8
        let s_306_0: bool = false;
        // D s_306_1: write-var gs#141554 <= s_306_0
        fn_state.gs_141554 = s_306_0;
        // N s_306_2: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_307_0: read-var gs#141554:u8
        let s_307_0: bool = fn_state.gs_141554;
        // N s_307_1: branch s_307_0 b345 b308
        if s_307_0 {
            return block_345(state, tracer, fn_state);
        } else {
            return block_308(state, tracer, fn_state);
        };
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_308_0: const #0u : u8
        let s_308_0: bool = false;
        // D s_308_1: write-var gs#141556 <= s_308_0
        fn_state.gs_141556 = s_308_0;
        // N s_308_2: jump b309
        return block_309(state, tracer, fn_state);
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_309_0: read-var gs#141556:u8
        let s_309_0: bool = fn_state.gs_141556;
        // N s_309_1: branch s_309_0 b344 b310
        if s_309_0 {
            return block_344(state, tracer, fn_state);
        } else {
            return block_310(state, tracer, fn_state);
        };
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_310_0: const #0u : u8
        let s_310_0: bool = false;
        // D s_310_1: write-var gs#141558 <= s_310_0
        fn_state.gs_141558 = s_310_0;
        // N s_310_2: jump b311
        return block_311(state, tracer, fn_state);
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_311_0: read-var gs#141558:u8
        let s_311_0: bool = fn_state.gs_141558;
        // N s_311_1: branch s_311_0 b343 b312
        if s_311_0 {
            return block_343(state, tracer, fn_state);
        } else {
            return block_312(state, tracer, fn_state);
        };
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_312_0: jump b313
        return block_313(state, tracer, fn_state);
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_313_0: const #3s : i
        let s_313_0: i128 = 3;
        // D s_313_1: read-var op0:i
        let s_313_1: i128 = fn_state.op0;
        // D s_313_2: cmp-eq s_313_1 s_313_0
        let s_313_2: bool = ((s_313_1) == (s_313_0));
        // N s_313_3: branch s_313_2 b342 b314
        if s_313_2 {
            return block_342(state, tracer, fn_state);
        } else {
            return block_314(state, tracer, fn_state);
        };
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_314_0: const #0u : u8
        let s_314_0: bool = false;
        // D s_314_1: write-var gs#141561 <= s_314_0
        fn_state.gs_141561 = s_314_0;
        // N s_314_2: jump b315
        return block_315(state, tracer, fn_state);
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_315_0: read-var gs#141561:u8
        let s_315_0: bool = fn_state.gs_141561;
        // N s_315_1: branch s_315_0 b341 b316
        if s_315_0 {
            return block_341(state, tracer, fn_state);
        } else {
            return block_316(state, tracer, fn_state);
        };
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_316_0: const #0u : u8
        let s_316_0: bool = false;
        // D s_316_1: write-var gs#141563 <= s_316_0
        fn_state.gs_141563 = s_316_0;
        // N s_316_2: jump b317
        return block_317(state, tracer, fn_state);
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_317_0: read-var gs#141563:u8
        let s_317_0: bool = fn_state.gs_141563;
        // N s_317_1: branch s_317_0 b340 b318
        if s_317_0 {
            return block_340(state, tracer, fn_state);
        } else {
            return block_318(state, tracer, fn_state);
        };
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_318_0: const #0u : u8
        let s_318_0: bool = false;
        // D s_318_1: write-var gs#141565 <= s_318_0
        fn_state.gs_141565 = s_318_0;
        // N s_318_2: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_319_0: read-var gs#141565:u8
        let s_319_0: bool = fn_state.gs_141565;
        // N s_319_1: branch s_319_0 b339 b320
        if s_319_0 {
            return block_339(state, tracer, fn_state);
        } else {
            return block_320(state, tracer, fn_state);
        };
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_320_0: const #0u : u8
        let s_320_0: bool = false;
        // D s_320_1: write-var gs#141567 <= s_320_0
        fn_state.gs_141567 = s_320_0;
        // N s_320_2: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_321_0: read-var gs#141567:u8
        let s_321_0: bool = fn_state.gs_141567;
        // N s_321_1: branch s_321_0 b338 b322
        if s_321_0 {
            return block_338(state, tracer, fn_state);
        } else {
            return block_322(state, tracer, fn_state);
        };
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_322_0: jump b323
        return block_323(state, tracer, fn_state);
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_323_0: const #3s : i
        let s_323_0: i128 = 3;
        // D s_323_1: read-var op0:i
        let s_323_1: i128 = fn_state.op0;
        // D s_323_2: cmp-eq s_323_1 s_323_0
        let s_323_2: bool = ((s_323_1) == (s_323_0));
        // N s_323_3: branch s_323_2 b337 b324
        if s_323_2 {
            return block_337(state, tracer, fn_state);
        } else {
            return block_324(state, tracer, fn_state);
        };
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_324_0: const #0u : u8
        let s_324_0: bool = false;
        // D s_324_1: write-var gs#141570 <= s_324_0
        fn_state.gs_141570 = s_324_0;
        // N s_324_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_325_0: read-var gs#141570:u8
        let s_325_0: bool = fn_state.gs_141570;
        // N s_325_1: branch s_325_0 b336 b326
        if s_325_0 {
            return block_336(state, tracer, fn_state);
        } else {
            return block_326(state, tracer, fn_state);
        };
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_326_0: const #0u : u8
        let s_326_0: bool = false;
        // D s_326_1: write-var gs#141572 <= s_326_0
        fn_state.gs_141572 = s_326_0;
        // N s_326_2: jump b327
        return block_327(state, tracer, fn_state);
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_327_0: read-var gs#141572:u8
        let s_327_0: bool = fn_state.gs_141572;
        // N s_327_1: branch s_327_0 b335 b328
        if s_327_0 {
            return block_335(state, tracer, fn_state);
        } else {
            return block_328(state, tracer, fn_state);
        };
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_328_0: const #0u : u8
        let s_328_0: bool = false;
        // D s_328_1: write-var gs#141574 <= s_328_0
        fn_state.gs_141574 = s_328_0;
        // N s_328_2: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_329_0: read-var gs#141574:u8
        let s_329_0: bool = fn_state.gs_141574;
        // N s_329_1: branch s_329_0 b334 b330
        if s_329_0 {
            return block_334(state, tracer, fn_state);
        } else {
            return block_330(state, tracer, fn_state);
        };
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_330_0: const #0u : u8
        let s_330_0: bool = false;
        // D s_330_1: write-var gs#141576 <= s_330_0
        fn_state.gs_141576 = s_330_0;
        // N s_330_2: jump b331
        return block_331(state, tracer, fn_state);
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_331_0: read-var gs#141576:u8
        let s_331_0: bool = fn_state.gs_141576;
        // N s_331_1: branch s_331_0 b333 b332
        if s_331_0 {
            return block_333(state, tracer, fn_state);
        } else {
            return block_332(state, tracer, fn_state);
        };
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_332_0: return
        return;
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_333_0: const #10600u : u32
        let s_333_0: u32 = 10600;
        // D s_333_1: read-reg s_333_0:struct
        let s_333_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_0 as isize);
            tracer.read_register(s_333_0 as isize, value);
            value
        };
        // D s_333_2: call _get_ICV_IGRPEN1_EL1_Type_Enable(s_333_1)
        let s_333_2: bool = u_get_ICV_IGRPEN1_EL1_Type_Enable(state, tracer, s_333_1);
        // C s_333_3: const #15376u : u32
        let s_333_3: u32 = 15376;
        // D s_333_4: read-reg s_333_3:struct
        let s_333_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_3 as isize);
            tracer.read_register(s_333_3 as isize, value);
            value
        };
        // C s_333_5: const #15376u : u32
        let s_333_5: u32 = 15376;
        // N s_333_6: write-reg s_333_5 <= s_333_4
        let s_333_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_5 as isize, s_333_4);
            tracer.write_register(s_333_5 as isize, s_333_4);
        };
        // N s_333_7: return
        return;
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_334_0: const #12s : i
        let s_334_0: i128 = 12;
        // D s_334_1: read-var crm:i
        let s_334_1: i128 = fn_state.crm;
        // D s_334_2: cmp-eq s_334_1 s_334_0
        let s_334_2: bool = ((s_334_1) == (s_334_0));
        // D s_334_3: write-var gs#141576 <= s_334_2
        fn_state.gs_141576 = s_334_2;
        // N s_334_4: jump b331
        return block_331(state, tracer, fn_state);
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_335_0: const #7s : i
        let s_335_0: i128 = 7;
        // D s_335_1: read-var op2:i
        let s_335_1: i128 = fn_state.op2;
        // D s_335_2: cmp-eq s_335_1 s_335_0
        let s_335_2: bool = ((s_335_1) == (s_335_0));
        // D s_335_3: write-var gs#141574 <= s_335_2
        fn_state.gs_141574 = s_335_2;
        // N s_335_4: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_336_0: const #0s : i
        let s_336_0: i128 = 0;
        // D s_336_1: read-var op1:i
        let s_336_1: i128 = fn_state.op1;
        // D s_336_2: cmp-eq s_336_1 s_336_0
        let s_336_2: bool = ((s_336_1) == (s_336_0));
        // D s_336_3: write-var gs#141572 <= s_336_2
        fn_state.gs_141572 = s_336_2;
        // N s_336_4: jump b327
        return block_327(state, tracer, fn_state);
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_337_0: const #12s : i
        let s_337_0: i128 = 12;
        // D s_337_1: read-var crn:i
        let s_337_1: i128 = fn_state.crn;
        // D s_337_2: cmp-eq s_337_1 s_337_0
        let s_337_2: bool = ((s_337_1) == (s_337_0));
        // D s_337_3: write-var gs#141570 <= s_337_2
        fn_state.gs_141570 = s_337_2;
        // N s_337_4: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_338_0: const #12048u : u32
        let s_338_0: u32 = 12048;
        // D s_338_1: read-reg s_338_0:struct
        let s_338_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_338_0 as isize);
            tracer.read_register(s_338_0 as isize, value);
            value
        };
        // D s_338_2: call _get_ICV_IGRPEN0_EL1_Type_Enable(s_338_1)
        let s_338_2: bool = u_get_ICV_IGRPEN0_EL1_Type_Enable(state, tracer, s_338_1);
        // C s_338_3: const #15376u : u32
        let s_338_3: u32 = 15376;
        // D s_338_4: read-reg s_338_3:struct
        let s_338_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_338_3 as isize);
            tracer.read_register(s_338_3 as isize, value);
            value
        };
        // C s_338_5: const #15376u : u32
        let s_338_5: u32 = 15376;
        // N s_338_6: write-reg s_338_5 <= s_338_4
        let s_338_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_338_5 as isize, s_338_4);
            tracer.write_register(s_338_5 as isize, s_338_4);
        };
        // N s_338_7: jump b323
        return block_323(state, tracer, fn_state);
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #12s : i
        let s_339_0: i128 = 12;
        // D s_339_1: read-var crm:i
        let s_339_1: i128 = fn_state.crm;
        // D s_339_2: cmp-eq s_339_1 s_339_0
        let s_339_2: bool = ((s_339_1) == (s_339_0));
        // D s_339_3: write-var gs#141567 <= s_339_2
        fn_state.gs_141567 = s_339_2;
        // N s_339_4: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_340_0: const #6s : i
        let s_340_0: i128 = 6;
        // D s_340_1: read-var op2:i
        let s_340_1: i128 = fn_state.op2;
        // D s_340_2: cmp-eq s_340_1 s_340_0
        let s_340_2: bool = ((s_340_1) == (s_340_0));
        // D s_340_3: write-var gs#141565 <= s_340_2
        fn_state.gs_141565 = s_340_2;
        // N s_340_4: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_341_0: const #0s : i
        let s_341_0: i128 = 0;
        // D s_341_1: read-var op1:i
        let s_341_1: i128 = fn_state.op1;
        // D s_341_2: cmp-eq s_341_1 s_341_0
        let s_341_2: bool = ((s_341_1) == (s_341_0));
        // D s_341_3: write-var gs#141563 <= s_341_2
        fn_state.gs_141563 = s_341_2;
        // N s_341_4: jump b317
        return block_317(state, tracer, fn_state);
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_342_0: const #12s : i
        let s_342_0: i128 = 12;
        // D s_342_1: read-var crn:i
        let s_342_1: i128 = fn_state.crn;
        // D s_342_2: cmp-eq s_342_1 s_342_0
        let s_342_2: bool = ((s_342_1) == (s_342_0));
        // D s_342_3: write-var gs#141561 <= s_342_2
        fn_state.gs_141561 = s_342_2;
        // N s_342_4: jump b315
        return block_315(state, tracer, fn_state);
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_343_0: const #12824u : u32
        let s_343_0: u32 = 12824;
        // D s_343_1: read-reg s_343_0:struct
        let s_343_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_343_0 as isize);
            tracer.read_register(s_343_0 as isize, value);
            value
        };
        // D s_343_2: call _get_ICV_CTLR_EL1_Type_EOImode(s_343_1)
        let s_343_2: bool = u_get_ICV_CTLR_EL1_Type_EOImode(state, tracer, s_343_1);
        // C s_343_3: const #15376u : u32
        let s_343_3: u32 = 15376;
        // D s_343_4: read-reg s_343_3:struct
        let s_343_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_343_3 as isize);
            tracer.read_register(s_343_3 as isize, value);
            value
        };
        // C s_343_5: const #15376u : u32
        let s_343_5: u32 = 15376;
        // N s_343_6: write-reg s_343_5 <= s_343_4
        let s_343_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_343_5 as isize, s_343_4);
            tracer.write_register(s_343_5 as isize, s_343_4);
        };
        // C s_343_7: const #12824u : u32
        let s_343_7: u32 = 12824;
        // D s_343_8: read-reg s_343_7:struct
        let s_343_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_343_7 as isize);
            tracer.read_register(s_343_7 as isize, value);
            value
        };
        // D s_343_9: call _get_ICV_CTLR_EL1_Type_CBPR(s_343_8)
        let s_343_9: bool = u_get_ICV_CTLR_EL1_Type_CBPR(state, tracer, s_343_8);
        // C s_343_10: const #15376u : u32
        let s_343_10: u32 = 15376;
        // D s_343_11: read-reg s_343_10:struct
        let s_343_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_343_10 as isize);
            tracer.read_register(s_343_10 as isize, value);
            value
        };
        // C s_343_12: const #15376u : u32
        let s_343_12: u32 = 15376;
        // N s_343_13: write-reg s_343_12 <= s_343_11
        let s_343_13: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_343_12 as isize, s_343_11);
            tracer.write_register(s_343_12 as isize, s_343_11);
        };
        // N s_343_14: jump b313
        return block_313(state, tracer, fn_state);
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_344_0: const #12s : i
        let s_344_0: i128 = 12;
        // D s_344_1: read-var crm:i
        let s_344_1: i128 = fn_state.crm;
        // D s_344_2: cmp-eq s_344_1 s_344_0
        let s_344_2: bool = ((s_344_1) == (s_344_0));
        // D s_344_3: write-var gs#141558 <= s_344_2
        fn_state.gs_141558 = s_344_2;
        // N s_344_4: jump b311
        return block_311(state, tracer, fn_state);
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_345_0: const #4s : i
        let s_345_0: i128 = 4;
        // D s_345_1: read-var op2:i
        let s_345_1: i128 = fn_state.op2;
        // D s_345_2: cmp-eq s_345_1 s_345_0
        let s_345_2: bool = ((s_345_1) == (s_345_0));
        // D s_345_3: write-var gs#141556 <= s_345_2
        fn_state.gs_141556 = s_345_2;
        // N s_345_4: jump b309
        return block_309(state, tracer, fn_state);
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_346_0: const #0s : i
        let s_346_0: i128 = 0;
        // D s_346_1: read-var op1:i
        let s_346_1: i128 = fn_state.op1;
        // D s_346_2: cmp-eq s_346_1 s_346_0
        let s_346_2: bool = ((s_346_1) == (s_346_0));
        // D s_346_3: write-var gs#141554 <= s_346_2
        fn_state.gs_141554 = s_346_2;
        // N s_346_4: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_347_0: const #12s : i
        let s_347_0: i128 = 12;
        // D s_347_1: read-var crn:i
        let s_347_1: i128 = fn_state.crn;
        // D s_347_2: cmp-eq s_347_1 s_347_0
        let s_347_2: bool = ((s_347_1) == (s_347_0));
        // D s_347_3: write-var gs#141552 <= s_347_2
        fn_state.gs_141552 = s_347_2;
        // N s_347_4: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_348_0: const #102456u : u32
        let s_348_0: u32 = 102456;
        // D s_348_1: read-reg s_348_0:struct
        let s_348_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_348_0 as isize);
            tracer.read_register(s_348_0 as isize, value);
            value
        };
        // D s_348_2: call _get_ICV_BPR1_EL1_Type_BinaryPoint(s_348_1)
        let s_348_2: u8 = u_get_ICV_BPR1_EL1_Type_BinaryPoint(state, tracer, s_348_1);
        // C s_348_3: const #15376u : u32
        let s_348_3: u32 = 15376;
        // D s_348_4: read-reg s_348_3:struct
        let s_348_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_348_3 as isize);
            tracer.read_register(s_348_3 as isize, value);
            value
        };
        // C s_348_5: const #15376u : u32
        let s_348_5: u32 = 15376;
        // N s_348_6: write-reg s_348_5 <= s_348_4
        let s_348_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_348_5 as isize, s_348_4);
            tracer.write_register(s_348_5 as isize, s_348_4);
        };
        // N s_348_7: jump b303
        return block_303(state, tracer, fn_state);
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_349_0: const #12s : i
        let s_349_0: i128 = 12;
        // D s_349_1: read-var crm:i
        let s_349_1: i128 = fn_state.crm;
        // D s_349_2: cmp-eq s_349_1 s_349_0
        let s_349_2: bool = ((s_349_1) == (s_349_0));
        // D s_349_3: write-var gs#141549 <= s_349_2
        fn_state.gs_141549 = s_349_2;
        // N s_349_4: jump b301
        return block_301(state, tracer, fn_state);
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_350_0: const #3s : i
        let s_350_0: i128 = 3;
        // D s_350_1: read-var op2:i
        let s_350_1: i128 = fn_state.op2;
        // D s_350_2: cmp-eq s_350_1 s_350_0
        let s_350_2: bool = ((s_350_1) == (s_350_0));
        // D s_350_3: write-var gs#141547 <= s_350_2
        fn_state.gs_141547 = s_350_2;
        // N s_350_4: jump b299
        return block_299(state, tracer, fn_state);
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_351_0: const #0s : i
        let s_351_0: i128 = 0;
        // D s_351_1: read-var op1:i
        let s_351_1: i128 = fn_state.op1;
        // D s_351_2: cmp-eq s_351_1 s_351_0
        let s_351_2: bool = ((s_351_1) == (s_351_0));
        // D s_351_3: write-var gs#141545 <= s_351_2
        fn_state.gs_141545 = s_351_2;
        // N s_351_4: jump b297
        return block_297(state, tracer, fn_state);
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_352_0: const #12s : i
        let s_352_0: i128 = 12;
        // D s_352_1: read-var crn:i
        let s_352_1: i128 = fn_state.crn;
        // D s_352_2: cmp-eq s_352_1 s_352_0
        let s_352_2: bool = ((s_352_1) == (s_352_0));
        // D s_352_3: write-var gs#141543 <= s_352_2
        fn_state.gs_141543 = s_352_2;
        // N s_352_4: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_353_0: const #90344u : u32
        let s_353_0: u32 = 90344;
        // D s_353_1: read-reg s_353_0:struct
        let s_353_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_353_0 as isize);
            tracer.read_register(s_353_0 as isize, value);
            value
        };
        // D s_353_2: call _get_ICV_BPR0_EL1_Type_BinaryPoint(s_353_1)
        let s_353_2: u8 = u_get_ICV_BPR0_EL1_Type_BinaryPoint(state, tracer, s_353_1);
        // C s_353_3: const #15376u : u32
        let s_353_3: u32 = 15376;
        // D s_353_4: read-reg s_353_3:struct
        let s_353_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_353_3 as isize);
            tracer.read_register(s_353_3 as isize, value);
            value
        };
        // C s_353_5: const #15376u : u32
        let s_353_5: u32 = 15376;
        // N s_353_6: write-reg s_353_5 <= s_353_4
        let s_353_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_353_5 as isize, s_353_4);
            tracer.write_register(s_353_5 as isize, s_353_4);
        };
        // N s_353_7: jump b293
        return block_293(state, tracer, fn_state);
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_354_0: const #8s : i
        let s_354_0: i128 = 8;
        // D s_354_1: read-var crm:i
        let s_354_1: i128 = fn_state.crm;
        // D s_354_2: cmp-eq s_354_1 s_354_0
        let s_354_2: bool = ((s_354_1) == (s_354_0));
        // D s_354_3: write-var gs#141540 <= s_354_2
        fn_state.gs_141540 = s_354_2;
        // N s_354_4: jump b291
        return block_291(state, tracer, fn_state);
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_355_0: const #3s : i
        let s_355_0: i128 = 3;
        // D s_355_1: read-var op2:i
        let s_355_1: i128 = fn_state.op2;
        // D s_355_2: cmp-eq s_355_1 s_355_0
        let s_355_2: bool = ((s_355_1) == (s_355_0));
        // D s_355_3: write-var gs#141538 <= s_355_2
        fn_state.gs_141538 = s_355_2;
        // N s_355_4: jump b289
        return block_289(state, tracer, fn_state);
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_356_0: const #0s : i
        let s_356_0: i128 = 0;
        // D s_356_1: read-var op1:i
        let s_356_1: i128 = fn_state.op1;
        // D s_356_2: cmp-eq s_356_1 s_356_0
        let s_356_2: bool = ((s_356_1) == (s_356_0));
        // D s_356_3: write-var gs#141536 <= s_356_2
        fn_state.gs_141536 = s_356_2;
        // N s_356_4: jump b287
        return block_287(state, tracer, fn_state);
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_357_0: const #12s : i
        let s_357_0: i128 = 12;
        // D s_357_1: read-var crn:i
        let s_357_1: i128 = fn_state.crn;
        // D s_357_2: cmp-eq s_357_1 s_357_0
        let s_357_2: bool = ((s_357_1) == (s_357_0));
        // D s_357_3: write-var gs#141534 <= s_357_2
        fn_state.gs_141534 = s_357_2;
        // N s_357_4: jump b285
        return block_285(state, tracer, fn_state);
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_358_0: const #15064u : u32
        let s_358_0: u32 = 15064;
        // D s_358_1: read-reg s_358_0:struct
        let s_358_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_358_0 as isize);
            tracer.read_register(s_358_0 as isize, value);
            value
        };
        // D s_358_2: call _get_ICV_PMR_EL1_Type_Priority(s_358_1)
        let s_358_2: u8 = u_get_ICV_PMR_EL1_Type_Priority(state, tracer, s_358_1);
        // C s_358_3: const #15376u : u32
        let s_358_3: u32 = 15376;
        // D s_358_4: read-reg s_358_3:struct
        let s_358_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_358_3 as isize);
            tracer.read_register(s_358_3 as isize, value);
            value
        };
        // C s_358_5: const #15376u : u32
        let s_358_5: u32 = 15376;
        // N s_358_6: write-reg s_358_5 <= s_358_4
        let s_358_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_358_5 as isize, s_358_4);
            tracer.write_register(s_358_5 as isize, s_358_4);
        };
        // N s_358_7: jump b283
        return block_283(state, tracer, fn_state);
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_359_0: const #6s : i
        let s_359_0: i128 = 6;
        // D s_359_1: read-var crm:i
        let s_359_1: i128 = fn_state.crm;
        // D s_359_2: cmp-eq s_359_1 s_359_0
        let s_359_2: bool = ((s_359_1) == (s_359_0));
        // D s_359_3: write-var gs#141531 <= s_359_2
        fn_state.gs_141531 = s_359_2;
        // N s_359_4: jump b281
        return block_281(state, tracer, fn_state);
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_360_0: const #0s : i
        let s_360_0: i128 = 0;
        // D s_360_1: read-var op2:i
        let s_360_1: i128 = fn_state.op2;
        // D s_360_2: cmp-eq s_360_1 s_360_0
        let s_360_2: bool = ((s_360_1) == (s_360_0));
        // D s_360_3: write-var gs#141529 <= s_360_2
        fn_state.gs_141529 = s_360_2;
        // N s_360_4: jump b279
        return block_279(state, tracer, fn_state);
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_361_0: const #0s : i
        let s_361_0: i128 = 0;
        // D s_361_1: read-var op1:i
        let s_361_1: i128 = fn_state.op1;
        // D s_361_2: cmp-eq s_361_1 s_361_0
        let s_361_2: bool = ((s_361_1) == (s_361_0));
        // D s_361_3: write-var gs#141527 <= s_361_2
        fn_state.gs_141527 = s_361_2;
        // N s_361_4: jump b277
        return block_277(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_362_0: const #4s : i
        let s_362_0: i128 = 4;
        // D s_362_1: read-var crn:i
        let s_362_1: i128 = fn_state.crn;
        // D s_362_2: cmp-eq s_362_1 s_362_0
        let s_362_2: bool = ((s_362_1) == (s_362_0));
        // D s_362_3: write-var gs#141525 <= s_362_2
        fn_state.gs_141525 = s_362_2;
        // N s_362_4: jump b275
        return block_275(state, tracer, fn_state);
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_363_0: const #15376u : u32
        let s_363_0: u32 = 15376;
        // D s_363_1: read-reg s_363_0:struct
        let s_363_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_0 as isize);
            tracer.read_register(s_363_0 as isize, value);
            value
        };
        // D s_363_2: call _get_ICH_VMCR_EL2_Type_VPMR(s_363_1)
        let s_363_2: u8 = u_get_ICH_VMCR_EL2_Type_VPMR(state, tracer, s_363_1);
        // C s_363_3: const #15064u : u32
        let s_363_3: u32 = 15064;
        // D s_363_4: read-reg s_363_3:struct
        let s_363_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_3 as isize);
            tracer.read_register(s_363_3 as isize, value);
            value
        };
        // C s_363_5: const #15064u : u32
        let s_363_5: u32 = 15064;
        // N s_363_6: write-reg s_363_5 <= s_363_4
        let s_363_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_363_5 as isize, s_363_4);
            tracer.write_register(s_363_5 as isize, s_363_4);
        };
        // C s_363_7: const #15376u : u32
        let s_363_7: u32 = 15376;
        // D s_363_8: read-reg s_363_7:struct
        let s_363_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_7 as isize);
            tracer.read_register(s_363_7 as isize, value);
            value
        };
        // D s_363_9: call _get_ICH_VMCR_EL2_Type_VBPR0(s_363_8)
        let s_363_9: u8 = u_get_ICH_VMCR_EL2_Type_VBPR0(state, tracer, s_363_8);
        // C s_363_10: const #90344u : u32
        let s_363_10: u32 = 90344;
        // D s_363_11: read-reg s_363_10:struct
        let s_363_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_10 as isize);
            tracer.read_register(s_363_10 as isize, value);
            value
        };
        // C s_363_12: const #90344u : u32
        let s_363_12: u32 = 90344;
        // N s_363_13: write-reg s_363_12 <= s_363_11
        let s_363_13: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_363_12 as isize, s_363_11);
            tracer.write_register(s_363_12 as isize, s_363_11);
        };
        // C s_363_14: const #15376u : u32
        let s_363_14: u32 = 15376;
        // D s_363_15: read-reg s_363_14:struct
        let s_363_15: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_14 as isize);
            tracer.read_register(s_363_14 as isize, value);
            value
        };
        // D s_363_16: call _get_ICH_VMCR_EL2_Type_VBPR1(s_363_15)
        let s_363_16: u8 = u_get_ICH_VMCR_EL2_Type_VBPR1(state, tracer, s_363_15);
        // C s_363_17: const #102456u : u32
        let s_363_17: u32 = 102456;
        // D s_363_18: read-reg s_363_17:struct
        let s_363_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_17 as isize);
            tracer.read_register(s_363_17 as isize, value);
            value
        };
        // C s_363_19: const #102456u : u32
        let s_363_19: u32 = 102456;
        // N s_363_20: write-reg s_363_19 <= s_363_18
        let s_363_20: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_363_19 as isize, s_363_18);
            tracer.write_register(s_363_19 as isize, s_363_18);
        };
        // C s_363_21: const #15376u : u32
        let s_363_21: u32 = 15376;
        // D s_363_22: read-reg s_363_21:struct
        let s_363_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_21 as isize);
            tracer.read_register(s_363_21 as isize, value);
            value
        };
        // D s_363_23: call _get_ICH_VMCR_EL2_Type_VENG0(s_363_22)
        let s_363_23: bool = u_get_ICH_VMCR_EL2_Type_VENG0(state, tracer, s_363_22);
        // C s_363_24: const #12048u : u32
        let s_363_24: u32 = 12048;
        // D s_363_25: read-reg s_363_24:struct
        let s_363_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_24 as isize);
            tracer.read_register(s_363_24 as isize, value);
            value
        };
        // C s_363_26: const #12048u : u32
        let s_363_26: u32 = 12048;
        // N s_363_27: write-reg s_363_26 <= s_363_25
        let s_363_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_363_26 as isize, s_363_25);
            tracer.write_register(s_363_26 as isize, s_363_25);
        };
        // C s_363_28: const #15376u : u32
        let s_363_28: u32 = 15376;
        // D s_363_29: read-reg s_363_28:struct
        let s_363_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_28 as isize);
            tracer.read_register(s_363_28 as isize, value);
            value
        };
        // D s_363_30: call _get_ICH_VMCR_EL2_Type_VENG1(s_363_29)
        let s_363_30: bool = u_get_ICH_VMCR_EL2_Type_VENG1(state, tracer, s_363_29);
        // C s_363_31: const #10600u : u32
        let s_363_31: u32 = 10600;
        // D s_363_32: read-reg s_363_31:struct
        let s_363_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_31 as isize);
            tracer.read_register(s_363_31 as isize, value);
            value
        };
        // C s_363_33: const #10600u : u32
        let s_363_33: u32 = 10600;
        // N s_363_34: write-reg s_363_33 <= s_363_32
        let s_363_34: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_363_33 as isize, s_363_32);
            tracer.write_register(s_363_33 as isize, s_363_32);
        };
        // C s_363_35: const #15376u : u32
        let s_363_35: u32 = 15376;
        // D s_363_36: read-reg s_363_35:struct
        let s_363_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_35 as isize);
            tracer.read_register(s_363_35 as isize, value);
            value
        };
        // D s_363_37: call _get_ICH_VMCR_EL2_Type_VEOIM(s_363_36)
        let s_363_37: bool = u_get_ICH_VMCR_EL2_Type_VEOIM(state, tracer, s_363_36);
        // C s_363_38: const #12824u : u32
        let s_363_38: u32 = 12824;
        // D s_363_39: read-reg s_363_38:struct
        let s_363_39: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_38 as isize);
            tracer.read_register(s_363_38 as isize, value);
            value
        };
        // C s_363_40: const #12824u : u32
        let s_363_40: u32 = 12824;
        // N s_363_41: write-reg s_363_40 <= s_363_39
        let s_363_41: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_363_40 as isize, s_363_39);
            tracer.write_register(s_363_40 as isize, s_363_39);
        };
        // C s_363_42: const #15376u : u32
        let s_363_42: u32 = 15376;
        // D s_363_43: read-reg s_363_42:struct
        let s_363_43: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_42 as isize);
            tracer.read_register(s_363_42 as isize, value);
            value
        };
        // D s_363_44: call _get_ICH_VMCR_EL2_Type_VCBPR(s_363_43)
        let s_363_44: bool = u_get_ICH_VMCR_EL2_Type_VCBPR(state, tracer, s_363_43);
        // C s_363_45: const #12824u : u32
        let s_363_45: u32 = 12824;
        // D s_363_46: read-reg s_363_45:struct
        let s_363_46: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_363_45 as isize);
            tracer.read_register(s_363_45 as isize, value);
            value
        };
        // C s_363_47: const #12824u : u32
        let s_363_47: u32 = 12824;
        // N s_363_48: write-reg s_363_47 <= s_363_46
        let s_363_48: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_363_47 as isize, s_363_46);
            tracer.write_register(s_363_47 as isize, s_363_46);
        };
        // N s_363_49: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_364_0: const #11s : i
        let s_364_0: i128 = 11;
        // D s_364_1: read-var crm:i
        let s_364_1: i128 = fn_state.crm;
        // D s_364_2: cmp-eq s_364_1 s_364_0
        let s_364_2: bool = ((s_364_1) == (s_364_0));
        // D s_364_3: write-var gs#141522 <= s_364_2
        fn_state.gs_141522 = s_364_2;
        // N s_364_4: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_365_0: const #7s : i
        let s_365_0: i128 = 7;
        // D s_365_1: read-var op2:i
        let s_365_1: i128 = fn_state.op2;
        // D s_365_2: cmp-eq s_365_1 s_365_0
        let s_365_2: bool = ((s_365_1) == (s_365_0));
        // D s_365_3: write-var gs#141520 <= s_365_2
        fn_state.gs_141520 = s_365_2;
        // N s_365_4: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_366_0: const #4s : i
        let s_366_0: i128 = 4;
        // D s_366_1: read-var op1:i
        let s_366_1: i128 = fn_state.op1;
        // D s_366_2: cmp-eq s_366_1 s_366_0
        let s_366_2: bool = ((s_366_1) == (s_366_0));
        // D s_366_3: write-var gs#141518 <= s_366_2
        fn_state.gs_141518 = s_366_2;
        // N s_366_4: jump b267
        return block_267(state, tracer, fn_state);
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_367_0: const #12s : i
        let s_367_0: i128 = 12;
        // D s_367_1: read-var crn:i
        let s_367_1: i128 = fn_state.crn;
        // D s_367_2: cmp-eq s_367_1 s_367_0
        let s_367_2: bool = ((s_367_1) == (s_367_0));
        // D s_367_3: write-var gs#141516 <= s_367_2
        fn_state.gs_141516 = s_367_2;
        // N s_367_4: jump b265
        return block_265(state, tracer, fn_state);
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_368_0: const #() : ()
        let s_368_0: () = ();
        // S s_368_1: call DBGDSCRint_read(s_368_0)
        let s_368_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_368_0,
        );
        // C s_368_2: const #1u : u8
        let s_368_2: bool = true;
        // S s_368_3: call _update_DBGDSCRint_Type_TXfull(s_368_1, s_368_2)
        let s_368_3: ProductType700c18a878c5601b = u_update_DBGDSCRint_Type_TXfull(
            state,
            tracer,
            s_368_1,
            s_368_2,
        );
        // S s_368_4: call DBGDSCRint_write(s_368_3)
        let s_368_4: () = DBGDSCRint_write(state, tracer, s_368_3);
        // C s_368_5: const #16832u : u32
        let s_368_5: u32 = 16832;
        // D s_368_6: read-reg s_368_5:struct
        let s_368_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_368_5 as isize);
            tracer.read_register(s_368_5 as isize, value);
            value
        };
        // C s_368_7: const #16832u : u32
        let s_368_7: u32 = 16832;
        // N s_368_8: write-reg s_368_7 <= s_368_6
        let s_368_8: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_368_7 as isize, s_368_6);
            tracer.write_register(s_368_7 as isize, s_368_6);
        };
        // C s_368_9: const #() : ()
        let s_368_9: () = ();
        // S s_368_10: call DBGDSCRext_read(s_368_9)
        let s_368_10: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_368_9,
        );
        // C s_368_11: const #1u : u8
        let s_368_11: bool = true;
        // S s_368_12: call _update_DBGDSCRext_Type_TXfull(s_368_10, s_368_11)
        let s_368_12: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_TXfull(
            state,
            tracer,
            s_368_10,
            s_368_11,
        );
        // S s_368_13: call DBGDSCRext_write(s_368_12)
        let s_368_13: () = DBGDSCRext_write(state, tracer, s_368_12);
        // C s_368_14: const #() : ()
        let s_368_14: () = ();
        // S s_368_15: call EDSCR_read(s_368_14)
        let s_368_15: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_368_14);
        // C s_368_16: const #1u : u8
        let s_368_16: bool = true;
        // S s_368_17: call _update_EDSCR_Type_TXfull(s_368_15, s_368_16)
        let s_368_17: ProductType700c18a878c5601b = u_update_EDSCR_Type_TXfull(
            state,
            tracer,
            s_368_15,
            s_368_16,
        );
        // S s_368_18: call EDSCR_write(s_368_17)
        let s_368_18: () = EDSCR_write(state, tracer, s_368_17);
        // C s_368_19: const #() : ()
        let s_368_19: () = ();
        // S s_368_20: call DBGDTR_EL0_read__1(s_368_19)
        let s_368_20: ProductType5c790c8ef59cc8b2 = DBGDTR_EL0_read__1(
            state,
            tracer,
            s_368_19,
        );
        // D s_368_21: write-var ga#247801 <= s_368_20
        fn_state.ga_247801 = s_368_20;
        // D s_368_22: read-var ga#247801.0:struct
        let s_368_22: u64 = fn_state.ga_247801._0;
        // C s_368_23: const #0s : i
        let s_368_23: i128 = 0;
        // C s_368_24: const #32s : i
        let s_368_24: i128 = 32;
        // D s_368_25: cast zx s_368_22 -> bv
        let s_368_25: Bits = Bits::new(s_368_22 as u128, 64u16);
        // D s_368_26: bit-extract s_368_25 s_368_23 s_368_24
        let s_368_26: Bits = (Bits::new(
            ((s_368_25) >> (s_368_23)).value(),
            u16::try_from(s_368_24).unwrap(),
        ));
        // D s_368_27: cast reint s_368_26 -> u32
        let s_368_27: u32 = (s_368_26.value() as u32);
        // C s_368_28: const #0s : i
        let s_368_28: i128 = 0;
        // C s_368_29: const #17232u : u32
        let s_368_29: u32 = 17232;
        // D s_368_30: read-reg s_368_29:u64
        let s_368_30: u64 = {
            let value = state.read_register::<u64>(s_368_29 as isize);
            tracer.read_register(s_368_29 as isize, value);
            value
        };
        // D s_368_31: cast zx s_368_30 -> bv
        let s_368_31: Bits = Bits::new(s_368_30 as u128, 64u16);
        // D s_368_32: cast zx s_368_27 -> bv
        let s_368_32: Bits = Bits::new(s_368_27 as u128, 32u16);
        // C s_368_33: const #31s : i
        let s_368_33: i128 = 31;
        // C s_368_34: const #1u : u64
        let s_368_34: u64 = 1;
        // C s_368_35: cast zx s_368_34 -> bv
        let s_368_35: Bits = Bits::new(s_368_34 as u128, 64u16);
        // C s_368_36: lsl s_368_35 s_368_33
        let s_368_36: Bits = s_368_35 << s_368_33;
        // C s_368_37: sub s_368_36 s_368_35
        let s_368_37: Bits = ((s_368_36) - (s_368_35));
        // D s_368_38: and s_368_32 s_368_37
        let s_368_38: Bits = ((s_368_32) & (s_368_37));
        // D s_368_39: lsl s_368_38 s_368_28
        let s_368_39: Bits = s_368_38 << s_368_28;
        // C s_368_40: lsl s_368_37 s_368_28
        let s_368_40: Bits = s_368_37 << s_368_28;
        // C s_368_41: cmpl s_368_40
        let s_368_41: Bits = !s_368_40;
        // D s_368_42: and s_368_31 s_368_41
        let s_368_42: Bits = ((s_368_31) & (s_368_41));
        // D s_368_43: or s_368_42 s_368_39
        let s_368_43: Bits = ((s_368_42) | (s_368_39));
        // D s_368_44: cast reint s_368_43 -> u64
        let s_368_44: u64 = (s_368_43.value() as u64);
        // C s_368_45: const #17232u : u32
        let s_368_45: u32 = 17232;
        // N s_368_46: write-reg s_368_45 <= s_368_44
        let s_368_46: () = {
            state.write_register::<u64>(s_368_45 as isize, s_368_44);
            tracer.write_register(s_368_45 as isize, s_368_44);
        };
        // C s_368_47: const #() : ()
        let s_368_47: () = ();
        // S s_368_48: call DBGDTR_EL0_read__1(s_368_47)
        let s_368_48: ProductType5c790c8ef59cc8b2 = DBGDTR_EL0_read__1(
            state,
            tracer,
            s_368_47,
        );
        // D s_368_49: write-var ga#247804 <= s_368_48
        fn_state.ga_247804 = s_368_48;
        // D s_368_50: read-var ga#247804.0:struct
        let s_368_50: u64 = fn_state.ga_247804._0;
        // C s_368_51: const #32s : i
        let s_368_51: i128 = 32;
        // C s_368_52: const #32s : i
        let s_368_52: i128 = 32;
        // D s_368_53: cast zx s_368_50 -> bv
        let s_368_53: Bits = Bits::new(s_368_50 as u128, 64u16);
        // D s_368_54: bit-extract s_368_53 s_368_51 s_368_52
        let s_368_54: Bits = (Bits::new(
            ((s_368_53) >> (s_368_51)).value(),
            u16::try_from(s_368_52).unwrap(),
        ));
        // D s_368_55: cast reint s_368_54 -> u32
        let s_368_55: u32 = (s_368_54.value() as u32);
        // C s_368_56: const #0s : i
        let s_368_56: i128 = 0;
        // C s_368_57: const #103184u : u32
        let s_368_57: u32 = 103184;
        // D s_368_58: read-reg s_368_57:u64
        let s_368_58: u64 = {
            let value = state.read_register::<u64>(s_368_57 as isize);
            tracer.read_register(s_368_57 as isize, value);
            value
        };
        // D s_368_59: cast zx s_368_58 -> bv
        let s_368_59: Bits = Bits::new(s_368_58 as u128, 64u16);
        // D s_368_60: cast zx s_368_55 -> bv
        let s_368_60: Bits = Bits::new(s_368_55 as u128, 32u16);
        // C s_368_61: const #31s : i
        let s_368_61: i128 = 31;
        // C s_368_62: const #1u : u64
        let s_368_62: u64 = 1;
        // C s_368_63: cast zx s_368_62 -> bv
        let s_368_63: Bits = Bits::new(s_368_62 as u128, 64u16);
        // C s_368_64: lsl s_368_63 s_368_61
        let s_368_64: Bits = s_368_63 << s_368_61;
        // C s_368_65: sub s_368_64 s_368_63
        let s_368_65: Bits = ((s_368_64) - (s_368_63));
        // D s_368_66: and s_368_60 s_368_65
        let s_368_66: Bits = ((s_368_60) & (s_368_65));
        // D s_368_67: lsl s_368_66 s_368_56
        let s_368_67: Bits = s_368_66 << s_368_56;
        // C s_368_68: lsl s_368_65 s_368_56
        let s_368_68: Bits = s_368_65 << s_368_56;
        // C s_368_69: cmpl s_368_68
        let s_368_69: Bits = !s_368_68;
        // D s_368_70: and s_368_59 s_368_69
        let s_368_70: Bits = ((s_368_59) & (s_368_69));
        // D s_368_71: or s_368_70 s_368_67
        let s_368_71: Bits = ((s_368_70) | (s_368_67));
        // D s_368_72: cast reint s_368_71 -> u64
        let s_368_72: u64 = (s_368_71.value() as u64);
        // C s_368_73: const #103184u : u32
        let s_368_73: u32 = 103184;
        // N s_368_74: write-reg s_368_73 <= s_368_72
        let s_368_74: () = {
            state.write_register::<u64>(s_368_73 as isize, s_368_72);
            tracer.write_register(s_368_73 as isize, s_368_72);
        };
        // N s_368_75: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_369_0: const #4s : i
        let s_369_0: i128 = 4;
        // D s_369_1: read-var crm:i
        let s_369_1: i128 = fn_state.crm;
        // D s_369_2: cmp-eq s_369_1 s_369_0
        let s_369_2: bool = ((s_369_1) == (s_369_0));
        // D s_369_3: write-var gs#141513 <= s_369_2
        fn_state.gs_141513 = s_369_2;
        // N s_369_4: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_370_0: const #0s : i
        let s_370_0: i128 = 0;
        // D s_370_1: read-var op2:i
        let s_370_1: i128 = fn_state.op2;
        // D s_370_2: cmp-eq s_370_1 s_370_0
        let s_370_2: bool = ((s_370_1) == (s_370_0));
        // D s_370_3: write-var gs#141511 <= s_370_2
        fn_state.gs_141511 = s_370_2;
        // N s_370_4: jump b259
        return block_259(state, tracer, fn_state);
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_371_0: const #3s : i
        let s_371_0: i128 = 3;
        // D s_371_1: read-var op1:i
        let s_371_1: i128 = fn_state.op1;
        // D s_371_2: cmp-eq s_371_1 s_371_0
        let s_371_2: bool = ((s_371_1) == (s_371_0));
        // D s_371_3: write-var gs#141509 <= s_371_2
        fn_state.gs_141509 = s_371_2;
        // N s_371_4: jump b257
        return block_257(state, tracer, fn_state);
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_372_0: const #0s : i
        let s_372_0: i128 = 0;
        // D s_372_1: read-var crn:i
        let s_372_1: i128 = fn_state.crn;
        // D s_372_2: cmp-eq s_372_1 s_372_0
        let s_372_2: bool = ((s_372_1) == (s_372_0));
        // D s_372_3: write-var gs#141507 <= s_372_2
        fn_state.gs_141507 = s_372_2;
        // N s_372_4: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_373_0: const #() : ()
        let s_373_0: () = ();
        // S s_373_1: call DBGDSCRint_read(s_373_0)
        let s_373_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_373_0,
        );
        // C s_373_2: const #1u : u8
        let s_373_2: bool = true;
        // S s_373_3: call _update_DBGDSCRint_Type_TXfull(s_373_1, s_373_2)
        let s_373_3: ProductType700c18a878c5601b = u_update_DBGDSCRint_Type_TXfull(
            state,
            tracer,
            s_373_1,
            s_373_2,
        );
        // S s_373_4: call DBGDSCRint_write(s_373_3)
        let s_373_4: () = DBGDSCRint_write(state, tracer, s_373_3);
        // C s_373_5: const #16832u : u32
        let s_373_5: u32 = 16832;
        // D s_373_6: read-reg s_373_5:struct
        let s_373_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_373_5 as isize);
            tracer.read_register(s_373_5 as isize, value);
            value
        };
        // C s_373_7: const #16832u : u32
        let s_373_7: u32 = 16832;
        // N s_373_8: write-reg s_373_7 <= s_373_6
        let s_373_8: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_373_7 as isize, s_373_6);
            tracer.write_register(s_373_7 as isize, s_373_6);
        };
        // C s_373_9: const #() : ()
        let s_373_9: () = ();
        // S s_373_10: call DBGDSCRext_read(s_373_9)
        let s_373_10: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_373_9,
        );
        // C s_373_11: const #1u : u8
        let s_373_11: bool = true;
        // S s_373_12: call _update_DBGDSCRext_Type_TXfull(s_373_10, s_373_11)
        let s_373_12: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_TXfull(
            state,
            tracer,
            s_373_10,
            s_373_11,
        );
        // S s_373_13: call DBGDSCRext_write(s_373_12)
        let s_373_13: () = DBGDSCRext_write(state, tracer, s_373_12);
        // C s_373_14: const #() : ()
        let s_373_14: () = ();
        // S s_373_15: call EDSCR_read(s_373_14)
        let s_373_15: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_373_14);
        // C s_373_16: const #1u : u8
        let s_373_16: bool = true;
        // S s_373_17: call _update_EDSCR_Type_TXfull(s_373_15, s_373_16)
        let s_373_17: ProductType700c18a878c5601b = u_update_EDSCR_Type_TXfull(
            state,
            tracer,
            s_373_15,
            s_373_16,
        );
        // S s_373_18: call EDSCR_write(s_373_17)
        let s_373_18: () = EDSCR_write(state, tracer, s_373_17);
        // N s_373_19: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_374_0: const #5s : i
        let s_374_0: i128 = 5;
        // D s_374_1: read-var crm:i
        let s_374_1: i128 = fn_state.crm;
        // D s_374_2: cmp-eq s_374_1 s_374_0
        let s_374_2: bool = ((s_374_1) == (s_374_0));
        // D s_374_3: write-var gs#141504 <= s_374_2
        fn_state.gs_141504 = s_374_2;
        // N s_374_4: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_375_0: const #0s : i
        let s_375_0: i128 = 0;
        // D s_375_1: read-var op2:i
        let s_375_1: i128 = fn_state.op2;
        // D s_375_2: cmp-eq s_375_1 s_375_0
        let s_375_2: bool = ((s_375_1) == (s_375_0));
        // D s_375_3: write-var gs#141502 <= s_375_2
        fn_state.gs_141502 = s_375_2;
        // N s_375_4: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_376_0: const #3s : i
        let s_376_0: i128 = 3;
        // D s_376_1: read-var op1:i
        let s_376_1: i128 = fn_state.op1;
        // D s_376_2: cmp-eq s_376_1 s_376_0
        let s_376_2: bool = ((s_376_1) == (s_376_0));
        // D s_376_3: write-var gs#141500 <= s_376_2
        fn_state.gs_141500 = s_376_2;
        // N s_376_4: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_377_0: const #0s : i
        let s_377_0: i128 = 0;
        // D s_377_1: read-var crn:i
        let s_377_1: i128 = fn_state.crn;
        // D s_377_2: cmp-eq s_377_1 s_377_0
        let s_377_2: bool = ((s_377_1) == (s_377_0));
        // D s_377_3: write-var gs#141498 <= s_377_2
        fn_state.gs_141498 = s_377_2;
        // N s_377_4: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_378_0: const #0s : i
        let s_378_0: i128 = 0;
        // D s_378_1: read-var tempxt:u64
        let s_378_1: u64 = fn_state.tempxt;
        // D s_378_2: cast zx s_378_1 -> bv
        let s_378_2: Bits = Bits::new(s_378_1 as u128, 64u16);
        // C s_378_3: const #1s : i64
        let s_378_3: i64 = 1;
        // C s_378_4: cast zx s_378_3 -> i
        let s_378_4: i128 = (i128::try_from(s_378_3).unwrap());
        // C s_378_5: const #31s : i
        let s_378_5: i128 = 31;
        // C s_378_6: add s_378_5 s_378_4
        let s_378_6: i128 = (s_378_5 + s_378_4);
        // D s_378_7: bit-extract s_378_2 s_378_0 s_378_6
        let s_378_7: Bits = (Bits::new(
            ((s_378_2) >> (s_378_0)).value(),
            u16::try_from(s_378_6).unwrap(),
        ));
        // D s_378_8: cast reint s_378_7 -> u32
        let s_378_8: u32 = (s_378_7.value() as u32);
        // D s_378_9: call AArch64_PMUSwIncrement(s_378_8)
        let s_378_9: () = AArch64_PMUSwIncrement(state, tracer, s_378_8);
        // N s_378_10: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_379_0: const #12s : i
        let s_379_0: i128 = 12;
        // D s_379_1: read-var crm:i
        let s_379_1: i128 = fn_state.crm;
        // D s_379_2: cmp-eq s_379_1 s_379_0
        let s_379_2: bool = ((s_379_1) == (s_379_0));
        // D s_379_3: write-var gs#141495 <= s_379_2
        fn_state.gs_141495 = s_379_2;
        // N s_379_4: jump b241
        return block_241(state, tracer, fn_state);
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_380_0: const #4s : i
        let s_380_0: i128 = 4;
        // D s_380_1: read-var op2:i
        let s_380_1: i128 = fn_state.op2;
        // D s_380_2: cmp-eq s_380_1 s_380_0
        let s_380_2: bool = ((s_380_1) == (s_380_0));
        // D s_380_3: write-var gs#141493 <= s_380_2
        fn_state.gs_141493 = s_380_2;
        // N s_380_4: jump b239
        return block_239(state, tracer, fn_state);
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_381_0: const #3s : i
        let s_381_0: i128 = 3;
        // D s_381_1: read-var op1:i
        let s_381_1: i128 = fn_state.op1;
        // D s_381_2: cmp-eq s_381_1 s_381_0
        let s_381_2: bool = ((s_381_1) == (s_381_0));
        // D s_381_3: write-var gs#141491 <= s_381_2
        fn_state.gs_141491 = s_381_2;
        // N s_381_4: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_382_0: const #9s : i
        let s_382_0: i128 = 9;
        // D s_382_1: read-var crn:i
        let s_382_1: i128 = fn_state.crn;
        // D s_382_2: cmp-eq s_382_1 s_382_0
        let s_382_2: bool = ((s_382_1) == (s_382_0));
        // D s_382_3: write-var gs#141489 <= s_382_2
        fn_state.gs_141489 = s_382_2;
        // N s_382_4: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_383_0: const #1s : i
        let s_383_0: i128 = 1;
        // C s_383_1: const #0s : i
        let s_383_1: i128 = 0;
        // D s_383_2: read-var crm:i
        let s_383_2: i128 = fn_state.crm;
        // D s_383_3: call integer_subrange(s_383_2, s_383_0, s_383_1)
        let s_383_3: Bits = integer_subrange(state, tracer, s_383_2, s_383_0, s_383_1);
        // D s_383_4: cast reint s_383_3 -> u8
        let s_383_4: u8 = (s_383_3.value() as u8);
        // C s_383_5: const #2s : i
        let s_383_5: i128 = 2;
        // C s_383_6: const #0s : i
        let s_383_6: i128 = 0;
        // D s_383_7: read-var op2:i
        let s_383_7: i128 = fn_state.op2;
        // D s_383_8: call integer_subrange(s_383_7, s_383_5, s_383_6)
        let s_383_8: Bits = integer_subrange(state, tracer, s_383_7, s_383_5, s_383_6);
        // D s_383_9: cast reint s_383_8 -> u8
        let s_383_9: u8 = (s_383_8.value() as u8);
        // D s_383_10: cast zx s_383_4 -> bv
        let s_383_10: Bits = Bits::new(s_383_4 as u128, 2u16);
        // D s_383_11: cast zx s_383_9 -> bv
        let s_383_11: Bits = Bits::new(s_383_9 as u128, 3u16);
        // D s_383_12: cast reint s_383_10 -> u128
        let s_383_12: u128 = (s_383_10.value() as u128);
        // D s_383_13: size-of s_383_10
        let s_383_13: u16 = s_383_10.length();
        // D s_383_14: cast reint s_383_11 -> u128
        let s_383_14: u128 = (s_383_11.value() as u128);
        // D s_383_15: size-of s_383_11
        let s_383_15: u16 = s_383_11.length();
        // D s_383_16: lsl s_383_12 s_383_15
        let s_383_16: u128 = s_383_12 << s_383_15;
        // D s_383_17: or s_383_16 s_383_14
        let s_383_17: u128 = ((s_383_16) | (s_383_14));
        // D s_383_18: add s_383_13 s_383_15
        let s_383_18: u16 = (s_383_13 + s_383_15);
        // D s_383_19: create-bits s_383_17 s_383_18
        let s_383_19: Bits = Bits::new(s_383_17, s_383_18);
        // D s_383_20: cast reint s_383_19 -> u8
        let s_383_20: u8 = (s_383_19.value() as u8);
        // D s_383_21: cast zx s_383_20 -> bv
        let s_383_21: Bits = Bits::new(s_383_20 as u128, 5u16);
        // D s_383_22: cast zx s_383_21 -> i
        let s_383_22: i128 = (s_383_21.value() as i128);
        // D s_383_23: cast reint s_383_22 -> i64
        let s_383_23: i64 = (s_383_22 as i64);
        // D s_383_24: write-var index <= s_383_23
        fn_state.index = s_383_23;
        // C s_383_25: const #31s : i
        let s_383_25: i128 = 31;
        // D s_383_26: read-var index:i64
        let s_383_26: i64 = fn_state.index;
        // D s_383_27: cast zx s_383_26 -> i
        let s_383_27: i128 = (i128::try_from(s_383_26).unwrap());
        // D s_383_28: cmp-eq s_383_27 s_383_25
        let s_383_28: bool = ((s_383_27) == (s_383_25));
        // N s_383_29: branch s_383_28 b386 b384
        if s_383_28 {
            return block_386(state, tracer, fn_state);
        } else {
            return block_384(state, tracer, fn_state);
        };
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_384_0: read-var tempxt:u64
        let s_384_0: u64 = fn_state.tempxt;
        // D s_384_1: call Mk_PMEVTYPER_EL0_Type(s_384_0)
        let s_384_1: ProductType5c790c8ef59cc8b2 = Mk_PMEVTYPER_EL0_Type(
            state,
            tracer,
            s_384_0,
        );
        // D s_384_2: call __get_PMEVTYPER_EL0(s_384_1)
        let s_384_2: ProductType5c790c8ef59cc8b2 = u__get_PMEVTYPER_EL0(
            state,
            tracer,
            s_384_1,
        );
        // C s_384_3: const #11208u : u32
        let s_384_3: u32 = 11208;
        // D s_384_4: read-reg s_384_3:[struct; 32]
        let s_384_4: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_384_3 as isize);
            tracer.read_register(s_384_3 as isize, value);
            value
        };
        // D s_384_5: read-var index:i64
        let s_384_5: i64 = fn_state.index;
        // D s_384_6: cast zx s_384_5 -> i
        let s_384_6: i128 = (i128::try_from(s_384_5).unwrap());
        // D s_384_7: mutate-element s_384_4[s_384_6] <= s_384_2
        let s_384_7: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut local = s_384_4.clone();
            local[(s_384_6) as usize] = s_384_2;
            local
        };
        // D s_384_8: cast cvt s_384_7 -> [struct; 0]
        let s_384_8: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_384_7,
        );
        // D s_384_9: cast cvt s_384_8 -> [struct; 32]
        let s_384_9: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_384_8);
            buf
        };
        // C s_384_10: const #11208u : u32
        let s_384_10: u32 = 11208;
        // N s_384_11: write-reg s_384_10 <= s_384_9
        let s_384_11: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_384_10 as isize, s_384_9);
            tracer.write_register(s_384_10 as isize, s_384_9);
        };
        // N s_384_12: jump b385
        return block_385(state, tracer, fn_state);
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_385_0: jump b233
        return block_233(state, tracer, fn_state);
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_386_0: read-var tempxt:u64
        let s_386_0: u64 = fn_state.tempxt;
        // D s_386_1: call Mk_PMCCFILTR_EL0_Type(s_386_0)
        let s_386_1: ProductType5c790c8ef59cc8b2 = Mk_PMCCFILTR_EL0_Type(
            state,
            tracer,
            s_386_0,
        );
        // D s_386_2: call __get_PMCCFILTR_EL0(s_386_1)
        let s_386_2: ProductType5c790c8ef59cc8b2 = u__get_PMCCFILTR_EL0(
            state,
            tracer,
            s_386_1,
        );
        // C s_386_3: const #15864u : u32
        let s_386_3: u32 = 15864;
        // N s_386_4: write-reg s_386_3 <= s_386_2
        let s_386_4: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_386_3 as isize, s_386_2);
            tracer.write_register(s_386_3 as isize, s_386_2);
        };
        // N s_386_5: jump b385
        return block_385(state, tracer, fn_state);
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_387_0: const #3s : i
        let s_387_0: i128 = 3;
        // C s_387_1: const #2s : i
        let s_387_1: i128 = 2;
        // D s_387_2: read-var crm:i
        let s_387_2: i128 = fn_state.crm;
        // D s_387_3: call integer_subrange(s_387_2, s_387_0, s_387_1)
        let s_387_3: Bits = integer_subrange(state, tracer, s_387_2, s_387_0, s_387_1);
        // D s_387_4: cast reint s_387_3 -> u8
        let s_387_4: u8 = (s_387_3.value() as u8);
        // D s_387_5: cast zx s_387_4 -> bv
        let s_387_5: Bits = Bits::new(s_387_4 as u128, 2u16);
        // C s_387_6: const #3u : u8
        let s_387_6: u8 = 3;
        // C s_387_7: cast zx s_387_6 -> bv
        let s_387_7: Bits = Bits::new(s_387_6 as u128, 2u16);
        // D s_387_8: cmp-eq s_387_5 s_387_7
        let s_387_8: bool = ((s_387_5) == (s_387_7));
        // D s_387_9: write-var gs#141486 <= s_387_8
        fn_state.gs_141486 = s_387_8;
        // N s_387_10: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_388_0: const #14s : i
        let s_388_0: i128 = 14;
        // D s_388_1: read-var crn:i
        let s_388_1: i128 = fn_state.crn;
        // D s_388_2: cmp-eq s_388_1 s_388_0
        let s_388_2: bool = ((s_388_1) == (s_388_0));
        // D s_388_3: write-var gs#141483 <= s_388_2
        fn_state.gs_141483 = s_388_2;
        // N s_388_4: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_389_0: const #3s : i
        let s_389_0: i128 = 3;
        // D s_389_1: read-var op1:i
        let s_389_1: i128 = fn_state.op1;
        // D s_389_2: cmp-eq s_389_1 s_389_0
        let s_389_2: bool = ((s_389_1) == (s_389_0));
        // D s_389_3: write-var gs#141481 <= s_389_2
        fn_state.gs_141481 = s_389_2;
        // N s_389_4: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_390_0: const #1s : i
        let s_390_0: i128 = 1;
        // D s_390_1: read-var op2:i
        let s_390_1: i128 = fn_state.op2;
        // D s_390_2: cmp-eq s_390_1 s_390_0
        let s_390_2: bool = ((s_390_1) == (s_390_0));
        // N s_390_3: branch s_390_2 b396 b391
        if s_390_2 {
            return block_396(state, tracer, fn_state);
        } else {
            return block_391(state, tracer, fn_state);
        };
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_391_0: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_392_0: const #2s : i
        let s_392_0: i128 = 2;
        // D s_392_1: read-var op2:i
        let s_392_1: i128 = fn_state.op2;
        // D s_392_2: cmp-eq s_392_1 s_392_0
        let s_392_2: bool = ((s_392_1) == (s_392_0));
        // N s_392_3: branch s_392_2 b395 b393
        if s_392_2 {
            return block_395(state, tracer, fn_state);
        } else {
            return block_393(state, tracer, fn_state);
        };
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_393_0: jump b394
        return block_394(state, tracer, fn_state);
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_394_0: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_395_0: const #19136u : u32
        let s_395_0: u32 = 19136;
        // D s_395_1: read-reg s_395_0:struct
        let s_395_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_395_0 as isize);
            tracer.read_register(s_395_0 as isize, value);
            value
        };
        // D s_395_2: call _get_PMSELR_EL0_Type_SEL(s_395_1)
        let s_395_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_395_1);
        // D s_395_3: cast zx s_395_2 -> bv
        let s_395_3: Bits = Bits::new(s_395_2 as u128, 5u16);
        // D s_395_4: cast zx s_395_3 -> i
        let s_395_4: i128 = (s_395_3.value() as i128);
        // D s_395_5: cast reint s_395_4 -> i64
        let s_395_5: i64 = (s_395_4 as i64);
        // C s_395_6: const #31s : i
        let s_395_6: i128 = 31;
        // D s_395_7: cast zx s_395_5 -> i
        let s_395_7: i128 = (i128::try_from(s_395_5).unwrap());
        // D s_395_8: cmp-lt s_395_7 s_395_6
        let s_395_8: bool = ((s_395_7) < (s_395_6));
        // N s_395_9: assert s_395_8
        let s_395_9: () = assert!(s_395_8);
        // C s_395_10: const #10744u : u32
        let s_395_10: u32 = 10744;
        // D s_395_11: read-reg s_395_10:[u64; 32]
        let s_395_11: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_395_10 as isize);
            tracer.read_register(s_395_10 as isize, value);
            value
        };
        // D s_395_12: cast zx s_395_5 -> i
        let s_395_12: i128 = (i128::try_from(s_395_5).unwrap());
        // D s_395_13: read-var tempxt:u64
        let s_395_13: u64 = fn_state.tempxt;
        // D s_395_14: mutate-element s_395_11[s_395_12] <= s_395_13
        let s_395_14: [u64; 32usize] = {
            let mut local = s_395_11.clone();
            local[(s_395_12) as usize] = s_395_13;
            local
        };
        // D s_395_15: cast cvt s_395_14 -> [u64; 0]
        let s_395_15: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_395_14);
        // D s_395_16: cast cvt s_395_15 -> [u64; 32]
        let s_395_16: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_395_15);
            buf
        };
        // C s_395_17: const #10744u : u32
        let s_395_17: u32 = 10744;
        // N s_395_18: write-reg s_395_17 <= s_395_16
        let s_395_18: () = {
            state.write_register::<[u64; 32usize]>(s_395_17 as isize, s_395_16);
            tracer.write_register(s_395_17 as isize, s_395_16);
        };
        // N s_395_19: jump b394
        return block_394(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_396_0: const #19136u : u32
        let s_396_0: u32 = 19136;
        // D s_396_1: read-reg s_396_0:struct
        let s_396_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_396_0 as isize);
            tracer.read_register(s_396_0 as isize, value);
            value
        };
        // D s_396_2: call _get_PMSELR_EL0_Type_SEL(s_396_1)
        let s_396_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_396_1);
        // D s_396_3: cast zx s_396_2 -> bv
        let s_396_3: Bits = Bits::new(s_396_2 as u128, 5u16);
        // C s_396_4: const #31u : u8
        let s_396_4: u8 = 31;
        // C s_396_5: cast zx s_396_4 -> bv
        let s_396_5: Bits = Bits::new(s_396_4 as u128, 5u16);
        // D s_396_6: cmp-eq s_396_3 s_396_5
        let s_396_6: bool = ((s_396_3) == (s_396_5));
        // N s_396_7: branch s_396_6 b398 b397
        if s_396_6 {
            return block_398(state, tracer, fn_state);
        } else {
            return block_397(state, tracer, fn_state);
        };
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_397_0: const #19136u : u32
        let s_397_0: u32 = 19136;
        // D s_397_1: read-reg s_397_0:struct
        let s_397_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_397_0 as isize);
            tracer.read_register(s_397_0 as isize, value);
            value
        };
        // D s_397_2: call _get_PMSELR_EL0_Type_SEL(s_397_1)
        let s_397_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_397_1);
        // D s_397_3: cast zx s_397_2 -> bv
        let s_397_3: Bits = Bits::new(s_397_2 as u128, 5u16);
        // D s_397_4: cast zx s_397_3 -> i
        let s_397_4: i128 = (s_397_3.value() as i128);
        // D s_397_5: cast reint s_397_4 -> i64
        let s_397_5: i64 = (s_397_4 as i64);
        // C s_397_6: const #31s : i
        let s_397_6: i128 = 31;
        // D s_397_7: cast zx s_397_5 -> i
        let s_397_7: i128 = (i128::try_from(s_397_5).unwrap());
        // D s_397_8: cmp-lt s_397_7 s_397_6
        let s_397_8: bool = ((s_397_7) < (s_397_6));
        // N s_397_9: assert s_397_8
        let s_397_9: () = assert!(s_397_8);
        // D s_397_10: read-var tempxt:u64
        let s_397_10: u64 = fn_state.tempxt;
        // D s_397_11: call Mk_PMEVTYPER_EL0_Type(s_397_10)
        let s_397_11: ProductType5c790c8ef59cc8b2 = Mk_PMEVTYPER_EL0_Type(
            state,
            tracer,
            s_397_10,
        );
        // D s_397_12: call __get_PMEVTYPER_EL0(s_397_11)
        let s_397_12: ProductType5c790c8ef59cc8b2 = u__get_PMEVTYPER_EL0(
            state,
            tracer,
            s_397_11,
        );
        // C s_397_13: const #11208u : u32
        let s_397_13: u32 = 11208;
        // D s_397_14: read-reg s_397_13:[struct; 32]
        let s_397_14: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_397_13 as isize);
            tracer.read_register(s_397_13 as isize, value);
            value
        };
        // D s_397_15: cast zx s_397_5 -> i
        let s_397_15: i128 = (i128::try_from(s_397_5).unwrap());
        // D s_397_16: mutate-element s_397_14[s_397_15] <= s_397_12
        let s_397_16: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut local = s_397_14.clone();
            local[(s_397_15) as usize] = s_397_12;
            local
        };
        // D s_397_17: cast cvt s_397_16 -> [struct; 0]
        let s_397_17: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_397_16,
        );
        // D s_397_18: cast cvt s_397_17 -> [struct; 32]
        let s_397_18: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_397_17);
            buf
        };
        // C s_397_19: const #11208u : u32
        let s_397_19: u32 = 11208;
        // N s_397_20: write-reg s_397_19 <= s_397_18
        let s_397_20: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_397_19 as isize, s_397_18);
            tracer.write_register(s_397_19 as isize, s_397_18);
        };
        // N s_397_21: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_398_0: read-var tempxt:u64
        let s_398_0: u64 = fn_state.tempxt;
        // D s_398_1: call Mk_PMCCFILTR_EL0_Type(s_398_0)
        let s_398_1: ProductType5c790c8ef59cc8b2 = Mk_PMCCFILTR_EL0_Type(
            state,
            tracer,
            s_398_0,
        );
        // D s_398_2: call __get_PMCCFILTR_EL0(s_398_1)
        let s_398_2: ProductType5c790c8ef59cc8b2 = u__get_PMCCFILTR_EL0(
            state,
            tracer,
            s_398_1,
        );
        // C s_398_3: const #15864u : u32
        let s_398_3: u32 = 15864;
        // N s_398_4: write-reg s_398_3 <= s_398_2
        let s_398_4: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_398_3 as isize, s_398_2);
            tracer.write_register(s_398_3 as isize, s_398_2);
        };
        // N s_398_5: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_399_0: const #1s : i
        let s_399_0: i128 = 1;
        // D s_399_1: read-var op2:i
        let s_399_1: i128 = fn_state.op2;
        // D s_399_2: cmp-eq s_399_1 s_399_0
        let s_399_2: bool = ((s_399_1) == (s_399_0));
        // N s_399_3: branch s_399_2 b402 b400
        if s_399_2 {
            return block_402(state, tracer, fn_state);
        } else {
            return block_400(state, tracer, fn_state);
        };
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_400_0: const #2s : i
        let s_400_0: i128 = 2;
        // D s_400_1: read-var op2:i
        let s_400_1: i128 = fn_state.op2;
        // D s_400_2: cmp-eq s_400_1 s_400_0
        let s_400_2: bool = ((s_400_1) == (s_400_0));
        // D s_400_3: write-var gs#141477 <= s_400_2
        fn_state.gs_141477 = s_400_2;
        // N s_400_4: jump b401
        return block_401(state, tracer, fn_state);
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_401_0: read-var gs#141477:u8
        let s_401_0: bool = fn_state.gs_141477;
        // D s_401_1: write-var gs#141478 <= s_401_0
        fn_state.gs_141478 = s_401_0;
        // N s_401_2: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_402_0: const #1u : u8
        let s_402_0: bool = true;
        // D s_402_1: write-var gs#141477 <= s_402_0
        fn_state.gs_141477 = s_402_0;
        // N s_402_2: jump b401
        return block_401(state, tracer, fn_state);
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_403_0: const #13s : i
        let s_403_0: i128 = 13;
        // D s_403_1: read-var crm:i
        let s_403_1: i128 = fn_state.crm;
        // D s_403_2: cmp-eq s_403_1 s_403_0
        let s_403_2: bool = ((s_403_1) == (s_403_0));
        // D s_403_3: write-var gs#141474 <= s_403_2
        fn_state.gs_141474 = s_403_2;
        // N s_403_4: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_404_0: const #9s : i
        let s_404_0: i128 = 9;
        // D s_404_1: read-var crn:i
        let s_404_1: i128 = fn_state.crn;
        // D s_404_2: cmp-eq s_404_1 s_404_0
        let s_404_2: bool = ((s_404_1) == (s_404_0));
        // D s_404_3: write-var gs#141472 <= s_404_2
        fn_state.gs_141472 = s_404_2;
        // N s_404_4: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_405_0: const #3s : i
        let s_405_0: i128 = 3;
        // D s_405_1: read-var op1:i
        let s_405_1: i128 = fn_state.op1;
        // D s_405_2: cmp-eq s_405_1 s_405_0
        let s_405_2: bool = ((s_405_1) == (s_405_0));
        // D s_405_3: write-var gs#141470 <= s_405_2
        fn_state.gs_141470 = s_405_2;
        // N s_405_4: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_406_0: const #2s : i
        let s_406_0: i128 = 2;
        // D s_406_1: read-var tempxt:u64
        let s_406_1: u64 = fn_state.tempxt;
        // D s_406_2: cast zx s_406_1 -> bv
        let s_406_2: Bits = Bits::new(s_406_1 as u128, 64u16);
        // C s_406_3: const #1u : u64
        let s_406_3: u64 = 1;
        // D s_406_4: bit-extract s_406_2 s_406_0 s_406_3
        let s_406_4: Bits = (Bits::new(
            ((s_406_2) >> (s_406_0)).value(),
            u16::try_from(s_406_3).unwrap(),
        ));
        // D s_406_5: cast reint s_406_4 -> u8
        let s_406_5: bool = ((s_406_4.value()) != 0);
        // C s_406_6: const #0s : i
        let s_406_6: i128 = 0;
        // C s_406_7: const #0u : u64
        let s_406_7: u64 = 0;
        // D s_406_8: cast zx s_406_5 -> u64
        let s_406_8: u64 = (s_406_5 as u64);
        // C s_406_9: const #1u : u64
        let s_406_9: u64 = 1;
        // D s_406_10: and s_406_8 s_406_9
        let s_406_10: u64 = ((s_406_8) & (s_406_9));
        // D s_406_11: cmp-eq s_406_10 s_406_9
        let s_406_11: bool = ((s_406_10) == (s_406_9));
        // D s_406_12: lsl s_406_8 s_406_6
        let s_406_12: u64 = s_406_8 << s_406_6;
        // D s_406_13: or s_406_7 s_406_12
        let s_406_13: u64 = ((s_406_7) | (s_406_12));
        // D s_406_14: cmpl s_406_12
        let s_406_14: u64 = !s_406_12;
        // D s_406_15: and s_406_7 s_406_14
        let s_406_15: u64 = ((s_406_7) & (s_406_14));
        // D s_406_16: select s_406_11 s_406_13 s_406_15
        let s_406_16: u64 = if s_406_11 { s_406_13 } else { s_406_15 };
        // D s_406_17: cast trunc s_406_16 -> u8
        let s_406_17: bool = ((s_406_16) != 0);
        // D s_406_18: cast zx s_406_17 -> bv
        let s_406_18: Bits = Bits::new(s_406_17 as u128, 1u16);
        // C s_406_19: const #1u : u8
        let s_406_19: bool = true;
        // C s_406_20: cast zx s_406_19 -> bv
        let s_406_20: Bits = Bits::new(s_406_19 as u128, 1u16);
        // D s_406_21: cmp-eq s_406_18 s_406_20
        let s_406_21: bool = ((s_406_18) == (s_406_20));
        // N s_406_22: branch s_406_21 b412 b407
        if s_406_21 {
            return block_412(state, tracer, fn_state);
        } else {
            return block_407(state, tracer, fn_state);
        };
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_407_0: jump b408
        return block_408(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_408_0: const #1s : i
        let s_408_0: i128 = 1;
        // D s_408_1: read-var tempxt:u64
        let s_408_1: u64 = fn_state.tempxt;
        // D s_408_2: cast zx s_408_1 -> bv
        let s_408_2: Bits = Bits::new(s_408_1 as u128, 64u16);
        // C s_408_3: const #1u : u64
        let s_408_3: u64 = 1;
        // D s_408_4: bit-extract s_408_2 s_408_0 s_408_3
        let s_408_4: Bits = (Bits::new(
            ((s_408_2) >> (s_408_0)).value(),
            u16::try_from(s_408_3).unwrap(),
        ));
        // D s_408_5: cast reint s_408_4 -> u8
        let s_408_5: bool = ((s_408_4.value()) != 0);
        // C s_408_6: const #0s : i
        let s_408_6: i128 = 0;
        // C s_408_7: const #0u : u64
        let s_408_7: u64 = 0;
        // D s_408_8: cast zx s_408_5 -> u64
        let s_408_8: u64 = (s_408_5 as u64);
        // C s_408_9: const #1u : u64
        let s_408_9: u64 = 1;
        // D s_408_10: and s_408_8 s_408_9
        let s_408_10: u64 = ((s_408_8) & (s_408_9));
        // D s_408_11: cmp-eq s_408_10 s_408_9
        let s_408_11: bool = ((s_408_10) == (s_408_9));
        // D s_408_12: lsl s_408_8 s_408_6
        let s_408_12: u64 = s_408_8 << s_408_6;
        // D s_408_13: or s_408_7 s_408_12
        let s_408_13: u64 = ((s_408_7) | (s_408_12));
        // D s_408_14: cmpl s_408_12
        let s_408_14: u64 = !s_408_12;
        // D s_408_15: and s_408_7 s_408_14
        let s_408_15: u64 = ((s_408_7) & (s_408_14));
        // D s_408_16: select s_408_11 s_408_13 s_408_15
        let s_408_16: u64 = if s_408_11 { s_408_13 } else { s_408_15 };
        // D s_408_17: cast trunc s_408_16 -> u8
        let s_408_17: bool = ((s_408_16) != 0);
        // D s_408_18: cast zx s_408_17 -> bv
        let s_408_18: Bits = Bits::new(s_408_17 as u128, 1u16);
        // C s_408_19: const #1u : u8
        let s_408_19: bool = true;
        // C s_408_20: cast zx s_408_19 -> bv
        let s_408_20: Bits = Bits::new(s_408_19 as u128, 1u16);
        // D s_408_21: cmp-eq s_408_18 s_408_20
        let s_408_21: bool = ((s_408_18) == (s_408_20));
        // N s_408_22: branch s_408_21 b411 b409
        if s_408_21 {
            return block_411(state, tracer, fn_state);
        } else {
            return block_409(state, tracer, fn_state);
        };
    }
    fn block_409<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_409_0: jump b410
        return block_410(state, tracer, fn_state);
    }
    fn block_410<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_410_0: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_411_0: const #() : ()
        let s_411_0: () = ();
        // S s_411_1: call AArch64_ClearEventCounters(s_411_0)
        let s_411_1: () = AArch64_ClearEventCounters(state, tracer, s_411_0);
        // N s_411_2: jump b410
        return block_410(state, tracer, fn_state);
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_412_0: const #64s : i
        let s_412_0: i128 = 64;
        // S s_412_1: call Zeros(s_412_0)
        let s_412_1: Bits = Zeros(state, tracer, s_412_0);
        // S s_412_2: cast reint s_412_1 -> u64
        let s_412_2: u64 = (s_412_1.value() as u64);
        // S s_412_3: call Mk_PMCCNTR_EL0_Type(s_412_2)
        let s_412_3: ProductType5c790c8ef59cc8b2 = Mk_PMCCNTR_EL0_Type(
            state,
            tracer,
            s_412_2,
        );
        // C s_412_4: const #104776u : u32
        let s_412_4: u32 = 104776;
        // N s_412_5: write-reg s_412_4 <= s_412_3
        let s_412_5: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_412_4 as isize, s_412_3);
            tracer.write_register(s_412_4 as isize, s_412_3);
        };
        // N s_412_6: jump b408
        return block_408(state, tracer, fn_state);
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_413_0: const #12s : i
        let s_413_0: i128 = 12;
        // D s_413_1: read-var crm:i
        let s_413_1: i128 = fn_state.crm;
        // D s_413_2: cmp-eq s_413_1 s_413_0
        let s_413_2: bool = ((s_413_1) == (s_413_0));
        // D s_413_3: write-var gs#141467 <= s_413_2
        fn_state.gs_141467 = s_413_2;
        // N s_413_4: jump b213
        return block_213(state, tracer, fn_state);
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_414_0: const #0s : i
        let s_414_0: i128 = 0;
        // D s_414_1: read-var op2:i
        let s_414_1: i128 = fn_state.op2;
        // D s_414_2: cmp-eq s_414_1 s_414_0
        let s_414_2: bool = ((s_414_1) == (s_414_0));
        // D s_414_3: write-var gs#141465 <= s_414_2
        fn_state.gs_141465 = s_414_2;
        // N s_414_4: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_415_0: const #3s : i
        let s_415_0: i128 = 3;
        // D s_415_1: read-var op1:i
        let s_415_1: i128 = fn_state.op1;
        // D s_415_2: cmp-eq s_415_1 s_415_0
        let s_415_2: bool = ((s_415_1) == (s_415_0));
        // D s_415_3: write-var gs#141463 <= s_415_2
        fn_state.gs_141463 = s_415_2;
        // N s_415_4: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_416_0: const #9s : i
        let s_416_0: i128 = 9;
        // D s_416_1: read-var crn:i
        let s_416_1: i128 = fn_state.crn;
        // D s_416_2: cmp-eq s_416_1 s_416_0
        let s_416_2: bool = ((s_416_1) == (s_416_0));
        // D s_416_3: write-var gs#141461 <= s_416_2
        fn_state.gs_141461 = s_416_2;
        // N s_416_4: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_417_0: const #2s : i
        let s_417_0: i128 = 2;
        // D s_417_1: read-var op2:i
        let s_417_1: i128 = fn_state.op2;
        // D s_417_2: cmp-eq s_417_1 s_417_0
        let s_417_2: bool = ((s_417_1) == (s_417_0));
        // N s_417_3: branch s_417_2 b423 b418
        if s_417_2 {
            return block_423(state, tracer, fn_state);
        } else {
            return block_418(state, tracer, fn_state);
        };
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_418_0: jump b419
        return block_419(state, tracer, fn_state);
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_419_0: const #1s : i
        let s_419_0: i128 = 1;
        // D s_419_1: read-var op2:i
        let s_419_1: i128 = fn_state.op2;
        // D s_419_2: cmp-eq s_419_1 s_419_0
        let s_419_2: bool = ((s_419_1) == (s_419_0));
        // N s_419_3: branch s_419_2 b422 b420
        if s_419_2 {
            return block_422(state, tracer, fn_state);
        } else {
            return block_420(state, tracer, fn_state);
        };
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_420_0: jump b421
        return block_421(state, tracer, fn_state);
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_421_0: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_422_0: read-var tempxt:u64
        let s_422_0: u64 = fn_state.tempxt;
        // D s_422_1: call Mk_PMINTENCLR_EL1_Type(s_422_0)
        let s_422_1: ProductType5c790c8ef59cc8b2 = Mk_PMINTENCLR_EL1_Type(
            state,
            tracer,
            s_422_0,
        );
        // C s_422_2: const #15448u : u32
        let s_422_2: u32 = 15448;
        // N s_422_3: write-reg s_422_2 <= s_422_1
        let s_422_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_422_2 as isize, s_422_1);
            tracer.write_register(s_422_2 as isize, s_422_1);
        };
        // N s_422_4: jump b421
        return block_421(state, tracer, fn_state);
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_423_0: read-var tempxt:u64
        let s_423_0: u64 = fn_state.tempxt;
        // D s_423_1: call Mk_PMINTENSET_EL1_Type(s_423_0)
        let s_423_1: ProductType5c790c8ef59cc8b2 = Mk_PMINTENSET_EL1_Type(
            state,
            tracer,
            s_423_0,
        );
        // C s_423_2: const #90320u : u32
        let s_423_2: u32 = 90320;
        // N s_423_3: write-reg s_423_2 <= s_423_1
        let s_423_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_423_2 as isize, s_423_1);
            tracer.write_register(s_423_2 as isize, s_423_1);
        };
        // N s_423_4: jump b419
        return block_419(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_424_0: const #14s : i
        let s_424_0: i128 = 14;
        // D s_424_1: read-var crm:i
        let s_424_1: i128 = fn_state.crm;
        // D s_424_2: cmp-eq s_424_1 s_424_0
        let s_424_2: bool = ((s_424_1) == (s_424_0));
        // D s_424_3: write-var gs#141458 <= s_424_2
        fn_state.gs_141458 = s_424_2;
        // N s_424_4: jump b203
        return block_203(state, tracer, fn_state);
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_425_0: const #1s : i
        let s_425_0: i128 = 1;
        // D s_425_1: read-var op2:i
        let s_425_1: i128 = fn_state.op2;
        // D s_425_2: cmp-eq s_425_1 s_425_0
        let s_425_2: bool = ((s_425_1) == (s_425_0));
        // N s_425_3: branch s_425_2 b428 b426
        if s_425_2 {
            return block_428(state, tracer, fn_state);
        } else {
            return block_426(state, tracer, fn_state);
        };
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_426_0: const #2s : i
        let s_426_0: i128 = 2;
        // D s_426_1: read-var op2:i
        let s_426_1: i128 = fn_state.op2;
        // D s_426_2: cmp-eq s_426_1 s_426_0
        let s_426_2: bool = ((s_426_1) == (s_426_0));
        // D s_426_3: write-var gs#141455 <= s_426_2
        fn_state.gs_141455 = s_426_2;
        // N s_426_4: jump b427
        return block_427(state, tracer, fn_state);
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_427_0: read-var gs#141455:u8
        let s_427_0: bool = fn_state.gs_141455;
        // D s_427_1: write-var gs#141456 <= s_427_0
        fn_state.gs_141456 = s_427_0;
        // N s_427_2: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_428_0: const #1u : u8
        let s_428_0: bool = true;
        // D s_428_1: write-var gs#141455 <= s_428_0
        fn_state.gs_141455 = s_428_0;
        // N s_428_2: jump b427
        return block_427(state, tracer, fn_state);
    }
    fn block_429<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_429_0: const #0s : i
        let s_429_0: i128 = 0;
        // D s_429_1: read-var op1:i
        let s_429_1: i128 = fn_state.op1;
        // D s_429_2: cmp-eq s_429_1 s_429_0
        let s_429_2: bool = ((s_429_1) == (s_429_0));
        // D s_429_3: write-var gs#141452 <= s_429_2
        fn_state.gs_141452 = s_429_2;
        // N s_429_4: jump b199
        return block_199(state, tracer, fn_state);
    }
    fn block_430<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_430_0: const #9s : i
        let s_430_0: i128 = 9;
        // D s_430_1: read-var crn:i
        let s_430_1: i128 = fn_state.crn;
        // D s_430_2: cmp-eq s_430_1 s_430_0
        let s_430_2: bool = ((s_430_1) == (s_430_0));
        // D s_430_3: write-var gs#141450 <= s_430_2
        fn_state.gs_141450 = s_430_2;
        // N s_430_4: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_431<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_431_0: const #2s : i
        let s_431_0: i128 = 2;
        // D s_431_1: read-var op2:i
        let s_431_1: i128 = fn_state.op2;
        // D s_431_2: cmp-eq s_431_1 s_431_0
        let s_431_2: bool = ((s_431_1) == (s_431_0));
        // N s_431_3: branch s_431_2 b437 b432
        if s_431_2 {
            return block_437(state, tracer, fn_state);
        } else {
            return block_432(state, tracer, fn_state);
        };
    }
    fn block_432<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_432_0: jump b433
        return block_433(state, tracer, fn_state);
    }
    fn block_433<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_433_0: const #1s : i
        let s_433_0: i128 = 1;
        // D s_433_1: read-var op2:i
        let s_433_1: i128 = fn_state.op2;
        // D s_433_2: cmp-eq s_433_1 s_433_0
        let s_433_2: bool = ((s_433_1) == (s_433_0));
        // N s_433_3: branch s_433_2 b436 b434
        if s_433_2 {
            return block_436(state, tracer, fn_state);
        } else {
            return block_434(state, tracer, fn_state);
        };
    }
    fn block_434<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_434_0: jump b435
        return block_435(state, tracer, fn_state);
    }
    fn block_435<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_435_0: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_436<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_436_0: read-var tempxt:u64
        let s_436_0: u64 = fn_state.tempxt;
        // D s_436_1: call Mk_PMCNTENCLR_EL0_Type(s_436_0)
        let s_436_1: ProductType5c790c8ef59cc8b2 = Mk_PMCNTENCLR_EL0_Type(
            state,
            tracer,
            s_436_0,
        );
        // C s_436_2: const #22632u : u32
        let s_436_2: u32 = 22632;
        // N s_436_3: write-reg s_436_2 <= s_436_1
        let s_436_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_436_2 as isize, s_436_1);
            tracer.write_register(s_436_2 as isize, s_436_1);
        };
        // N s_436_4: jump b435
        return block_435(state, tracer, fn_state);
    }
    fn block_437<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_437_0: read-var tempxt:u64
        let s_437_0: u64 = fn_state.tempxt;
        // D s_437_1: call Mk_PMCNTENSET_EL0_Type(s_437_0)
        let s_437_1: ProductType5c790c8ef59cc8b2 = Mk_PMCNTENSET_EL0_Type(
            state,
            tracer,
            s_437_0,
        );
        // C s_437_2: const #11736u : u32
        let s_437_2: u32 = 11736;
        // N s_437_3: write-reg s_437_2 <= s_437_1
        let s_437_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_437_2 as isize, s_437_1);
            tracer.write_register(s_437_2 as isize, s_437_1);
        };
        // N s_437_4: jump b433
        return block_433(state, tracer, fn_state);
    }
    fn block_438<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_438_0: const #12s : i
        let s_438_0: i128 = 12;
        // D s_438_1: read-var crm:i
        let s_438_1: i128 = fn_state.crm;
        // D s_438_2: cmp-eq s_438_1 s_438_0
        let s_438_2: bool = ((s_438_1) == (s_438_0));
        // D s_438_3: write-var gs#141447 <= s_438_2
        fn_state.gs_141447 = s_438_2;
        // N s_438_4: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_439<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_439_0: const #1s : i
        let s_439_0: i128 = 1;
        // D s_439_1: read-var op2:i
        let s_439_1: i128 = fn_state.op2;
        // D s_439_2: cmp-eq s_439_1 s_439_0
        let s_439_2: bool = ((s_439_1) == (s_439_0));
        // N s_439_3: branch s_439_2 b442 b440
        if s_439_2 {
            return block_442(state, tracer, fn_state);
        } else {
            return block_440(state, tracer, fn_state);
        };
    }
    fn block_440<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_440_0: const #2s : i
        let s_440_0: i128 = 2;
        // D s_440_1: read-var op2:i
        let s_440_1: i128 = fn_state.op2;
        // D s_440_2: cmp-eq s_440_1 s_440_0
        let s_440_2: bool = ((s_440_1) == (s_440_0));
        // D s_440_3: write-var gs#141444 <= s_440_2
        fn_state.gs_141444 = s_440_2;
        // N s_440_4: jump b441
        return block_441(state, tracer, fn_state);
    }
    fn block_441<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_441_0: read-var gs#141444:u8
        let s_441_0: bool = fn_state.gs_141444;
        // D s_441_1: write-var gs#141445 <= s_441_0
        fn_state.gs_141445 = s_441_0;
        // N s_441_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_442<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_442_0: const #1u : u8
        let s_442_0: bool = true;
        // D s_442_1: write-var gs#141444 <= s_442_0
        fn_state.gs_141444 = s_442_0;
        // N s_442_2: jump b441
        return block_441(state, tracer, fn_state);
    }
    fn block_443<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_443_0: const #3s : i
        let s_443_0: i128 = 3;
        // D s_443_1: read-var op1:i
        let s_443_1: i128 = fn_state.op1;
        // D s_443_2: cmp-eq s_443_1 s_443_0
        let s_443_2: bool = ((s_443_1) == (s_443_0));
        // D s_443_3: write-var gs#141441 <= s_443_2
        fn_state.gs_141441 = s_443_2;
        // N s_443_4: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_444<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_444_0: const #9s : i
        let s_444_0: i128 = 9;
        // D s_444_1: read-var crn:i
        let s_444_1: i128 = fn_state.crn;
        // D s_444_2: cmp-eq s_444_1 s_444_0
        let s_444_2: bool = ((s_444_1) == (s_444_0));
        // D s_444_3: write-var gs#141439 <= s_444_2
        fn_state.gs_141439 = s_444_2;
        // N s_444_4: jump b187
        return block_187(state, tracer, fn_state);
    }
    fn block_445<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_445_0: const #12s : i
        let s_445_0: i128 = 12;
        // D s_445_1: read-var crm:i
        let s_445_1: i128 = fn_state.crm;
        // D s_445_2: cmp-eq s_445_1 s_445_0
        let s_445_2: bool = ((s_445_1) == (s_445_0));
        // N s_445_3: branch s_445_2 b451 b446
        if s_445_2 {
            return block_451(state, tracer, fn_state);
        } else {
            return block_446(state, tracer, fn_state);
        };
    }
    fn block_446<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_446_0: jump b447
        return block_447(state, tracer, fn_state);
    }
    fn block_447<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_447_0: const #14s : i
        let s_447_0: i128 = 14;
        // D s_447_1: read-var crm:i
        let s_447_1: i128 = fn_state.crm;
        // D s_447_2: cmp-eq s_447_1 s_447_0
        let s_447_2: bool = ((s_447_1) == (s_447_0));
        // N s_447_3: branch s_447_2 b450 b448
        if s_447_2 {
            return block_450(state, tracer, fn_state);
        } else {
            return block_448(state, tracer, fn_state);
        };
    }
    fn block_448<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_448_0: jump b449
        return block_449(state, tracer, fn_state);
    }
    fn block_449<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_449_0: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_450<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_450_0: read-var tempxt:u64
        let s_450_0: u64 = fn_state.tempxt;
        // D s_450_1: call Mk_PMOVSCLR_EL0_Type(s_450_0)
        let s_450_1: ProductType5c790c8ef59cc8b2 = Mk_PMOVSCLR_EL0_Type(
            state,
            tracer,
            s_450_0,
        );
        // C s_450_2: const #104888u : u32
        let s_450_2: u32 = 104888;
        // N s_450_3: write-reg s_450_2 <= s_450_1
        let s_450_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_450_2 as isize, s_450_1);
            tracer.write_register(s_450_2 as isize, s_450_1);
        };
        // N s_450_4: jump b449
        return block_449(state, tracer, fn_state);
    }
    fn block_451<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_451_0: read-var tempxt:u64
        let s_451_0: u64 = fn_state.tempxt;
        // D s_451_1: call Mk_PMOVSSET_EL0_Type(s_451_0)
        let s_451_1: ProductType5c790c8ef59cc8b2 = Mk_PMOVSSET_EL0_Type(
            state,
            tracer,
            s_451_0,
        );
        // C s_451_2: const #104640u : u32
        let s_451_2: u32 = 104640;
        // N s_451_3: write-reg s_451_2 <= s_451_1
        let s_451_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_451_2 as isize, s_451_1);
            tracer.write_register(s_451_2 as isize, s_451_1);
        };
        // N s_451_4: jump b447
        return block_447(state, tracer, fn_state);
    }
    fn block_452<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_452_0: const #12s : i
        let s_452_0: i128 = 12;
        // D s_452_1: read-var crm:i
        let s_452_1: i128 = fn_state.crm;
        // D s_452_2: cmp-eq s_452_1 s_452_0
        let s_452_2: bool = ((s_452_1) == (s_452_0));
        // N s_452_3: branch s_452_2 b455 b453
        if s_452_2 {
            return block_455(state, tracer, fn_state);
        } else {
            return block_453(state, tracer, fn_state);
        };
    }
    fn block_453<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_453_0: const #14s : i
        let s_453_0: i128 = 14;
        // D s_453_1: read-var crm:i
        let s_453_1: i128 = fn_state.crm;
        // D s_453_2: cmp-eq s_453_1 s_453_0
        let s_453_2: bool = ((s_453_1) == (s_453_0));
        // D s_453_3: write-var gs#141435 <= s_453_2
        fn_state.gs_141435 = s_453_2;
        // N s_453_4: jump b454
        return block_454(state, tracer, fn_state);
    }
    fn block_454<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_454_0: read-var gs#141435:u8
        let s_454_0: bool = fn_state.gs_141435;
        // D s_454_1: write-var gs#141436 <= s_454_0
        fn_state.gs_141436 = s_454_0;
        // N s_454_2: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_455<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_455_0: const #1u : u8
        let s_455_0: bool = true;
        // D s_455_1: write-var gs#141435 <= s_455_0
        fn_state.gs_141435 = s_455_0;
        // N s_455_2: jump b454
        return block_454(state, tracer, fn_state);
    }
    fn block_456<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_456_0: const #3s : i
        let s_456_0: i128 = 3;
        // D s_456_1: read-var op2:i
        let s_456_1: i128 = fn_state.op2;
        // D s_456_2: cmp-eq s_456_1 s_456_0
        let s_456_2: bool = ((s_456_1) == (s_456_0));
        // D s_456_3: write-var gs#141432 <= s_456_2
        fn_state.gs_141432 = s_456_2;
        // N s_456_4: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_457<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_457_0: const #3s : i
        let s_457_0: i128 = 3;
        // D s_457_1: read-var op1:i
        let s_457_1: i128 = fn_state.op1;
        // D s_457_2: cmp-eq s_457_1 s_457_0
        let s_457_2: bool = ((s_457_1) == (s_457_0));
        // D s_457_3: write-var gs#141430 <= s_457_2
        fn_state.gs_141430 = s_457_2;
        // N s_457_4: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_458<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_458_0: const #9s : i
        let s_458_0: i128 = 9;
        // D s_458_1: read-var crn:i
        let s_458_1: i128 = fn_state.crn;
        // D s_458_2: cmp-eq s_458_1 s_458_0
        let s_458_2: bool = ((s_458_1) == (s_458_0));
        // D s_458_3: write-var gs#141428 <= s_458_2
        fn_state.gs_141428 = s_458_2;
        // N s_458_4: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_459<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_459_0: const #0u : u8
        let s_459_0: bool = false;
        // S s_459_1: call TakeReset(s_459_0)
        let s_459_1: () = TakeReset(state, tracer, s_459_0);
        // N s_459_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_460<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_460_0: const #1s : i
        let s_460_0: i128 = 1;
        // D s_460_1: read-var tempxt:u64
        let s_460_1: u64 = fn_state.tempxt;
        // D s_460_2: cast zx s_460_1 -> bv
        let s_460_2: Bits = Bits::new(s_460_1 as u128, 64u16);
        // C s_460_3: const #1u : u64
        let s_460_3: u64 = 1;
        // D s_460_4: bit-extract s_460_2 s_460_0 s_460_3
        let s_460_4: Bits = (Bits::new(
            ((s_460_2) >> (s_460_0)).value(),
            u16::try_from(s_460_3).unwrap(),
        ));
        // D s_460_5: cast reint s_460_4 -> u8
        let s_460_5: bool = ((s_460_4.value()) != 0);
        // C s_460_6: const #0s : i
        let s_460_6: i128 = 0;
        // C s_460_7: const #0u : u64
        let s_460_7: u64 = 0;
        // D s_460_8: cast zx s_460_5 -> u64
        let s_460_8: u64 = (s_460_5 as u64);
        // C s_460_9: const #1u : u64
        let s_460_9: u64 = 1;
        // D s_460_10: and s_460_8 s_460_9
        let s_460_10: u64 = ((s_460_8) & (s_460_9));
        // D s_460_11: cmp-eq s_460_10 s_460_9
        let s_460_11: bool = ((s_460_10) == (s_460_9));
        // D s_460_12: lsl s_460_8 s_460_6
        let s_460_12: u64 = s_460_8 << s_460_6;
        // D s_460_13: or s_460_7 s_460_12
        let s_460_13: u64 = ((s_460_7) | (s_460_12));
        // D s_460_14: cmpl s_460_12
        let s_460_14: u64 = !s_460_12;
        // D s_460_15: and s_460_7 s_460_14
        let s_460_15: u64 = ((s_460_7) & (s_460_14));
        // D s_460_16: select s_460_11 s_460_13 s_460_15
        let s_460_16: u64 = if s_460_11 { s_460_13 } else { s_460_15 };
        // D s_460_17: cast trunc s_460_16 -> u8
        let s_460_17: bool = ((s_460_16) != 0);
        // D s_460_18: cast zx s_460_17 -> bv
        let s_460_18: Bits = Bits::new(s_460_17 as u128, 1u16);
        // C s_460_19: const #1u : u8
        let s_460_19: bool = true;
        // C s_460_20: cast zx s_460_19 -> bv
        let s_460_20: Bits = Bits::new(s_460_19 as u128, 1u16);
        // D s_460_21: cmp-eq s_460_18 s_460_20
        let s_460_21: bool = ((s_460_18) == (s_460_20));
        // D s_460_22: write-var gs#141425 <= s_460_21
        fn_state.gs_141425 = s_460_21;
        // N s_460_23: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_461<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_461_0: const #0s : i
        let s_461_0: i128 = 0;
        // D s_461_1: read-var crm:i
        let s_461_1: i128 = fn_state.crm;
        // D s_461_2: cmp-eq s_461_1 s_461_0
        let s_461_2: bool = ((s_461_1) == (s_461_0));
        // D s_461_3: write-var gs#141422 <= s_461_2
        fn_state.gs_141422 = s_461_2;
        // N s_461_4: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_462<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_462_0: const #2s : i
        let s_462_0: i128 = 2;
        // D s_462_1: read-var op2:i
        let s_462_1: i128 = fn_state.op2;
        // D s_462_2: cmp-eq s_462_1 s_462_0
        let s_462_2: bool = ((s_462_1) == (s_462_0));
        // D s_462_3: write-var gs#141420 <= s_462_2
        fn_state.gs_141420 = s_462_2;
        // N s_462_4: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_463<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_463_0: const #6s : i
        let s_463_0: i128 = 6;
        // D s_463_1: read-var op1:i
        let s_463_1: i128 = fn_state.op1;
        // D s_463_2: cmp-eq s_463_1 s_463_0
        let s_463_2: bool = ((s_463_1) == (s_463_0));
        // N s_463_3: branch s_463_2 b469 b464
        if s_463_2 {
            return block_469(state, tracer, fn_state);
        } else {
            return block_464(state, tracer, fn_state);
        };
    }
    fn block_464<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_464_0: const #4s : i
        let s_464_0: i128 = 4;
        // D s_464_1: read-var op1:i
        let s_464_1: i128 = fn_state.op1;
        // D s_464_2: cmp-eq s_464_1 s_464_0
        let s_464_2: bool = ((s_464_1) == (s_464_0));
        // D s_464_3: write-var gs#141415 <= s_464_2
        fn_state.gs_141415 = s_464_2;
        // N s_464_4: jump b465
        return block_465(state, tracer, fn_state);
    }
    fn block_465<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_465_0: read-var gs#141415:u8
        let s_465_0: bool = fn_state.gs_141415;
        // N s_465_1: branch s_465_0 b468 b466
        if s_465_0 {
            return block_468(state, tracer, fn_state);
        } else {
            return block_466(state, tracer, fn_state);
        };
    }
    fn block_466<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_466_0: const #0s : i
        let s_466_0: i128 = 0;
        // D s_466_1: read-var op1:i
        let s_466_1: i128 = fn_state.op1;
        // D s_466_2: cmp-eq s_466_1 s_466_0
        let s_466_2: bool = ((s_466_1) == (s_466_0));
        // D s_466_3: write-var gs#141417 <= s_466_2
        fn_state.gs_141417 = s_466_2;
        // N s_466_4: jump b467
        return block_467(state, tracer, fn_state);
    }
    fn block_467<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_467_0: read-var gs#141417:u8
        let s_467_0: bool = fn_state.gs_141417;
        // D s_467_1: write-var gs#141418 <= s_467_0
        fn_state.gs_141418 = s_467_0;
        // N s_467_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_468<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_468_0: const #1u : u8
        let s_468_0: bool = true;
        // D s_468_1: write-var gs#141417 <= s_468_0
        fn_state.gs_141417 = s_468_0;
        // N s_468_2: jump b467
        return block_467(state, tracer, fn_state);
    }
    fn block_469<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_469_0: const #1u : u8
        let s_469_0: bool = true;
        // D s_469_1: write-var gs#141415 <= s_469_0
        fn_state.gs_141415 = s_469_0;
        // N s_469_2: jump b465
        return block_465(state, tracer, fn_state);
    }
    fn block_470<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_470_0: const #12u : u8
        let s_470_0: u8 = 12;
        // C s_470_1: cast zx s_470_0 -> bv
        let s_470_1: Bits = Bits::new(s_470_0 as u128, 4u16);
        // C s_470_2: cast zx s_470_1 -> i
        let s_470_2: i128 = (s_470_1.value() as i128);
        // C s_470_3: cast reint s_470_2 -> i64
        let s_470_3: i64 = (s_470_2 as i64);
        // C s_470_4: cast zx s_470_3 -> i
        let s_470_4: i128 = (i128::try_from(s_470_3).unwrap());
        // D s_470_5: read-var crn:i
        let s_470_5: i128 = fn_state.crn;
        // D s_470_6: cmp-eq s_470_5 s_470_4
        let s_470_6: bool = ((s_470_5) == (s_470_4));
        // D s_470_7: write-var gs#141412 <= s_470_6
        fn_state.gs_141412 = s_470_6;
        // N s_470_8: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_471<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_471_0: const #8s : i
        let s_471_0: i128 = 8;
        // D s_471_1: read-var crm:i
        let s_471_1: i128 = fn_state.crm;
        // D s_471_2: cmp-eq s_471_1 s_471_0
        let s_471_2: bool = ((s_471_1) == (s_471_0));
        // N s_471_3: branch s_471_2 b475 b472
        if s_471_2 {
            return block_475(state, tracer, fn_state);
        } else {
            return block_472(state, tracer, fn_state);
        };
    }
    fn block_472<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_472_0: const #9s : i
        let s_472_0: i128 = 9;
        // D s_472_1: read-var crm:i
        let s_472_1: i128 = fn_state.crm;
        // D s_472_2: cmp-eq s_472_1 s_472_0
        let s_472_2: bool = ((s_472_1) == (s_472_0));
        // N s_472_3: branch s_472_2 b474 b473
        if s_472_2 {
            return block_474(state, tracer, fn_state);
        } else {
            return block_473(state, tracer, fn_state);
        };
    }
    fn block_473<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_473_0: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_474<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_474_0: const #0s : i
        let s_474_0: i128 = 0;
        // D s_474_1: read-var tempxt:u64
        let s_474_1: u64 = fn_state.tempxt;
        // D s_474_2: cast zx s_474_1 -> bv
        let s_474_2: Bits = Bits::new(s_474_1 as u128, 64u16);
        // C s_474_3: const #1s : i64
        let s_474_3: i64 = 1;
        // C s_474_4: cast zx s_474_3 -> i
        let s_474_4: i128 = (i128::try_from(s_474_3).unwrap());
        // C s_474_5: const #31s : i
        let s_474_5: i128 = 31;
        // C s_474_6: add s_474_5 s_474_4
        let s_474_6: i128 = (s_474_5 + s_474_4);
        // D s_474_7: bit-extract s_474_2 s_474_0 s_474_6
        let s_474_7: Bits = (Bits::new(
            ((s_474_2) >> (s_474_0)).value(),
            u16::try_from(s_474_6).unwrap(),
        ));
        // D s_474_8: cast reint s_474_7 -> u32
        let s_474_8: u32 = (s_474_7.value() as u32);
        // D s_474_9: cast zx s_474_8 -> bv
        let s_474_9: Bits = Bits::new(s_474_8 as u128, 32u16);
        // D s_474_10: not s_474_9
        let s_474_10: Bits = !s_474_9;
        // D s_474_11: cast reint s_474_10 -> u32
        let s_474_11: u32 = (s_474_10.value() as u32);
        // C s_474_12: const #22680u : u32
        let s_474_12: u32 = 22680;
        // D s_474_13: read-reg s_474_12:u32
        let s_474_13: u32 = {
            let value = state.read_register::<u32>(s_474_12 as isize);
            tracer.read_register(s_474_12 as isize, value);
            value
        };
        // D s_474_14: cast zx s_474_13 -> bv
        let s_474_14: Bits = Bits::new(s_474_13 as u128, 32u16);
        // D s_474_15: cast zx s_474_11 -> bv
        let s_474_15: Bits = Bits::new(s_474_11 as u128, 32u16);
        // D s_474_16: and s_474_14 s_474_15
        let s_474_16: Bits = ((s_474_14) & (s_474_15));
        // D s_474_17: cast reint s_474_16 -> u32
        let s_474_17: u32 = (s_474_16.value() as u32);
        // C s_474_18: const #22680u : u32
        let s_474_18: u32 = 22680;
        // N s_474_19: write-reg s_474_18 <= s_474_17
        let s_474_19: () = {
            state.write_register::<u32>(s_474_18 as isize, s_474_17);
            tracer.write_register(s_474_18 as isize, s_474_17);
        };
        // N s_474_20: return
        return;
    }
    fn block_475<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_475_0: const #0s : i
        let s_475_0: i128 = 0;
        // D s_475_1: read-var tempxt:u64
        let s_475_1: u64 = fn_state.tempxt;
        // D s_475_2: cast zx s_475_1 -> bv
        let s_475_2: Bits = Bits::new(s_475_1 as u128, 64u16);
        // C s_475_3: const #1s : i64
        let s_475_3: i64 = 1;
        // C s_475_4: cast zx s_475_3 -> i
        let s_475_4: i128 = (i128::try_from(s_475_3).unwrap());
        // C s_475_5: const #31s : i
        let s_475_5: i128 = 31;
        // C s_475_6: add s_475_5 s_475_4
        let s_475_6: i128 = (s_475_5 + s_475_4);
        // D s_475_7: bit-extract s_475_2 s_475_0 s_475_6
        let s_475_7: Bits = (Bits::new(
            ((s_475_2) >> (s_475_0)).value(),
            u16::try_from(s_475_6).unwrap(),
        ));
        // D s_475_8: cast reint s_475_7 -> u32
        let s_475_8: u32 = (s_475_7.value() as u32);
        // C s_475_9: const #22680u : u32
        let s_475_9: u32 = 22680;
        // D s_475_10: read-reg s_475_9:u32
        let s_475_10: u32 = {
            let value = state.read_register::<u32>(s_475_9 as isize);
            tracer.read_register(s_475_9 as isize, value);
            value
        };
        // D s_475_11: cast zx s_475_10 -> bv
        let s_475_11: Bits = Bits::new(s_475_10 as u128, 32u16);
        // D s_475_12: cast zx s_475_8 -> bv
        let s_475_12: Bits = Bits::new(s_475_8 as u128, 32u16);
        // D s_475_13: or s_475_11 s_475_12
        let s_475_13: Bits = ((s_475_11) | (s_475_12));
        // D s_475_14: cast reint s_475_13 -> u32
        let s_475_14: u32 = (s_475_13.value() as u32);
        // C s_475_15: const #22680u : u32
        let s_475_15: u32 = 22680;
        // N s_475_16: write-reg s_475_15 <= s_475_14
        let s_475_16: () = {
            state.write_register::<u32>(s_475_15 as isize, s_475_14);
            tracer.write_register(s_475_15 as isize, s_475_14);
        };
        // N s_475_17: return
        return;
    }
    fn block_476<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_476_0: const #6s : i
        let s_476_0: i128 = 6;
        // D s_476_1: read-var op2:i
        let s_476_1: i128 = fn_state.op2;
        // D s_476_2: cmp-eq s_476_1 s_476_0
        let s_476_2: bool = ((s_476_1) == (s_476_0));
        // D s_476_3: write-var gs#141410 <= s_476_2
        fn_state.gs_141410 = s_476_2;
        // N s_476_4: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_477<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_477_0: const #7s : i
        let s_477_0: i128 = 7;
        // D s_477_1: read-var crn:i
        let s_477_1: i128 = fn_state.crn;
        // D s_477_2: cmp-eq s_477_1 s_477_0
        let s_477_2: bool = ((s_477_1) == (s_477_0));
        // D s_477_3: write-var gs#141408 <= s_477_2
        fn_state.gs_141408 = s_477_2;
        // N s_477_4: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_478<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_478_0: const #1s : i
        let s_478_0: i128 = 1;
        // D s_478_1: read-var op1:i
        let s_478_1: i128 = fn_state.op1;
        // D s_478_2: cmp-eq s_478_1 s_478_0
        let s_478_2: bool = ((s_478_1) == (s_478_0));
        // D s_478_3: write-var gs#141406 <= s_478_2
        fn_state.gs_141406 = s_478_2;
        // N s_478_4: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_479<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_479_0: const #10128u : u32
        let s_479_0: u32 = 10128;
        // D s_479_1: read-reg s_479_0:struct
        let s_479_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_479_0 as isize);
            tracer.read_register(s_479_0 as isize, value);
            value
        };
        // D s_479_2: call _get_OSLSR_EL1_Type_OSLK(s_479_1)
        let s_479_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_479_1);
        // D s_479_3: cast zx s_479_2 -> bv
        let s_479_3: Bits = Bits::new(s_479_2 as u128, 1u16);
        // C s_479_4: const #1u : u8
        let s_479_4: bool = true;
        // C s_479_5: cast zx s_479_4 -> bv
        let s_479_5: Bits = Bits::new(s_479_4 as u128, 1u16);
        // D s_479_6: cmp-eq s_479_3 s_479_5
        let s_479_6: bool = ((s_479_3) == (s_479_5));
        // N s_479_7: branch s_479_6 b485 b480
        if s_479_6 {
            return block_485(state, tracer, fn_state);
        } else {
            return block_480(state, tracer, fn_state);
        };
    }
    fn block_480<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_480_0: const #0u : u8
        let s_480_0: bool = false;
        // D s_480_1: write-var gs#141717 <= s_480_0
        fn_state.gs_141717 = s_480_0;
        // N s_480_2: jump b481
        return block_481(state, tracer, fn_state);
    }
    fn block_481<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_481_0: read-var gs#141717:u8
        let s_481_0: bool = fn_state.gs_141717;
        // N s_481_1: branch s_481_0 b484 b482
        if s_481_0 {
            return block_484(state, tracer, fn_state);
        } else {
            return block_482(state, tracer, fn_state);
        };
    }
    fn block_482<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_482_0: jump b483
        return block_483(state, tracer, fn_state);
    }
    fn block_483<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_483_0: const #90800u : u32
        let s_483_0: u32 = 90800;
        // D s_483_1: read-reg s_483_0:struct
        let s_483_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_483_0 as isize);
            tracer.read_register(s_483_0 as isize, value);
            value
        };
        // D s_483_2: call _get_OSLAR_EL1_Type_OSLK(s_483_1)
        let s_483_2: bool = u_get_OSLAR_EL1_Type_OSLK(state, tracer, s_483_1);
        // C s_483_3: const #10128u : u32
        let s_483_3: u32 = 10128;
        // D s_483_4: read-reg s_483_3:struct
        let s_483_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_483_3 as isize);
            tracer.read_register(s_483_3 as isize, value);
            value
        };
        // C s_483_5: const #10128u : u32
        let s_483_5: u32 = 10128;
        // N s_483_6: write-reg s_483_5 <= s_483_4
        let s_483_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_483_5 as isize, s_483_4);
            tracer.write_register(s_483_5 as isize, s_483_4);
        };
        // C s_483_7: const #10128u : u32
        let s_483_7: u32 = 10128;
        // D s_483_8: read-reg s_483_7:struct
        let s_483_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_483_7 as isize);
            tracer.read_register(s_483_7 as isize, value);
            value
        };
        // D s_483_9: call _get_OSLSR_EL1_Type_OSLK(s_483_8)
        let s_483_9: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_483_8);
        // C s_483_10: const #14040u : u32
        let s_483_10: u32 = 14040;
        // D s_483_11: read-reg s_483_10:struct
        let s_483_11: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_483_10 as isize);
            tracer.read_register(s_483_10 as isize, value);
            value
        };
        // C s_483_12: const #14040u : u32
        let s_483_12: u32 = 14040;
        // N s_483_13: write-reg s_483_12 <= s_483_11
        let s_483_13: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_483_12 as isize, s_483_11);
            tracer.write_register(s_483_12 as isize, s_483_11);
        };
        // C s_483_14: const #() : ()
        let s_483_14: () = ();
        // S s_483_15: call DBGOSLSR_read(s_483_14)
        let s_483_15: ProductType700c18a878c5601b = DBGOSLSR_read(
            state,
            tracer,
            s_483_14,
        );
        // C s_483_16: const #10128u : u32
        let s_483_16: u32 = 10128;
        // D s_483_17: read-reg s_483_16:struct
        let s_483_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_483_16 as isize);
            tracer.read_register(s_483_16 as isize, value);
            value
        };
        // D s_483_18: call _get_OSLSR_EL1_Type_OSLK(s_483_17)
        let s_483_18: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_483_17);
        // D s_483_19: call _update_DBGOSLSR_Type_OSLK(s_483_15, s_483_18)
        let s_483_19: ProductType700c18a878c5601b = u_update_DBGOSLSR_Type_OSLK(
            state,
            tracer,
            s_483_15,
            s_483_18,
        );
        // D s_483_20: call DBGOSLSR_write(s_483_19)
        let s_483_20: () = DBGOSLSR_write(state, tracer, s_483_19);
        // N s_483_21: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_484<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_484_0: const #() : ()
        let s_484_0: () = ();
        // S s_484_1: call CheckOSUnlockCatch(s_484_0)
        let s_484_1: () = CheckOSUnlockCatch(state, tracer, s_484_0);
        // N s_484_2: jump b483
        return block_483(state, tracer, fn_state);
    }
    fn block_485<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_485_0: const #90800u : u32
        let s_485_0: u32 = 90800;
        // D s_485_1: read-reg s_485_0:struct
        let s_485_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_485_0 as isize);
            tracer.read_register(s_485_0 as isize, value);
            value
        };
        // D s_485_2: call _get_OSLAR_EL1_Type_OSLK(s_485_1)
        let s_485_2: bool = u_get_OSLAR_EL1_Type_OSLK(state, tracer, s_485_1);
        // D s_485_3: cast zx s_485_2 -> bv
        let s_485_3: Bits = Bits::new(s_485_2 as u128, 1u16);
        // C s_485_4: const #0u : u8
        let s_485_4: bool = false;
        // C s_485_5: cast zx s_485_4 -> bv
        let s_485_5: Bits = Bits::new(s_485_4 as u128, 1u16);
        // D s_485_6: cmp-eq s_485_3 s_485_5
        let s_485_6: bool = ((s_485_3) == (s_485_5));
        // D s_485_7: write-var gs#141717 <= s_485_6
        fn_state.gs_141717 = s_485_6;
        // N s_485_8: jump b481
        return block_481(state, tracer, fn_state);
    }
    fn block_486<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_486_0: const #4s : i
        let s_486_0: i128 = 4;
        // D s_486_1: read-var op2:i
        let s_486_1: i128 = fn_state.op2;
        // D s_486_2: cmp-eq s_486_1 s_486_0
        let s_486_2: bool = ((s_486_1) == (s_486_0));
        // D s_486_3: write-var gs#141403 <= s_486_2
        fn_state.gs_141403 = s_486_2;
        // N s_486_4: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_487<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_487_0: const #0s : i
        let s_487_0: i128 = 0;
        // D s_487_1: read-var op1:i
        let s_487_1: i128 = fn_state.op1;
        // D s_487_2: cmp-eq s_487_1 s_487_0
        let s_487_2: bool = ((s_487_1) == (s_487_0));
        // D s_487_3: write-var gs#141401 <= s_487_2
        fn_state.gs_141401 = s_487_2;
        // N s_487_4: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_488<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_488_0: const #2s : i
        let s_488_0: i128 = 2;
        // D s_488_1: read-var op0:i
        let s_488_1: i128 = fn_state.op0;
        // D s_488_2: cmp-eq s_488_1 s_488_0
        let s_488_2: bool = ((s_488_1) == (s_488_0));
        // D s_488_3: write-var gs#141399 <= s_488_2
        fn_state.gs_141399 = s_488_2;
        // N s_488_4: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_489<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_489_0: const #1s : i
        let s_489_0: i128 = 1;
        // D s_489_1: read-var crn:i
        let s_489_1: i128 = fn_state.crn;
        // D s_489_2: cmp-eq s_489_1 s_489_0
        let s_489_2: bool = ((s_489_1) == (s_489_0));
        // D s_489_3: write-var gs#141397 <= s_489_2
        fn_state.gs_141397 = s_489_2;
        // N s_489_4: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_490<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_490_0: const #21088u : u32
        let s_490_0: u32 = 21088;
        // D s_490_1: read-reg s_490_0:struct
        let s_490_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_490_0 as isize);
            tracer.read_register(s_490_0 as isize, value);
            value
        };
        // C s_490_2: const #21088u : u32
        let s_490_2: u32 = 21088;
        // N s_490_3: write-reg s_490_2 <= s_490_1
        let s_490_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_490_2 as isize, s_490_1);
            tracer.write_register(s_490_2 as isize, s_490_1);
        };
        // N s_490_4: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_491<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_491_0: const #8s : i
        let s_491_0: i128 = 8;
        // D s_491_1: read-var crm:i
        let s_491_1: i128 = fn_state.crm;
        // D s_491_2: cmp-eq s_491_1 s_491_0
        let s_491_2: bool = ((s_491_1) == (s_491_0));
        // D s_491_3: write-var gs#141394 <= s_491_2
        fn_state.gs_141394 = s_491_2;
        // N s_491_4: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_492<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_492_0: const #6s : i
        let s_492_0: i128 = 6;
        // D s_492_1: read-var op2:i
        let s_492_1: i128 = fn_state.op2;
        // D s_492_2: cmp-eq s_492_1 s_492_0
        let s_492_2: bool = ((s_492_1) == (s_492_0));
        // D s_492_3: write-var gs#141392 <= s_492_2
        fn_state.gs_141392 = s_492_2;
        // N s_492_4: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_493<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_493_0: const #7s : i
        let s_493_0: i128 = 7;
        // D s_493_1: read-var crn:i
        let s_493_1: i128 = fn_state.crn;
        // D s_493_2: cmp-eq s_493_1 s_493_0
        let s_493_2: bool = ((s_493_1) == (s_493_0));
        // D s_493_3: write-var gs#141390 <= s_493_2
        fn_state.gs_141390 = s_493_2;
        // N s_493_4: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_494<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_494_0: const #0s : i
        let s_494_0: i128 = 0;
        // D s_494_1: read-var op1:i
        let s_494_1: i128 = fn_state.op1;
        // D s_494_2: cmp-eq s_494_1 s_494_0
        let s_494_2: bool = ((s_494_1) == (s_494_0));
        // D s_494_3: write-var gs#141388 <= s_494_2
        fn_state.gs_141388 = s_494_2;
        // N s_494_4: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_495<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_495_0: const #64s : i64
        let s_495_0: i64 = 64;
        // D s_495_1: read-var tempxt2:u64
        let s_495_1: u64 = fn_state.tempxt2;
        // D s_495_2: cast zx s_495_1 -> bv
        let s_495_2: Bits = Bits::new(s_495_1 as u128, 64u16);
        // D s_495_3: read-var t:i
        let s_495_3: i128 = fn_state.t;
        // D s_495_4: call X_set(s_495_3, s_495_0, s_495_2)
        let s_495_4: () = X_set(state, tracer, s_495_3, s_495_0, s_495_2);
        // N s_495_5: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_496<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_496_0: const #64s : i64
        let s_496_0: i64 = 64;
        // D s_496_1: read-var t:i
        let s_496_1: i128 = fn_state.t;
        // D s_496_2: call X_read(s_496_1, s_496_0)
        let s_496_2: Bits = X_read(state, tracer, s_496_1, s_496_0);
        // D s_496_3: cast reint s_496_2 -> u64
        let s_496_3: u64 = (s_496_2.value() as u64);
        // D s_496_4: write-var tempxt2 <= s_496_3
        fn_state.tempxt2 = s_496_3;
        // C s_496_5: const #64s : i64
        let s_496_5: i64 = 64;
        // D s_496_6: read-var tempxt:u64
        let s_496_6: u64 = fn_state.tempxt;
        // D s_496_7: cast zx s_496_6 -> bv
        let s_496_7: Bits = Bits::new(s_496_6 as u128, 64u16);
        // D s_496_8: read-var t:i
        let s_496_8: i128 = fn_state.t;
        // D s_496_9: call X_set(s_496_8, s_496_5, s_496_7)
        let s_496_9: () = X_set(state, tracer, s_496_8, s_496_5, s_496_7);
        // C s_496_10: const #1u : u8
        let s_496_10: bool = true;
        // D s_496_11: write-var restore_xt <= s_496_10
        fn_state.restore_xt = s_496_10;
        // N s_496_12: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_497<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_497_0: const #8s : i
        let s_497_0: i128 = 8;
        // D s_497_1: read-var crm:i
        let s_497_1: i128 = fn_state.crm;
        // D s_497_2: cmp-eq s_497_1 s_497_0
        let s_497_2: bool = ((s_497_1) == (s_497_0));
        // N s_497_3: branch s_497_2 b503 b498
        if s_497_2 {
            return block_503(state, tracer, fn_state);
        } else {
            return block_498(state, tracer, fn_state);
        };
    }
    fn block_498<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_498_0: jump b499
        return block_499(state, tracer, fn_state);
    }
    fn block_499<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_499_0: const #9s : i
        let s_499_0: i128 = 9;
        // D s_499_1: read-var crm:i
        let s_499_1: i128 = fn_state.crm;
        // D s_499_2: cmp-eq s_499_1 s_499_0
        let s_499_2: bool = ((s_499_1) == (s_499_0));
        // N s_499_3: branch s_499_2 b502 b500
        if s_499_2 {
            return block_502(state, tracer, fn_state);
        } else {
            return block_500(state, tracer, fn_state);
        };
    }
    fn block_500<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_500_0: jump b501
        return block_501(state, tracer, fn_state);
    }
    fn block_501<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_501_0: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_502<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_502_0: const #21088u : u32
        let s_502_0: u32 = 21088;
        // D s_502_1: read-reg s_502_0:u64
        let s_502_1: u64 = {
            let value = state.read_register::<u64>(s_502_0 as isize);
            tracer.read_register(s_502_0 as isize, value);
            value
        };
        // C s_502_2: const #0s : i
        let s_502_2: i128 = 0;
        // D s_502_3: cast zx s_502_1 -> bv
        let s_502_3: Bits = Bits::new(s_502_1 as u128, 64u16);
        // C s_502_4: const #1s : i64
        let s_502_4: i64 = 1;
        // C s_502_5: cast zx s_502_4 -> i
        let s_502_5: i128 = (i128::try_from(s_502_4).unwrap());
        // C s_502_6: const #7s : i
        let s_502_6: i128 = 7;
        // C s_502_7: add s_502_6 s_502_5
        let s_502_7: i128 = (s_502_6 + s_502_5);
        // D s_502_8: bit-extract s_502_3 s_502_2 s_502_7
        let s_502_8: Bits = (Bits::new(
            ((s_502_3) >> (s_502_2)).value(),
            u16::try_from(s_502_7).unwrap(),
        ));
        // D s_502_9: cast reint s_502_8 -> u8
        let s_502_9: u8 = (s_502_8.value() as u8);
        // C s_502_10: const #0s : i
        let s_502_10: i128 = 0;
        // D s_502_11: read-var tempxt:u64
        let s_502_11: u64 = fn_state.tempxt;
        // D s_502_12: cast zx s_502_11 -> bv
        let s_502_12: Bits = Bits::new(s_502_11 as u128, 64u16);
        // C s_502_13: const #1s : i64
        let s_502_13: i64 = 1;
        // C s_502_14: cast zx s_502_13 -> i
        let s_502_14: i128 = (i128::try_from(s_502_13).unwrap());
        // C s_502_15: const #7s : i
        let s_502_15: i128 = 7;
        // C s_502_16: add s_502_15 s_502_14
        let s_502_16: i128 = (s_502_15 + s_502_14);
        // D s_502_17: bit-extract s_502_12 s_502_10 s_502_16
        let s_502_17: Bits = (Bits::new(
            ((s_502_12) >> (s_502_10)).value(),
            u16::try_from(s_502_16).unwrap(),
        ));
        // D s_502_18: cast reint s_502_17 -> u8
        let s_502_18: u8 = (s_502_17.value() as u8);
        // D s_502_19: cast zx s_502_18 -> bv
        let s_502_19: Bits = Bits::new(s_502_18 as u128, 8u16);
        // D s_502_20: not s_502_19
        let s_502_20: Bits = !s_502_19;
        // D s_502_21: cast reint s_502_20 -> u8
        let s_502_21: u8 = (s_502_20.value() as u8);
        // D s_502_22: cast zx s_502_9 -> bv
        let s_502_22: Bits = Bits::new(s_502_9 as u128, 8u16);
        // D s_502_23: cast zx s_502_21 -> bv
        let s_502_23: Bits = Bits::new(s_502_21 as u128, 8u16);
        // D s_502_24: and s_502_22 s_502_23
        let s_502_24: Bits = ((s_502_22) & (s_502_23));
        // D s_502_25: cast reint s_502_24 -> u8
        let s_502_25: u8 = (s_502_24.value() as u8);
        // C s_502_26: const #0s : i
        let s_502_26: i128 = 0;
        // D s_502_27: read-var tempxt:u64
        let s_502_27: u64 = fn_state.tempxt;
        // D s_502_28: cast zx s_502_27 -> bv
        let s_502_28: Bits = Bits::new(s_502_27 as u128, 64u16);
        // D s_502_29: cast zx s_502_25 -> bv
        let s_502_29: Bits = Bits::new(s_502_25 as u128, 8u16);
        // C s_502_30: const #7s : i
        let s_502_30: i128 = 7;
        // C s_502_31: const #1u : u64
        let s_502_31: u64 = 1;
        // C s_502_32: cast zx s_502_31 -> bv
        let s_502_32: Bits = Bits::new(s_502_31 as u128, 64u16);
        // C s_502_33: lsl s_502_32 s_502_30
        let s_502_33: Bits = s_502_32 << s_502_30;
        // C s_502_34: sub s_502_33 s_502_32
        let s_502_34: Bits = ((s_502_33) - (s_502_32));
        // D s_502_35: and s_502_29 s_502_34
        let s_502_35: Bits = ((s_502_29) & (s_502_34));
        // D s_502_36: lsl s_502_35 s_502_26
        let s_502_36: Bits = s_502_35 << s_502_26;
        // C s_502_37: lsl s_502_34 s_502_26
        let s_502_37: Bits = s_502_34 << s_502_26;
        // C s_502_38: cmpl s_502_37
        let s_502_38: Bits = !s_502_37;
        // D s_502_39: and s_502_28 s_502_38
        let s_502_39: Bits = ((s_502_28) & (s_502_38));
        // D s_502_40: or s_502_39 s_502_36
        let s_502_40: Bits = ((s_502_39) | (s_502_36));
        // D s_502_41: cast reint s_502_40 -> u64
        let s_502_41: u64 = (s_502_40.value() as u64);
        // D s_502_42: write-var tempxt <= s_502_41
        fn_state.tempxt = s_502_41;
        // N s_502_43: jump b501
        return block_501(state, tracer, fn_state);
    }
    fn block_503<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_503_0: const #21088u : u32
        let s_503_0: u32 = 21088;
        // D s_503_1: read-reg s_503_0:u64
        let s_503_1: u64 = {
            let value = state.read_register::<u64>(s_503_0 as isize);
            tracer.read_register(s_503_0 as isize, value);
            value
        };
        // C s_503_2: const #0s : i
        let s_503_2: i128 = 0;
        // D s_503_3: cast zx s_503_1 -> bv
        let s_503_3: Bits = Bits::new(s_503_1 as u128, 64u16);
        // C s_503_4: const #1s : i64
        let s_503_4: i64 = 1;
        // C s_503_5: cast zx s_503_4 -> i
        let s_503_5: i128 = (i128::try_from(s_503_4).unwrap());
        // C s_503_6: const #7s : i
        let s_503_6: i128 = 7;
        // C s_503_7: add s_503_6 s_503_5
        let s_503_7: i128 = (s_503_6 + s_503_5);
        // D s_503_8: bit-extract s_503_3 s_503_2 s_503_7
        let s_503_8: Bits = (Bits::new(
            ((s_503_3) >> (s_503_2)).value(),
            u16::try_from(s_503_7).unwrap(),
        ));
        // D s_503_9: cast reint s_503_8 -> u8
        let s_503_9: u8 = (s_503_8.value() as u8);
        // C s_503_10: const #0s : i
        let s_503_10: i128 = 0;
        // D s_503_11: read-var tempxt:u64
        let s_503_11: u64 = fn_state.tempxt;
        // D s_503_12: cast zx s_503_11 -> bv
        let s_503_12: Bits = Bits::new(s_503_11 as u128, 64u16);
        // C s_503_13: const #1s : i64
        let s_503_13: i64 = 1;
        // C s_503_14: cast zx s_503_13 -> i
        let s_503_14: i128 = (i128::try_from(s_503_13).unwrap());
        // C s_503_15: const #7s : i
        let s_503_15: i128 = 7;
        // C s_503_16: add s_503_15 s_503_14
        let s_503_16: i128 = (s_503_15 + s_503_14);
        // D s_503_17: bit-extract s_503_12 s_503_10 s_503_16
        let s_503_17: Bits = (Bits::new(
            ((s_503_12) >> (s_503_10)).value(),
            u16::try_from(s_503_16).unwrap(),
        ));
        // D s_503_18: cast reint s_503_17 -> u8
        let s_503_18: u8 = (s_503_17.value() as u8);
        // D s_503_19: cast zx s_503_9 -> bv
        let s_503_19: Bits = Bits::new(s_503_9 as u128, 8u16);
        // D s_503_20: cast zx s_503_18 -> bv
        let s_503_20: Bits = Bits::new(s_503_18 as u128, 8u16);
        // D s_503_21: or s_503_19 s_503_20
        let s_503_21: Bits = ((s_503_19) | (s_503_20));
        // D s_503_22: cast reint s_503_21 -> u8
        let s_503_22: u8 = (s_503_21.value() as u8);
        // C s_503_23: const #0s : i
        let s_503_23: i128 = 0;
        // D s_503_24: read-var tempxt:u64
        let s_503_24: u64 = fn_state.tempxt;
        // D s_503_25: cast zx s_503_24 -> bv
        let s_503_25: Bits = Bits::new(s_503_24 as u128, 64u16);
        // D s_503_26: cast zx s_503_22 -> bv
        let s_503_26: Bits = Bits::new(s_503_22 as u128, 8u16);
        // C s_503_27: const #7s : i
        let s_503_27: i128 = 7;
        // C s_503_28: const #1u : u64
        let s_503_28: u64 = 1;
        // C s_503_29: cast zx s_503_28 -> bv
        let s_503_29: Bits = Bits::new(s_503_28 as u128, 64u16);
        // C s_503_30: lsl s_503_29 s_503_27
        let s_503_30: Bits = s_503_29 << s_503_27;
        // C s_503_31: sub s_503_30 s_503_29
        let s_503_31: Bits = ((s_503_30) - (s_503_29));
        // D s_503_32: and s_503_26 s_503_31
        let s_503_32: Bits = ((s_503_26) & (s_503_31));
        // D s_503_33: lsl s_503_32 s_503_23
        let s_503_33: Bits = s_503_32 << s_503_23;
        // C s_503_34: lsl s_503_31 s_503_23
        let s_503_34: Bits = s_503_31 << s_503_23;
        // C s_503_35: cmpl s_503_34
        let s_503_35: Bits = !s_503_34;
        // D s_503_36: and s_503_25 s_503_35
        let s_503_36: Bits = ((s_503_25) & (s_503_35));
        // D s_503_37: or s_503_36 s_503_33
        let s_503_37: Bits = ((s_503_36) | (s_503_33));
        // D s_503_38: cast reint s_503_37 -> u64
        let s_503_38: u64 = (s_503_37.value() as u64);
        // D s_503_39: write-var tempxt <= s_503_38
        fn_state.tempxt = s_503_38;
        // N s_503_40: jump b499
        return block_499(state, tracer, fn_state);
    }
    fn block_504<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_504_0: const #6s : i
        let s_504_0: i128 = 6;
        // D s_504_1: read-var op2:i
        let s_504_1: i128 = fn_state.op2;
        // D s_504_2: cmp-eq s_504_1 s_504_0
        let s_504_2: bool = ((s_504_1) == (s_504_0));
        // D s_504_3: write-var gs#141298 <= s_504_2
        fn_state.gs_141298 = s_504_2;
        // N s_504_4: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_505<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_505_0: const #7s : i
        let s_505_0: i128 = 7;
        // D s_505_1: read-var crn:i
        let s_505_1: i128 = fn_state.crn;
        // D s_505_2: cmp-eq s_505_1 s_505_0
        let s_505_2: bool = ((s_505_1) == (s_505_0));
        // D s_505_3: write-var gs#141296 <= s_505_2
        fn_state.gs_141296 = s_505_2;
        // N s_505_4: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_506<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_506_0: const #0s : i
        let s_506_0: i128 = 0;
        // D s_506_1: read-var op1:i
        let s_506_1: i128 = fn_state.op1;
        // D s_506_2: cmp-eq s_506_1 s_506_0
        let s_506_2: bool = ((s_506_1) == (s_506_0));
        // D s_506_3: write-var gs#141294 <= s_506_2
        fn_state.gs_141294 = s_506_2;
        // N s_506_4: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_507<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_507_0: const #19136u : u32
        let s_507_0: u32 = 19136;
        // D s_507_1: read-reg s_507_0:struct
        let s_507_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_507_0 as isize);
            tracer.read_register(s_507_0 as isize, value);
            value
        };
        // D s_507_2: call _get_PMSELR_EL0_Type_SEL(s_507_1)
        let s_507_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_507_1);
        // D s_507_3: cast zx s_507_2 -> bv
        let s_507_3: Bits = Bits::new(s_507_2 as u128, 5u16);
        // D s_507_4: cast zx s_507_3 -> i
        let s_507_4: i128 = (s_507_3.value() as i128);
        // D s_507_5: cast reint s_507_4 -> i64
        let s_507_5: i64 = (s_507_4 as i64);
        // C s_507_6: const #() : ()
        let s_507_6: () = ();
        // S s_507_7: call GetNumEventCounters(s_507_6)
        let s_507_7: i128 = GetNumEventCounters(state, tracer, s_507_6);
        // C s_507_8: const #1s : i
        let s_507_8: i128 = 1;
        // S s_507_9: sub s_507_7 s_507_8
        let s_507_9: i128 = ((s_507_7) - (s_507_8));
        // D s_507_10: cast zx s_507_5 -> i
        let s_507_10: i128 = (i128::try_from(s_507_5).unwrap());
        // D s_507_11: cmp-gt s_507_10 s_507_9
        let s_507_11: bool = ((s_507_10) > (s_507_9));
        // N s_507_12: branch s_507_11 b530 b508
        if s_507_11 {
            return block_530(state, tracer, fn_state);
        } else {
            return block_508(state, tracer, fn_state);
        };
    }
    fn block_508<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_508_0: const #0u : u8
        let s_508_0: bool = false;
        // D s_508_1: write-var gs#141318 <= s_508_0
        fn_state.gs_141318 = s_508_0;
        // N s_508_2: jump b509
        return block_509(state, tracer, fn_state);
    }
    fn block_509<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_509_0: read-var gs#141318:u8
        let s_509_0: bool = fn_state.gs_141318;
        // N s_509_1: branch s_509_0 b529 b510
        if s_509_0 {
            return block_529(state, tracer, fn_state);
        } else {
            return block_510(state, tracer, fn_state);
        };
    }
    fn block_510<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_510_0: const #() : ()
        let s_510_0: () = ();
        // S s_510_1: call EL2Enabled(s_510_0)
        let s_510_1: bool = EL2Enabled(state, tracer, s_510_0);
        // N s_510_2: branch s_510_1 b525 b511
        if s_510_1 {
            return block_525(state, tracer, fn_state);
        } else {
            return block_511(state, tracer, fn_state);
        };
    }
    fn block_511<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_511_0: const #0u : u8
        let s_511_0: bool = false;
        // D s_511_1: write-var gs#141320 <= s_511_0
        fn_state.gs_141320 = s_511_0;
        // N s_511_2: jump b512
        return block_512(state, tracer, fn_state);
    }
    fn block_512<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_512_0: read-var gs#141320:u8
        let s_512_0: bool = fn_state.gs_141320;
        // N s_512_1: branch s_512_0 b524 b513
        if s_512_0 {
            return block_524(state, tracer, fn_state);
        } else {
            return block_513(state, tracer, fn_state);
        };
    }
    fn block_513<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_513_0: const #0u : u8
        let s_513_0: bool = false;
        // D s_513_1: write-var gs#141321 <= s_513_0
        fn_state.gs_141321 = s_513_0;
        // N s_513_2: jump b514
        return block_514(state, tracer, fn_state);
    }
    fn block_514<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_514_0: read-var gs#141321:u8
        let s_514_0: bool = fn_state.gs_141321;
        // N s_514_1: branch s_514_0 b523 b515
        if s_514_0 {
            return block_523(state, tracer, fn_state);
        } else {
            return block_515(state, tracer, fn_state);
        };
    }
    fn block_515<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_515_0: const #0u : u8
        let s_515_0: bool = false;
        // D s_515_1: write-var gs#141323 <= s_515_0
        fn_state.gs_141323 = s_515_0;
        // N s_515_2: jump b516
        return block_516(state, tracer, fn_state);
    }
    fn block_516<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_516_0: read-var gs#141323:u8
        let s_516_0: bool = fn_state.gs_141323;
        // D s_516_1: write-var gs#141324 <= s_516_0
        fn_state.gs_141324 = s_516_0;
        // N s_516_2: jump b517
        return block_517(state, tracer, fn_state);
    }
    fn block_517<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_517_0: read-var gs#141324:u8
        let s_517_0: bool = fn_state.gs_141324;
        // N s_517_1: branch s_517_0 b520 b518
        if s_517_0 {
            return block_520(state, tracer, fn_state);
        } else {
            return block_518(state, tracer, fn_state);
        };
    }
    fn block_518<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_518_0: jump b519
        return block_519(state, tracer, fn_state);
    }
    fn block_519<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_519_0: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_520<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_520_0: const #19136u : u32
        let s_520_0: u32 = 19136;
        // D s_520_1: read-reg s_520_0:struct
        let s_520_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_520_0 as isize);
            tracer.read_register(s_520_0 as isize, value);
            value
        };
        // D s_520_2: call _get_PMSELR_EL0_Type_SEL(s_520_1)
        let s_520_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_520_1);
        // D s_520_3: cast zx s_520_2 -> bv
        let s_520_3: Bits = Bits::new(s_520_2 as u128, 5u16);
        // D s_520_4: cast zx s_520_3 -> i
        let s_520_4: i128 = (s_520_3.value() as i128);
        // D s_520_5: cast reint s_520_4 -> i64
        let s_520_5: i64 = (s_520_4 as i64);
        // C s_520_6: const #() : ()
        let s_520_6: () = ();
        // S s_520_7: call GetNumEventCounters(s_520_6)
        let s_520_7: i128 = GetNumEventCounters(state, tracer, s_520_6);
        // C s_520_8: const #1s : i
        let s_520_8: i128 = 1;
        // S s_520_9: sub s_520_7 s_520_8
        let s_520_9: i128 = ((s_520_7) - (s_520_8));
        // D s_520_10: cast zx s_520_5 -> i
        let s_520_10: i128 = (i128::try_from(s_520_5).unwrap());
        // D s_520_11: cmp-gt s_520_10 s_520_9
        let s_520_11: bool = ((s_520_10) > (s_520_9));
        // N s_520_12: branch s_520_11 b522 b521
        if s_520_11 {
            return block_522(state, tracer, fn_state);
        } else {
            return block_521(state, tracer, fn_state);
        };
    }
    fn block_521<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_521_0: const #24u : u8
        let s_521_0: u8 = 24;
        // C s_521_1: cast zx s_521_0 -> bv
        let s_521_1: Bits = Bits::new(s_521_0 as u128, 8u16);
        // C s_521_2: cast zx s_521_1 -> i
        let s_521_2: i128 = (s_521_1.value() as i128);
        // C s_521_3: cast reint s_521_2 -> i64
        let s_521_3: i64 = (s_521_2 as i64);
        // C s_521_4: cast zx s_521_3 -> i
        let s_521_4: i128 = (i128::try_from(s_521_3).unwrap());
        // C s_521_5: const #432u : u32
        let s_521_5: u32 = 432;
        // D s_521_6: read-reg s_521_5:u8
        let s_521_6: u8 = {
            let value = state.read_register::<u8>(s_521_5 as isize);
            tracer.read_register(s_521_5 as isize, value);
            value
        };
        // D s_521_7: call AArch64_SystemAccessTrap(s_521_6, s_521_4)
        let s_521_7: () = AArch64_SystemAccessTrap(state, tracer, s_521_6, s_521_4);
        // N s_521_8: jump b519
        return block_519(state, tracer, fn_state);
    }
    fn block_522<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_522_0: panic
        panic!("{:?}", ());
        // N s_522_1: return
        return;
    }
    fn block_523<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_523_0: const #19136u : u32
        let s_523_0: u32 = 19136;
        // D s_523_1: read-reg s_523_0:struct
        let s_523_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_523_0 as isize);
            tracer.read_register(s_523_0 as isize, value);
            value
        };
        // D s_523_2: call _get_PMSELR_EL0_Type_SEL(s_523_1)
        let s_523_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_523_1);
        // D s_523_3: cast zx s_523_2 -> bv
        let s_523_3: Bits = Bits::new(s_523_2 as u128, 5u16);
        // D s_523_4: cast zx s_523_3 -> i
        let s_523_4: i128 = (s_523_3.value() as i128);
        // D s_523_5: cast reint s_523_4 -> i64
        let s_523_5: i64 = (s_523_4 as i64);
        // C s_523_6: const #() : ()
        let s_523_6: () = ();
        // S s_523_7: call AArch64_GetNumEventCountersAccessible(s_523_6)
        let s_523_7: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_523_6,
        );
        // C s_523_8: const #1s : i
        let s_523_8: i128 = 1;
        // S s_523_9: sub s_523_7 s_523_8
        let s_523_9: i128 = ((s_523_7) - (s_523_8));
        // D s_523_10: cast zx s_523_5 -> i
        let s_523_10: i128 = (i128::try_from(s_523_5).unwrap());
        // D s_523_11: cmp-gt s_523_10 s_523_9
        let s_523_11: bool = ((s_523_10) > (s_523_9));
        // D s_523_12: write-var gs#141323 <= s_523_11
        fn_state.gs_141323 = s_523_11;
        // N s_523_13: jump b516
        return block_516(state, tracer, fn_state);
    }
    fn block_524<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_524_0: const #102624u : u32
        let s_524_0: u32 = 102624;
        // D s_524_1: read-reg s_524_0:struct
        let s_524_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_524_0 as isize);
            tracer.read_register(s_524_0 as isize, value);
            value
        };
        // D s_524_2: call _get_PMUSERENR_EL0_Type_EN(s_524_1)
        let s_524_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_524_1);
        // D s_524_3: cast zx s_524_2 -> bv
        let s_524_3: Bits = Bits::new(s_524_2 as u128, 1u16);
        // C s_524_4: const #1u : u8
        let s_524_4: bool = true;
        // C s_524_5: cast zx s_524_4 -> bv
        let s_524_5: Bits = Bits::new(s_524_4 as u128, 1u16);
        // D s_524_6: cmp-eq s_524_3 s_524_5
        let s_524_6: bool = ((s_524_3) == (s_524_5));
        // D s_524_7: write-var gs#141321 <= s_524_6
        fn_state.gs_141321 = s_524_6;
        // N s_524_8: jump b514
        return block_514(state, tracer, fn_state);
    }
    fn block_525<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_525_0: const #16975u : u32
        let s_525_0: u32 = 16975;
        // D s_525_1: read-reg s_525_0:u8
        let s_525_1: u8 = {
            let value = state.read_register::<u8>(s_525_0 as isize);
            tracer.read_register(s_525_0 as isize, value);
            value
        };
        // D s_525_2: cast zx s_525_1 -> bv
        let s_525_2: Bits = Bits::new(s_525_1 as u128, 2u16);
        // C s_525_3: const #448u : u32
        let s_525_3: u32 = 448;
        // D s_525_4: read-reg s_525_3:u8
        let s_525_4: u8 = {
            let value = state.read_register::<u8>(s_525_3 as isize);
            tracer.read_register(s_525_3 as isize, value);
            value
        };
        // D s_525_5: cast zx s_525_4 -> bv
        let s_525_5: Bits = Bits::new(s_525_4 as u128, 2u16);
        // D s_525_6: cmp-eq s_525_2 s_525_5
        let s_525_6: bool = ((s_525_2) == (s_525_5));
        // N s_525_7: branch s_525_6 b528 b526
        if s_525_6 {
            return block_528(state, tracer, fn_state);
        } else {
            return block_526(state, tracer, fn_state);
        };
    }
    fn block_526<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_526_0: const #16975u : u32
        let s_526_0: u32 = 16975;
        // D s_526_1: read-reg s_526_0:u8
        let s_526_1: u8 = {
            let value = state.read_register::<u8>(s_526_0 as isize);
            tracer.read_register(s_526_0 as isize, value);
            value
        };
        // D s_526_2: cast zx s_526_1 -> bv
        let s_526_2: Bits = Bits::new(s_526_1 as u128, 2u16);
        // C s_526_3: const #440u : u32
        let s_526_3: u32 = 440;
        // D s_526_4: read-reg s_526_3:u8
        let s_526_4: u8 = {
            let value = state.read_register::<u8>(s_526_3 as isize);
            tracer.read_register(s_526_3 as isize, value);
            value
        };
        // D s_526_5: cast zx s_526_4 -> bv
        let s_526_5: Bits = Bits::new(s_526_4 as u128, 2u16);
        // D s_526_6: cmp-eq s_526_2 s_526_5
        let s_526_6: bool = ((s_526_2) == (s_526_5));
        // D s_526_7: write-var gs#141319 <= s_526_6
        fn_state.gs_141319 = s_526_6;
        // N s_526_8: jump b527
        return block_527(state, tracer, fn_state);
    }
    fn block_527<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_527_0: read-var gs#141319:u8
        let s_527_0: bool = fn_state.gs_141319;
        // D s_527_1: write-var gs#141320 <= s_527_0
        fn_state.gs_141320 = s_527_0;
        // N s_527_2: jump b512
        return block_512(state, tracer, fn_state);
    }
    fn block_528<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_528_0: const #1u : u8
        let s_528_0: bool = true;
        // D s_528_1: write-var gs#141319 <= s_528_0
        fn_state.gs_141319 = s_528_0;
        // N s_528_2: jump b527
        return block_527(state, tracer, fn_state);
    }
    fn block_529<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_529_0: const #1u : u8
        let s_529_0: bool = true;
        // D s_529_1: write-var gs#141324 <= s_529_0
        fn_state.gs_141324 = s_529_0;
        // N s_529_2: jump b517
        return block_517(state, tracer, fn_state);
    }
    fn block_530<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_530_0: const #19136u : u32
        let s_530_0: u32 = 19136;
        // D s_530_1: read-reg s_530_0:struct
        let s_530_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_530_0 as isize);
            tracer.read_register(s_530_0 as isize, value);
            value
        };
        // D s_530_2: call _get_PMSELR_EL0_Type_SEL(s_530_1)
        let s_530_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_530_1);
        // D s_530_3: cast zx s_530_2 -> bv
        let s_530_3: Bits = Bits::new(s_530_2 as u128, 5u16);
        // C s_530_4: const #31u : u8
        let s_530_4: u8 = 31;
        // C s_530_5: cast zx s_530_4 -> bv
        let s_530_5: Bits = Bits::new(s_530_4 as u128, 5u16);
        // D s_530_6: cmp-ne s_530_3 s_530_5
        let s_530_6: bool = ((s_530_3) != (s_530_5));
        // D s_530_7: write-var gs#141318 <= s_530_6
        fn_state.gs_141318 = s_530_6;
        // N s_530_8: jump b509
        return block_509(state, tracer, fn_state);
    }
    fn block_531<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_531_0: const #2s : i
        let s_531_0: i128 = 2;
        // D s_531_1: read-var op2:i
        let s_531_1: i128 = fn_state.op2;
        // D s_531_2: cmp-eq s_531_1 s_531_0
        let s_531_2: bool = ((s_531_1) == (s_531_0));
        // N s_531_3: branch s_531_2 b537 b532
        if s_531_2 {
            return block_537(state, tracer, fn_state);
        } else {
            return block_532(state, tracer, fn_state);
        };
    }
    fn block_532<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_532_0: const #1s : i
        let s_532_0: i128 = 1;
        // D s_532_1: read-var op2:i
        let s_532_1: i128 = fn_state.op2;
        // D s_532_2: cmp-eq s_532_1 s_532_0
        let s_532_2: bool = ((s_532_1) == (s_532_0));
        // N s_532_3: branch s_532_2 b536 b533
        if s_532_2 {
            return block_536(state, tracer, fn_state);
        } else {
            return block_533(state, tracer, fn_state);
        };
    }
    fn block_533<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_533_0: const #0u : u8
        let s_533_0: bool = false;
        // D s_533_1: write-var gs#141289 <= s_533_0
        fn_state.gs_141289 = s_533_0;
        // N s_533_2: jump b534
        return block_534(state, tracer, fn_state);
    }
    fn block_534<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_534_0: read-var gs#141289:u8
        let s_534_0: bool = fn_state.gs_141289;
        // D s_534_1: write-var gs#141290 <= s_534_0
        fn_state.gs_141290 = s_534_0;
        // N s_534_2: jump b535
        return block_535(state, tracer, fn_state);
    }
    fn block_535<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_535_0: read-var gs#141290:u8
        let s_535_0: bool = fn_state.gs_141290;
        // D s_535_1: write-var gs#141291 <= s_535_0
        fn_state.gs_141291 = s_535_0;
        // N s_535_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_536<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_536_0: const #19136u : u32
        let s_536_0: u32 = 19136;
        // D s_536_1: read-reg s_536_0:struct
        let s_536_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_536_0 as isize);
            tracer.read_register(s_536_0 as isize, value);
            value
        };
        // D s_536_2: call _get_PMSELR_EL0_Type_SEL(s_536_1)
        let s_536_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_536_1);
        // D s_536_3: cast zx s_536_2 -> bv
        let s_536_3: Bits = Bits::new(s_536_2 as u128, 5u16);
        // C s_536_4: const #31u : u8
        let s_536_4: u8 = 31;
        // C s_536_5: cast zx s_536_4 -> bv
        let s_536_5: Bits = Bits::new(s_536_4 as u128, 5u16);
        // D s_536_6: cmp-ne s_536_3 s_536_5
        let s_536_6: bool = ((s_536_3) != (s_536_5));
        // D s_536_7: write-var gs#141289 <= s_536_6
        fn_state.gs_141289 = s_536_6;
        // N s_536_8: jump b534
        return block_534(state, tracer, fn_state);
    }
    fn block_537<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_537_0: const #1u : u8
        let s_537_0: bool = true;
        // D s_537_1: write-var gs#141290 <= s_537_0
        fn_state.gs_141290 = s_537_0;
        // N s_537_2: jump b535
        return block_535(state, tracer, fn_state);
    }
    fn block_538<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_538_0: const #13s : i
        let s_538_0: i128 = 13;
        // D s_538_1: read-var crm:i
        let s_538_1: i128 = fn_state.crm;
        // D s_538_2: cmp-eq s_538_1 s_538_0
        let s_538_2: bool = ((s_538_1) == (s_538_0));
        // D s_538_3: write-var gs#141286 <= s_538_2
        fn_state.gs_141286 = s_538_2;
        // N s_538_4: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_539<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_539_0: const #9s : i
        let s_539_0: i128 = 9;
        // D s_539_1: read-var crn:i
        let s_539_1: i128 = fn_state.crn;
        // D s_539_2: cmp-eq s_539_1 s_539_0
        let s_539_2: bool = ((s_539_1) == (s_539_0));
        // D s_539_3: write-var gs#141284 <= s_539_2
        fn_state.gs_141284 = s_539_2;
        // N s_539_4: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_540<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_540_0: const #3s : i
        let s_540_0: i128 = 3;
        // D s_540_1: read-var op1:i
        let s_540_1: i128 = fn_state.op1;
        // D s_540_2: cmp-eq s_540_1 s_540_0
        let s_540_2: bool = ((s_540_1) == (s_540_0));
        // D s_540_3: write-var gs#141282 <= s_540_2
        fn_state.gs_141282 = s_540_2;
        // N s_540_4: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_541<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_541_0: const #1s : i
        let s_541_0: i128 = 1;
        // C s_541_1: const #0s : i
        let s_541_1: i128 = 0;
        // D s_541_2: read-var crm:i
        let s_541_2: i128 = fn_state.crm;
        // D s_541_3: call integer_subrange(s_541_2, s_541_0, s_541_1)
        let s_541_3: Bits = integer_subrange(state, tracer, s_541_2, s_541_0, s_541_1);
        // D s_541_4: cast reint s_541_3 -> u8
        let s_541_4: u8 = (s_541_3.value() as u8);
        // C s_541_5: const #2s : i
        let s_541_5: i128 = 2;
        // C s_541_6: const #0s : i
        let s_541_6: i128 = 0;
        // D s_541_7: read-var op2:i
        let s_541_7: i128 = fn_state.op2;
        // D s_541_8: call integer_subrange(s_541_7, s_541_5, s_541_6)
        let s_541_8: Bits = integer_subrange(state, tracer, s_541_7, s_541_5, s_541_6);
        // D s_541_9: cast reint s_541_8 -> u8
        let s_541_9: u8 = (s_541_8.value() as u8);
        // D s_541_10: cast zx s_541_4 -> bv
        let s_541_10: Bits = Bits::new(s_541_4 as u128, 2u16);
        // D s_541_11: cast zx s_541_9 -> bv
        let s_541_11: Bits = Bits::new(s_541_9 as u128, 3u16);
        // D s_541_12: cast reint s_541_10 -> u128
        let s_541_12: u128 = (s_541_10.value() as u128);
        // D s_541_13: size-of s_541_10
        let s_541_13: u16 = s_541_10.length();
        // D s_541_14: cast reint s_541_11 -> u128
        let s_541_14: u128 = (s_541_11.value() as u128);
        // D s_541_15: size-of s_541_11
        let s_541_15: u16 = s_541_11.length();
        // D s_541_16: lsl s_541_12 s_541_15
        let s_541_16: u128 = s_541_12 << s_541_15;
        // D s_541_17: or s_541_16 s_541_14
        let s_541_17: u128 = ((s_541_16) | (s_541_14));
        // D s_541_18: add s_541_13 s_541_15
        let s_541_18: u16 = (s_541_13 + s_541_15);
        // D s_541_19: create-bits s_541_17 s_541_18
        let s_541_19: Bits = Bits::new(s_541_17, s_541_18);
        // D s_541_20: cast reint s_541_19 -> u8
        let s_541_20: u8 = (s_541_19.value() as u8);
        // D s_541_21: cast zx s_541_20 -> bv
        let s_541_21: Bits = Bits::new(s_541_20 as u128, 5u16);
        // D s_541_22: cast zx s_541_21 -> i
        let s_541_22: i128 = (s_541_21.value() as i128);
        // D s_541_23: cast reint s_541_22 -> i64
        let s_541_23: i64 = (s_541_22 as i64);
        // C s_541_24: const #() : ()
        let s_541_24: () = ();
        // S s_541_25: call GetNumEventCounters(s_541_24)
        let s_541_25: i128 = GetNumEventCounters(state, tracer, s_541_24);
        // C s_541_26: const #1s : i
        let s_541_26: i128 = 1;
        // S s_541_27: sub s_541_25 s_541_26
        let s_541_27: i128 = ((s_541_25) - (s_541_26));
        // D s_541_28: cast zx s_541_23 -> i
        let s_541_28: i128 = (i128::try_from(s_541_23).unwrap());
        // D s_541_29: cmp-gt s_541_28 s_541_27
        let s_541_29: bool = ((s_541_28) > (s_541_27));
        // N s_541_30: branch s_541_29 b564 b542
        if s_541_29 {
            return block_564(state, tracer, fn_state);
        } else {
            return block_542(state, tracer, fn_state);
        };
    }
    fn block_542<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_542_0: const #0u : u8
        let s_542_0: bool = false;
        // D s_542_1: write-var gs#141337 <= s_542_0
        fn_state.gs_141337 = s_542_0;
        // N s_542_2: jump b543
        return block_543(state, tracer, fn_state);
    }
    fn block_543<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_543_0: read-var gs#141337:u8
        let s_543_0: bool = fn_state.gs_141337;
        // N s_543_1: branch s_543_0 b563 b544
        if s_543_0 {
            return block_563(state, tracer, fn_state);
        } else {
            return block_544(state, tracer, fn_state);
        };
    }
    fn block_544<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_544_0: const #() : ()
        let s_544_0: () = ();
        // S s_544_1: call EL2Enabled(s_544_0)
        let s_544_1: bool = EL2Enabled(state, tracer, s_544_0);
        // N s_544_2: branch s_544_1 b559 b545
        if s_544_1 {
            return block_559(state, tracer, fn_state);
        } else {
            return block_545(state, tracer, fn_state);
        };
    }
    fn block_545<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_545_0: const #0u : u8
        let s_545_0: bool = false;
        // D s_545_1: write-var gs#141339 <= s_545_0
        fn_state.gs_141339 = s_545_0;
        // N s_545_2: jump b546
        return block_546(state, tracer, fn_state);
    }
    fn block_546<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_546_0: read-var gs#141339:u8
        let s_546_0: bool = fn_state.gs_141339;
        // N s_546_1: branch s_546_0 b558 b547
        if s_546_0 {
            return block_558(state, tracer, fn_state);
        } else {
            return block_547(state, tracer, fn_state);
        };
    }
    fn block_547<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_547_0: const #0u : u8
        let s_547_0: bool = false;
        // D s_547_1: write-var gs#141340 <= s_547_0
        fn_state.gs_141340 = s_547_0;
        // N s_547_2: jump b548
        return block_548(state, tracer, fn_state);
    }
    fn block_548<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_548_0: read-var gs#141340:u8
        let s_548_0: bool = fn_state.gs_141340;
        // N s_548_1: branch s_548_0 b557 b549
        if s_548_0 {
            return block_557(state, tracer, fn_state);
        } else {
            return block_549(state, tracer, fn_state);
        };
    }
    fn block_549<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_549_0: const #0u : u8
        let s_549_0: bool = false;
        // D s_549_1: write-var gs#141346 <= s_549_0
        fn_state.gs_141346 = s_549_0;
        // N s_549_2: jump b550
        return block_550(state, tracer, fn_state);
    }
    fn block_550<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_550_0: read-var gs#141346:u8
        let s_550_0: bool = fn_state.gs_141346;
        // D s_550_1: write-var gs#141347 <= s_550_0
        fn_state.gs_141347 = s_550_0;
        // N s_550_2: jump b551
        return block_551(state, tracer, fn_state);
    }
    fn block_551<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_551_0: read-var gs#141347:u8
        let s_551_0: bool = fn_state.gs_141347;
        // N s_551_1: branch s_551_0 b554 b552
        if s_551_0 {
            return block_554(state, tracer, fn_state);
        } else {
            return block_552(state, tracer, fn_state);
        };
    }
    fn block_552<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_552_0: jump b553
        return block_553(state, tracer, fn_state);
    }
    fn block_553<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_553_0: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_554<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_554_0: const #1s : i
        let s_554_0: i128 = 1;
        // C s_554_1: const #0s : i
        let s_554_1: i128 = 0;
        // D s_554_2: read-var crm:i
        let s_554_2: i128 = fn_state.crm;
        // D s_554_3: call integer_subrange(s_554_2, s_554_0, s_554_1)
        let s_554_3: Bits = integer_subrange(state, tracer, s_554_2, s_554_0, s_554_1);
        // D s_554_4: cast reint s_554_3 -> u8
        let s_554_4: u8 = (s_554_3.value() as u8);
        // C s_554_5: const #2s : i
        let s_554_5: i128 = 2;
        // C s_554_6: const #0s : i
        let s_554_6: i128 = 0;
        // D s_554_7: read-var op2:i
        let s_554_7: i128 = fn_state.op2;
        // D s_554_8: call integer_subrange(s_554_7, s_554_5, s_554_6)
        let s_554_8: Bits = integer_subrange(state, tracer, s_554_7, s_554_5, s_554_6);
        // D s_554_9: cast reint s_554_8 -> u8
        let s_554_9: u8 = (s_554_8.value() as u8);
        // D s_554_10: cast zx s_554_4 -> bv
        let s_554_10: Bits = Bits::new(s_554_4 as u128, 2u16);
        // D s_554_11: cast zx s_554_9 -> bv
        let s_554_11: Bits = Bits::new(s_554_9 as u128, 3u16);
        // D s_554_12: cast reint s_554_10 -> u128
        let s_554_12: u128 = (s_554_10.value() as u128);
        // D s_554_13: size-of s_554_10
        let s_554_13: u16 = s_554_10.length();
        // D s_554_14: cast reint s_554_11 -> u128
        let s_554_14: u128 = (s_554_11.value() as u128);
        // D s_554_15: size-of s_554_11
        let s_554_15: u16 = s_554_11.length();
        // D s_554_16: lsl s_554_12 s_554_15
        let s_554_16: u128 = s_554_12 << s_554_15;
        // D s_554_17: or s_554_16 s_554_14
        let s_554_17: u128 = ((s_554_16) | (s_554_14));
        // D s_554_18: add s_554_13 s_554_15
        let s_554_18: u16 = (s_554_13 + s_554_15);
        // D s_554_19: create-bits s_554_17 s_554_18
        let s_554_19: Bits = Bits::new(s_554_17, s_554_18);
        // D s_554_20: cast reint s_554_19 -> u8
        let s_554_20: u8 = (s_554_19.value() as u8);
        // D s_554_21: cast zx s_554_20 -> bv
        let s_554_21: Bits = Bits::new(s_554_20 as u128, 5u16);
        // D s_554_22: cast zx s_554_21 -> i
        let s_554_22: i128 = (s_554_21.value() as i128);
        // D s_554_23: cast reint s_554_22 -> i64
        let s_554_23: i64 = (s_554_22 as i64);
        // C s_554_24: const #() : ()
        let s_554_24: () = ();
        // S s_554_25: call GetNumEventCounters(s_554_24)
        let s_554_25: i128 = GetNumEventCounters(state, tracer, s_554_24);
        // C s_554_26: const #1s : i
        let s_554_26: i128 = 1;
        // S s_554_27: sub s_554_25 s_554_26
        let s_554_27: i128 = ((s_554_25) - (s_554_26));
        // D s_554_28: cast zx s_554_23 -> i
        let s_554_28: i128 = (i128::try_from(s_554_23).unwrap());
        // D s_554_29: cmp-gt s_554_28 s_554_27
        let s_554_29: bool = ((s_554_28) > (s_554_27));
        // N s_554_30: branch s_554_29 b556 b555
        if s_554_29 {
            return block_556(state, tracer, fn_state);
        } else {
            return block_555(state, tracer, fn_state);
        };
    }
    fn block_555<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_555_0: const #24u : u8
        let s_555_0: u8 = 24;
        // C s_555_1: cast zx s_555_0 -> bv
        let s_555_1: Bits = Bits::new(s_555_0 as u128, 8u16);
        // C s_555_2: cast zx s_555_1 -> i
        let s_555_2: i128 = (s_555_1.value() as i128);
        // C s_555_3: cast reint s_555_2 -> i64
        let s_555_3: i64 = (s_555_2 as i64);
        // C s_555_4: cast zx s_555_3 -> i
        let s_555_4: i128 = (i128::try_from(s_555_3).unwrap());
        // C s_555_5: const #432u : u32
        let s_555_5: u32 = 432;
        // D s_555_6: read-reg s_555_5:u8
        let s_555_6: u8 = {
            let value = state.read_register::<u8>(s_555_5 as isize);
            tracer.read_register(s_555_5 as isize, value);
            value
        };
        // D s_555_7: call AArch64_SystemAccessTrap(s_555_6, s_555_4)
        let s_555_7: () = AArch64_SystemAccessTrap(state, tracer, s_555_6, s_555_4);
        // N s_555_8: jump b553
        return block_553(state, tracer, fn_state);
    }
    fn block_556<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_556_0: panic
        panic!("{:?}", ());
        // N s_556_1: return
        return;
    }
    fn block_557<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_557_0: const #1s : i
        let s_557_0: i128 = 1;
        // C s_557_1: const #0s : i
        let s_557_1: i128 = 0;
        // D s_557_2: read-var crm:i
        let s_557_2: i128 = fn_state.crm;
        // D s_557_3: call integer_subrange(s_557_2, s_557_0, s_557_1)
        let s_557_3: Bits = integer_subrange(state, tracer, s_557_2, s_557_0, s_557_1);
        // D s_557_4: cast reint s_557_3 -> u8
        let s_557_4: u8 = (s_557_3.value() as u8);
        // C s_557_5: const #2s : i
        let s_557_5: i128 = 2;
        // C s_557_6: const #0s : i
        let s_557_6: i128 = 0;
        // D s_557_7: read-var op2:i
        let s_557_7: i128 = fn_state.op2;
        // D s_557_8: call integer_subrange(s_557_7, s_557_5, s_557_6)
        let s_557_8: Bits = integer_subrange(state, tracer, s_557_7, s_557_5, s_557_6);
        // D s_557_9: cast reint s_557_8 -> u8
        let s_557_9: u8 = (s_557_8.value() as u8);
        // D s_557_10: cast zx s_557_4 -> bv
        let s_557_10: Bits = Bits::new(s_557_4 as u128, 2u16);
        // D s_557_11: cast zx s_557_9 -> bv
        let s_557_11: Bits = Bits::new(s_557_9 as u128, 3u16);
        // D s_557_12: cast reint s_557_10 -> u128
        let s_557_12: u128 = (s_557_10.value() as u128);
        // D s_557_13: size-of s_557_10
        let s_557_13: u16 = s_557_10.length();
        // D s_557_14: cast reint s_557_11 -> u128
        let s_557_14: u128 = (s_557_11.value() as u128);
        // D s_557_15: size-of s_557_11
        let s_557_15: u16 = s_557_11.length();
        // D s_557_16: lsl s_557_12 s_557_15
        let s_557_16: u128 = s_557_12 << s_557_15;
        // D s_557_17: or s_557_16 s_557_14
        let s_557_17: u128 = ((s_557_16) | (s_557_14));
        // D s_557_18: add s_557_13 s_557_15
        let s_557_18: u16 = (s_557_13 + s_557_15);
        // D s_557_19: create-bits s_557_17 s_557_18
        let s_557_19: Bits = Bits::new(s_557_17, s_557_18);
        // D s_557_20: cast reint s_557_19 -> u8
        let s_557_20: u8 = (s_557_19.value() as u8);
        // D s_557_21: cast zx s_557_20 -> bv
        let s_557_21: Bits = Bits::new(s_557_20 as u128, 5u16);
        // D s_557_22: cast zx s_557_21 -> i
        let s_557_22: i128 = (s_557_21.value() as i128);
        // D s_557_23: cast reint s_557_22 -> i64
        let s_557_23: i64 = (s_557_22 as i64);
        // C s_557_24: const #() : ()
        let s_557_24: () = ();
        // S s_557_25: call AArch64_GetNumEventCountersAccessible(s_557_24)
        let s_557_25: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_557_24,
        );
        // C s_557_26: const #1s : i
        let s_557_26: i128 = 1;
        // S s_557_27: sub s_557_25 s_557_26
        let s_557_27: i128 = ((s_557_25) - (s_557_26));
        // D s_557_28: cast zx s_557_23 -> i
        let s_557_28: i128 = (i128::try_from(s_557_23).unwrap());
        // D s_557_29: cmp-gt s_557_28 s_557_27
        let s_557_29: bool = ((s_557_28) > (s_557_27));
        // D s_557_30: write-var gs#141346 <= s_557_29
        fn_state.gs_141346 = s_557_29;
        // N s_557_31: jump b550
        return block_550(state, tracer, fn_state);
    }
    fn block_558<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_558_0: const #102624u : u32
        let s_558_0: u32 = 102624;
        // D s_558_1: read-reg s_558_0:struct
        let s_558_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_558_0 as isize);
            tracer.read_register(s_558_0 as isize, value);
            value
        };
        // D s_558_2: call _get_PMUSERENR_EL0_Type_EN(s_558_1)
        let s_558_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_558_1);
        // D s_558_3: cast zx s_558_2 -> bv
        let s_558_3: Bits = Bits::new(s_558_2 as u128, 1u16);
        // C s_558_4: const #1u : u8
        let s_558_4: bool = true;
        // C s_558_5: cast zx s_558_4 -> bv
        let s_558_5: Bits = Bits::new(s_558_4 as u128, 1u16);
        // D s_558_6: cmp-eq s_558_3 s_558_5
        let s_558_6: bool = ((s_558_3) == (s_558_5));
        // D s_558_7: write-var gs#141340 <= s_558_6
        fn_state.gs_141340 = s_558_6;
        // N s_558_8: jump b548
        return block_548(state, tracer, fn_state);
    }
    fn block_559<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_559_0: const #16975u : u32
        let s_559_0: u32 = 16975;
        // D s_559_1: read-reg s_559_0:u8
        let s_559_1: u8 = {
            let value = state.read_register::<u8>(s_559_0 as isize);
            tracer.read_register(s_559_0 as isize, value);
            value
        };
        // D s_559_2: cast zx s_559_1 -> bv
        let s_559_2: Bits = Bits::new(s_559_1 as u128, 2u16);
        // C s_559_3: const #448u : u32
        let s_559_3: u32 = 448;
        // D s_559_4: read-reg s_559_3:u8
        let s_559_4: u8 = {
            let value = state.read_register::<u8>(s_559_3 as isize);
            tracer.read_register(s_559_3 as isize, value);
            value
        };
        // D s_559_5: cast zx s_559_4 -> bv
        let s_559_5: Bits = Bits::new(s_559_4 as u128, 2u16);
        // D s_559_6: cmp-eq s_559_2 s_559_5
        let s_559_6: bool = ((s_559_2) == (s_559_5));
        // N s_559_7: branch s_559_6 b562 b560
        if s_559_6 {
            return block_562(state, tracer, fn_state);
        } else {
            return block_560(state, tracer, fn_state);
        };
    }
    fn block_560<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_560_0: const #16975u : u32
        let s_560_0: u32 = 16975;
        // D s_560_1: read-reg s_560_0:u8
        let s_560_1: u8 = {
            let value = state.read_register::<u8>(s_560_0 as isize);
            tracer.read_register(s_560_0 as isize, value);
            value
        };
        // D s_560_2: cast zx s_560_1 -> bv
        let s_560_2: Bits = Bits::new(s_560_1 as u128, 2u16);
        // C s_560_3: const #440u : u32
        let s_560_3: u32 = 440;
        // D s_560_4: read-reg s_560_3:u8
        let s_560_4: u8 = {
            let value = state.read_register::<u8>(s_560_3 as isize);
            tracer.read_register(s_560_3 as isize, value);
            value
        };
        // D s_560_5: cast zx s_560_4 -> bv
        let s_560_5: Bits = Bits::new(s_560_4 as u128, 2u16);
        // D s_560_6: cmp-eq s_560_2 s_560_5
        let s_560_6: bool = ((s_560_2) == (s_560_5));
        // D s_560_7: write-var gs#141338 <= s_560_6
        fn_state.gs_141338 = s_560_6;
        // N s_560_8: jump b561
        return block_561(state, tracer, fn_state);
    }
    fn block_561<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_561_0: read-var gs#141338:u8
        let s_561_0: bool = fn_state.gs_141338;
        // D s_561_1: write-var gs#141339 <= s_561_0
        fn_state.gs_141339 = s_561_0;
        // N s_561_2: jump b546
        return block_546(state, tracer, fn_state);
    }
    fn block_562<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_562_0: const #1u : u8
        let s_562_0: bool = true;
        // D s_562_1: write-var gs#141338 <= s_562_0
        fn_state.gs_141338 = s_562_0;
        // N s_562_2: jump b561
        return block_561(state, tracer, fn_state);
    }
    fn block_563<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_563_0: const #1u : u8
        let s_563_0: bool = true;
        // D s_563_1: write-var gs#141347 <= s_563_0
        fn_state.gs_141347 = s_563_0;
        // N s_563_2: jump b551
        return block_551(state, tracer, fn_state);
    }
    fn block_564<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_564_0: const #1s : i
        let s_564_0: i128 = 1;
        // C s_564_1: const #0s : i
        let s_564_1: i128 = 0;
        // D s_564_2: read-var crm:i
        let s_564_2: i128 = fn_state.crm;
        // D s_564_3: call integer_subrange(s_564_2, s_564_0, s_564_1)
        let s_564_3: Bits = integer_subrange(state, tracer, s_564_2, s_564_0, s_564_1);
        // D s_564_4: cast reint s_564_3 -> u8
        let s_564_4: u8 = (s_564_3.value() as u8);
        // C s_564_5: const #2s : i
        let s_564_5: i128 = 2;
        // C s_564_6: const #0s : i
        let s_564_6: i128 = 0;
        // D s_564_7: read-var op2:i
        let s_564_7: i128 = fn_state.op2;
        // D s_564_8: call integer_subrange(s_564_7, s_564_5, s_564_6)
        let s_564_8: Bits = integer_subrange(state, tracer, s_564_7, s_564_5, s_564_6);
        // D s_564_9: cast reint s_564_8 -> u8
        let s_564_9: u8 = (s_564_8.value() as u8);
        // D s_564_10: cast zx s_564_4 -> bv
        let s_564_10: Bits = Bits::new(s_564_4 as u128, 2u16);
        // D s_564_11: cast zx s_564_9 -> bv
        let s_564_11: Bits = Bits::new(s_564_9 as u128, 3u16);
        // D s_564_12: cast reint s_564_10 -> u128
        let s_564_12: u128 = (s_564_10.value() as u128);
        // D s_564_13: size-of s_564_10
        let s_564_13: u16 = s_564_10.length();
        // D s_564_14: cast reint s_564_11 -> u128
        let s_564_14: u128 = (s_564_11.value() as u128);
        // D s_564_15: size-of s_564_11
        let s_564_15: u16 = s_564_11.length();
        // D s_564_16: lsl s_564_12 s_564_15
        let s_564_16: u128 = s_564_12 << s_564_15;
        // D s_564_17: or s_564_16 s_564_14
        let s_564_17: u128 = ((s_564_16) | (s_564_14));
        // D s_564_18: add s_564_13 s_564_15
        let s_564_18: u16 = (s_564_13 + s_564_15);
        // D s_564_19: create-bits s_564_17 s_564_18
        let s_564_19: Bits = Bits::new(s_564_17, s_564_18);
        // D s_564_20: cast reint s_564_19 -> u8
        let s_564_20: u8 = (s_564_19.value() as u8);
        // D s_564_21: cast zx s_564_20 -> bv
        let s_564_21: Bits = Bits::new(s_564_20 as u128, 5u16);
        // C s_564_22: const #31u : u8
        let s_564_22: u8 = 31;
        // C s_564_23: cast zx s_564_22 -> bv
        let s_564_23: Bits = Bits::new(s_564_22 as u128, 5u16);
        // D s_564_24: cmp-ne s_564_21 s_564_23
        let s_564_24: bool = ((s_564_21) != (s_564_23));
        // D s_564_25: write-var gs#141337 <= s_564_24
        fn_state.gs_141337 = s_564_24;
        // N s_564_26: jump b543
        return block_543(state, tracer, fn_state);
    }
    fn block_565<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_565_0: const #3s : i
        let s_565_0: i128 = 3;
        // C s_565_1: const #2s : i
        let s_565_1: i128 = 2;
        // D s_565_2: read-var crm:i
        let s_565_2: i128 = fn_state.crm;
        // D s_565_3: call integer_subrange(s_565_2, s_565_0, s_565_1)
        let s_565_3: Bits = integer_subrange(state, tracer, s_565_2, s_565_0, s_565_1);
        // D s_565_4: cast reint s_565_3 -> u8
        let s_565_4: u8 = (s_565_3.value() as u8);
        // D s_565_5: cast zx s_565_4 -> bv
        let s_565_5: Bits = Bits::new(s_565_4 as u128, 2u16);
        // C s_565_6: const #2u : u8
        let s_565_6: u8 = 2;
        // C s_565_7: cast zx s_565_6 -> bv
        let s_565_7: Bits = Bits::new(s_565_6 as u128, 2u16);
        // D s_565_8: cmp-eq s_565_5 s_565_7
        let s_565_8: bool = ((s_565_5) == (s_565_7));
        // N s_565_9: branch s_565_8 b571 b566
        if s_565_8 {
            return block_571(state, tracer, fn_state);
        } else {
            return block_566(state, tracer, fn_state);
        };
    }
    fn block_566<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_566_0: const #3s : i
        let s_566_0: i128 = 3;
        // C s_566_1: const #2s : i
        let s_566_1: i128 = 2;
        // D s_566_2: read-var crm:i
        let s_566_2: i128 = fn_state.crm;
        // D s_566_3: call integer_subrange(s_566_2, s_566_0, s_566_1)
        let s_566_3: Bits = integer_subrange(state, tracer, s_566_2, s_566_0, s_566_1);
        // D s_566_4: cast reint s_566_3 -> u8
        let s_566_4: u8 = (s_566_3.value() as u8);
        // D s_566_5: cast zx s_566_4 -> bv
        let s_566_5: Bits = Bits::new(s_566_4 as u128, 2u16);
        // C s_566_6: const #3u : u8
        let s_566_6: u8 = 3;
        // C s_566_7: cast zx s_566_6 -> bv
        let s_566_7: Bits = Bits::new(s_566_6 as u128, 2u16);
        // D s_566_8: cmp-eq s_566_5 s_566_7
        let s_566_8: bool = ((s_566_5) == (s_566_7));
        // N s_566_9: branch s_566_8 b570 b567
        if s_566_8 {
            return block_570(state, tracer, fn_state);
        } else {
            return block_567(state, tracer, fn_state);
        };
    }
    fn block_567<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_567_0: const #0u : u8
        let s_567_0: bool = false;
        // D s_567_1: write-var gs#141277 <= s_567_0
        fn_state.gs_141277 = s_567_0;
        // N s_567_2: jump b568
        return block_568(state, tracer, fn_state);
    }
    fn block_568<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_568_0: read-var gs#141277:u8
        let s_568_0: bool = fn_state.gs_141277;
        // D s_568_1: write-var gs#141278 <= s_568_0
        fn_state.gs_141278 = s_568_0;
        // N s_568_2: jump b569
        return block_569(state, tracer, fn_state);
    }
    fn block_569<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_569_0: read-var gs#141278:u8
        let s_569_0: bool = fn_state.gs_141278;
        // D s_569_1: write-var gs#141279 <= s_569_0
        fn_state.gs_141279 = s_569_0;
        // N s_569_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_570<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_570_0: const #1s : i
        let s_570_0: i128 = 1;
        // C s_570_1: const #0s : i
        let s_570_1: i128 = 0;
        // D s_570_2: read-var crm:i
        let s_570_2: i128 = fn_state.crm;
        // D s_570_3: call integer_subrange(s_570_2, s_570_0, s_570_1)
        let s_570_3: Bits = integer_subrange(state, tracer, s_570_2, s_570_0, s_570_1);
        // D s_570_4: cast reint s_570_3 -> u8
        let s_570_4: u8 = (s_570_3.value() as u8);
        // C s_570_5: const #2s : i
        let s_570_5: i128 = 2;
        // C s_570_6: const #0s : i
        let s_570_6: i128 = 0;
        // D s_570_7: read-var op2:i
        let s_570_7: i128 = fn_state.op2;
        // D s_570_8: call integer_subrange(s_570_7, s_570_5, s_570_6)
        let s_570_8: Bits = integer_subrange(state, tracer, s_570_7, s_570_5, s_570_6);
        // D s_570_9: cast reint s_570_8 -> u8
        let s_570_9: u8 = (s_570_8.value() as u8);
        // D s_570_10: cast zx s_570_4 -> bv
        let s_570_10: Bits = Bits::new(s_570_4 as u128, 2u16);
        // D s_570_11: cast zx s_570_9 -> bv
        let s_570_11: Bits = Bits::new(s_570_9 as u128, 3u16);
        // D s_570_12: cast reint s_570_10 -> u128
        let s_570_12: u128 = (s_570_10.value() as u128);
        // D s_570_13: size-of s_570_10
        let s_570_13: u16 = s_570_10.length();
        // D s_570_14: cast reint s_570_11 -> u128
        let s_570_14: u128 = (s_570_11.value() as u128);
        // D s_570_15: size-of s_570_11
        let s_570_15: u16 = s_570_11.length();
        // D s_570_16: lsl s_570_12 s_570_15
        let s_570_16: u128 = s_570_12 << s_570_15;
        // D s_570_17: or s_570_16 s_570_14
        let s_570_17: u128 = ((s_570_16) | (s_570_14));
        // D s_570_18: add s_570_13 s_570_15
        let s_570_18: u16 = (s_570_13 + s_570_15);
        // D s_570_19: create-bits s_570_17 s_570_18
        let s_570_19: Bits = Bits::new(s_570_17, s_570_18);
        // D s_570_20: cast reint s_570_19 -> u8
        let s_570_20: u8 = (s_570_19.value() as u8);
        // D s_570_21: cast zx s_570_20 -> bv
        let s_570_21: Bits = Bits::new(s_570_20 as u128, 5u16);
        // C s_570_22: const #31u : u8
        let s_570_22: u8 = 31;
        // C s_570_23: cast zx s_570_22 -> bv
        let s_570_23: Bits = Bits::new(s_570_22 as u128, 5u16);
        // D s_570_24: cmp-ne s_570_21 s_570_23
        let s_570_24: bool = ((s_570_21) != (s_570_23));
        // D s_570_25: write-var gs#141277 <= s_570_24
        fn_state.gs_141277 = s_570_24;
        // N s_570_26: jump b568
        return block_568(state, tracer, fn_state);
    }
    fn block_571<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_571_0: const #1u : u8
        let s_571_0: bool = true;
        // D s_571_1: write-var gs#141278 <= s_571_0
        fn_state.gs_141278 = s_571_0;
        // N s_571_2: jump b569
        return block_569(state, tracer, fn_state);
    }
    fn block_572<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_572_0: const #3s : i
        let s_572_0: i128 = 3;
        // D s_572_1: read-var op1:i
        let s_572_1: i128 = fn_state.op1;
        // D s_572_2: cmp-eq s_572_1 s_572_0
        let s_572_2: bool = ((s_572_1) == (s_572_0));
        // D s_572_3: write-var gs#141268 <= s_572_2
        fn_state.gs_141268 = s_572_2;
        // N s_572_4: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_573<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_573_0: const #14s : i
        let s_573_0: i128 = 14;
        // D s_573_1: read-var crn:i
        let s_573_1: i128 = fn_state.crn;
        // D s_573_2: cmp-eq s_573_1 s_573_0
        let s_573_2: bool = ((s_573_1) == (s_573_0));
        // D s_573_3: write-var gs#141266 <= s_573_2
        fn_state.gs_141266 = s_573_2;
        // N s_573_4: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_574<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_574_0: const #3s : i
        let s_574_0: i128 = 3;
        // D s_574_1: read-var tempxt:u64
        let s_574_1: u64 = fn_state.tempxt;
        // D s_574_2: cast zx s_574_1 -> bv
        let s_574_2: Bits = Bits::new(s_574_1 as u128, 64u16);
        // C s_574_3: const #1u : u64
        let s_574_3: u64 = 1;
        // D s_574_4: bit-extract s_574_2 s_574_0 s_574_3
        let s_574_4: Bits = (Bits::new(
            ((s_574_2) >> (s_574_0)).value(),
            u16::try_from(s_574_3).unwrap(),
        ));
        // D s_574_5: cast reint s_574_4 -> u8
        let s_574_5: bool = ((s_574_4.value()) != 0);
        // C s_574_6: const #0s : i
        let s_574_6: i128 = 0;
        // C s_574_7: const #0u : u64
        let s_574_7: u64 = 0;
        // D s_574_8: cast zx s_574_5 -> u64
        let s_574_8: u64 = (s_574_5 as u64);
        // C s_574_9: const #1u : u64
        let s_574_9: u64 = 1;
        // D s_574_10: and s_574_8 s_574_9
        let s_574_10: u64 = ((s_574_8) & (s_574_9));
        // D s_574_11: cmp-eq s_574_10 s_574_9
        let s_574_11: bool = ((s_574_10) == (s_574_9));
        // D s_574_12: lsl s_574_8 s_574_6
        let s_574_12: u64 = s_574_8 << s_574_6;
        // D s_574_13: or s_574_7 s_574_12
        let s_574_13: u64 = ((s_574_7) | (s_574_12));
        // D s_574_14: cmpl s_574_12
        let s_574_14: u64 = !s_574_12;
        // D s_574_15: and s_574_7 s_574_14
        let s_574_15: u64 = ((s_574_7) & (s_574_14));
        // D s_574_16: select s_574_11 s_574_13 s_574_15
        let s_574_16: u64 = if s_574_11 { s_574_13 } else { s_574_15 };
        // D s_574_17: cast trunc s_574_16 -> u8
        let s_574_17: bool = ((s_574_16) != 0);
        // D s_574_18: cast zx s_574_17 -> bv
        let s_574_18: Bits = Bits::new(s_574_17 as u128, 1u16);
        // C s_574_19: const #1u : u8
        let s_574_19: bool = true;
        // C s_574_20: cast zx s_574_19 -> bv
        let s_574_20: Bits = Bits::new(s_574_19 as u128, 1u16);
        // D s_574_21: cmp-eq s_574_18 s_574_20
        let s_574_21: bool = ((s_574_18) == (s_574_20));
        // N s_574_22: branch s_574_21 b580 b575
        if s_574_21 {
            return block_580(state, tracer, fn_state);
        } else {
            return block_575(state, tracer, fn_state);
        };
    }
    fn block_575<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_575_0: const #0u : u8
        let s_575_0: bool = false;
        // D s_575_1: write-var gs#141357 <= s_575_0
        fn_state.gs_141357 = s_575_0;
        // N s_575_2: jump b576
        return block_576(state, tracer, fn_state);
    }
    fn block_576<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_576_0: read-var gs#141357:u8
        let s_576_0: bool = fn_state.gs_141357;
        // N s_576_1: branch s_576_0 b579 b577
        if s_576_0 {
            return block_579(state, tracer, fn_state);
        } else {
            return block_577(state, tracer, fn_state);
        };
    }
    fn block_577<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_577_0: jump b578
        return block_578(state, tracer, fn_state);
    }
    fn block_578<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_578_0: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_579<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_579_0: const #63s : i
        let s_579_0: i128 = 63;
        // C s_579_1: const #15848u : u32
        let s_579_1: u32 = 15848;
        // N s_579_2: write-reg s_579_1 <= s_579_0
        let s_579_2: () = {
            state.write_register::<i128>(s_579_1 as isize, s_579_0);
            tracer.write_register(s_579_1 as isize, s_579_0);
        };
        // N s_579_3: jump b578
        return block_578(state, tracer, fn_state);
    }
    fn block_580<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_580_0: const #21016u : u32
        let s_580_0: u32 = 21016;
        // D s_580_1: read-reg s_580_0:struct
        let s_580_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_580_0 as isize);
            tracer.read_register(s_580_0 as isize, value);
            value
        };
        // D s_580_2: call _get_PMCR_EL0_Type_D(s_580_1)
        let s_580_2: bool = u_get_PMCR_EL0_Type_D(state, tracer, s_580_1);
        // D s_580_3: cast zx s_580_2 -> bv
        let s_580_3: Bits = Bits::new(s_580_2 as u128, 1u16);
        // C s_580_4: const #0u : u8
        let s_580_4: bool = false;
        // C s_580_5: cast zx s_580_4 -> bv
        let s_580_5: Bits = Bits::new(s_580_4 as u128, 1u16);
        // D s_580_6: cmp-eq s_580_3 s_580_5
        let s_580_6: bool = ((s_580_3) == (s_580_5));
        // D s_580_7: write-var gs#141357 <= s_580_6
        fn_state.gs_141357 = s_580_6;
        // N s_580_8: jump b576
        return block_576(state, tracer, fn_state);
    }
    fn block_581<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_581_0: const #12s : i
        let s_581_0: i128 = 12;
        // D s_581_1: read-var crm:i
        let s_581_1: i128 = fn_state.crm;
        // D s_581_2: cmp-eq s_581_1 s_581_0
        let s_581_2: bool = ((s_581_1) == (s_581_0));
        // D s_581_3: write-var gs#141263 <= s_581_2
        fn_state.gs_141263 = s_581_2;
        // N s_581_4: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_582<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_582_0: const #0s : i
        let s_582_0: i128 = 0;
        // D s_582_1: read-var op2:i
        let s_582_1: i128 = fn_state.op2;
        // D s_582_2: cmp-eq s_582_1 s_582_0
        let s_582_2: bool = ((s_582_1) == (s_582_0));
        // D s_582_3: write-var gs#141261 <= s_582_2
        fn_state.gs_141261 = s_582_2;
        // N s_582_4: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_583<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_583_0: const #3s : i
        let s_583_0: i128 = 3;
        // D s_583_1: read-var op1:i
        let s_583_1: i128 = fn_state.op1;
        // D s_583_2: cmp-eq s_583_1 s_583_0
        let s_583_2: bool = ((s_583_1) == (s_583_0));
        // D s_583_3: write-var gs#141259 <= s_583_2
        fn_state.gs_141259 = s_583_2;
        // N s_583_4: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_584<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_584_0: const #9s : i
        let s_584_0: i128 = 9;
        // D s_584_1: read-var crn:i
        let s_584_1: i128 = fn_state.crn;
        // D s_584_2: cmp-eq s_584_1 s_584_0
        let s_584_2: bool = ((s_584_1) == (s_584_0));
        // D s_584_3: write-var gs#141257 <= s_584_2
        fn_state.gs_141257 = s_584_2;
        // N s_584_4: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_585<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_585_0: const #2s : i
        let s_585_0: i128 = 2;
        // D s_585_1: read-var op2:i
        let s_585_1: i128 = fn_state.op2;
        // D s_585_2: cmp-eq s_585_1 s_585_0
        let s_585_2: bool = ((s_585_1) == (s_585_0));
        // N s_585_3: branch s_585_2 b591 b586
        if s_585_2 {
            return block_591(state, tracer, fn_state);
        } else {
            return block_586(state, tracer, fn_state);
        };
    }
    fn block_586<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_586_0: jump b587
        return block_587(state, tracer, fn_state);
    }
    fn block_587<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_587_0: const #1s : i
        let s_587_0: i128 = 1;
        // D s_587_1: read-var op2:i
        let s_587_1: i128 = fn_state.op2;
        // D s_587_2: cmp-eq s_587_1 s_587_0
        let s_587_2: bool = ((s_587_1) == (s_587_0));
        // N s_587_3: branch s_587_2 b590 b588
        if s_587_2 {
            return block_590(state, tracer, fn_state);
        } else {
            return block_588(state, tracer, fn_state);
        };
    }
    fn block_588<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_588_0: jump b589
        return block_589(state, tracer, fn_state);
    }
    fn block_589<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_589_0: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_590<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_590_0: const #90320u : u32
        let s_590_0: u32 = 90320;
        // D s_590_1: read-reg s_590_0:u64
        let s_590_1: u64 = {
            let value = state.read_register::<u64>(s_590_0 as isize);
            tracer.read_register(s_590_0 as isize, value);
            value
        };
        // D s_590_2: read-var tempxt:u64
        let s_590_2: u64 = fn_state.tempxt;
        // D s_590_3: cast zx s_590_2 -> bv
        let s_590_3: Bits = Bits::new(s_590_2 as u128, 64u16);
        // D s_590_4: read-var mask:u64
        let s_590_4: u64 = fn_state.mask;
        // D s_590_5: cast zx s_590_4 -> bv
        let s_590_5: Bits = Bits::new(s_590_4 as u128, 64u16);
        // D s_590_6: and s_590_3 s_590_5
        let s_590_6: Bits = ((s_590_3) & (s_590_5));
        // D s_590_7: cast reint s_590_6 -> u64
        let s_590_7: u64 = (s_590_6.value() as u64);
        // D s_590_8: cast zx s_590_1 -> bv
        let s_590_8: Bits = Bits::new(s_590_1 as u128, 64u16);
        // D s_590_9: cast zx s_590_7 -> bv
        let s_590_9: Bits = Bits::new(s_590_7 as u128, 64u16);
        // D s_590_10: or s_590_8 s_590_9
        let s_590_10: Bits = ((s_590_8) | (s_590_9));
        // D s_590_11: cast reint s_590_10 -> u64
        let s_590_11: u64 = (s_590_10.value() as u64);
        // D s_590_12: write-var tempxt <= s_590_11
        fn_state.tempxt = s_590_11;
        // N s_590_13: jump b589
        return block_589(state, tracer, fn_state);
    }
    fn block_591<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_591_0: const #90320u : u32
        let s_591_0: u32 = 90320;
        // D s_591_1: read-reg s_591_0:u64
        let s_591_1: u64 = {
            let value = state.read_register::<u64>(s_591_0 as isize);
            tracer.read_register(s_591_0 as isize, value);
            value
        };
        // D s_591_2: read-var tempxt:u64
        let s_591_2: u64 = fn_state.tempxt;
        // D s_591_3: cast zx s_591_2 -> bv
        let s_591_3: Bits = Bits::new(s_591_2 as u128, 64u16);
        // D s_591_4: read-var mask:u64
        let s_591_4: u64 = fn_state.mask;
        // D s_591_5: cast zx s_591_4 -> bv
        let s_591_5: Bits = Bits::new(s_591_4 as u128, 64u16);
        // D s_591_6: and s_591_3 s_591_5
        let s_591_6: Bits = ((s_591_3) & (s_591_5));
        // D s_591_7: cast reint s_591_6 -> u64
        let s_591_7: u64 = (s_591_6.value() as u64);
        // D s_591_8: cast zx s_591_7 -> bv
        let s_591_8: Bits = Bits::new(s_591_7 as u128, 64u16);
        // D s_591_9: not s_591_8
        let s_591_9: Bits = !s_591_8;
        // D s_591_10: cast reint s_591_9 -> u64
        let s_591_10: u64 = (s_591_9.value() as u64);
        // D s_591_11: cast zx s_591_1 -> bv
        let s_591_11: Bits = Bits::new(s_591_1 as u128, 64u16);
        // D s_591_12: cast zx s_591_10 -> bv
        let s_591_12: Bits = Bits::new(s_591_10 as u128, 64u16);
        // D s_591_13: and s_591_11 s_591_12
        let s_591_13: Bits = ((s_591_11) & (s_591_12));
        // D s_591_14: cast reint s_591_13 -> u64
        let s_591_14: u64 = (s_591_13.value() as u64);
        // D s_591_15: write-var tempxt <= s_591_14
        fn_state.tempxt = s_591_14;
        // N s_591_16: jump b587
        return block_587(state, tracer, fn_state);
    }
    fn block_592<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_592_0: const #14s : i
        let s_592_0: i128 = 14;
        // D s_592_1: read-var crm:i
        let s_592_1: i128 = fn_state.crm;
        // D s_592_2: cmp-eq s_592_1 s_592_0
        let s_592_2: bool = ((s_592_1) == (s_592_0));
        // D s_592_3: write-var gs#141254 <= s_592_2
        fn_state.gs_141254 = s_592_2;
        // N s_592_4: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_593<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_593_0: const #1s : i
        let s_593_0: i128 = 1;
        // D s_593_1: read-var op2:i
        let s_593_1: i128 = fn_state.op2;
        // D s_593_2: cmp-eq s_593_1 s_593_0
        let s_593_2: bool = ((s_593_1) == (s_593_0));
        // N s_593_3: branch s_593_2 b596 b594
        if s_593_2 {
            return block_596(state, tracer, fn_state);
        } else {
            return block_594(state, tracer, fn_state);
        };
    }
    fn block_594<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_594_0: const #2s : i
        let s_594_0: i128 = 2;
        // D s_594_1: read-var op2:i
        let s_594_1: i128 = fn_state.op2;
        // D s_594_2: cmp-eq s_594_1 s_594_0
        let s_594_2: bool = ((s_594_1) == (s_594_0));
        // D s_594_3: write-var gs#141251 <= s_594_2
        fn_state.gs_141251 = s_594_2;
        // N s_594_4: jump b595
        return block_595(state, tracer, fn_state);
    }
    fn block_595<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_595_0: read-var gs#141251:u8
        let s_595_0: bool = fn_state.gs_141251;
        // D s_595_1: write-var gs#141252 <= s_595_0
        fn_state.gs_141252 = s_595_0;
        // N s_595_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_596<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_596_0: const #1u : u8
        let s_596_0: bool = true;
        // D s_596_1: write-var gs#141251 <= s_596_0
        fn_state.gs_141251 = s_596_0;
        // N s_596_2: jump b595
        return block_595(state, tracer, fn_state);
    }
    fn block_597<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_597_0: const #0s : i
        let s_597_0: i128 = 0;
        // D s_597_1: read-var op1:i
        let s_597_1: i128 = fn_state.op1;
        // D s_597_2: cmp-eq s_597_1 s_597_0
        let s_597_2: bool = ((s_597_1) == (s_597_0));
        // D s_597_3: write-var gs#141248 <= s_597_2
        fn_state.gs_141248 = s_597_2;
        // N s_597_4: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_598<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_598_0: const #9s : i
        let s_598_0: i128 = 9;
        // D s_598_1: read-var crn:i
        let s_598_1: i128 = fn_state.crn;
        // D s_598_2: cmp-eq s_598_1 s_598_0
        let s_598_2: bool = ((s_598_1) == (s_598_0));
        // D s_598_3: write-var gs#141246 <= s_598_2
        fn_state.gs_141246 = s_598_2;
        // N s_598_4: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_599<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_599_0: const #2s : i
        let s_599_0: i128 = 2;
        // D s_599_1: read-var op2:i
        let s_599_1: i128 = fn_state.op2;
        // D s_599_2: cmp-eq s_599_1 s_599_0
        let s_599_2: bool = ((s_599_1) == (s_599_0));
        // N s_599_3: branch s_599_2 b605 b600
        if s_599_2 {
            return block_605(state, tracer, fn_state);
        } else {
            return block_600(state, tracer, fn_state);
        };
    }
    fn block_600<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_600_0: jump b601
        return block_601(state, tracer, fn_state);
    }
    fn block_601<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_601_0: const #1s : i
        let s_601_0: i128 = 1;
        // D s_601_1: read-var op2:i
        let s_601_1: i128 = fn_state.op2;
        // D s_601_2: cmp-eq s_601_1 s_601_0
        let s_601_2: bool = ((s_601_1) == (s_601_0));
        // N s_601_3: branch s_601_2 b604 b602
        if s_601_2 {
            return block_604(state, tracer, fn_state);
        } else {
            return block_602(state, tracer, fn_state);
        };
    }
    fn block_602<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_602_0: jump b603
        return block_603(state, tracer, fn_state);
    }
    fn block_603<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_603_0: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_604<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_604_0: const #11736u : u32
        let s_604_0: u32 = 11736;
        // D s_604_1: read-reg s_604_0:u64
        let s_604_1: u64 = {
            let value = state.read_register::<u64>(s_604_0 as isize);
            tracer.read_register(s_604_0 as isize, value);
            value
        };
        // D s_604_2: read-var tempxt:u64
        let s_604_2: u64 = fn_state.tempxt;
        // D s_604_3: cast zx s_604_2 -> bv
        let s_604_3: Bits = Bits::new(s_604_2 as u128, 64u16);
        // D s_604_4: read-var mask:u64
        let s_604_4: u64 = fn_state.mask;
        // D s_604_5: cast zx s_604_4 -> bv
        let s_604_5: Bits = Bits::new(s_604_4 as u128, 64u16);
        // D s_604_6: and s_604_3 s_604_5
        let s_604_6: Bits = ((s_604_3) & (s_604_5));
        // D s_604_7: cast reint s_604_6 -> u64
        let s_604_7: u64 = (s_604_6.value() as u64);
        // D s_604_8: cast zx s_604_1 -> bv
        let s_604_8: Bits = Bits::new(s_604_1 as u128, 64u16);
        // D s_604_9: cast zx s_604_7 -> bv
        let s_604_9: Bits = Bits::new(s_604_7 as u128, 64u16);
        // D s_604_10: or s_604_8 s_604_9
        let s_604_10: Bits = ((s_604_8) | (s_604_9));
        // D s_604_11: cast reint s_604_10 -> u64
        let s_604_11: u64 = (s_604_10.value() as u64);
        // D s_604_12: write-var tempxt <= s_604_11
        fn_state.tempxt = s_604_11;
        // N s_604_13: jump b603
        return block_603(state, tracer, fn_state);
    }
    fn block_605<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_605_0: const #11736u : u32
        let s_605_0: u32 = 11736;
        // D s_605_1: read-reg s_605_0:u64
        let s_605_1: u64 = {
            let value = state.read_register::<u64>(s_605_0 as isize);
            tracer.read_register(s_605_0 as isize, value);
            value
        };
        // D s_605_2: read-var tempxt:u64
        let s_605_2: u64 = fn_state.tempxt;
        // D s_605_3: cast zx s_605_2 -> bv
        let s_605_3: Bits = Bits::new(s_605_2 as u128, 64u16);
        // D s_605_4: read-var mask:u64
        let s_605_4: u64 = fn_state.mask;
        // D s_605_5: cast zx s_605_4 -> bv
        let s_605_5: Bits = Bits::new(s_605_4 as u128, 64u16);
        // D s_605_6: and s_605_3 s_605_5
        let s_605_6: Bits = ((s_605_3) & (s_605_5));
        // D s_605_7: cast reint s_605_6 -> u64
        let s_605_7: u64 = (s_605_6.value() as u64);
        // D s_605_8: cast zx s_605_7 -> bv
        let s_605_8: Bits = Bits::new(s_605_7 as u128, 64u16);
        // D s_605_9: not s_605_8
        let s_605_9: Bits = !s_605_8;
        // D s_605_10: cast reint s_605_9 -> u64
        let s_605_10: u64 = (s_605_9.value() as u64);
        // D s_605_11: cast zx s_605_1 -> bv
        let s_605_11: Bits = Bits::new(s_605_1 as u128, 64u16);
        // D s_605_12: cast zx s_605_10 -> bv
        let s_605_12: Bits = Bits::new(s_605_10 as u128, 64u16);
        // D s_605_13: and s_605_11 s_605_12
        let s_605_13: Bits = ((s_605_11) & (s_605_12));
        // D s_605_14: cast reint s_605_13 -> u64
        let s_605_14: u64 = (s_605_13.value() as u64);
        // D s_605_15: write-var tempxt <= s_605_14
        fn_state.tempxt = s_605_14;
        // N s_605_16: jump b601
        return block_601(state, tracer, fn_state);
    }
    fn block_606<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_606_0: const #12s : i
        let s_606_0: i128 = 12;
        // D s_606_1: read-var crm:i
        let s_606_1: i128 = fn_state.crm;
        // D s_606_2: cmp-eq s_606_1 s_606_0
        let s_606_2: bool = ((s_606_1) == (s_606_0));
        // D s_606_3: write-var gs#141243 <= s_606_2
        fn_state.gs_141243 = s_606_2;
        // N s_606_4: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_607<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_607_0: const #1s : i
        let s_607_0: i128 = 1;
        // D s_607_1: read-var op2:i
        let s_607_1: i128 = fn_state.op2;
        // D s_607_2: cmp-eq s_607_1 s_607_0
        let s_607_2: bool = ((s_607_1) == (s_607_0));
        // N s_607_3: branch s_607_2 b610 b608
        if s_607_2 {
            return block_610(state, tracer, fn_state);
        } else {
            return block_608(state, tracer, fn_state);
        };
    }
    fn block_608<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_608_0: const #2s : i
        let s_608_0: i128 = 2;
        // D s_608_1: read-var op2:i
        let s_608_1: i128 = fn_state.op2;
        // D s_608_2: cmp-eq s_608_1 s_608_0
        let s_608_2: bool = ((s_608_1) == (s_608_0));
        // D s_608_3: write-var gs#141240 <= s_608_2
        fn_state.gs_141240 = s_608_2;
        // N s_608_4: jump b609
        return block_609(state, tracer, fn_state);
    }
    fn block_609<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_609_0: read-var gs#141240:u8
        let s_609_0: bool = fn_state.gs_141240;
        // D s_609_1: write-var gs#141241 <= s_609_0
        fn_state.gs_141241 = s_609_0;
        // N s_609_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_610<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_610_0: const #1u : u8
        let s_610_0: bool = true;
        // D s_610_1: write-var gs#141240 <= s_610_0
        fn_state.gs_141240 = s_610_0;
        // N s_610_2: jump b609
        return block_609(state, tracer, fn_state);
    }
    fn block_611<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_611_0: const #3s : i
        let s_611_0: i128 = 3;
        // D s_611_1: read-var op1:i
        let s_611_1: i128 = fn_state.op1;
        // D s_611_2: cmp-eq s_611_1 s_611_0
        let s_611_2: bool = ((s_611_1) == (s_611_0));
        // D s_611_3: write-var gs#141237 <= s_611_2
        fn_state.gs_141237 = s_611_2;
        // N s_611_4: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_612<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_612_0: const #9s : i
        let s_612_0: i128 = 9;
        // D s_612_1: read-var crn:i
        let s_612_1: i128 = fn_state.crn;
        // D s_612_2: cmp-eq s_612_1 s_612_0
        let s_612_2: bool = ((s_612_1) == (s_612_0));
        // D s_612_3: write-var gs#141235 <= s_612_2
        fn_state.gs_141235 = s_612_2;
        // N s_612_4: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_613<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_613_0: const #12s : i
        let s_613_0: i128 = 12;
        // D s_613_1: read-var crm:i
        let s_613_1: i128 = fn_state.crm;
        // D s_613_2: cmp-eq s_613_1 s_613_0
        let s_613_2: bool = ((s_613_1) == (s_613_0));
        // N s_613_3: branch s_613_2 b619 b614
        if s_613_2 {
            return block_619(state, tracer, fn_state);
        } else {
            return block_614(state, tracer, fn_state);
        };
    }
    fn block_614<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_614_0: jump b615
        return block_615(state, tracer, fn_state);
    }
    fn block_615<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_615_0: const #14s : i
        let s_615_0: i128 = 14;
        // D s_615_1: read-var crm:i
        let s_615_1: i128 = fn_state.crm;
        // D s_615_2: cmp-eq s_615_1 s_615_0
        let s_615_2: bool = ((s_615_1) == (s_615_0));
        // N s_615_3: branch s_615_2 b618 b616
        if s_615_2 {
            return block_618(state, tracer, fn_state);
        } else {
            return block_616(state, tracer, fn_state);
        };
    }
    fn block_616<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_616_0: jump b617
        return block_617(state, tracer, fn_state);
    }
    fn block_617<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_617_0: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_618<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_618_0: const #104640u : u32
        let s_618_0: u32 = 104640;
        // D s_618_1: read-reg s_618_0:u64
        let s_618_1: u64 = {
            let value = state.read_register::<u64>(s_618_0 as isize);
            tracer.read_register(s_618_0 as isize, value);
            value
        };
        // D s_618_2: read-var tempxt:u64
        let s_618_2: u64 = fn_state.tempxt;
        // D s_618_3: cast zx s_618_2 -> bv
        let s_618_3: Bits = Bits::new(s_618_2 as u128, 64u16);
        // D s_618_4: read-var mask:u64
        let s_618_4: u64 = fn_state.mask;
        // D s_618_5: cast zx s_618_4 -> bv
        let s_618_5: Bits = Bits::new(s_618_4 as u128, 64u16);
        // D s_618_6: and s_618_3 s_618_5
        let s_618_6: Bits = ((s_618_3) & (s_618_5));
        // D s_618_7: cast reint s_618_6 -> u64
        let s_618_7: u64 = (s_618_6.value() as u64);
        // D s_618_8: cast zx s_618_1 -> bv
        let s_618_8: Bits = Bits::new(s_618_1 as u128, 64u16);
        // D s_618_9: cast zx s_618_7 -> bv
        let s_618_9: Bits = Bits::new(s_618_7 as u128, 64u16);
        // D s_618_10: or s_618_8 s_618_9
        let s_618_10: Bits = ((s_618_8) | (s_618_9));
        // D s_618_11: cast reint s_618_10 -> u64
        let s_618_11: u64 = (s_618_10.value() as u64);
        // D s_618_12: write-var tempxt <= s_618_11
        fn_state.tempxt = s_618_11;
        // N s_618_13: jump b617
        return block_617(state, tracer, fn_state);
    }
    fn block_619<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_619_0: const #104640u : u32
        let s_619_0: u32 = 104640;
        // D s_619_1: read-reg s_619_0:u64
        let s_619_1: u64 = {
            let value = state.read_register::<u64>(s_619_0 as isize);
            tracer.read_register(s_619_0 as isize, value);
            value
        };
        // D s_619_2: read-var tempxt:u64
        let s_619_2: u64 = fn_state.tempxt;
        // D s_619_3: cast zx s_619_2 -> bv
        let s_619_3: Bits = Bits::new(s_619_2 as u128, 64u16);
        // D s_619_4: read-var mask:u64
        let s_619_4: u64 = fn_state.mask;
        // D s_619_5: cast zx s_619_4 -> bv
        let s_619_5: Bits = Bits::new(s_619_4 as u128, 64u16);
        // D s_619_6: and s_619_3 s_619_5
        let s_619_6: Bits = ((s_619_3) & (s_619_5));
        // D s_619_7: cast reint s_619_6 -> u64
        let s_619_7: u64 = (s_619_6.value() as u64);
        // D s_619_8: cast zx s_619_7 -> bv
        let s_619_8: Bits = Bits::new(s_619_7 as u128, 64u16);
        // D s_619_9: not s_619_8
        let s_619_9: Bits = !s_619_8;
        // D s_619_10: cast reint s_619_9 -> u64
        let s_619_10: u64 = (s_619_9.value() as u64);
        // D s_619_11: cast zx s_619_1 -> bv
        let s_619_11: Bits = Bits::new(s_619_1 as u128, 64u16);
        // D s_619_12: cast zx s_619_10 -> bv
        let s_619_12: Bits = Bits::new(s_619_10 as u128, 64u16);
        // D s_619_13: and s_619_11 s_619_12
        let s_619_13: Bits = ((s_619_11) & (s_619_12));
        // D s_619_14: cast reint s_619_13 -> u64
        let s_619_14: u64 = (s_619_13.value() as u64);
        // D s_619_15: write-var tempxt <= s_619_14
        fn_state.tempxt = s_619_14;
        // N s_619_16: jump b615
        return block_615(state, tracer, fn_state);
    }
    fn block_620<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_620_0: const #12s : i
        let s_620_0: i128 = 12;
        // D s_620_1: read-var crm:i
        let s_620_1: i128 = fn_state.crm;
        // D s_620_2: cmp-eq s_620_1 s_620_0
        let s_620_2: bool = ((s_620_1) == (s_620_0));
        // N s_620_3: branch s_620_2 b623 b621
        if s_620_2 {
            return block_623(state, tracer, fn_state);
        } else {
            return block_621(state, tracer, fn_state);
        };
    }
    fn block_621<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_621_0: const #14s : i
        let s_621_0: i128 = 14;
        // D s_621_1: read-var crm:i
        let s_621_1: i128 = fn_state.crm;
        // D s_621_2: cmp-eq s_621_1 s_621_0
        let s_621_2: bool = ((s_621_1) == (s_621_0));
        // D s_621_3: write-var gs#141231 <= s_621_2
        fn_state.gs_141231 = s_621_2;
        // N s_621_4: jump b622
        return block_622(state, tracer, fn_state);
    }
    fn block_622<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_622_0: read-var gs#141231:u8
        let s_622_0: bool = fn_state.gs_141231;
        // D s_622_1: write-var gs#141232 <= s_622_0
        fn_state.gs_141232 = s_622_0;
        // N s_622_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_623<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_623_0: const #1u : u8
        let s_623_0: bool = true;
        // D s_623_1: write-var gs#141231 <= s_623_0
        fn_state.gs_141231 = s_623_0;
        // N s_623_2: jump b622
        return block_622(state, tracer, fn_state);
    }
    fn block_624<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_624_0: const #3s : i
        let s_624_0: i128 = 3;
        // D s_624_1: read-var op2:i
        let s_624_1: i128 = fn_state.op2;
        // D s_624_2: cmp-eq s_624_1 s_624_0
        let s_624_2: bool = ((s_624_1) == (s_624_0));
        // D s_624_3: write-var gs#141228 <= s_624_2
        fn_state.gs_141228 = s_624_2;
        // N s_624_4: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_625<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_625_0: const #3s : i
        let s_625_0: i128 = 3;
        // D s_625_1: read-var op1:i
        let s_625_1: i128 = fn_state.op1;
        // D s_625_2: cmp-eq s_625_1 s_625_0
        let s_625_2: bool = ((s_625_1) == (s_625_0));
        // D s_625_3: write-var gs#141226 <= s_625_2
        fn_state.gs_141226 = s_625_2;
        // N s_625_4: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_626<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_626_0: const #9s : i
        let s_626_0: i128 = 9;
        // D s_626_1: read-var crn:i
        let s_626_1: i128 = fn_state.crn;
        // D s_626_2: cmp-eq s_626_1 s_626_0
        let s_626_2: bool = ((s_626_1) == (s_626_0));
        // D s_626_3: write-var gs#141224 <= s_626_2
        fn_state.gs_141224 = s_626_2;
        // N s_626_4: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_627<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_627_0: const #0s : i
        let s_627_0: i128 = 0;
        // D s_627_1: read-var crm:i
        let s_627_1: i128 = fn_state.crm;
        // D s_627_2: call integer_access(s_627_1, s_627_0)
        let s_627_2: bool = integer_access(state, tracer, s_627_1, s_627_0);
        // C s_627_3: const #0s : i
        let s_627_3: i128 = 0;
        // C s_627_4: const #0u : u64
        let s_627_4: u64 = 0;
        // D s_627_5: cast zx s_627_2 -> u64
        let s_627_5: u64 = (s_627_2 as u64);
        // C s_627_6: const #1u : u64
        let s_627_6: u64 = 1;
        // D s_627_7: and s_627_5 s_627_6
        let s_627_7: u64 = ((s_627_5) & (s_627_6));
        // D s_627_8: cmp-eq s_627_7 s_627_6
        let s_627_8: bool = ((s_627_7) == (s_627_6));
        // D s_627_9: lsl s_627_5 s_627_3
        let s_627_9: u64 = s_627_5 << s_627_3;
        // D s_627_10: or s_627_4 s_627_9
        let s_627_10: u64 = ((s_627_4) | (s_627_9));
        // D s_627_11: cmpl s_627_9
        let s_627_11: u64 = !s_627_9;
        // D s_627_12: and s_627_4 s_627_11
        let s_627_12: u64 = ((s_627_4) & (s_627_11));
        // D s_627_13: select s_627_8 s_627_10 s_627_12
        let s_627_13: u64 = if s_627_8 { s_627_10 } else { s_627_12 };
        // D s_627_14: cast trunc s_627_13 -> u8
        let s_627_14: bool = ((s_627_13) != 0);
        // C s_627_15: const #2s : i
        let s_627_15: i128 = 2;
        // C s_627_16: const #0s : i
        let s_627_16: i128 = 0;
        // D s_627_17: read-var op2:i
        let s_627_17: i128 = fn_state.op2;
        // D s_627_18: call integer_subrange(s_627_17, s_627_15, s_627_16)
        let s_627_18: Bits = integer_subrange(
            state,
            tracer,
            s_627_17,
            s_627_15,
            s_627_16,
        );
        // D s_627_19: cast reint s_627_18 -> u8
        let s_627_19: u8 = (s_627_18.value() as u8);
        // D s_627_20: cast zx s_627_14 -> bv
        let s_627_20: Bits = Bits::new(s_627_14 as u128, 1u16);
        // D s_627_21: cast zx s_627_19 -> bv
        let s_627_21: Bits = Bits::new(s_627_19 as u128, 3u16);
        // D s_627_22: cast reint s_627_20 -> u128
        let s_627_22: u128 = (s_627_20.value() as u128);
        // D s_627_23: size-of s_627_20
        let s_627_23: u16 = s_627_20.length();
        // D s_627_24: cast reint s_627_21 -> u128
        let s_627_24: u128 = (s_627_21.value() as u128);
        // D s_627_25: size-of s_627_21
        let s_627_25: u16 = s_627_21.length();
        // D s_627_26: lsl s_627_22 s_627_25
        let s_627_26: u128 = s_627_22 << s_627_25;
        // D s_627_27: or s_627_26 s_627_24
        let s_627_27: u128 = ((s_627_26) | (s_627_24));
        // D s_627_28: add s_627_23 s_627_25
        let s_627_28: u16 = (s_627_23 + s_627_25);
        // D s_627_29: create-bits s_627_27 s_627_28
        let s_627_29: Bits = Bits::new(s_627_27, s_627_28);
        // D s_627_30: cast reint s_627_29 -> u8
        let s_627_30: u8 = (s_627_29.value() as u8);
        // D s_627_31: cast zx s_627_30 -> bv
        let s_627_31: Bits = Bits::new(s_627_30 as u128, 4u16);
        // D s_627_32: cast zx s_627_31 -> i
        let s_627_32: i128 = (s_627_31.value() as i128);
        // D s_627_33: cast reint s_627_32 -> i64
        let s_627_33: i64 = (s_627_32 as i64);
        // D s_627_34: write-var nshadow#1008 <= s_627_33
        fn_state.nshadow_1008 = s_627_33;
        // C s_627_35: const #3s : i
        let s_627_35: i128 = 3;
        // C s_627_36: const #1s : i
        let s_627_36: i128 = 1;
        // D s_627_37: read-var crm:i
        let s_627_37: i128 = fn_state.crm;
        // D s_627_38: call integer_subrange(s_627_37, s_627_35, s_627_36)
        let s_627_38: Bits = integer_subrange(
            state,
            tracer,
            s_627_37,
            s_627_35,
            s_627_36,
        );
        // D s_627_39: cast reint s_627_38 -> u8
        let s_627_39: u8 = (s_627_38.value() as u8);
        // D s_627_40: cast zx s_627_39 -> bv
        let s_627_40: Bits = Bits::new(s_627_39 as u128, 3u16);
        // C s_627_41: const #2u : u8
        let s_627_41: u8 = 2;
        // C s_627_42: cast zx s_627_41 -> bv
        let s_627_42: Bits = Bits::new(s_627_41 as u128, 3u16);
        // D s_627_43: cmp-eq s_627_40 s_627_42
        let s_627_43: bool = ((s_627_40) == (s_627_42));
        // N s_627_44: branch s_627_43 b642 b628
        if s_627_43 {
            return block_642(state, tracer, fn_state);
        } else {
            return block_628(state, tracer, fn_state);
        };
    }
    fn block_628<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_628_0: const #3s : i
        let s_628_0: i128 = 3;
        // C s_628_1: const #1s : i
        let s_628_1: i128 = 1;
        // D s_628_2: read-var crm:i
        let s_628_2: i128 = fn_state.crm;
        // D s_628_3: call integer_subrange(s_628_2, s_628_0, s_628_1)
        let s_628_3: Bits = integer_subrange(state, tracer, s_628_2, s_628_0, s_628_1);
        // D s_628_4: cast reint s_628_3 -> u8
        let s_628_4: u8 = (s_628_3.value() as u8);
        // D s_628_5: cast zx s_628_4 -> bv
        let s_628_5: Bits = Bits::new(s_628_4 as u128, 3u16);
        // C s_628_6: const #3u : u8
        let s_628_6: u8 = 3;
        // C s_628_7: cast zx s_628_6 -> bv
        let s_628_7: Bits = Bits::new(s_628_6 as u128, 3u16);
        // D s_628_8: cmp-eq s_628_5 s_628_7
        let s_628_8: bool = ((s_628_5) == (s_628_7));
        // D s_628_9: write-var gs#141114 <= s_628_8
        fn_state.gs_141114 = s_628_8;
        // N s_628_10: jump b629
        return block_629(state, tracer, fn_state);
    }
    fn block_629<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_629_0: read-var gs#141114:u8
        let s_629_0: bool = fn_state.gs_141114;
        // N s_629_1: branch s_629_0 b639 b630
        if s_629_0 {
            return block_639(state, tracer, fn_state);
        } else {
            return block_630(state, tracer, fn_state);
        };
    }
    fn block_630<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_630_0: const #3s : i
        let s_630_0: i128 = 3;
        // C s_630_1: const #1s : i
        let s_630_1: i128 = 1;
        // D s_630_2: read-var crm:i
        let s_630_2: i128 = fn_state.crm;
        // D s_630_3: call integer_subrange(s_630_2, s_630_0, s_630_1)
        let s_630_3: Bits = integer_subrange(state, tracer, s_630_2, s_630_0, s_630_1);
        // D s_630_4: cast reint s_630_3 -> u8
        let s_630_4: u8 = (s_630_3.value() as u8);
        // D s_630_5: cast zx s_630_4 -> bv
        let s_630_5: Bits = Bits::new(s_630_4 as u128, 3u16);
        // C s_630_6: const #6u : u8
        let s_630_6: u8 = 6;
        // C s_630_7: cast zx s_630_6 -> bv
        let s_630_7: Bits = Bits::new(s_630_6 as u128, 3u16);
        // D s_630_8: cmp-eq s_630_5 s_630_7
        let s_630_8: bool = ((s_630_5) == (s_630_7));
        // N s_630_9: branch s_630_8 b638 b631
        if s_630_8 {
            return block_638(state, tracer, fn_state);
        } else {
            return block_631(state, tracer, fn_state);
        };
    }
    fn block_631<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_631_0: const #3s : i
        let s_631_0: i128 = 3;
        // C s_631_1: const #1s : i
        let s_631_1: i128 = 1;
        // D s_631_2: read-var crm:i
        let s_631_2: i128 = fn_state.crm;
        // D s_631_3: call integer_subrange(s_631_2, s_631_0, s_631_1)
        let s_631_3: Bits = integer_subrange(state, tracer, s_631_2, s_631_0, s_631_1);
        // D s_631_4: cast reint s_631_3 -> u8
        let s_631_4: u8 = (s_631_3.value() as u8);
        // D s_631_5: cast zx s_631_4 -> bv
        let s_631_5: Bits = Bits::new(s_631_4 as u128, 3u16);
        // C s_631_6: const #7u : u8
        let s_631_6: u8 = 7;
        // C s_631_7: cast zx s_631_6 -> bv
        let s_631_7: Bits = Bits::new(s_631_6 as u128, 3u16);
        // D s_631_8: cmp-eq s_631_5 s_631_7
        let s_631_8: bool = ((s_631_5) == (s_631_7));
        // D s_631_9: write-var gs#141120 <= s_631_8
        fn_state.gs_141120 = s_631_8;
        // N s_631_10: jump b632
        return block_632(state, tracer, fn_state);
    }
    fn block_632<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_632_0: read-var gs#141120:u8
        let s_632_0: bool = fn_state.gs_141120;
        // N s_632_1: branch s_632_0 b635 b633
        if s_632_0 {
            return block_635(state, tracer, fn_state);
        } else {
            return block_633(state, tracer, fn_state);
        };
    }
    fn block_633<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_633_0: jump b634
        return block_634(state, tracer, fn_state);
    }
    fn block_634<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_634_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_635<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_635_0: const #14656u : u32
        let s_635_0: u32 = 14656;
        // D s_635_1: read-reg s_635_0:struct
        let s_635_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_635_0 as isize);
            tracer.read_register(s_635_0 as isize, value);
            value
        };
        // D s_635_2: call _get_AMCGCR_EL0_Type_CG1NC(s_635_1)
        let s_635_2: u8 = u_get_AMCGCR_EL0_Type_CG1NC(state, tracer, s_635_1);
        // D s_635_3: cast zx s_635_2 -> bv
        let s_635_3: Bits = Bits::new(s_635_2 as u128, 8u16);
        // D s_635_4: cast zx s_635_3 -> i
        let s_635_4: i128 = (s_635_3.value() as i128);
        // D s_635_5: cast reint s_635_4 -> i64
        let s_635_5: i64 = (s_635_4 as i64);
        // D s_635_6: read-var nshadow#1008:i64
        let s_635_6: i64 = fn_state.nshadow_1008;
        // D s_635_7: cast zx s_635_6 -> i
        let s_635_7: i128 = (i128::try_from(s_635_6).unwrap());
        // D s_635_8: cast zx s_635_5 -> i
        let s_635_8: i128 = (i128::try_from(s_635_5).unwrap());
        // D s_635_9: cmp-ge s_635_7 s_635_8
        let s_635_9: bool = ((s_635_7) >= (s_635_8));
        // N s_635_10: branch s_635_9 b637 b636
        if s_635_9 {
            return block_637(state, tracer, fn_state);
        } else {
            return block_636(state, tracer, fn_state);
        };
    }
    fn block_636<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_636_0: jump b634
        return block_634(state, tracer, fn_state);
    }
    fn block_637<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_637_0: panic
        panic!("{:?}", ());
        // N s_637_1: return
        return;
    }
    fn block_638<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_638_0: const #1u : u8
        let s_638_0: bool = true;
        // D s_638_1: write-var gs#141120 <= s_638_0
        fn_state.gs_141120 = s_638_0;
        // N s_638_2: jump b632
        return block_632(state, tracer, fn_state);
    }
    fn block_639<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_639_0: const #14656u : u32
        let s_639_0: u32 = 14656;
        // D s_639_1: read-reg s_639_0:struct
        let s_639_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_639_0 as isize);
            tracer.read_register(s_639_0 as isize, value);
            value
        };
        // D s_639_2: call _get_AMCGCR_EL0_Type_CG0NC(s_639_1)
        let s_639_2: u8 = u_get_AMCGCR_EL0_Type_CG0NC(state, tracer, s_639_1);
        // D s_639_3: cast zx s_639_2 -> bv
        let s_639_3: Bits = Bits::new(s_639_2 as u128, 8u16);
        // D s_639_4: cast zx s_639_3 -> i
        let s_639_4: i128 = (s_639_3.value() as i128);
        // D s_639_5: cast reint s_639_4 -> i64
        let s_639_5: i64 = (s_639_4 as i64);
        // D s_639_6: read-var nshadow#1008:i64
        let s_639_6: i64 = fn_state.nshadow_1008;
        // D s_639_7: cast zx s_639_6 -> i
        let s_639_7: i128 = (i128::try_from(s_639_6).unwrap());
        // D s_639_8: cast zx s_639_5 -> i
        let s_639_8: i128 = (i128::try_from(s_639_5).unwrap());
        // D s_639_9: cmp-ge s_639_7 s_639_8
        let s_639_9: bool = ((s_639_7) >= (s_639_8));
        // N s_639_10: branch s_639_9 b641 b640
        if s_639_9 {
            return block_641(state, tracer, fn_state);
        } else {
            return block_640(state, tracer, fn_state);
        };
    }
    fn block_640<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_640_0: jump b634
        return block_634(state, tracer, fn_state);
    }
    fn block_641<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_641_0: panic
        panic!("{:?}", ());
        // N s_641_1: return
        return;
    }
    fn block_642<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_642_0: const #1u : u8
        let s_642_0: bool = true;
        // D s_642_1: write-var gs#141114 <= s_642_0
        fn_state.gs_141114 = s_642_0;
        // N s_642_2: jump b629
        return block_629(state, tracer, fn_state);
    }
    fn block_643<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_643_0: const #13s : i
        let s_643_0: i128 = 13;
        // D s_643_1: read-var crn:i
        let s_643_1: i128 = fn_state.crn;
        // D s_643_2: cmp-eq s_643_1 s_643_0
        let s_643_2: bool = ((s_643_1) == (s_643_0));
        // D s_643_3: write-var gs#141103 <= s_643_2
        fn_state.gs_141103 = s_643_2;
        // N s_643_4: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_644<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_644_0: const #3s : i
        let s_644_0: i128 = 3;
        // D s_644_1: read-var op1:i
        let s_644_1: i128 = fn_state.op1;
        // D s_644_2: cmp-eq s_644_1 s_644_0
        let s_644_2: bool = ((s_644_1) == (s_644_0));
        // D s_644_3: write-var gs#141101 <= s_644_2
        fn_state.gs_141101 = s_644_2;
        // N s_644_4: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_645<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_645_0: const #12824u : u32
        let s_645_0: u32 = 12824;
        // D s_645_1: read-reg s_645_0:struct
        let s_645_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_645_0 as isize);
            tracer.read_register(s_645_0 as isize, value);
            value
        };
        // D s_645_2: call _get_ICV_CTLR_EL1_Type_CBPR(s_645_1)
        let s_645_2: bool = u_get_ICV_CTLR_EL1_Type_CBPR(state, tracer, s_645_1);
        // D s_645_3: cast zx s_645_2 -> bv
        let s_645_3: Bits = Bits::new(s_645_2 as u128, 1u16);
        // C s_645_4: const #0u : u8
        let s_645_4: bool = false;
        // C s_645_5: cast zx s_645_4 -> bv
        let s_645_5: Bits = Bits::new(s_645_4 as u128, 1u16);
        // D s_645_6: cmp-eq s_645_3 s_645_5
        let s_645_6: bool = ((s_645_3) == (s_645_5));
        // N s_645_7: branch s_645_6 b647 b646
        if s_645_6 {
            return block_647(state, tracer, fn_state);
        } else {
            return block_646(state, tracer, fn_state);
        };
    }
    fn block_646<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_646_0: const #102456u : u32
        let s_646_0: u32 = 102456;
        // D s_646_1: read-reg s_646_0:struct
        let s_646_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_646_0 as isize);
            tracer.read_register(s_646_0 as isize, value);
            value
        };
        // D s_646_2: call _get_ICV_BPR1_EL1_Type_BinaryPoint(s_646_1)
        let s_646_2: u8 = u_get_ICV_BPR1_EL1_Type_BinaryPoint(state, tracer, s_646_1);
        // C s_646_3: const #0s : i
        let s_646_3: i128 = 0;
        // D s_646_4: read-var tempxt:u64
        let s_646_4: u64 = fn_state.tempxt;
        // D s_646_5: cast zx s_646_4 -> bv
        let s_646_5: Bits = Bits::new(s_646_4 as u128, 64u16);
        // D s_646_6: cast zx s_646_2 -> bv
        let s_646_6: Bits = Bits::new(s_646_2 as u128, 3u16);
        // C s_646_7: const #2s : i
        let s_646_7: i128 = 2;
        // C s_646_8: const #1u : u64
        let s_646_8: u64 = 1;
        // C s_646_9: cast zx s_646_8 -> bv
        let s_646_9: Bits = Bits::new(s_646_8 as u128, 64u16);
        // C s_646_10: lsl s_646_9 s_646_7
        let s_646_10: Bits = s_646_9 << s_646_7;
        // C s_646_11: sub s_646_10 s_646_9
        let s_646_11: Bits = ((s_646_10) - (s_646_9));
        // D s_646_12: and s_646_6 s_646_11
        let s_646_12: Bits = ((s_646_6) & (s_646_11));
        // D s_646_13: lsl s_646_12 s_646_3
        let s_646_13: Bits = s_646_12 << s_646_3;
        // C s_646_14: lsl s_646_11 s_646_3
        let s_646_14: Bits = s_646_11 << s_646_3;
        // C s_646_15: cmpl s_646_14
        let s_646_15: Bits = !s_646_14;
        // D s_646_16: and s_646_5 s_646_15
        let s_646_16: Bits = ((s_646_5) & (s_646_15));
        // D s_646_17: or s_646_16 s_646_13
        let s_646_17: Bits = ((s_646_16) | (s_646_13));
        // D s_646_18: cast reint s_646_17 -> u64
        let s_646_18: u64 = (s_646_17.value() as u64);
        // D s_646_19: write-var tempxt <= s_646_18
        fn_state.tempxt = s_646_18;
        // N s_646_20: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_647<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_647_0: const #() : ()
        let s_647_0: () = ();
        // S s_647_1: call CurrentSecurityState(s_647_0)
        let s_647_1: u32 = CurrentSecurityState(state, tracer, s_647_0);
        // C s_647_2: const #3u : u32
        let s_647_2: u32 = 3;
        // S s_647_3: cmp-eq s_647_1 s_647_2
        let s_647_3: bool = ((s_647_1) == (s_647_2));
        // N s_647_4: branch s_647_3 b652 b648
        if s_647_3 {
            return block_652(state, tracer, fn_state);
        } else {
            return block_648(state, tracer, fn_state);
        };
    }
    fn block_648<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_648_0: const #0s : i
        let s_648_0: i128 = 0;
        // D s_648_1: read-var tempxt:u64
        let s_648_1: u64 = fn_state.tempxt;
        // D s_648_2: cast zx s_648_1 -> bv
        let s_648_2: Bits = Bits::new(s_648_1 as u128, 64u16);
        // C s_648_3: const #1s : i64
        let s_648_3: i64 = 1;
        // C s_648_4: cast zx s_648_3 -> i
        let s_648_4: i128 = (i128::try_from(s_648_3).unwrap());
        // C s_648_5: const #2s : i
        let s_648_5: i128 = 2;
        // C s_648_6: add s_648_5 s_648_4
        let s_648_6: i128 = (s_648_5 + s_648_4);
        // D s_648_7: bit-extract s_648_2 s_648_0 s_648_6
        let s_648_7: Bits = (Bits::new(
            ((s_648_2) >> (s_648_0)).value(),
            u16::try_from(s_648_6).unwrap(),
        ));
        // D s_648_8: cast reint s_648_7 -> u8
        let s_648_8: u8 = (s_648_7.value() as u8);
        // D s_648_9: cast zx s_648_8 -> bv
        let s_648_9: Bits = Bits::new(s_648_8 as u128, 3u16);
        // D s_648_10: cast zx s_648_9 -> i
        let s_648_10: i128 = (s_648_9.value() as i128);
        // D s_648_11: cast reint s_648_10 -> i64
        let s_648_11: i64 = (s_648_10 as i64);
        // C s_648_12: const #17352u : u32
        let s_648_12: u32 = 17352;
        // D s_648_13: read-reg s_648_12:struct
        let s_648_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_648_12 as isize);
            tracer.read_register(s_648_12 as isize, value);
            value
        };
        // D s_648_14: call _get_ICH_VTR_EL2_Type_PREbits(s_648_13)
        let s_648_14: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_648_13);
        // D s_648_15: cast zx s_648_14 -> bv
        let s_648_15: Bits = Bits::new(s_648_14 as u128, 3u16);
        // D s_648_16: cast zx s_648_15 -> i
        let s_648_16: i128 = (s_648_15.value() as i128);
        // D s_648_17: cast reint s_648_16 -> i64
        let s_648_17: i64 = (s_648_16 as i64);
        // C s_648_18: const #7s : i
        let s_648_18: i128 = 7;
        // D s_648_19: cast zx s_648_17 -> i
        let s_648_19: i128 = (i128::try_from(s_648_17).unwrap());
        // D s_648_20: sub s_648_18 s_648_19
        let s_648_20: i128 = ((s_648_18) - (s_648_19));
        // D s_648_21: cast reint s_648_20 -> i64
        let s_648_21: i64 = (s_648_20 as i64);
        // D s_648_22: cast zx s_648_11 -> i
        let s_648_22: i128 = (i128::try_from(s_648_11).unwrap());
        // D s_648_23: cast zx s_648_21 -> i
        let s_648_23: i128 = (i128::try_from(s_648_21).unwrap());
        // D s_648_24: cmp-lt s_648_22 s_648_23
        let s_648_24: bool = ((s_648_22) < (s_648_23));
        // N s_648_25: branch s_648_24 b651 b649
        if s_648_24 {
            return block_651(state, tracer, fn_state);
        } else {
            return block_649(state, tracer, fn_state);
        };
    }
    fn block_649<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_649_0: jump b650
        return block_650(state, tracer, fn_state);
    }
    fn block_650<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_650_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_651<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_651_0: const #17352u : u32
        let s_651_0: u32 = 17352;
        // D s_651_1: read-reg s_651_0:struct
        let s_651_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_651_0 as isize);
            tracer.read_register(s_651_0 as isize, value);
            value
        };
        // D s_651_2: call _get_ICH_VTR_EL2_Type_PREbits(s_651_1)
        let s_651_2: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_651_1);
        // D s_651_3: cast zx s_651_2 -> bv
        let s_651_3: Bits = Bits::new(s_651_2 as u128, 3u16);
        // D s_651_4: cast zx s_651_3 -> i
        let s_651_4: i128 = (s_651_3.value() as i128);
        // D s_651_5: cast reint s_651_4 -> i64
        let s_651_5: i64 = (s_651_4 as i64);
        // C s_651_6: const #7s : i
        let s_651_6: i128 = 7;
        // D s_651_7: cast zx s_651_5 -> i
        let s_651_7: i128 = (i128::try_from(s_651_5).unwrap());
        // D s_651_8: sub s_651_6 s_651_7
        let s_651_8: i128 = ((s_651_6) - (s_651_7));
        // D s_651_9: cast reint s_651_8 -> i64
        let s_651_9: i64 = (s_651_8 as i64);
        // C s_651_10: const #2s : i
        let s_651_10: i128 = 2;
        // C s_651_11: const #0s : i
        let s_651_11: i128 = 0;
        // D s_651_12: cast zx s_651_9 -> i
        let s_651_12: i128 = (i128::try_from(s_651_9).unwrap());
        // D s_651_13: call integer_subrange(s_651_12, s_651_10, s_651_11)
        let s_651_13: Bits = integer_subrange(
            state,
            tracer,
            s_651_12,
            s_651_10,
            s_651_11,
        );
        // D s_651_14: cast reint s_651_13 -> u8
        let s_651_14: u8 = (s_651_13.value() as u8);
        // C s_651_15: const #0s : i
        let s_651_15: i128 = 0;
        // D s_651_16: read-var tempxt:u64
        let s_651_16: u64 = fn_state.tempxt;
        // D s_651_17: cast zx s_651_16 -> bv
        let s_651_17: Bits = Bits::new(s_651_16 as u128, 64u16);
        // D s_651_18: cast zx s_651_14 -> bv
        let s_651_18: Bits = Bits::new(s_651_14 as u128, 3u16);
        // C s_651_19: const #2s : i
        let s_651_19: i128 = 2;
        // C s_651_20: const #1u : u64
        let s_651_20: u64 = 1;
        // C s_651_21: cast zx s_651_20 -> bv
        let s_651_21: Bits = Bits::new(s_651_20 as u128, 64u16);
        // C s_651_22: lsl s_651_21 s_651_19
        let s_651_22: Bits = s_651_21 << s_651_19;
        // C s_651_23: sub s_651_22 s_651_21
        let s_651_23: Bits = ((s_651_22) - (s_651_21));
        // D s_651_24: and s_651_18 s_651_23
        let s_651_24: Bits = ((s_651_18) & (s_651_23));
        // D s_651_25: lsl s_651_24 s_651_15
        let s_651_25: Bits = s_651_24 << s_651_15;
        // C s_651_26: lsl s_651_23 s_651_15
        let s_651_26: Bits = s_651_23 << s_651_15;
        // C s_651_27: cmpl s_651_26
        let s_651_27: Bits = !s_651_26;
        // D s_651_28: and s_651_17 s_651_27
        let s_651_28: Bits = ((s_651_17) & (s_651_27));
        // D s_651_29: or s_651_28 s_651_25
        let s_651_29: Bits = ((s_651_28) | (s_651_25));
        // D s_651_30: cast reint s_651_29 -> u64
        let s_651_30: u64 = (s_651_29.value() as u64);
        // D s_651_31: write-var tempxt <= s_651_30
        fn_state.tempxt = s_651_30;
        // N s_651_32: jump b650
        return block_650(state, tracer, fn_state);
    }
    fn block_652<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_652_0: const #0s : i
        let s_652_0: i128 = 0;
        // D s_652_1: read-var tempxt:u64
        let s_652_1: u64 = fn_state.tempxt;
        // D s_652_2: cast zx s_652_1 -> bv
        let s_652_2: Bits = Bits::new(s_652_1 as u128, 64u16);
        // C s_652_3: const #1s : i64
        let s_652_3: i64 = 1;
        // C s_652_4: cast zx s_652_3 -> i
        let s_652_4: i128 = (i128::try_from(s_652_3).unwrap());
        // C s_652_5: const #2s : i
        let s_652_5: i128 = 2;
        // C s_652_6: add s_652_5 s_652_4
        let s_652_6: i128 = (s_652_5 + s_652_4);
        // D s_652_7: bit-extract s_652_2 s_652_0 s_652_6
        let s_652_7: Bits = (Bits::new(
            ((s_652_2) >> (s_652_0)).value(),
            u16::try_from(s_652_6).unwrap(),
        ));
        // D s_652_8: cast reint s_652_7 -> u8
        let s_652_8: u8 = (s_652_7.value() as u8);
        // D s_652_9: cast zx s_652_8 -> bv
        let s_652_9: Bits = Bits::new(s_652_8 as u128, 3u16);
        // D s_652_10: cast zx s_652_9 -> i
        let s_652_10: i128 = (s_652_9.value() as i128);
        // D s_652_11: cast reint s_652_10 -> i64
        let s_652_11: i64 = (s_652_10 as i64);
        // C s_652_12: const #17352u : u32
        let s_652_12: u32 = 17352;
        // D s_652_13: read-reg s_652_12:struct
        let s_652_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_652_12 as isize);
            tracer.read_register(s_652_12 as isize, value);
            value
        };
        // D s_652_14: call _get_ICH_VTR_EL2_Type_PREbits(s_652_13)
        let s_652_14: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_652_13);
        // D s_652_15: cast zx s_652_14 -> bv
        let s_652_15: Bits = Bits::new(s_652_14 as u128, 3u16);
        // D s_652_16: cast zx s_652_15 -> i
        let s_652_16: i128 = (s_652_15.value() as i128);
        // D s_652_17: cast reint s_652_16 -> i64
        let s_652_17: i64 = (s_652_16 as i64);
        // C s_652_18: const #6s : i
        let s_652_18: i128 = 6;
        // D s_652_19: cast zx s_652_17 -> i
        let s_652_19: i128 = (i128::try_from(s_652_17).unwrap());
        // D s_652_20: sub s_652_18 s_652_19
        let s_652_20: i128 = ((s_652_18) - (s_652_19));
        // D s_652_21: cast reint s_652_20 -> i64
        let s_652_21: i64 = (s_652_20 as i64);
        // D s_652_22: cast zx s_652_11 -> i
        let s_652_22: i128 = (i128::try_from(s_652_11).unwrap());
        // D s_652_23: cast zx s_652_21 -> i
        let s_652_23: i128 = (i128::try_from(s_652_21).unwrap());
        // D s_652_24: cmp-lt s_652_22 s_652_23
        let s_652_24: bool = ((s_652_22) < (s_652_23));
        // N s_652_25: branch s_652_24 b655 b653
        if s_652_24 {
            return block_655(state, tracer, fn_state);
        } else {
            return block_653(state, tracer, fn_state);
        };
    }
    fn block_653<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_653_0: jump b654
        return block_654(state, tracer, fn_state);
    }
    fn block_654<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_654_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_655<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_655_0: const #17352u : u32
        let s_655_0: u32 = 17352;
        // D s_655_1: read-reg s_655_0:struct
        let s_655_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_655_0 as isize);
            tracer.read_register(s_655_0 as isize, value);
            value
        };
        // D s_655_2: call _get_ICH_VTR_EL2_Type_PREbits(s_655_1)
        let s_655_2: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_655_1);
        // D s_655_3: cast zx s_655_2 -> bv
        let s_655_3: Bits = Bits::new(s_655_2 as u128, 3u16);
        // D s_655_4: cast zx s_655_3 -> i
        let s_655_4: i128 = (s_655_3.value() as i128);
        // D s_655_5: cast reint s_655_4 -> i64
        let s_655_5: i64 = (s_655_4 as i64);
        // C s_655_6: const #6s : i
        let s_655_6: i128 = 6;
        // D s_655_7: cast zx s_655_5 -> i
        let s_655_7: i128 = (i128::try_from(s_655_5).unwrap());
        // D s_655_8: sub s_655_6 s_655_7
        let s_655_8: i128 = ((s_655_6) - (s_655_7));
        // D s_655_9: cast reint s_655_8 -> i64
        let s_655_9: i64 = (s_655_8 as i64);
        // C s_655_10: const #2s : i
        let s_655_10: i128 = 2;
        // C s_655_11: const #0s : i
        let s_655_11: i128 = 0;
        // D s_655_12: cast zx s_655_9 -> i
        let s_655_12: i128 = (i128::try_from(s_655_9).unwrap());
        // D s_655_13: call integer_subrange(s_655_12, s_655_10, s_655_11)
        let s_655_13: Bits = integer_subrange(
            state,
            tracer,
            s_655_12,
            s_655_10,
            s_655_11,
        );
        // D s_655_14: cast reint s_655_13 -> u8
        let s_655_14: u8 = (s_655_13.value() as u8);
        // C s_655_15: const #0s : i
        let s_655_15: i128 = 0;
        // D s_655_16: read-var tempxt:u64
        let s_655_16: u64 = fn_state.tempxt;
        // D s_655_17: cast zx s_655_16 -> bv
        let s_655_17: Bits = Bits::new(s_655_16 as u128, 64u16);
        // D s_655_18: cast zx s_655_14 -> bv
        let s_655_18: Bits = Bits::new(s_655_14 as u128, 3u16);
        // C s_655_19: const #2s : i
        let s_655_19: i128 = 2;
        // C s_655_20: const #1u : u64
        let s_655_20: u64 = 1;
        // C s_655_21: cast zx s_655_20 -> bv
        let s_655_21: Bits = Bits::new(s_655_20 as u128, 64u16);
        // C s_655_22: lsl s_655_21 s_655_19
        let s_655_22: Bits = s_655_21 << s_655_19;
        // C s_655_23: sub s_655_22 s_655_21
        let s_655_23: Bits = ((s_655_22) - (s_655_21));
        // D s_655_24: and s_655_18 s_655_23
        let s_655_24: Bits = ((s_655_18) & (s_655_23));
        // D s_655_25: lsl s_655_24 s_655_15
        let s_655_25: Bits = s_655_24 << s_655_15;
        // C s_655_26: lsl s_655_23 s_655_15
        let s_655_26: Bits = s_655_23 << s_655_15;
        // C s_655_27: cmpl s_655_26
        let s_655_27: Bits = !s_655_26;
        // D s_655_28: and s_655_17 s_655_27
        let s_655_28: Bits = ((s_655_17) & (s_655_27));
        // D s_655_29: or s_655_28 s_655_25
        let s_655_29: Bits = ((s_655_28) | (s_655_25));
        // D s_655_30: cast reint s_655_29 -> u64
        let s_655_30: u64 = (s_655_29.value() as u64);
        // D s_655_31: write-var tempxt <= s_655_30
        fn_state.tempxt = s_655_30;
        // N s_655_32: jump b654
        return block_654(state, tracer, fn_state);
    }
    fn block_656<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_656_0: const #12s : i
        let s_656_0: i128 = 12;
        // D s_656_1: read-var crm:i
        let s_656_1: i128 = fn_state.crm;
        // D s_656_2: cmp-eq s_656_1 s_656_0
        let s_656_2: bool = ((s_656_1) == (s_656_0));
        // D s_656_3: write-var gs#141098 <= s_656_2
        fn_state.gs_141098 = s_656_2;
        // N s_656_4: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_657<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_657_0: const #3s : i
        let s_657_0: i128 = 3;
        // D s_657_1: read-var op2:i
        let s_657_1: i128 = fn_state.op2;
        // D s_657_2: cmp-eq s_657_1 s_657_0
        let s_657_2: bool = ((s_657_1) == (s_657_0));
        // D s_657_3: write-var gs#141096 <= s_657_2
        fn_state.gs_141096 = s_657_2;
        // N s_657_4: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_658<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_658_0: const #0s : i
        let s_658_0: i128 = 0;
        // D s_658_1: read-var op1:i
        let s_658_1: i128 = fn_state.op1;
        // D s_658_2: cmp-eq s_658_1 s_658_0
        let s_658_2: bool = ((s_658_1) == (s_658_0));
        // D s_658_3: write-var gs#141094 <= s_658_2
        fn_state.gs_141094 = s_658_2;
        // N s_658_4: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_659<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_659_0: const #12s : i
        let s_659_0: i128 = 12;
        // D s_659_1: read-var crn:i
        let s_659_1: i128 = fn_state.crn;
        // D s_659_2: cmp-eq s_659_1 s_659_0
        let s_659_2: bool = ((s_659_1) == (s_659_0));
        // D s_659_3: write-var gs#141092 <= s_659_2
        fn_state.gs_141092 = s_659_2;
        // N s_659_4: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_660<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_660_0: const #0s : i
        let s_660_0: i128 = 0;
        // D s_660_1: read-var tempxt:u64
        let s_660_1: u64 = fn_state.tempxt;
        // D s_660_2: cast zx s_660_1 -> bv
        let s_660_2: Bits = Bits::new(s_660_1 as u128, 64u16);
        // C s_660_3: const #1s : i64
        let s_660_3: i64 = 1;
        // C s_660_4: cast zx s_660_3 -> i
        let s_660_4: i128 = (i128::try_from(s_660_3).unwrap());
        // C s_660_5: const #2s : i
        let s_660_5: i128 = 2;
        // C s_660_6: add s_660_5 s_660_4
        let s_660_6: i128 = (s_660_5 + s_660_4);
        // D s_660_7: bit-extract s_660_2 s_660_0 s_660_6
        let s_660_7: Bits = (Bits::new(
            ((s_660_2) >> (s_660_0)).value(),
            u16::try_from(s_660_6).unwrap(),
        ));
        // D s_660_8: cast reint s_660_7 -> u8
        let s_660_8: u8 = (s_660_7.value() as u8);
        // D s_660_9: cast zx s_660_8 -> bv
        let s_660_9: Bits = Bits::new(s_660_8 as u128, 3u16);
        // D s_660_10: cast zx s_660_9 -> i
        let s_660_10: i128 = (s_660_9.value() as i128);
        // D s_660_11: cast reint s_660_10 -> i64
        let s_660_11: i64 = (s_660_10 as i64);
        // C s_660_12: const #17352u : u32
        let s_660_12: u32 = 17352;
        // D s_660_13: read-reg s_660_12:struct
        let s_660_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_660_12 as isize);
            tracer.read_register(s_660_12 as isize, value);
            value
        };
        // D s_660_14: call _get_ICH_VTR_EL2_Type_PREbits(s_660_13)
        let s_660_14: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_660_13);
        // D s_660_15: cast zx s_660_14 -> bv
        let s_660_15: Bits = Bits::new(s_660_14 as u128, 3u16);
        // D s_660_16: cast zx s_660_15 -> i
        let s_660_16: i128 = (s_660_15.value() as i128);
        // D s_660_17: cast reint s_660_16 -> i64
        let s_660_17: i64 = (s_660_16 as i64);
        // C s_660_18: const #6s : i
        let s_660_18: i128 = 6;
        // D s_660_19: cast zx s_660_17 -> i
        let s_660_19: i128 = (i128::try_from(s_660_17).unwrap());
        // D s_660_20: sub s_660_18 s_660_19
        let s_660_20: i128 = ((s_660_18) - (s_660_19));
        // D s_660_21: cast reint s_660_20 -> i64
        let s_660_21: i64 = (s_660_20 as i64);
        // D s_660_22: cast zx s_660_11 -> i
        let s_660_22: i128 = (i128::try_from(s_660_11).unwrap());
        // D s_660_23: cast zx s_660_21 -> i
        let s_660_23: i128 = (i128::try_from(s_660_21).unwrap());
        // D s_660_24: cmp-lt s_660_22 s_660_23
        let s_660_24: bool = ((s_660_22) < (s_660_23));
        // N s_660_25: branch s_660_24 b663 b661
        if s_660_24 {
            return block_663(state, tracer, fn_state);
        } else {
            return block_661(state, tracer, fn_state);
        };
    }
    fn block_661<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_661_0: jump b662
        return block_662(state, tracer, fn_state);
    }
    fn block_662<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_662_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_663<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_663_0: const #17352u : u32
        let s_663_0: u32 = 17352;
        // D s_663_1: read-reg s_663_0:struct
        let s_663_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_663_0 as isize);
            tracer.read_register(s_663_0 as isize, value);
            value
        };
        // D s_663_2: call _get_ICH_VTR_EL2_Type_PREbits(s_663_1)
        let s_663_2: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_663_1);
        // D s_663_3: cast zx s_663_2 -> bv
        let s_663_3: Bits = Bits::new(s_663_2 as u128, 3u16);
        // D s_663_4: cast zx s_663_3 -> i
        let s_663_4: i128 = (s_663_3.value() as i128);
        // D s_663_5: cast reint s_663_4 -> i64
        let s_663_5: i64 = (s_663_4 as i64);
        // C s_663_6: const #6s : i
        let s_663_6: i128 = 6;
        // D s_663_7: cast zx s_663_5 -> i
        let s_663_7: i128 = (i128::try_from(s_663_5).unwrap());
        // D s_663_8: sub s_663_6 s_663_7
        let s_663_8: i128 = ((s_663_6) - (s_663_7));
        // D s_663_9: cast reint s_663_8 -> i64
        let s_663_9: i64 = (s_663_8 as i64);
        // C s_663_10: const #2s : i
        let s_663_10: i128 = 2;
        // C s_663_11: const #0s : i
        let s_663_11: i128 = 0;
        // D s_663_12: cast zx s_663_9 -> i
        let s_663_12: i128 = (i128::try_from(s_663_9).unwrap());
        // D s_663_13: call integer_subrange(s_663_12, s_663_10, s_663_11)
        let s_663_13: Bits = integer_subrange(
            state,
            tracer,
            s_663_12,
            s_663_10,
            s_663_11,
        );
        // D s_663_14: cast reint s_663_13 -> u8
        let s_663_14: u8 = (s_663_13.value() as u8);
        // C s_663_15: const #0s : i
        let s_663_15: i128 = 0;
        // D s_663_16: read-var tempxt:u64
        let s_663_16: u64 = fn_state.tempxt;
        // D s_663_17: cast zx s_663_16 -> bv
        let s_663_17: Bits = Bits::new(s_663_16 as u128, 64u16);
        // D s_663_18: cast zx s_663_14 -> bv
        let s_663_18: Bits = Bits::new(s_663_14 as u128, 3u16);
        // C s_663_19: const #2s : i
        let s_663_19: i128 = 2;
        // C s_663_20: const #1u : u64
        let s_663_20: u64 = 1;
        // C s_663_21: cast zx s_663_20 -> bv
        let s_663_21: Bits = Bits::new(s_663_20 as u128, 64u16);
        // C s_663_22: lsl s_663_21 s_663_19
        let s_663_22: Bits = s_663_21 << s_663_19;
        // C s_663_23: sub s_663_22 s_663_21
        let s_663_23: Bits = ((s_663_22) - (s_663_21));
        // D s_663_24: and s_663_18 s_663_23
        let s_663_24: Bits = ((s_663_18) & (s_663_23));
        // D s_663_25: lsl s_663_24 s_663_15
        let s_663_25: Bits = s_663_24 << s_663_15;
        // C s_663_26: lsl s_663_23 s_663_15
        let s_663_26: Bits = s_663_23 << s_663_15;
        // C s_663_27: cmpl s_663_26
        let s_663_27: Bits = !s_663_26;
        // D s_663_28: and s_663_17 s_663_27
        let s_663_28: Bits = ((s_663_17) & (s_663_27));
        // D s_663_29: or s_663_28 s_663_25
        let s_663_29: Bits = ((s_663_28) | (s_663_25));
        // D s_663_30: cast reint s_663_29 -> u64
        let s_663_30: u64 = (s_663_29.value() as u64);
        // D s_663_31: write-var tempxt <= s_663_30
        fn_state.tempxt = s_663_30;
        // N s_663_32: jump b662
        return block_662(state, tracer, fn_state);
    }
    fn block_664<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_664_0: const #8s : i
        let s_664_0: i128 = 8;
        // D s_664_1: read-var crm:i
        let s_664_1: i128 = fn_state.crm;
        // D s_664_2: cmp-eq s_664_1 s_664_0
        let s_664_2: bool = ((s_664_1) == (s_664_0));
        // D s_664_3: write-var gs#141089 <= s_664_2
        fn_state.gs_141089 = s_664_2;
        // N s_664_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_665<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_665_0: const #3s : i
        let s_665_0: i128 = 3;
        // D s_665_1: read-var op2:i
        let s_665_1: i128 = fn_state.op2;
        // D s_665_2: cmp-eq s_665_1 s_665_0
        let s_665_2: bool = ((s_665_1) == (s_665_0));
        // D s_665_3: write-var gs#141087 <= s_665_2
        fn_state.gs_141087 = s_665_2;
        // N s_665_4: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_666<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_666_0: const #0s : i
        let s_666_0: i128 = 0;
        // D s_666_1: read-var op1:i
        let s_666_1: i128 = fn_state.op1;
        // D s_666_2: cmp-eq s_666_1 s_666_0
        let s_666_2: bool = ((s_666_1) == (s_666_0));
        // D s_666_3: write-var gs#141085 <= s_666_2
        fn_state.gs_141085 = s_666_2;
        // N s_666_4: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_667<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_667_0: const #12s : i
        let s_667_0: i128 = 12;
        // D s_667_1: read-var crn:i
        let s_667_1: i128 = fn_state.crn;
        // D s_667_2: cmp-eq s_667_1 s_667_0
        let s_667_2: bool = ((s_667_1) == (s_667_0));
        // D s_667_3: write-var gs#141083 <= s_667_2
        fn_state.gs_141083 = s_667_2;
        // N s_667_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_668<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_668_0: const #16975u : u32
        let s_668_0: u32 = 16975;
        // D s_668_1: read-reg s_668_0:u8
        let s_668_1: u8 = {
            let value = state.read_register::<u8>(s_668_0 as isize);
            tracer.read_register(s_668_0 as isize, value);
            value
        };
        // D s_668_2: cast zx s_668_1 -> bv
        let s_668_2: Bits = Bits::new(s_668_1 as u128, 2u16);
        // C s_668_3: const #432u : u32
        let s_668_3: u32 = 432;
        // D s_668_4: read-reg s_668_3:u8
        let s_668_4: u8 = {
            let value = state.read_register::<u8>(s_668_3 as isize);
            tracer.read_register(s_668_3 as isize, value);
            value
        };
        // D s_668_5: cast zx s_668_4 -> bv
        let s_668_5: Bits = Bits::new(s_668_4 as u128, 2u16);
        // D s_668_6: cmp-eq s_668_2 s_668_5
        let s_668_6: bool = ((s_668_2) == (s_668_5));
        // N s_668_7: branch s_668_6 b689 b669
        if s_668_6 {
            return block_689(state, tracer, fn_state);
        } else {
            return block_669(state, tracer, fn_state);
        };
    }
    fn block_669<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_669_0: const #16975u : u32
        let s_669_0: u32 = 16975;
        // D s_669_1: read-reg s_669_0:u8
        let s_669_1: u8 = {
            let value = state.read_register::<u8>(s_669_0 as isize);
            tracer.read_register(s_669_0 as isize, value);
            value
        };
        // D s_669_2: cast zx s_669_1 -> bv
        let s_669_2: Bits = Bits::new(s_669_1 as u128, 2u16);
        // C s_669_3: const #424u : u32
        let s_669_3: u32 = 424;
        // D s_669_4: read-reg s_669_3:u8
        let s_669_4: u8 = {
            let value = state.read_register::<u8>(s_669_3 as isize);
            tracer.read_register(s_669_3 as isize, value);
            value
        };
        // D s_669_5: cast zx s_669_4 -> bv
        let s_669_5: Bits = Bits::new(s_669_4 as u128, 2u16);
        // D s_669_6: cmp-eq s_669_2 s_669_5
        let s_669_6: bool = ((s_669_2) == (s_669_5));
        // D s_669_7: write-var gs#141155 <= s_669_6
        fn_state.gs_141155 = s_669_6;
        // N s_669_8: jump b670
        return block_670(state, tracer, fn_state);
    }
    fn block_670<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_670_0: read-var gs#141155:u8
        let s_670_0: bool = fn_state.gs_141155;
        // N s_670_1: branch s_670_0 b673 b671
        if s_670_0 {
            return block_673(state, tracer, fn_state);
        } else {
            return block_671(state, tracer, fn_state);
        };
    }
    fn block_671<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_671_0: jump b672
        return block_672(state, tracer, fn_state);
    }
    fn block_672<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_672_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_673<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_673_0: const #21s : i
        let s_673_0: i128 = 21;
        // D s_673_1: read-var tempxt:u64
        let s_673_1: u64 = fn_state.tempxt;
        // D s_673_2: cast zx s_673_1 -> bv
        let s_673_2: Bits = Bits::new(s_673_1 as u128, 64u16);
        // C s_673_3: const #1s : i64
        let s_673_3: i64 = 1;
        // C s_673_4: cast zx s_673_3 -> i
        let s_673_4: i128 = (i128::try_from(s_673_3).unwrap());
        // C s_673_5: const #2s : i
        let s_673_5: i128 = 2;
        // C s_673_6: add s_673_5 s_673_4
        let s_673_6: i128 = (s_673_5 + s_673_4);
        // D s_673_7: bit-extract s_673_2 s_673_0 s_673_6
        let s_673_7: Bits = (Bits::new(
            ((s_673_2) >> (s_673_0)).value(),
            u16::try_from(s_673_6).unwrap(),
        ));
        // D s_673_8: cast reint s_673_7 -> u8
        let s_673_8: u8 = (s_673_7.value() as u8);
        // D s_673_9: cast zx s_673_8 -> bv
        let s_673_9: Bits = Bits::new(s_673_8 as u128, 3u16);
        // D s_673_10: cast zx s_673_9 -> i
        let s_673_10: i128 = (s_673_9.value() as i128);
        // D s_673_11: cast reint s_673_10 -> i64
        let s_673_11: i64 = (s_673_10 as i64);
        // C s_673_12: const #17352u : u32
        let s_673_12: u32 = 17352;
        // D s_673_13: read-reg s_673_12:struct
        let s_673_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_673_12 as isize);
            tracer.read_register(s_673_12 as isize, value);
            value
        };
        // D s_673_14: call _get_ICH_VTR_EL2_Type_PREbits(s_673_13)
        let s_673_14: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_673_13);
        // D s_673_15: cast zx s_673_14 -> bv
        let s_673_15: Bits = Bits::new(s_673_14 as u128, 3u16);
        // D s_673_16: cast zx s_673_15 -> i
        let s_673_16: i128 = (s_673_15.value() as i128);
        // D s_673_17: cast reint s_673_16 -> i64
        let s_673_17: i64 = (s_673_16 as i64);
        // C s_673_18: const #6s : i
        let s_673_18: i128 = 6;
        // D s_673_19: cast zx s_673_17 -> i
        let s_673_19: i128 = (i128::try_from(s_673_17).unwrap());
        // D s_673_20: sub s_673_18 s_673_19
        let s_673_20: i128 = ((s_673_18) - (s_673_19));
        // D s_673_21: cast reint s_673_20 -> i64
        let s_673_21: i64 = (s_673_20 as i64);
        // D s_673_22: cast zx s_673_11 -> i
        let s_673_22: i128 = (i128::try_from(s_673_11).unwrap());
        // D s_673_23: cast zx s_673_21 -> i
        let s_673_23: i128 = (i128::try_from(s_673_21).unwrap());
        // D s_673_24: cmp-lt s_673_22 s_673_23
        let s_673_24: bool = ((s_673_22) < (s_673_23));
        // N s_673_25: branch s_673_24 b688 b674
        if s_673_24 {
            return block_688(state, tracer, fn_state);
        } else {
            return block_674(state, tracer, fn_state);
        };
    }
    fn block_674<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_674_0: jump b675
        return block_675(state, tracer, fn_state);
    }
    fn block_675<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_675_0: const #() : ()
        let s_675_0: () = ();
        // S s_675_1: call CurrentSecurityState(s_675_0)
        let s_675_1: u32 = CurrentSecurityState(state, tracer, s_675_0);
        // C s_675_2: const #3u : u32
        let s_675_2: u32 = 3;
        // S s_675_3: cmp-eq s_675_1 s_675_2
        let s_675_3: bool = ((s_675_1) == (s_675_2));
        // N s_675_4: branch s_675_3 b684 b676
        if s_675_3 {
            return block_684(state, tracer, fn_state);
        } else {
            return block_676(state, tracer, fn_state);
        };
    }
    fn block_676<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_676_0: const #18s : i
        let s_676_0: i128 = 18;
        // D s_676_1: read-var tempxt:u64
        let s_676_1: u64 = fn_state.tempxt;
        // D s_676_2: cast zx s_676_1 -> bv
        let s_676_2: Bits = Bits::new(s_676_1 as u128, 64u16);
        // C s_676_3: const #1s : i64
        let s_676_3: i64 = 1;
        // C s_676_4: cast zx s_676_3 -> i
        let s_676_4: i128 = (i128::try_from(s_676_3).unwrap());
        // C s_676_5: const #2s : i
        let s_676_5: i128 = 2;
        // C s_676_6: add s_676_5 s_676_4
        let s_676_6: i128 = (s_676_5 + s_676_4);
        // D s_676_7: bit-extract s_676_2 s_676_0 s_676_6
        let s_676_7: Bits = (Bits::new(
            ((s_676_2) >> (s_676_0)).value(),
            u16::try_from(s_676_6).unwrap(),
        ));
        // D s_676_8: cast reint s_676_7 -> u8
        let s_676_8: u8 = (s_676_7.value() as u8);
        // D s_676_9: cast zx s_676_8 -> bv
        let s_676_9: Bits = Bits::new(s_676_8 as u128, 3u16);
        // D s_676_10: cast zx s_676_9 -> i
        let s_676_10: i128 = (s_676_9.value() as i128);
        // D s_676_11: cast reint s_676_10 -> i64
        let s_676_11: i64 = (s_676_10 as i64);
        // C s_676_12: const #17352u : u32
        let s_676_12: u32 = 17352;
        // D s_676_13: read-reg s_676_12:struct
        let s_676_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_676_12 as isize);
            tracer.read_register(s_676_12 as isize, value);
            value
        };
        // D s_676_14: call _get_ICH_VTR_EL2_Type_PREbits(s_676_13)
        let s_676_14: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_676_13);
        // D s_676_15: cast zx s_676_14 -> bv
        let s_676_15: Bits = Bits::new(s_676_14 as u128, 3u16);
        // D s_676_16: cast zx s_676_15 -> i
        let s_676_16: i128 = (s_676_15.value() as i128);
        // D s_676_17: cast reint s_676_16 -> i64
        let s_676_17: i64 = (s_676_16 as i64);
        // C s_676_18: const #7s : i
        let s_676_18: i128 = 7;
        // D s_676_19: cast zx s_676_17 -> i
        let s_676_19: i128 = (i128::try_from(s_676_17).unwrap());
        // D s_676_20: sub s_676_18 s_676_19
        let s_676_20: i128 = ((s_676_18) - (s_676_19));
        // D s_676_21: cast reint s_676_20 -> i64
        let s_676_21: i64 = (s_676_20 as i64);
        // D s_676_22: cast zx s_676_11 -> i
        let s_676_22: i128 = (i128::try_from(s_676_11).unwrap());
        // D s_676_23: cast zx s_676_21 -> i
        let s_676_23: i128 = (i128::try_from(s_676_21).unwrap());
        // D s_676_24: cmp-lt s_676_22 s_676_23
        let s_676_24: bool = ((s_676_22) < (s_676_23));
        // N s_676_25: branch s_676_24 b683 b677
        if s_676_24 {
            return block_683(state, tracer, fn_state);
        } else {
            return block_677(state, tracer, fn_state);
        };
    }
    fn block_677<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_677_0: jump b678
        return block_678(state, tracer, fn_state);
    }
    fn block_678<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_678_0: jump b679
        return block_679(state, tracer, fn_state);
    }
    fn block_679<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_679_0: const #22960u : u32
        let s_679_0: u32 = 22960;
        // D s_679_1: read-reg s_679_0:struct
        let s_679_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_679_0 as isize);
            tracer.read_register(s_679_0 as isize, value);
            value
        };
        // D s_679_2: call _get_ICC_SRE_EL1_Type_SRE(s_679_1)
        let s_679_2: bool = u_get_ICC_SRE_EL1_Type_SRE(state, tracer, s_679_1);
        // D s_679_3: cast zx s_679_2 -> bv
        let s_679_3: Bits = Bits::new(s_679_2 as u128, 1u16);
        // C s_679_4: const #1u : u8
        let s_679_4: bool = true;
        // C s_679_5: cast zx s_679_4 -> bv
        let s_679_5: Bits = Bits::new(s_679_4 as u128, 1u16);
        // D s_679_6: cmp-eq s_679_3 s_679_5
        let s_679_6: bool = ((s_679_3) == (s_679_5));
        // N s_679_7: branch s_679_6 b682 b680
        if s_679_6 {
            return block_682(state, tracer, fn_state);
        } else {
            return block_680(state, tracer, fn_state);
        };
    }
    fn block_680<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_680_0: jump b681
        return block_681(state, tracer, fn_state);
    }
    fn block_681<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_681_0: jump b672
        return block_672(state, tracer, fn_state);
    }
    fn block_682<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_682_0: const #2s : i
        let s_682_0: i128 = 2;
        // D s_682_1: read-var tempxt:u64
        let s_682_1: u64 = fn_state.tempxt;
        // D s_682_2: cast zx s_682_1 -> bv
        let s_682_2: Bits = Bits::new(s_682_1 as u128, 64u16);
        // C s_682_3: const #2u : u8
        let s_682_3: u8 = 2;
        // C s_682_4: cast zx s_682_3 -> bv
        let s_682_4: Bits = Bits::new(s_682_3 as u128, 2u16);
        // C s_682_5: const #1s : i
        let s_682_5: i128 = 1;
        // C s_682_6: const #1u : u64
        let s_682_6: u64 = 1;
        // C s_682_7: cast zx s_682_6 -> bv
        let s_682_7: Bits = Bits::new(s_682_6 as u128, 64u16);
        // C s_682_8: lsl s_682_7 s_682_5
        let s_682_8: Bits = s_682_7 << s_682_5;
        // C s_682_9: sub s_682_8 s_682_7
        let s_682_9: Bits = ((s_682_8) - (s_682_7));
        // C s_682_10: and s_682_4 s_682_9
        let s_682_10: Bits = ((s_682_4) & (s_682_9));
        // C s_682_11: lsl s_682_10 s_682_0
        let s_682_11: Bits = s_682_10 << s_682_0;
        // C s_682_12: lsl s_682_9 s_682_0
        let s_682_12: Bits = s_682_9 << s_682_0;
        // C s_682_13: cmpl s_682_12
        let s_682_13: Bits = !s_682_12;
        // D s_682_14: and s_682_2 s_682_13
        let s_682_14: Bits = ((s_682_2) & (s_682_13));
        // D s_682_15: or s_682_14 s_682_11
        let s_682_15: Bits = ((s_682_14) | (s_682_11));
        // D s_682_16: cast reint s_682_15 -> u64
        let s_682_16: u64 = (s_682_15.value() as u64);
        // D s_682_17: write-var tempxt <= s_682_16
        fn_state.tempxt = s_682_16;
        // N s_682_18: jump b681
        return block_681(state, tracer, fn_state);
    }
    fn block_683<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_683_0: const #17352u : u32
        let s_683_0: u32 = 17352;
        // D s_683_1: read-reg s_683_0:struct
        let s_683_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_683_0 as isize);
            tracer.read_register(s_683_0 as isize, value);
            value
        };
        // D s_683_2: call _get_ICH_VTR_EL2_Type_PREbits(s_683_1)
        let s_683_2: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_683_1);
        // D s_683_3: cast zx s_683_2 -> bv
        let s_683_3: Bits = Bits::new(s_683_2 as u128, 3u16);
        // D s_683_4: cast zx s_683_3 -> i
        let s_683_4: i128 = (s_683_3.value() as i128);
        // D s_683_5: cast reint s_683_4 -> i64
        let s_683_5: i64 = (s_683_4 as i64);
        // C s_683_6: const #7s : i
        let s_683_6: i128 = 7;
        // D s_683_7: cast zx s_683_5 -> i
        let s_683_7: i128 = (i128::try_from(s_683_5).unwrap());
        // D s_683_8: sub s_683_6 s_683_7
        let s_683_8: i128 = ((s_683_6) - (s_683_7));
        // D s_683_9: cast reint s_683_8 -> i64
        let s_683_9: i64 = (s_683_8 as i64);
        // C s_683_10: const #2s : i
        let s_683_10: i128 = 2;
        // C s_683_11: const #0s : i
        let s_683_11: i128 = 0;
        // D s_683_12: cast zx s_683_9 -> i
        let s_683_12: i128 = (i128::try_from(s_683_9).unwrap());
        // D s_683_13: call integer_subrange(s_683_12, s_683_10, s_683_11)
        let s_683_13: Bits = integer_subrange(
            state,
            tracer,
            s_683_12,
            s_683_10,
            s_683_11,
        );
        // D s_683_14: cast reint s_683_13 -> u8
        let s_683_14: u8 = (s_683_13.value() as u8);
        // C s_683_15: const #18s : i
        let s_683_15: i128 = 18;
        // D s_683_16: read-var tempxt:u64
        let s_683_16: u64 = fn_state.tempxt;
        // D s_683_17: cast zx s_683_16 -> bv
        let s_683_17: Bits = Bits::new(s_683_16 as u128, 64u16);
        // D s_683_18: cast zx s_683_14 -> bv
        let s_683_18: Bits = Bits::new(s_683_14 as u128, 3u16);
        // C s_683_19: const #2s : i
        let s_683_19: i128 = 2;
        // C s_683_20: const #1u : u64
        let s_683_20: u64 = 1;
        // C s_683_21: cast zx s_683_20 -> bv
        let s_683_21: Bits = Bits::new(s_683_20 as u128, 64u16);
        // C s_683_22: lsl s_683_21 s_683_19
        let s_683_22: Bits = s_683_21 << s_683_19;
        // C s_683_23: sub s_683_22 s_683_21
        let s_683_23: Bits = ((s_683_22) - (s_683_21));
        // D s_683_24: and s_683_18 s_683_23
        let s_683_24: Bits = ((s_683_18) & (s_683_23));
        // D s_683_25: lsl s_683_24 s_683_15
        let s_683_25: Bits = s_683_24 << s_683_15;
        // C s_683_26: lsl s_683_23 s_683_15
        let s_683_26: Bits = s_683_23 << s_683_15;
        // C s_683_27: cmpl s_683_26
        let s_683_27: Bits = !s_683_26;
        // D s_683_28: and s_683_17 s_683_27
        let s_683_28: Bits = ((s_683_17) & (s_683_27));
        // D s_683_29: or s_683_28 s_683_25
        let s_683_29: Bits = ((s_683_28) | (s_683_25));
        // D s_683_30: cast reint s_683_29 -> u64
        let s_683_30: u64 = (s_683_29.value() as u64);
        // D s_683_31: write-var tempxt <= s_683_30
        fn_state.tempxt = s_683_30;
        // N s_683_32: jump b678
        return block_678(state, tracer, fn_state);
    }
    fn block_684<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_684_0: const #18s : i
        let s_684_0: i128 = 18;
        // D s_684_1: read-var tempxt:u64
        let s_684_1: u64 = fn_state.tempxt;
        // D s_684_2: cast zx s_684_1 -> bv
        let s_684_2: Bits = Bits::new(s_684_1 as u128, 64u16);
        // C s_684_3: const #1s : i64
        let s_684_3: i64 = 1;
        // C s_684_4: cast zx s_684_3 -> i
        let s_684_4: i128 = (i128::try_from(s_684_3).unwrap());
        // C s_684_5: const #2s : i
        let s_684_5: i128 = 2;
        // C s_684_6: add s_684_5 s_684_4
        let s_684_6: i128 = (s_684_5 + s_684_4);
        // D s_684_7: bit-extract s_684_2 s_684_0 s_684_6
        let s_684_7: Bits = (Bits::new(
            ((s_684_2) >> (s_684_0)).value(),
            u16::try_from(s_684_6).unwrap(),
        ));
        // D s_684_8: cast reint s_684_7 -> u8
        let s_684_8: u8 = (s_684_7.value() as u8);
        // D s_684_9: cast zx s_684_8 -> bv
        let s_684_9: Bits = Bits::new(s_684_8 as u128, 3u16);
        // D s_684_10: cast zx s_684_9 -> i
        let s_684_10: i128 = (s_684_9.value() as i128);
        // D s_684_11: cast reint s_684_10 -> i64
        let s_684_11: i64 = (s_684_10 as i64);
        // C s_684_12: const #17352u : u32
        let s_684_12: u32 = 17352;
        // D s_684_13: read-reg s_684_12:struct
        let s_684_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_684_12 as isize);
            tracer.read_register(s_684_12 as isize, value);
            value
        };
        // D s_684_14: call _get_ICH_VTR_EL2_Type_PREbits(s_684_13)
        let s_684_14: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_684_13);
        // D s_684_15: cast zx s_684_14 -> bv
        let s_684_15: Bits = Bits::new(s_684_14 as u128, 3u16);
        // D s_684_16: cast zx s_684_15 -> i
        let s_684_16: i128 = (s_684_15.value() as i128);
        // D s_684_17: cast reint s_684_16 -> i64
        let s_684_17: i64 = (s_684_16 as i64);
        // C s_684_18: const #6s : i
        let s_684_18: i128 = 6;
        // D s_684_19: cast zx s_684_17 -> i
        let s_684_19: i128 = (i128::try_from(s_684_17).unwrap());
        // D s_684_20: sub s_684_18 s_684_19
        let s_684_20: i128 = ((s_684_18) - (s_684_19));
        // D s_684_21: cast reint s_684_20 -> i64
        let s_684_21: i64 = (s_684_20 as i64);
        // D s_684_22: cast zx s_684_11 -> i
        let s_684_22: i128 = (i128::try_from(s_684_11).unwrap());
        // D s_684_23: cast zx s_684_21 -> i
        let s_684_23: i128 = (i128::try_from(s_684_21).unwrap());
        // D s_684_24: cmp-lt s_684_22 s_684_23
        let s_684_24: bool = ((s_684_22) < (s_684_23));
        // N s_684_25: branch s_684_24 b687 b685
        if s_684_24 {
            return block_687(state, tracer, fn_state);
        } else {
            return block_685(state, tracer, fn_state);
        };
    }
    fn block_685<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_685_0: jump b686
        return block_686(state, tracer, fn_state);
    }
    fn block_686<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_686_0: jump b679
        return block_679(state, tracer, fn_state);
    }
    fn block_687<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_687_0: const #17352u : u32
        let s_687_0: u32 = 17352;
        // D s_687_1: read-reg s_687_0:struct
        let s_687_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_687_0 as isize);
            tracer.read_register(s_687_0 as isize, value);
            value
        };
        // D s_687_2: call _get_ICH_VTR_EL2_Type_PREbits(s_687_1)
        let s_687_2: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_687_1);
        // D s_687_3: cast zx s_687_2 -> bv
        let s_687_3: Bits = Bits::new(s_687_2 as u128, 3u16);
        // D s_687_4: cast zx s_687_3 -> i
        let s_687_4: i128 = (s_687_3.value() as i128);
        // D s_687_5: cast reint s_687_4 -> i64
        let s_687_5: i64 = (s_687_4 as i64);
        // C s_687_6: const #6s : i
        let s_687_6: i128 = 6;
        // D s_687_7: cast zx s_687_5 -> i
        let s_687_7: i128 = (i128::try_from(s_687_5).unwrap());
        // D s_687_8: sub s_687_6 s_687_7
        let s_687_8: i128 = ((s_687_6) - (s_687_7));
        // D s_687_9: cast reint s_687_8 -> i64
        let s_687_9: i64 = (s_687_8 as i64);
        // C s_687_10: const #2s : i
        let s_687_10: i128 = 2;
        // C s_687_11: const #0s : i
        let s_687_11: i128 = 0;
        // D s_687_12: cast zx s_687_9 -> i
        let s_687_12: i128 = (i128::try_from(s_687_9).unwrap());
        // D s_687_13: call integer_subrange(s_687_12, s_687_10, s_687_11)
        let s_687_13: Bits = integer_subrange(
            state,
            tracer,
            s_687_12,
            s_687_10,
            s_687_11,
        );
        // D s_687_14: cast reint s_687_13 -> u8
        let s_687_14: u8 = (s_687_13.value() as u8);
        // C s_687_15: const #18s : i
        let s_687_15: i128 = 18;
        // D s_687_16: read-var tempxt:u64
        let s_687_16: u64 = fn_state.tempxt;
        // D s_687_17: cast zx s_687_16 -> bv
        let s_687_17: Bits = Bits::new(s_687_16 as u128, 64u16);
        // D s_687_18: cast zx s_687_14 -> bv
        let s_687_18: Bits = Bits::new(s_687_14 as u128, 3u16);
        // C s_687_19: const #2s : i
        let s_687_19: i128 = 2;
        // C s_687_20: const #1u : u64
        let s_687_20: u64 = 1;
        // C s_687_21: cast zx s_687_20 -> bv
        let s_687_21: Bits = Bits::new(s_687_20 as u128, 64u16);
        // C s_687_22: lsl s_687_21 s_687_19
        let s_687_22: Bits = s_687_21 << s_687_19;
        // C s_687_23: sub s_687_22 s_687_21
        let s_687_23: Bits = ((s_687_22) - (s_687_21));
        // D s_687_24: and s_687_18 s_687_23
        let s_687_24: Bits = ((s_687_18) & (s_687_23));
        // D s_687_25: lsl s_687_24 s_687_15
        let s_687_25: Bits = s_687_24 << s_687_15;
        // C s_687_26: lsl s_687_23 s_687_15
        let s_687_26: Bits = s_687_23 << s_687_15;
        // C s_687_27: cmpl s_687_26
        let s_687_27: Bits = !s_687_26;
        // D s_687_28: and s_687_17 s_687_27
        let s_687_28: Bits = ((s_687_17) & (s_687_27));
        // D s_687_29: or s_687_28 s_687_25
        let s_687_29: Bits = ((s_687_28) | (s_687_25));
        // D s_687_30: cast reint s_687_29 -> u64
        let s_687_30: u64 = (s_687_29.value() as u64);
        // D s_687_31: write-var tempxt <= s_687_30
        fn_state.tempxt = s_687_30;
        // N s_687_32: jump b686
        return block_686(state, tracer, fn_state);
    }
    fn block_688<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_688_0: const #17352u : u32
        let s_688_0: u32 = 17352;
        // D s_688_1: read-reg s_688_0:struct
        let s_688_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_688_0 as isize);
            tracer.read_register(s_688_0 as isize, value);
            value
        };
        // D s_688_2: call _get_ICH_VTR_EL2_Type_PREbits(s_688_1)
        let s_688_2: u8 = u_get_ICH_VTR_EL2_Type_PREbits(state, tracer, s_688_1);
        // D s_688_3: cast zx s_688_2 -> bv
        let s_688_3: Bits = Bits::new(s_688_2 as u128, 3u16);
        // D s_688_4: cast zx s_688_3 -> i
        let s_688_4: i128 = (s_688_3.value() as i128);
        // D s_688_5: cast reint s_688_4 -> i64
        let s_688_5: i64 = (s_688_4 as i64);
        // C s_688_6: const #6s : i
        let s_688_6: i128 = 6;
        // D s_688_7: cast zx s_688_5 -> i
        let s_688_7: i128 = (i128::try_from(s_688_5).unwrap());
        // D s_688_8: sub s_688_6 s_688_7
        let s_688_8: i128 = ((s_688_6) - (s_688_7));
        // D s_688_9: cast reint s_688_8 -> i64
        let s_688_9: i64 = (s_688_8 as i64);
        // C s_688_10: const #2s : i
        let s_688_10: i128 = 2;
        // C s_688_11: const #0s : i
        let s_688_11: i128 = 0;
        // D s_688_12: cast zx s_688_9 -> i
        let s_688_12: i128 = (i128::try_from(s_688_9).unwrap());
        // D s_688_13: call integer_subrange(s_688_12, s_688_10, s_688_11)
        let s_688_13: Bits = integer_subrange(
            state,
            tracer,
            s_688_12,
            s_688_10,
            s_688_11,
        );
        // D s_688_14: cast reint s_688_13 -> u8
        let s_688_14: u8 = (s_688_13.value() as u8);
        // C s_688_15: const #21s : i
        let s_688_15: i128 = 21;
        // D s_688_16: read-var tempxt:u64
        let s_688_16: u64 = fn_state.tempxt;
        // D s_688_17: cast zx s_688_16 -> bv
        let s_688_17: Bits = Bits::new(s_688_16 as u128, 64u16);
        // D s_688_18: cast zx s_688_14 -> bv
        let s_688_18: Bits = Bits::new(s_688_14 as u128, 3u16);
        // C s_688_19: const #2s : i
        let s_688_19: i128 = 2;
        // C s_688_20: const #1u : u64
        let s_688_20: u64 = 1;
        // C s_688_21: cast zx s_688_20 -> bv
        let s_688_21: Bits = Bits::new(s_688_20 as u128, 64u16);
        // C s_688_22: lsl s_688_21 s_688_19
        let s_688_22: Bits = s_688_21 << s_688_19;
        // C s_688_23: sub s_688_22 s_688_21
        let s_688_23: Bits = ((s_688_22) - (s_688_21));
        // D s_688_24: and s_688_18 s_688_23
        let s_688_24: Bits = ((s_688_18) & (s_688_23));
        // D s_688_25: lsl s_688_24 s_688_15
        let s_688_25: Bits = s_688_24 << s_688_15;
        // C s_688_26: lsl s_688_23 s_688_15
        let s_688_26: Bits = s_688_23 << s_688_15;
        // C s_688_27: cmpl s_688_26
        let s_688_27: Bits = !s_688_26;
        // D s_688_28: and s_688_17 s_688_27
        let s_688_28: Bits = ((s_688_17) & (s_688_27));
        // D s_688_29: or s_688_28 s_688_25
        let s_688_29: Bits = ((s_688_28) | (s_688_25));
        // D s_688_30: cast reint s_688_29 -> u64
        let s_688_30: u64 = (s_688_29.value() as u64);
        // D s_688_31: write-var tempxt <= s_688_30
        fn_state.tempxt = s_688_30;
        // N s_688_32: jump b675
        return block_675(state, tracer, fn_state);
    }
    fn block_689<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_689_0: const #1u : u8
        let s_689_0: bool = true;
        // D s_689_1: write-var gs#141155 <= s_689_0
        fn_state.gs_141155 = s_689_0;
        // N s_689_2: jump b670
        return block_670(state, tracer, fn_state);
    }
    fn block_690<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_690_0: const #11s : i
        let s_690_0: i128 = 11;
        // D s_690_1: read-var crm:i
        let s_690_1: i128 = fn_state.crm;
        // D s_690_2: cmp-eq s_690_1 s_690_0
        let s_690_2: bool = ((s_690_1) == (s_690_0));
        // D s_690_3: write-var gs#141080 <= s_690_2
        fn_state.gs_141080 = s_690_2;
        // N s_690_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_691<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_691_0: const #7s : i
        let s_691_0: i128 = 7;
        // D s_691_1: read-var op2:i
        let s_691_1: i128 = fn_state.op2;
        // D s_691_2: cmp-eq s_691_1 s_691_0
        let s_691_2: bool = ((s_691_1) == (s_691_0));
        // D s_691_3: write-var gs#141078 <= s_691_2
        fn_state.gs_141078 = s_691_2;
        // N s_691_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_692<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_692_0: const #4s : i
        let s_692_0: i128 = 4;
        // D s_692_1: read-var op1:i
        let s_692_1: i128 = fn_state.op1;
        // D s_692_2: cmp-eq s_692_1 s_692_0
        let s_692_2: bool = ((s_692_1) == (s_692_0));
        // D s_692_3: write-var gs#141076 <= s_692_2
        fn_state.gs_141076 = s_692_2;
        // N s_692_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_693<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_693_0: const #12s : i
        let s_693_0: i128 = 12;
        // D s_693_1: read-var crn:i
        let s_693_1: i128 = fn_state.crn;
        // D s_693_2: cmp-eq s_693_1 s_693_0
        let s_693_2: bool = ((s_693_1) == (s_693_0));
        // D s_693_3: write-var gs#141074 <= s_693_2
        fn_state.gs_141074 = s_693_2;
        // N s_693_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_694<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_694_0: const #11632u : u32
        let s_694_0: u32 = 11632;
        // D s_694_1: read-reg s_694_0:struct
        let s_694_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_694_0 as isize);
            tracer.read_register(s_694_0 as isize, value);
            value
        };
        // D s_694_2: call _get_ICC_CTLR_EL3_Type_ExtRange(s_694_1)
        let s_694_2: bool = u_get_ICC_CTLR_EL3_Type_ExtRange(state, tracer, s_694_1);
        // D s_694_3: call Bit(s_694_2)
        let s_694_3: bool = Bit(state, tracer, s_694_2);
        // C s_694_4: const #19s : i
        let s_694_4: i128 = 19;
        // D s_694_5: read-var tempxt:u64
        let s_694_5: u64 = fn_state.tempxt;
        // D s_694_6: cast zx s_694_5 -> bv
        let s_694_6: Bits = Bits::new(s_694_5 as u128, 64u16);
        // C s_694_7: const #1u : u64
        let s_694_7: u64 = 1;
        // D s_694_8: bit-insert s_694_6 s_694_6 s_694_4 s_694_7
        let s_694_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_694_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_694_6.length(),
            );
            (s_694_6 & mask) | (s_694_6 << s_694_4)
        };
        // D s_694_9: cast reint s_694_8 -> u64
        let s_694_9: u64 = (s_694_8.value() as u64);
        // D s_694_10: write-var tempxt <= s_694_9
        fn_state.tempxt = s_694_9;
        // C s_694_11: const #11632u : u32
        let s_694_11: u32 = 11632;
        // D s_694_12: read-reg s_694_11:struct
        let s_694_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_694_11 as isize);
            tracer.read_register(s_694_11 as isize, value);
            value
        };
        // D s_694_13: call _get_ICC_CTLR_EL3_Type_IDbits(s_694_12)
        let s_694_13: u8 = u_get_ICC_CTLR_EL3_Type_IDbits(state, tracer, s_694_12);
        // C s_694_14: const #11s : i
        let s_694_14: i128 = 11;
        // D s_694_15: read-var tempxt:u64
        let s_694_15: u64 = fn_state.tempxt;
        // D s_694_16: cast zx s_694_15 -> bv
        let s_694_16: Bits = Bits::new(s_694_15 as u128, 64u16);
        // D s_694_17: cast zx s_694_13 -> bv
        let s_694_17: Bits = Bits::new(s_694_13 as u128, 3u16);
        // C s_694_18: const #2s : i
        let s_694_18: i128 = 2;
        // C s_694_19: const #1u : u64
        let s_694_19: u64 = 1;
        // C s_694_20: cast zx s_694_19 -> bv
        let s_694_20: Bits = Bits::new(s_694_19 as u128, 64u16);
        // C s_694_21: lsl s_694_20 s_694_18
        let s_694_21: Bits = s_694_20 << s_694_18;
        // C s_694_22: sub s_694_21 s_694_20
        let s_694_22: Bits = ((s_694_21) - (s_694_20));
        // D s_694_23: and s_694_17 s_694_22
        let s_694_23: Bits = ((s_694_17) & (s_694_22));
        // D s_694_24: lsl s_694_23 s_694_14
        let s_694_24: Bits = s_694_23 << s_694_14;
        // C s_694_25: lsl s_694_22 s_694_14
        let s_694_25: Bits = s_694_22 << s_694_14;
        // C s_694_26: cmpl s_694_25
        let s_694_26: Bits = !s_694_25;
        // D s_694_27: and s_694_16 s_694_26
        let s_694_27: Bits = ((s_694_16) & (s_694_26));
        // D s_694_28: or s_694_27 s_694_24
        let s_694_28: Bits = ((s_694_27) | (s_694_24));
        // D s_694_29: cast reint s_694_28 -> u64
        let s_694_29: u64 = (s_694_28.value() as u64);
        // D s_694_30: write-var tempxt <= s_694_29
        fn_state.tempxt = s_694_29;
        // C s_694_31: const #11632u : u32
        let s_694_31: u32 = 11632;
        // D s_694_32: read-reg s_694_31:struct
        let s_694_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_694_31 as isize);
            tracer.read_register(s_694_31 as isize, value);
            value
        };
        // D s_694_33: call _get_ICC_CTLR_EL3_Type_PRIbits(s_694_32)
        let s_694_33: u8 = u_get_ICC_CTLR_EL3_Type_PRIbits(state, tracer, s_694_32);
        // C s_694_34: const #8s : i
        let s_694_34: i128 = 8;
        // D s_694_35: read-var tempxt:u64
        let s_694_35: u64 = fn_state.tempxt;
        // D s_694_36: cast zx s_694_35 -> bv
        let s_694_36: Bits = Bits::new(s_694_35 as u128, 64u16);
        // D s_694_37: cast zx s_694_33 -> bv
        let s_694_37: Bits = Bits::new(s_694_33 as u128, 3u16);
        // C s_694_38: const #2s : i
        let s_694_38: i128 = 2;
        // C s_694_39: const #1u : u64
        let s_694_39: u64 = 1;
        // C s_694_40: cast zx s_694_39 -> bv
        let s_694_40: Bits = Bits::new(s_694_39 as u128, 64u16);
        // C s_694_41: lsl s_694_40 s_694_38
        let s_694_41: Bits = s_694_40 << s_694_38;
        // C s_694_42: sub s_694_41 s_694_40
        let s_694_42: Bits = ((s_694_41) - (s_694_40));
        // D s_694_43: and s_694_37 s_694_42
        let s_694_43: Bits = ((s_694_37) & (s_694_42));
        // D s_694_44: lsl s_694_43 s_694_34
        let s_694_44: Bits = s_694_43 << s_694_34;
        // C s_694_45: lsl s_694_42 s_694_34
        let s_694_45: Bits = s_694_42 << s_694_34;
        // C s_694_46: cmpl s_694_45
        let s_694_46: Bits = !s_694_45;
        // D s_694_47: and s_694_36 s_694_46
        let s_694_47: Bits = ((s_694_36) & (s_694_46));
        // D s_694_48: or s_694_47 s_694_44
        let s_694_48: Bits = ((s_694_47) | (s_694_44));
        // D s_694_49: cast reint s_694_48 -> u64
        let s_694_49: u64 = (s_694_48.value() as u64);
        // D s_694_50: write-var tempxt <= s_694_49
        fn_state.tempxt = s_694_49;
        // C s_694_51: const #11632u : u32
        let s_694_51: u32 = 11632;
        // D s_694_52: read-reg s_694_51:struct
        let s_694_52: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_694_51 as isize);
            tracer.read_register(s_694_51 as isize, value);
            value
        };
        // D s_694_53: call _get_ICC_CTLR_EL3_Type_nDS(s_694_52)
        let s_694_53: bool = u_get_ICC_CTLR_EL3_Type_nDS(state, tracer, s_694_52);
        // D s_694_54: call Bit(s_694_53)
        let s_694_54: bool = Bit(state, tracer, s_694_53);
        // C s_694_55: const #17s : i
        let s_694_55: i128 = 17;
        // D s_694_56: read-var tempxt:u64
        let s_694_56: u64 = fn_state.tempxt;
        // D s_694_57: cast zx s_694_56 -> bv
        let s_694_57: Bits = Bits::new(s_694_56 as u128, 64u16);
        // C s_694_58: const #1u : u64
        let s_694_58: u64 = 1;
        // D s_694_59: bit-insert s_694_57 s_694_57 s_694_55 s_694_58
        let s_694_59: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_694_58 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_694_57.length(),
            );
            (s_694_57 & mask) | (s_694_57 << s_694_55)
        };
        // D s_694_60: cast reint s_694_59 -> u64
        let s_694_60: u64 = (s_694_59.value() as u64);
        // D s_694_61: write-var tempxt <= s_694_60
        fn_state.tempxt = s_694_60;
        // C s_694_62: const #11632u : u32
        let s_694_62: u32 = 11632;
        // D s_694_63: read-reg s_694_62:struct
        let s_694_63: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_694_62 as isize);
            tracer.read_register(s_694_62 as isize, value);
            value
        };
        // D s_694_64: call _get_ICC_CTLR_EL3_Type_A3V(s_694_63)
        let s_694_64: bool = u_get_ICC_CTLR_EL3_Type_A3V(state, tracer, s_694_63);
        // D s_694_65: call Bit(s_694_64)
        let s_694_65: bool = Bit(state, tracer, s_694_64);
        // C s_694_66: const #15s : i
        let s_694_66: i128 = 15;
        // D s_694_67: read-var tempxt:u64
        let s_694_67: u64 = fn_state.tempxt;
        // D s_694_68: cast zx s_694_67 -> bv
        let s_694_68: Bits = Bits::new(s_694_67 as u128, 64u16);
        // C s_694_69: const #1u : u64
        let s_694_69: u64 = 1;
        // D s_694_70: bit-insert s_694_68 s_694_68 s_694_66 s_694_69
        let s_694_70: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_694_69 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_694_68.length(),
            );
            (s_694_68 & mask) | (s_694_68 << s_694_66)
        };
        // D s_694_71: cast reint s_694_70 -> u64
        let s_694_71: u64 = (s_694_70.value() as u64);
        // D s_694_72: write-var tempxt <= s_694_71
        fn_state.tempxt = s_694_71;
        // C s_694_73: const #11632u : u32
        let s_694_73: u32 = 11632;
        // D s_694_74: read-reg s_694_73:struct
        let s_694_74: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_694_73 as isize);
            tracer.read_register(s_694_73 as isize, value);
            value
        };
        // D s_694_75: call _get_ICC_CTLR_EL3_Type_SEIS(s_694_74)
        let s_694_75: bool = u_get_ICC_CTLR_EL3_Type_SEIS(state, tracer, s_694_74);
        // D s_694_76: call Bit(s_694_75)
        let s_694_76: bool = Bit(state, tracer, s_694_75);
        // C s_694_77: const #14s : i
        let s_694_77: i128 = 14;
        // D s_694_78: read-var tempxt:u64
        let s_694_78: u64 = fn_state.tempxt;
        // D s_694_79: cast zx s_694_78 -> bv
        let s_694_79: Bits = Bits::new(s_694_78 as u128, 64u16);
        // C s_694_80: const #1u : u64
        let s_694_80: u64 = 1;
        // D s_694_81: bit-insert s_694_79 s_694_79 s_694_77 s_694_80
        let s_694_81: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_694_80 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_694_79.length(),
            );
            (s_694_79 & mask) | (s_694_79 << s_694_77)
        };
        // D s_694_82: cast reint s_694_81 -> u64
        let s_694_82: u64 = (s_694_81.value() as u64);
        // D s_694_83: write-var tempxt <= s_694_82
        fn_state.tempxt = s_694_82;
        // N s_694_84: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_695<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_695_0: const #12s : i
        let s_695_0: i128 = 12;
        // D s_695_1: read-var crm:i
        let s_695_1: i128 = fn_state.crm;
        // D s_695_2: cmp-eq s_695_1 s_695_0
        let s_695_2: bool = ((s_695_1) == (s_695_0));
        // D s_695_3: write-var gs#141071 <= s_695_2
        fn_state.gs_141071 = s_695_2;
        // N s_695_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_696<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_696_0: const #4s : i
        let s_696_0: i128 = 4;
        // D s_696_1: read-var op2:i
        let s_696_1: i128 = fn_state.op2;
        // D s_696_2: cmp-eq s_696_1 s_696_0
        let s_696_2: bool = ((s_696_1) == (s_696_0));
        // D s_696_3: write-var gs#141069 <= s_696_2
        fn_state.gs_141069 = s_696_2;
        // N s_696_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_697<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_697_0: const #6s : i
        let s_697_0: i128 = 6;
        // D s_697_1: read-var op1:i
        let s_697_1: i128 = fn_state.op1;
        // D s_697_2: cmp-eq s_697_1 s_697_0
        let s_697_2: bool = ((s_697_1) == (s_697_0));
        // D s_697_3: write-var gs#141067 <= s_697_2
        fn_state.gs_141067 = s_697_2;
        // N s_697_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_698<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_698_0: const #12s : i
        let s_698_0: i128 = 12;
        // D s_698_1: read-var crn:i
        let s_698_1: i128 = fn_state.crn;
        // D s_698_2: cmp-eq s_698_1 s_698_0
        let s_698_2: bool = ((s_698_1) == (s_698_0));
        // D s_698_3: write-var gs#141065 <= s_698_2
        fn_state.gs_141065 = s_698_2;
        // N s_698_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_699<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_699_0: const #12824u : u32
        let s_699_0: u32 = 12824;
        // D s_699_1: read-reg s_699_0:struct
        let s_699_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_699_0 as isize);
            tracer.read_register(s_699_0 as isize, value);
            value
        };
        // D s_699_2: call _get_ICV_CTLR_EL1_Type_ExtRange(s_699_1)
        let s_699_2: bool = u_get_ICV_CTLR_EL1_Type_ExtRange(state, tracer, s_699_1);
        // D s_699_3: call Bit(s_699_2)
        let s_699_3: bool = Bit(state, tracer, s_699_2);
        // C s_699_4: const #19s : i
        let s_699_4: i128 = 19;
        // D s_699_5: read-var tempxt:u64
        let s_699_5: u64 = fn_state.tempxt;
        // D s_699_6: cast zx s_699_5 -> bv
        let s_699_6: Bits = Bits::new(s_699_5 as u128, 64u16);
        // C s_699_7: const #1u : u64
        let s_699_7: u64 = 1;
        // D s_699_8: bit-insert s_699_6 s_699_6 s_699_4 s_699_7
        let s_699_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_699_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_699_6.length(),
            );
            (s_699_6 & mask) | (s_699_6 << s_699_4)
        };
        // D s_699_9: cast reint s_699_8 -> u64
        let s_699_9: u64 = (s_699_8.value() as u64);
        // D s_699_10: write-var tempxt <= s_699_9
        fn_state.tempxt = s_699_9;
        // C s_699_11: const #12824u : u32
        let s_699_11: u32 = 12824;
        // D s_699_12: read-reg s_699_11:struct
        let s_699_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_699_11 as isize);
            tracer.read_register(s_699_11 as isize, value);
            value
        };
        // D s_699_13: call _get_ICV_CTLR_EL1_Type_IDbits(s_699_12)
        let s_699_13: u8 = u_get_ICV_CTLR_EL1_Type_IDbits(state, tracer, s_699_12);
        // C s_699_14: const #11s : i
        let s_699_14: i128 = 11;
        // D s_699_15: read-var tempxt:u64
        let s_699_15: u64 = fn_state.tempxt;
        // D s_699_16: cast zx s_699_15 -> bv
        let s_699_16: Bits = Bits::new(s_699_15 as u128, 64u16);
        // D s_699_17: cast zx s_699_13 -> bv
        let s_699_17: Bits = Bits::new(s_699_13 as u128, 3u16);
        // C s_699_18: const #2s : i
        let s_699_18: i128 = 2;
        // C s_699_19: const #1u : u64
        let s_699_19: u64 = 1;
        // C s_699_20: cast zx s_699_19 -> bv
        let s_699_20: Bits = Bits::new(s_699_19 as u128, 64u16);
        // C s_699_21: lsl s_699_20 s_699_18
        let s_699_21: Bits = s_699_20 << s_699_18;
        // C s_699_22: sub s_699_21 s_699_20
        let s_699_22: Bits = ((s_699_21) - (s_699_20));
        // D s_699_23: and s_699_17 s_699_22
        let s_699_23: Bits = ((s_699_17) & (s_699_22));
        // D s_699_24: lsl s_699_23 s_699_14
        let s_699_24: Bits = s_699_23 << s_699_14;
        // C s_699_25: lsl s_699_22 s_699_14
        let s_699_25: Bits = s_699_22 << s_699_14;
        // C s_699_26: cmpl s_699_25
        let s_699_26: Bits = !s_699_25;
        // D s_699_27: and s_699_16 s_699_26
        let s_699_27: Bits = ((s_699_16) & (s_699_26));
        // D s_699_28: or s_699_27 s_699_24
        let s_699_28: Bits = ((s_699_27) | (s_699_24));
        // D s_699_29: cast reint s_699_28 -> u64
        let s_699_29: u64 = (s_699_28.value() as u64);
        // D s_699_30: write-var tempxt <= s_699_29
        fn_state.tempxt = s_699_29;
        // C s_699_31: const #12824u : u32
        let s_699_31: u32 = 12824;
        // D s_699_32: read-reg s_699_31:struct
        let s_699_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_699_31 as isize);
            tracer.read_register(s_699_31 as isize, value);
            value
        };
        // D s_699_33: call _get_ICV_CTLR_EL1_Type_PRIbits(s_699_32)
        let s_699_33: u8 = u_get_ICV_CTLR_EL1_Type_PRIbits(state, tracer, s_699_32);
        // C s_699_34: const #8s : i
        let s_699_34: i128 = 8;
        // D s_699_35: read-var tempxt:u64
        let s_699_35: u64 = fn_state.tempxt;
        // D s_699_36: cast zx s_699_35 -> bv
        let s_699_36: Bits = Bits::new(s_699_35 as u128, 64u16);
        // D s_699_37: cast zx s_699_33 -> bv
        let s_699_37: Bits = Bits::new(s_699_33 as u128, 3u16);
        // C s_699_38: const #2s : i
        let s_699_38: i128 = 2;
        // C s_699_39: const #1u : u64
        let s_699_39: u64 = 1;
        // C s_699_40: cast zx s_699_39 -> bv
        let s_699_40: Bits = Bits::new(s_699_39 as u128, 64u16);
        // C s_699_41: lsl s_699_40 s_699_38
        let s_699_41: Bits = s_699_40 << s_699_38;
        // C s_699_42: sub s_699_41 s_699_40
        let s_699_42: Bits = ((s_699_41) - (s_699_40));
        // D s_699_43: and s_699_37 s_699_42
        let s_699_43: Bits = ((s_699_37) & (s_699_42));
        // D s_699_44: lsl s_699_43 s_699_34
        let s_699_44: Bits = s_699_43 << s_699_34;
        // C s_699_45: lsl s_699_42 s_699_34
        let s_699_45: Bits = s_699_42 << s_699_34;
        // C s_699_46: cmpl s_699_45
        let s_699_46: Bits = !s_699_45;
        // D s_699_47: and s_699_36 s_699_46
        let s_699_47: Bits = ((s_699_36) & (s_699_46));
        // D s_699_48: or s_699_47 s_699_44
        let s_699_48: Bits = ((s_699_47) | (s_699_44));
        // D s_699_49: cast reint s_699_48 -> u64
        let s_699_49: u64 = (s_699_48.value() as u64);
        // D s_699_50: write-var tempxt <= s_699_49
        fn_state.tempxt = s_699_49;
        // C s_699_51: const #12824u : u32
        let s_699_51: u32 = 12824;
        // D s_699_52: read-reg s_699_51:struct
        let s_699_52: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_699_51 as isize);
            tracer.read_register(s_699_51 as isize, value);
            value
        };
        // D s_699_53: call _get_ICV_CTLR_EL1_Type_A3V(s_699_52)
        let s_699_53: bool = u_get_ICV_CTLR_EL1_Type_A3V(state, tracer, s_699_52);
        // D s_699_54: call Bit(s_699_53)
        let s_699_54: bool = Bit(state, tracer, s_699_53);
        // C s_699_55: const #15s : i
        let s_699_55: i128 = 15;
        // D s_699_56: read-var tempxt:u64
        let s_699_56: u64 = fn_state.tempxt;
        // D s_699_57: cast zx s_699_56 -> bv
        let s_699_57: Bits = Bits::new(s_699_56 as u128, 64u16);
        // C s_699_58: const #1u : u64
        let s_699_58: u64 = 1;
        // D s_699_59: bit-insert s_699_57 s_699_57 s_699_55 s_699_58
        let s_699_59: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_699_58 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_699_57.length(),
            );
            (s_699_57 & mask) | (s_699_57 << s_699_55)
        };
        // D s_699_60: cast reint s_699_59 -> u64
        let s_699_60: u64 = (s_699_59.value() as u64);
        // D s_699_61: write-var tempxt <= s_699_60
        fn_state.tempxt = s_699_60;
        // C s_699_62: const #12824u : u32
        let s_699_62: u32 = 12824;
        // D s_699_63: read-reg s_699_62:struct
        let s_699_63: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_699_62 as isize);
            tracer.read_register(s_699_62 as isize, value);
            value
        };
        // D s_699_64: call _get_ICV_CTLR_EL1_Type_SEIS(s_699_63)
        let s_699_64: bool = u_get_ICV_CTLR_EL1_Type_SEIS(state, tracer, s_699_63);
        // D s_699_65: call Bit(s_699_64)
        let s_699_65: bool = Bit(state, tracer, s_699_64);
        // C s_699_66: const #14s : i
        let s_699_66: i128 = 14;
        // D s_699_67: read-var tempxt:u64
        let s_699_67: u64 = fn_state.tempxt;
        // D s_699_68: cast zx s_699_67 -> bv
        let s_699_68: Bits = Bits::new(s_699_67 as u128, 64u16);
        // C s_699_69: const #1u : u64
        let s_699_69: u64 = 1;
        // D s_699_70: bit-insert s_699_68 s_699_68 s_699_66 s_699_69
        let s_699_70: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_699_69 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_699_68.length(),
            );
            (s_699_68 & mask) | (s_699_68 << s_699_66)
        };
        // D s_699_71: cast reint s_699_70 -> u64
        let s_699_71: u64 = (s_699_70.value() as u64);
        // D s_699_72: write-var tempxt <= s_699_71
        fn_state.tempxt = s_699_71;
        // N s_699_73: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_700<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_700_0: const #12s : i
        let s_700_0: i128 = 12;
        // D s_700_1: read-var crm:i
        let s_700_1: i128 = fn_state.crm;
        // D s_700_2: cmp-eq s_700_1 s_700_0
        let s_700_2: bool = ((s_700_1) == (s_700_0));
        // D s_700_3: write-var gs#141062 <= s_700_2
        fn_state.gs_141062 = s_700_2;
        // N s_700_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_701<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_701_0: const #4s : i
        let s_701_0: i128 = 4;
        // D s_701_1: read-var op2:i
        let s_701_1: i128 = fn_state.op2;
        // D s_701_2: cmp-eq s_701_1 s_701_0
        let s_701_2: bool = ((s_701_1) == (s_701_0));
        // D s_701_3: write-var gs#141060 <= s_701_2
        fn_state.gs_141060 = s_701_2;
        // N s_701_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_702<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_702_0: const #0s : i
        let s_702_0: i128 = 0;
        // D s_702_1: read-var op1:i
        let s_702_1: i128 = fn_state.op1;
        // D s_702_2: cmp-eq s_702_1 s_702_0
        let s_702_2: bool = ((s_702_1) == (s_702_0));
        // D s_702_3: write-var gs#141058 <= s_702_2
        fn_state.gs_141058 = s_702_2;
        // N s_702_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_703<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_703_0: const #12s : i
        let s_703_0: i128 = 12;
        // D s_703_1: read-var crn:i
        let s_703_1: i128 = fn_state.crn;
        // D s_703_2: cmp-eq s_703_1 s_703_0
        let s_703_2: bool = ((s_703_1) == (s_703_0));
        // D s_703_3: write-var gs#141056 <= s_703_2
        fn_state.gs_141056 = s_703_2;
        // N s_703_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_704<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_704_0: const #7s : i
        let s_704_0: i128 = 7;
        // D s_704_1: read-var tempxt:u64
        let s_704_1: u64 = fn_state.tempxt;
        // D s_704_2: cast zx s_704_1 -> bv
        let s_704_2: Bits = Bits::new(s_704_1 as u128, 64u16);
        // C s_704_3: const #1u : u64
        let s_704_3: u64 = 1;
        // D s_704_4: bit-extract s_704_2 s_704_0 s_704_3
        let s_704_4: Bits = (Bits::new(
            ((s_704_2) >> (s_704_0)).value(),
            u16::try_from(s_704_3).unwrap(),
        ));
        // D s_704_5: cast reint s_704_4 -> u8
        let s_704_5: bool = ((s_704_4.value()) != 0);
        // C s_704_6: const #0s : i
        let s_704_6: i128 = 0;
        // C s_704_7: const #0u : u64
        let s_704_7: u64 = 0;
        // D s_704_8: cast zx s_704_5 -> u64
        let s_704_8: u64 = (s_704_5 as u64);
        // C s_704_9: const #1u : u64
        let s_704_9: u64 = 1;
        // D s_704_10: and s_704_8 s_704_9
        let s_704_10: u64 = ((s_704_8) & (s_704_9));
        // D s_704_11: cmp-eq s_704_10 s_704_9
        let s_704_11: bool = ((s_704_10) == (s_704_9));
        // D s_704_12: lsl s_704_8 s_704_6
        let s_704_12: u64 = s_704_8 << s_704_6;
        // D s_704_13: or s_704_7 s_704_12
        let s_704_13: u64 = ((s_704_7) | (s_704_12));
        // D s_704_14: cmpl s_704_12
        let s_704_14: u64 = !s_704_12;
        // D s_704_15: and s_704_7 s_704_14
        let s_704_15: u64 = ((s_704_7) & (s_704_14));
        // D s_704_16: select s_704_11 s_704_13 s_704_15
        let s_704_16: u64 = if s_704_11 { s_704_13 } else { s_704_15 };
        // D s_704_17: cast trunc s_704_16 -> u8
        let s_704_17: bool = ((s_704_16) != 0);
        // D s_704_18: call Bit(s_704_17)
        let s_704_18: bool = Bit(state, tracer, s_704_17);
        // C s_704_19: const #8s : i
        let s_704_19: i128 = 8;
        // D s_704_20: read-var tempxt:u64
        let s_704_20: u64 = fn_state.tempxt;
        // D s_704_21: cast zx s_704_20 -> bv
        let s_704_21: Bits = Bits::new(s_704_20 as u128, 64u16);
        // C s_704_22: const #1u : u64
        let s_704_22: u64 = 1;
        // D s_704_23: bit-insert s_704_21 s_704_21 s_704_19 s_704_22
        let s_704_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_704_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_704_21.length(),
            );
            (s_704_21 & mask) | (s_704_21 << s_704_19)
        };
        // D s_704_24: cast reint s_704_23 -> u64
        let s_704_24: u64 = (s_704_23.value() as u64);
        // D s_704_25: write-var tempxt <= s_704_24
        fn_state.tempxt = s_704_24;
        // C s_704_26: const #5s : i
        let s_704_26: i128 = 5;
        // D s_704_27: read-var tempxt:u64
        let s_704_27: u64 = fn_state.tempxt;
        // D s_704_28: cast zx s_704_27 -> bv
        let s_704_28: Bits = Bits::new(s_704_27 as u128, 64u16);
        // C s_704_29: const #1u : u64
        let s_704_29: u64 = 1;
        // D s_704_30: bit-extract s_704_28 s_704_26 s_704_29
        let s_704_30: Bits = (Bits::new(
            ((s_704_28) >> (s_704_26)).value(),
            u16::try_from(s_704_29).unwrap(),
        ));
        // D s_704_31: cast reint s_704_30 -> u8
        let s_704_31: bool = ((s_704_30.value()) != 0);
        // C s_704_32: const #0s : i
        let s_704_32: i128 = 0;
        // C s_704_33: const #0u : u64
        let s_704_33: u64 = 0;
        // D s_704_34: cast zx s_704_31 -> u64
        let s_704_34: u64 = (s_704_31 as u64);
        // C s_704_35: const #1u : u64
        let s_704_35: u64 = 1;
        // D s_704_36: and s_704_34 s_704_35
        let s_704_36: u64 = ((s_704_34) & (s_704_35));
        // D s_704_37: cmp-eq s_704_36 s_704_35
        let s_704_37: bool = ((s_704_36) == (s_704_35));
        // D s_704_38: lsl s_704_34 s_704_32
        let s_704_38: u64 = s_704_34 << s_704_32;
        // D s_704_39: or s_704_33 s_704_38
        let s_704_39: u64 = ((s_704_33) | (s_704_38));
        // D s_704_40: cmpl s_704_38
        let s_704_40: u64 = !s_704_38;
        // D s_704_41: and s_704_33 s_704_40
        let s_704_41: u64 = ((s_704_33) & (s_704_40));
        // D s_704_42: select s_704_37 s_704_39 s_704_41
        let s_704_42: u64 = if s_704_37 { s_704_39 } else { s_704_41 };
        // D s_704_43: cast trunc s_704_42 -> u8
        let s_704_43: bool = ((s_704_42) != 0);
        // D s_704_44: call Bit(s_704_43)
        let s_704_44: bool = Bit(state, tracer, s_704_43);
        // C s_704_45: const #6s : i
        let s_704_45: i128 = 6;
        // D s_704_46: read-var tempxt:u64
        let s_704_46: u64 = fn_state.tempxt;
        // D s_704_47: cast zx s_704_46 -> bv
        let s_704_47: Bits = Bits::new(s_704_46 as u128, 64u16);
        // C s_704_48: const #1u : u64
        let s_704_48: u64 = 1;
        // D s_704_49: bit-insert s_704_47 s_704_47 s_704_45 s_704_48
        let s_704_49: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_704_48 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_704_47.length(),
            );
            (s_704_47 & mask) | (s_704_47 << s_704_45)
        };
        // D s_704_50: cast reint s_704_49 -> u64
        let s_704_50: u64 = (s_704_49.value() as u64);
        // D s_704_51: write-var tempxt <= s_704_50
        fn_state.tempxt = s_704_50;
        // N s_704_52: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_705<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_705_0: const #5s : i
        let s_705_0: i128 = 5;
        // D s_705_1: read-var op2:i
        let s_705_1: i128 = fn_state.op2;
        // D s_705_2: cmp-eq s_705_1 s_705_0
        let s_705_2: bool = ((s_705_1) == (s_705_0));
        // D s_705_3: write-var gs#141053 <= s_705_2
        fn_state.gs_141053 = s_705_2;
        // N s_705_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_706<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_706_0: const #0s : i
        let s_706_0: i128 = 0;
        // D s_706_1: read-var op1:i
        let s_706_1: i128 = fn_state.op1;
        // D s_706_2: cmp-eq s_706_1 s_706_0
        let s_706_2: bool = ((s_706_1) == (s_706_0));
        // D s_706_3: write-var gs#141051 <= s_706_2
        fn_state.gs_141051 = s_706_2;
        // N s_706_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_707<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_707_0: const #0s : i
        let s_707_0: i128 = 0;
        // D s_707_1: read-var crn:i
        let s_707_1: i128 = fn_state.crn;
        // D s_707_2: cmp-eq s_707_1 s_707_0
        let s_707_2: bool = ((s_707_1) == (s_707_0));
        // D s_707_3: write-var gs#141049 <= s_707_2
        fn_state.gs_141049 = s_707_2;
        // N s_707_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
