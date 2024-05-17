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
use EDSCR_write::*;
use AArch32_PMUSwIncrement::*;
use GetNumEventCounters::*;
use DBGDSCRext_write::*;
use DBGCLAIMCLR_write::*;
use PMINTENSET_read::*;
use Zeros::*;
use PMOVSSET_read::*;
use R_read::*;
use Mk_MIDR_Type::*;
use Mk_PMEVTYPER_Type::*;
use PMINTENSET_write::*;
use u_get_CPACR_Type_cp10::*;
use u__get_PMEVTYPER::*;
use u_update_DBGOSLSR_Type_OSLK::*;
use CPACR_read__1::*;
use AArch32_AutoGen_SysRegWrite32::*;
use u_update_DBGDSCRext_Type_TXfull::*;
use Mk_PMCCNTR_Type::*;
use Mk_PMINTENCLR_Type::*;
use u_get_PMSELR_Type_SEL::*;
use PMCNTENCLR_write::*;
use u_update_DBGDSCRint_Type_TXfull::*;
use AArch32_ClearEventCounters::*;
use Mk_DBGCLAIMCLR_Type::*;
use CheckOSUnlockCatch::*;
use ELUsingAArch32::*;
use integer_subrange::*;
use DBGDSCRint_write::*;
use DBGDSCRint_read::*;
use u_update_EDSCR_Type_TXfull::*;
use Mk_PMCCFILTR_Type::*;
use PMINTENCLR_write::*;
use Mk_PMCNTENCLR_Type::*;
use u_get_PMCR_Type_D::*;
use Bit::*;
use PMCCNTR_write::*;
use AArch32_TakeHypTrapException::*;
use PMSELR_read::*;
use DBGDSCRext_read::*;
use MIDR_write::*;
use PMCNTENSET_write::*;
use Mk_PMCNTENSET_Type::*;
use TakeReset::*;
use AArch32_GetNumEventCountersAccessible::*;
use PMEVCNTR_set::*;
use PMOVSSET_write::*;
use PMCNTENSET_read::*;
use DBGOSLSR_read::*;
use PMUCounterMask::*;
use EL2Enabled::*;
use Mk_PMOVSR_Type::*;
use u_get_DBGOSLSR_Type_OSLK::*;
use Mk_PMINTENSET_Type::*;
use u__get_PMCCFILTR::*;
use CurrentSecurityState::*;
use AArch64_AArch32SystemAccessTrap::*;
use PMOVSR_write::*;
use u_get_CPACR_Type_cp11::*;
use PMCCFILTR_write::*;
use R_set::*;
use ELFromM32::*;
use DBGCLAIMCLR_read::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use PMCR_read::*;
use u_get_NSACR_Type_cp10::*;
use Mk_PMOVSSET_Type::*;
use DBGOSLSR_write::*;
use EDSCR_read::*;
use PMEVTYPER_set::*;
use common::*;
pub fn AArch32_SysRegWrite<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cp_num: i128,
    instr: u32,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_142276: bool,
        gs_142419: bool,
        gs_142426: bool,
        gs_142331: bool,
        gs_142284: bool,
        mask: u32,
        gs_142471: bool,
        gs_142301: bool,
        gs_142422: bool,
        gs_142358: bool,
        CRn: u8,
        gs_142446: bool,
        gs_142437: bool,
        temprt2: u32,
        gs_142424: bool,
        gs_142505: bool,
        gs_142441: bool,
        ga_248524: ProductType700c18a878c5601b,
        gs_142312: bool,
        gs_142375: bool,
        gs_142501: bool,
        gs_142306: bool,
        gs_142280: bool,
        ga_248643: ProductType700c18a878c5601b,
        gs_142389: bool,
        gs_142274: bool,
        gs_142429: bool,
        gs_142377: bool,
        gs_142438: bool,
        gs_142459: bool,
        gs_142304: bool,
        gs_142330: bool,
        gs_142473: bool,
        gs_142273: bool,
        gs_142308: bool,
        gs_142502: bool,
        gs_142427: bool,
        opc1: u8,
        temprt: u32,
        gs_142336: bool,
        gs_142434: bool,
        gs_142428: bool,
        gs_142290: bool,
        ga_248518: ProductType700c18a878c5601b,
        ga_248649: ProductType700c18a878c5601b,
        gs_142350: bool,
        gs_142285: bool,
        gs_142376: bool,
        gs_142414: bool,
        gs_142310: bool,
        gs_142474: bool,
        gs_142409: bool,
        gs_142352: bool,
        gs_142369: bool,
        gs_142333: bool,
        gs_142379: bool,
        gs_142411: bool,
        gs_142410: bool,
        ga_248661: ProductType700c18a878c5601b,
        gs_142450: bool,
        gs_142457: bool,
        el: u8,
        gs_142311: bool,
        gs_142307: bool,
        gs_142455: bool,
        gs_142281: bool,
        gs_142289: bool,
        opc2: u8,
        ga_248513: ProductType700c18a878c5601b,
        ga_248535: ProductType700c18a878c5601b,
        gs_142470: bool,
        gs_142380: bool,
        gs_142448: bool,
        gs_142271: bool,
        gs_142351: bool,
        gs_142451: bool,
        gs_142420: bool,
        gs_142443: bool,
        gs_142335: bool,
        pmselr: i64,
        gs_142418: bool,
        gs_142433: bool,
        gs_142431: bool,
        gs_142272: bool,
        gs_142444: bool,
        gs_142300: bool,
        gs_142472: bool,
        gs_142503: bool,
        gs_142278: bool,
        gs_142388: bool,
        ga_248507: ProductType700c18a878c5601b,
        gs_142452: bool,
        gs_142449: bool,
        gs_142408: bool,
        gs_142458: bool,
        gs_142432: bool,
        gs_142268: bool,
        gs_142413: bool,
        gs_142277: bool,
        gs_142287: bool,
        gs_142390: bool,
        gs_142269: bool,
        gs_142267: bool,
        gs_142299: bool,
        gs_142279: bool,
        gs_142378: bool,
        gs_142303: bool,
        ga_248471: ProductTypea5cc8de4daab131c,
        gs_142421: bool,
        index: i64,
        gs_142305: bool,
        gs_142415: bool,
        gs_142286: bool,
        gs_142435: bool,
        CRm: u8,
        ga_248529: ProductType700c18a878c5601b,
        gs_142445: bool,
        gs_142416: bool,
        gs_142359: bool,
        gs_142504: bool,
        gs_142460: bool,
        gs_142282: bool,
        gs_142332: bool,
        gs_142506: bool,
        gs_142423: bool,
        gs_142349: bool,
        cp_num: i128,
        instr: u32,
        t: i128,
    }
    let fn_state = FunctionState {
        cp_num,
        instr,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var t:i
        let s_0_0: i128 = fn_state.t;
        // D s_0_1: call R_read(s_0_0)
        let s_0_1: u32 = R_read(state, tracer, s_0_0);
        // D s_0_2: write-var temprt <= s_0_1
        fn_state.temprt = s_0_1;
        // C s_0_3: const #16983u : u32
        let s_0_3: u32 = 16983;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call ELFromM32(s_0_4)
        let s_0_5: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_0_4);
        // D s_0_6: write-var ga#248471 <= s_0_5
        fn_state.ga_248471 = s_0_5;
        // D s_0_7: read-var ga#248471.1:struct
        let s_0_7: u8 = fn_state.ga_248471._1;
        // D s_0_8: write-var el <= s_0_7
        fn_state.el = s_0_7;
        // C s_0_9: const #21s : i
        let s_0_9: i128 = 21;
        // C s_0_10: const #3s : i
        let s_0_10: i128 = 3;
        // D s_0_11: read-var instr:u32
        let s_0_11: u32 = fn_state.instr;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 32u16);
        // D s_0_13: bit-extract s_0_12 s_0_9 s_0_10
        let s_0_13: Bits = (Bits::new(
            ((s_0_12) >> (s_0_9)).value(),
            u16::try_from(s_0_10).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u8
        let s_0_14: u8 = (s_0_13.value() as u8);
        // D s_0_15: write-var opc1 <= s_0_14
        fn_state.opc1 = s_0_14;
        // C s_0_16: const #16s : i
        let s_0_16: i128 = 16;
        // C s_0_17: const #4s : i
        let s_0_17: i128 = 4;
        // D s_0_18: read-var instr:u32
        let s_0_18: u32 = fn_state.instr;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 32u16);
        // D s_0_20: bit-extract s_0_19 s_0_16 s_0_17
        let s_0_20: Bits = (Bits::new(
            ((s_0_19) >> (s_0_16)).value(),
            u16::try_from(s_0_17).unwrap(),
        ));
        // D s_0_21: cast reint s_0_20 -> u8
        let s_0_21: u8 = (s_0_20.value() as u8);
        // D s_0_22: write-var CRn <= s_0_21
        fn_state.CRn = s_0_21;
        // C s_0_23: const #0s : i
        let s_0_23: i128 = 0;
        // C s_0_24: const #4s : i
        let s_0_24: i128 = 4;
        // D s_0_25: read-var instr:u32
        let s_0_25: u32 = fn_state.instr;
        // D s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 32u16);
        // D s_0_27: bit-extract s_0_26 s_0_23 s_0_24
        let s_0_27: Bits = (Bits::new(
            ((s_0_26) >> (s_0_23)).value(),
            u16::try_from(s_0_24).unwrap(),
        ));
        // D s_0_28: cast reint s_0_27 -> u8
        let s_0_28: u8 = (s_0_27.value() as u8);
        // D s_0_29: write-var CRm <= s_0_28
        fn_state.CRm = s_0_28;
        // C s_0_30: const #5s : i
        let s_0_30: i128 = 5;
        // C s_0_31: const #3s : i
        let s_0_31: i128 = 3;
        // D s_0_32: read-var instr:u32
        let s_0_32: u32 = fn_state.instr;
        // D s_0_33: cast zx s_0_32 -> bv
        let s_0_33: Bits = Bits::new(s_0_32 as u128, 32u16);
        // D s_0_34: bit-extract s_0_33 s_0_30 s_0_31
        let s_0_34: Bits = (Bits::new(
            ((s_0_33) >> (s_0_30)).value(),
            u16::try_from(s_0_31).unwrap(),
        ));
        // D s_0_35: cast reint s_0_34 -> u8
        let s_0_35: u8 = (s_0_34.value() as u8);
        // D s_0_36: write-var opc2 <= s_0_35
        fn_state.opc2 = s_0_35;
        // C s_0_37: const #14s : i
        let s_0_37: i128 = 14;
        // D s_0_38: read-var cp_num:i
        let s_0_38: i128 = fn_state.cp_num;
        // D s_0_39: cmp-eq s_0_38 s_0_37
        let s_0_39: bool = ((s_0_38) == (s_0_37));
        // N s_0_40: branch s_0_39 b452 b1
        if s_0_39 {
            return block_452(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#142267 <= s_1_0
        fn_state.gs_142267 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#142267:u8
        let s_2_0: bool = fn_state.gs_142267;
        // N s_2_1: branch s_2_0 b451 b3
        if s_2_0 {
            return block_451(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#142268 <= s_3_0
        fn_state.gs_142268 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#142268:u8
        let s_4_0: bool = fn_state.gs_142268;
        // N s_4_1: branch s_4_0 b450 b5
        if s_4_0 {
            return block_450(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#142269 <= s_5_0
        fn_state.gs_142269 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#142269:u8
        let s_6_0: bool = fn_state.gs_142269;
        // N s_6_1: branch s_6_0 b449 b7
        if s_6_0 {
            return block_449(state, tracer, fn_state);
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
        // C s_8_0: const #15s : i
        let s_8_0: i128 = 15;
        // D s_8_1: read-var cp_num:i
        let s_8_1: i128 = fn_state.cp_num;
        // D s_8_2: cmp-eq s_8_1 s_8_0
        let s_8_2: bool = ((s_8_1) == (s_8_0));
        // N s_8_3: branch s_8_2 b448 b9
        if s_8_2 {
            return block_448(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#142271 <= s_9_0
        fn_state.gs_142271 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#142271:u8
        let s_10_0: bool = fn_state.gs_142271;
        // N s_10_1: branch s_10_0 b447 b11
        if s_10_0 {
            return block_447(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#142272 <= s_11_0
        fn_state.gs_142272 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#142272:u8
        let s_12_0: bool = fn_state.gs_142272;
        // N s_12_1: branch s_12_0 b446 b13
        if s_12_0 {
            return block_446(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#142273 <= s_13_0
        fn_state.gs_142273 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#142273:u8
        let s_14_0: bool = fn_state.gs_142273;
        // N s_14_1: branch s_14_0 b445 b15
        if s_14_0 {
            return block_445(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#142274 <= s_15_0
        fn_state.gs_142274 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#142274:u8
        let s_16_0: bool = fn_state.gs_142274;
        // N s_16_1: branch s_16_0 b432 b17
        if s_16_0 {
            return block_432(state, tracer, fn_state);
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
        // C s_18_0: const #15s : i
        let s_18_0: i128 = 15;
        // D s_18_1: read-var cp_num:i
        let s_18_1: i128 = fn_state.cp_num;
        // D s_18_2: cmp-eq s_18_1 s_18_0
        let s_18_2: bool = ((s_18_1) == (s_18_0));
        // N s_18_3: branch s_18_2 b431 b19
        if s_18_2 {
            return block_431(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#142276 <= s_19_0
        fn_state.gs_142276 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#142276:u8
        let s_20_0: bool = fn_state.gs_142276;
        // N s_20_1: branch s_20_0 b430 b21
        if s_20_0 {
            return block_430(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#142277 <= s_21_0
        fn_state.gs_142277 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#142277:u8
        let s_22_0: bool = fn_state.gs_142277;
        // N s_22_1: branch s_22_0 b426 b23
        if s_22_0 {
            return block_426(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#142279 <= s_23_0
        fn_state.gs_142279 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#142279:u8
        let s_24_0: bool = fn_state.gs_142279;
        // N s_24_1: branch s_24_0 b419 b25
        if s_24_0 {
            return block_419(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#142282 <= s_25_0
        fn_state.gs_142282 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#142282:u8
        let s_26_0: bool = fn_state.gs_142282;
        // N s_26_1: branch s_26_0 b382 b27
        if s_26_0 {
            return block_382(state, tracer, fn_state);
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
        // C s_28_0: const #15s : i
        let s_28_0: i128 = 15;
        // D s_28_1: read-var cp_num:i
        let s_28_1: i128 = fn_state.cp_num;
        // D s_28_2: cmp-eq s_28_1 s_28_0
        let s_28_2: bool = ((s_28_1) == (s_28_0));
        // N s_28_3: branch s_28_2 b381 b29
        if s_28_2 {
            return block_381(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#142284 <= s_29_0
        fn_state.gs_142284 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#142284:u8
        let s_30_0: bool = fn_state.gs_142284;
        // N s_30_1: branch s_30_0 b380 b31
        if s_30_0 {
            return block_380(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#142285 <= s_31_0
        fn_state.gs_142285 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#142285:u8
        let s_32_0: bool = fn_state.gs_142285;
        // N s_32_1: branch s_32_0 b379 b33
        if s_32_0 {
            return block_379(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#142286 <= s_33_0
        fn_state.gs_142286 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#142286:u8
        let s_34_0: bool = fn_state.gs_142286;
        // N s_34_1: branch s_34_0 b378 b35
        if s_34_0 {
            return block_378(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#142287 <= s_35_0
        fn_state.gs_142287 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#142287:u8
        let s_36_0: bool = fn_state.gs_142287;
        // N s_36_1: branch s_36_0 b371 b37
        if s_36_0 {
            return block_371(state, tracer, fn_state);
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
        // C s_38_0: const #15s : i
        let s_38_0: i128 = 15;
        // D s_38_1: read-var cp_num:i
        let s_38_1: i128 = fn_state.cp_num;
        // D s_38_2: cmp-eq s_38_1 s_38_0
        let s_38_2: bool = ((s_38_1) == (s_38_0));
        // N s_38_3: branch s_38_2 b370 b39
        if s_38_2 {
            return block_370(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#142289 <= s_39_0
        fn_state.gs_142289 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#142289:u8
        let s_40_0: bool = fn_state.gs_142289;
        // N s_40_1: branch s_40_0 b369 b41
        if s_40_0 {
            return block_369(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#142290 <= s_41_0
        fn_state.gs_142290 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#142290:u8
        let s_42_0: bool = fn_state.gs_142290;
        // N s_42_1: branch s_42_0 b362 b43
        if s_42_0 {
            return block_362(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#142301 <= s_43_0
        fn_state.gs_142301 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#142301:u8
        let s_44_0: bool = fn_state.gs_142301;
        // N s_44_1: branch s_44_0 b336 b45
        if s_44_0 {
            return block_336(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #15s : i
        let s_46_0: i128 = 15;
        // D s_46_1: read-var cp_num:i
        let s_46_1: i128 = fn_state.cp_num;
        // D s_46_2: cmp-eq s_46_1 s_46_0
        let s_46_2: bool = ((s_46_1) == (s_46_0));
        // N s_46_3: branch s_46_2 b335 b47
        if s_46_2 {
            return block_335(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#142303 <= s_47_0
        fn_state.gs_142303 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#142303:u8
        let s_48_0: bool = fn_state.gs_142303;
        // N s_48_1: branch s_48_0 b334 b49
        if s_48_0 {
            return block_334(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#142304 <= s_49_0
        fn_state.gs_142304 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#142304:u8
        let s_50_0: bool = fn_state.gs_142304;
        // N s_50_1: branch s_50_0 b333 b51
        if s_50_0 {
            return block_333(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#142305 <= s_51_0
        fn_state.gs_142305 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#142305:u8
        let s_52_0: bool = fn_state.gs_142305;
        // N s_52_1: branch s_52_0 b326 b53
        if s_52_0 {
            return block_326(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#142308 <= s_53_0
        fn_state.gs_142308 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#142308:u8
        let s_54_0: bool = fn_state.gs_142308;
        // N s_54_1: branch s_54_0 b300 b55
        if s_54_0 {
            return block_300(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #14s : i
        let s_56_0: i128 = 14;
        // D s_56_1: read-var cp_num:i
        let s_56_1: i128 = fn_state.cp_num;
        // D s_56_2: cmp-eq s_56_1 s_56_0
        let s_56_2: bool = ((s_56_1) == (s_56_0));
        // N s_56_3: branch s_56_2 b299 b57
        if s_56_2 {
            return block_299(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#142310 <= s_57_0
        fn_state.gs_142310 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#142310:u8
        let s_58_0: bool = fn_state.gs_142310;
        // N s_58_1: branch s_58_0 b298 b59
        if s_58_0 {
            return block_298(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#142311 <= s_59_0
        fn_state.gs_142311 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#142311:u8
        let s_60_0: bool = fn_state.gs_142311;
        // N s_60_1: branch s_60_0 b297 b61
        if s_60_0 {
            return block_297(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#142312 <= s_61_0
        fn_state.gs_142312 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#142312:u8
        let s_62_0: bool = fn_state.gs_142312;
        // N s_62_1: branch s_62_0 b290 b63
        if s_62_0 {
            return block_290(state, tracer, fn_state);
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
        // D s_64_0: read-var t:i
        let s_64_0: i128 = fn_state.t;
        // D s_64_1: call R_read(s_64_0)
        let s_64_1: u32 = R_read(state, tracer, s_64_0);
        // D s_64_2: write-var temprt2 <= s_64_1
        fn_state.temprt2 = s_64_1;
        // D s_64_3: read-var t:i
        let s_64_3: i128 = fn_state.t;
        // D s_64_4: read-var temprt:u32
        let s_64_4: u32 = fn_state.temprt;
        // D s_64_5: call R_set(s_64_3, s_64_4)
        let s_64_5: () = R_set(state, tracer, s_64_3, s_64_4);
        // C s_64_6: const #3s : i
        let s_64_6: i128 = 3;
        // C s_64_7: const #0s : i
        let s_64_7: i128 = 0;
        // D s_64_8: read-var cp_num:i
        let s_64_8: i128 = fn_state.cp_num;
        // D s_64_9: call integer_subrange(s_64_8, s_64_6, s_64_7)
        let s_64_9: Bits = integer_subrange(state, tracer, s_64_8, s_64_6, s_64_7);
        // D s_64_10: cast reint s_64_9 -> u8
        let s_64_10: u8 = (s_64_9.value() as u8);
        // D s_64_11: read-var el:u8
        let s_64_11: u8 = fn_state.el;
        // D s_64_12: read-var opc1:u8
        let s_64_12: u8 = fn_state.opc1;
        // D s_64_13: read-var CRn:u8
        let s_64_13: u8 = fn_state.CRn;
        // D s_64_14: read-var opc2:u8
        let s_64_14: u8 = fn_state.opc2;
        // D s_64_15: read-var CRm:u8
        let s_64_15: u8 = fn_state.CRm;
        // D s_64_16: read-var t:i
        let s_64_16: i128 = fn_state.t;
        // D s_64_17: call AArch32_AutoGen_SysRegWrite32(s_64_11, s_64_10, s_64_12, s_64_13, s_64_14, s_64_15, s_64_16)
        let s_64_17: () = AArch32_AutoGen_SysRegWrite32(
            state,
            tracer,
            s_64_11,
            s_64_10,
            s_64_12,
            s_64_13,
            s_64_14,
            s_64_15,
            s_64_16,
        );
        // N s_64_18: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var t:i
        let s_65_0: i128 = fn_state.t;
        // D s_65_1: read-var temprt2:u32
        let s_65_1: u32 = fn_state.temprt2;
        // D s_65_2: call R_set(s_65_0, s_65_1)
        let s_65_2: () = R_set(state, tracer, s_65_0, s_65_1);
        // C s_65_3: const #14s : i
        let s_65_3: i128 = 14;
        // D s_65_4: read-var cp_num:i
        let s_65_4: i128 = fn_state.cp_num;
        // D s_65_5: cmp-eq s_65_4 s_65_3
        let s_65_5: bool = ((s_65_4) == (s_65_3));
        // N s_65_6: branch s_65_5 b289 b66
        if s_65_5 {
            return block_289(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#142408 <= s_66_0
        fn_state.gs_142408 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#142408:u8
        let s_67_0: bool = fn_state.gs_142408;
        // N s_67_1: branch s_67_0 b288 b68
        if s_67_0 {
            return block_288(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#142409 <= s_68_0
        fn_state.gs_142409 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#142409:u8
        let s_69_0: bool = fn_state.gs_142409;
        // N s_69_1: branch s_69_0 b287 b70
        if s_69_0 {
            return block_287(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#142410 <= s_70_0
        fn_state.gs_142410 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#142410:u8
        let s_71_0: bool = fn_state.gs_142410;
        // N s_71_1: branch s_71_0 b286 b72
        if s_71_0 {
            return block_286(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#142411 <= s_72_0
        fn_state.gs_142411 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#142411:u8
        let s_73_0: bool = fn_state.gs_142411;
        // N s_73_1: branch s_73_0 b285 b74
        if s_73_0 {
            return block_285(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #14s : i
        let s_75_0: i128 = 14;
        // D s_75_1: read-var cp_num:i
        let s_75_1: i128 = fn_state.cp_num;
        // D s_75_2: cmp-eq s_75_1 s_75_0
        let s_75_2: bool = ((s_75_1) == (s_75_0));
        // N s_75_3: branch s_75_2 b284 b76
        if s_75_2 {
            return block_284(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#142413 <= s_76_0
        fn_state.gs_142413 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#142413:u8
        let s_77_0: bool = fn_state.gs_142413;
        // N s_77_1: branch s_77_0 b283 b78
        if s_77_0 {
            return block_283(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#142414 <= s_78_0
        fn_state.gs_142414 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#142414:u8
        let s_79_0: bool = fn_state.gs_142414;
        // N s_79_1: branch s_79_0 b282 b80
        if s_79_0 {
            return block_282(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#142415 <= s_80_0
        fn_state.gs_142415 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#142415:u8
        let s_81_0: bool = fn_state.gs_142415;
        // N s_81_1: branch s_81_0 b281 b82
        if s_81_0 {
            return block_281(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#142416 <= s_82_0
        fn_state.gs_142416 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#142416:u8
        let s_83_0: bool = fn_state.gs_142416;
        // N s_83_1: branch s_83_0 b275 b84
        if s_83_0 {
            return block_275(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #15s : i
        let s_85_0: i128 = 15;
        // D s_85_1: read-var cp_num:i
        let s_85_1: i128 = fn_state.cp_num;
        // D s_85_2: cmp-eq s_85_1 s_85_0
        let s_85_2: bool = ((s_85_1) == (s_85_0));
        // N s_85_3: branch s_85_2 b274 b86
        if s_85_2 {
            return block_274(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#142418 <= s_86_0
        fn_state.gs_142418 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#142418:u8
        let s_87_0: bool = fn_state.gs_142418;
        // N s_87_1: branch s_87_0 b273 b88
        if s_87_0 {
            return block_273(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#142419 <= s_88_0
        fn_state.gs_142419 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#142419:u8
        let s_89_0: bool = fn_state.gs_142419;
        // N s_89_1: branch s_89_0 b269 b90
        if s_89_0 {
            return block_269(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#142421 <= s_90_0
        fn_state.gs_142421 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#142421:u8
        let s_91_0: bool = fn_state.gs_142421;
        // N s_91_1: branch s_91_0 b262 b92
        if s_91_0 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#142424 <= s_92_0
        fn_state.gs_142424 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#142424:u8
        let s_93_0: bool = fn_state.gs_142424;
        // N s_93_1: branch s_93_0 b225 b94
        if s_93_0 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_94_0: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #15s : i
        let s_95_0: i128 = 15;
        // D s_95_1: read-var cp_num:i
        let s_95_1: i128 = fn_state.cp_num;
        // D s_95_2: cmp-eq s_95_1 s_95_0
        let s_95_2: bool = ((s_95_1) == (s_95_0));
        // N s_95_3: branch s_95_2 b224 b96
        if s_95_2 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#142426 <= s_96_0
        fn_state.gs_142426 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#142426:u8
        let s_97_0: bool = fn_state.gs_142426;
        // N s_97_1: branch s_97_0 b223 b98
        if s_97_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#142427 <= s_98_0
        fn_state.gs_142427 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#142427:u8
        let s_99_0: bool = fn_state.gs_142427;
        // N s_99_1: branch s_99_0 b222 b100
        if s_99_0 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#142428 <= s_100_0
        fn_state.gs_142428 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#142428:u8
        let s_101_0: bool = fn_state.gs_142428;
        // N s_101_1: branch s_101_0 b221 b102
        if s_101_0 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#142429 <= s_102_0
        fn_state.gs_142429 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#142429:u8
        let s_103_0: bool = fn_state.gs_142429;
        // N s_103_1: branch s_103_0 b214 b104
        if s_103_0 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_104_0: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #15s : i
        let s_105_0: i128 = 15;
        // D s_105_1: read-var cp_num:i
        let s_105_1: i128 = fn_state.cp_num;
        // D s_105_2: cmp-eq s_105_1 s_105_0
        let s_105_2: bool = ((s_105_1) == (s_105_0));
        // N s_105_3: branch s_105_2 b213 b106
        if s_105_2 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#142431 <= s_106_0
        fn_state.gs_142431 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#142431:u8
        let s_107_0: bool = fn_state.gs_142431;
        // N s_107_1: branch s_107_0 b212 b108
        if s_107_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#142432 <= s_108_0
        fn_state.gs_142432 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#142432:u8
        let s_109_0: bool = fn_state.gs_142432;
        // N s_109_1: branch s_109_0 b211 b110
        if s_109_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#142433 <= s_110_0
        fn_state.gs_142433 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#142433:u8
        let s_111_0: bool = fn_state.gs_142433;
        // N s_111_1: branch s_111_0 b207 b112
        if s_111_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#142435 <= s_112_0
        fn_state.gs_142435 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#142435:u8
        let s_113_0: bool = fn_state.gs_142435;
        // N s_113_1: branch s_113_0 b198 b114
        if s_113_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_114_0: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #15s : i
        let s_115_0: i128 = 15;
        // D s_115_1: read-var cp_num:i
        let s_115_1: i128 = fn_state.cp_num;
        // D s_115_2: cmp-eq s_115_1 s_115_0
        let s_115_2: bool = ((s_115_1) == (s_115_0));
        // N s_115_3: branch s_115_2 b197 b116
        if s_115_2 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#142437 <= s_116_0
        fn_state.gs_142437 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#142437:u8
        let s_117_0: bool = fn_state.gs_142437;
        // N s_117_1: branch s_117_0 b196 b118
        if s_117_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#142438 <= s_118_0
        fn_state.gs_142438 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#142438:u8
        let s_119_0: bool = fn_state.gs_142438;
        // N s_119_1: branch s_119_0 b195 b120
        if s_119_0 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#142441 <= s_120_0
        fn_state.gs_142441 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#142441:u8
        let s_121_0: bool = fn_state.gs_142441;
        // N s_121_1: branch s_121_0 b191 b122
        if s_121_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_122_0: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #15s : i
        let s_123_0: i128 = 15;
        // D s_123_1: read-var cp_num:i
        let s_123_1: i128 = fn_state.cp_num;
        // D s_123_2: cmp-eq s_123_1 s_123_0
        let s_123_2: bool = ((s_123_1) == (s_123_0));
        // N s_123_3: branch s_123_2 b190 b124
        if s_123_2 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#142443 <= s_124_0
        fn_state.gs_142443 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#142443:u8
        let s_125_0: bool = fn_state.gs_142443;
        // N s_125_1: branch s_125_0 b189 b126
        if s_125_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#142444 <= s_126_0
        fn_state.gs_142444 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#142444:u8
        let s_127_0: bool = fn_state.gs_142444;
        // N s_127_1: branch s_127_0 b188 b128
        if s_127_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#142445 <= s_128_0
        fn_state.gs_142445 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#142445:u8
        let s_129_0: bool = fn_state.gs_142445;
        // N s_129_1: branch s_129_0 b187 b130
        if s_129_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#142446 <= s_130_0
        fn_state.gs_142446 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#142446:u8
        let s_131_0: bool = fn_state.gs_142446;
        // N s_131_1: branch s_131_0 b186 b132
        if s_131_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_132_0: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #15s : i
        let s_133_0: i128 = 15;
        // D s_133_1: read-var cp_num:i
        let s_133_1: i128 = fn_state.cp_num;
        // D s_133_2: cmp-eq s_133_1 s_133_0
        let s_133_2: bool = ((s_133_1) == (s_133_0));
        // N s_133_3: branch s_133_2 b182 b134
        if s_133_2 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#142449 <= s_134_0
        fn_state.gs_142449 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#142449:u8
        let s_135_0: bool = fn_state.gs_142449;
        // N s_135_1: branch s_135_0 b181 b136
        if s_135_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#142450 <= s_136_0
        fn_state.gs_142450 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#142450:u8
        let s_137_0: bool = fn_state.gs_142450;
        // N s_137_1: branch s_137_0 b180 b138
        if s_137_0 {
            return block_180(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#142451 <= s_138_0
        fn_state.gs_142451 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#142451:u8
        let s_139_0: bool = fn_state.gs_142451;
        // N s_139_1: branch s_139_0 b179 b140
        if s_139_0 {
            return block_179(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#142452 <= s_140_0
        fn_state.gs_142452 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#142452:u8
        let s_141_0: bool = fn_state.gs_142452;
        // N s_141_1: branch s_141_0 b178 b142
        if s_141_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#142455 <= s_142_0
        fn_state.gs_142455 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#142455:u8
        let s_143_0: bool = fn_state.gs_142455;
        // N s_143_1: branch s_143_0 b177 b144
        if s_143_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #15s : i
        let s_144_0: i128 = 15;
        // D s_144_1: read-var cp_num:i
        let s_144_1: i128 = fn_state.cp_num;
        // D s_144_2: cmp-eq s_144_1 s_144_0
        let s_144_2: bool = ((s_144_1) == (s_144_0));
        // N s_144_3: branch s_144_2 b176 b145
        if s_144_2 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0u : u8
        let s_145_0: bool = false;
        // D s_145_1: write-var gs#142470 <= s_145_0
        fn_state.gs_142470 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#142470:u8
        let s_146_0: bool = fn_state.gs_142470;
        // N s_146_1: branch s_146_0 b172 b147
        if s_146_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #0u : u8
        let s_147_0: bool = false;
        // D s_147_1: write-var gs#142472 <= s_147_0
        fn_state.gs_142472 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#142472:u8
        let s_148_0: bool = fn_state.gs_142472;
        // N s_148_1: branch s_148_0 b171 b149
        if s_148_0 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #0u : u8
        let s_149_0: bool = false;
        // D s_149_1: write-var gs#142473 <= s_149_0
        fn_state.gs_142473 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#142473:u8
        let s_150_0: bool = fn_state.gs_142473;
        // N s_150_1: branch s_150_0 b170 b151
        if s_150_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#142474 <= s_151_0
        fn_state.gs_142474 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#142474:u8
        let s_152_0: bool = fn_state.gs_142474;
        // N s_152_1: branch s_152_0 b169 b153
        if s_152_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_153_0: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #14s : i
        let s_154_0: i128 = 14;
        // D s_154_1: read-var cp_num:i
        let s_154_1: i128 = fn_state.cp_num;
        // D s_154_2: cmp-eq s_154_1 s_154_0
        let s_154_2: bool = ((s_154_1) == (s_154_0));
        // N s_154_3: branch s_154_2 b168 b155
        if s_154_2 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #0u : u8
        let s_155_0: bool = false;
        // D s_155_1: write-var gs#142457 <= s_155_0
        fn_state.gs_142457 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#142457:u8
        let s_156_0: bool = fn_state.gs_142457;
        // N s_156_1: branch s_156_0 b167 b157
        if s_156_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #0u : u8
        let s_157_0: bool = false;
        // D s_157_1: write-var gs#142458 <= s_157_0
        fn_state.gs_142458 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#142458:u8
        let s_158_0: bool = fn_state.gs_142458;
        // N s_158_1: branch s_158_0 b166 b159
        if s_158_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var gs#142459 <= s_159_0
        fn_state.gs_142459 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#142459:u8
        let s_160_0: bool = fn_state.gs_142459;
        // N s_160_1: branch s_160_0 b165 b161
        if s_160_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#142460 <= s_161_0
        fn_state.gs_142460 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#142460:u8
        let s_162_0: bool = fn_state.gs_142460;
        // N s_162_1: branch s_162_0 b164 b163
        if s_162_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_163_0: return
        return;
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #() : ()
        let s_164_0: () = ();
        // S s_164_1: call DBGDSCRint_read(s_164_0)
        let s_164_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_164_0,
        );
        // C s_164_2: const #1u : u8
        let s_164_2: bool = true;
        // S s_164_3: call _update_DBGDSCRint_Type_TXfull(s_164_1, s_164_2)
        let s_164_3: ProductType700c18a878c5601b = u_update_DBGDSCRint_Type_TXfull(
            state,
            tracer,
            s_164_1,
            s_164_2,
        );
        // S s_164_4: call DBGDSCRint_write(s_164_3)
        let s_164_4: () = DBGDSCRint_write(state, tracer, s_164_3);
        // C s_164_5: const #16832u : u32
        let s_164_5: u32 = 16832;
        // D s_164_6: read-reg s_164_5:struct
        let s_164_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_164_5 as isize);
            tracer.read_register(s_164_5 as isize, value);
            value
        };
        // C s_164_7: const #16832u : u32
        let s_164_7: u32 = 16832;
        // N s_164_8: write-reg s_164_7 <= s_164_6
        let s_164_8: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_164_7 as isize, s_164_6);
            tracer.write_register(s_164_7 as isize, s_164_6);
        };
        // C s_164_9: const #() : ()
        let s_164_9: () = ();
        // S s_164_10: call DBGDSCRext_read(s_164_9)
        let s_164_10: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_164_9,
        );
        // C s_164_11: const #1u : u8
        let s_164_11: bool = true;
        // S s_164_12: call _update_DBGDSCRext_Type_TXfull(s_164_10, s_164_11)
        let s_164_12: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_TXfull(
            state,
            tracer,
            s_164_10,
            s_164_11,
        );
        // S s_164_13: call DBGDSCRext_write(s_164_12)
        let s_164_13: () = DBGDSCRext_write(state, tracer, s_164_12);
        // C s_164_14: const #() : ()
        let s_164_14: () = ();
        // S s_164_15: call EDSCR_read(s_164_14)
        let s_164_15: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_164_14);
        // C s_164_16: const #1u : u8
        let s_164_16: bool = true;
        // S s_164_17: call _update_EDSCR_Type_TXfull(s_164_15, s_164_16)
        let s_164_17: ProductType700c18a878c5601b = u_update_EDSCR_Type_TXfull(
            state,
            tracer,
            s_164_15,
            s_164_16,
        );
        // S s_164_18: call EDSCR_write(s_164_17)
        let s_164_18: () = EDSCR_write(state, tracer, s_164_17);
        // N s_164_19: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var CRm:u8
        let s_165_0: u8 = fn_state.CRm;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 4u16);
        // C s_165_2: const #5u : u8
        let s_165_2: u8 = 5;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 4u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#142460 <= s_165_4
        fn_state.gs_142460 = s_165_4;
        // N s_165_6: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var opc2:u8
        let s_166_0: u8 = fn_state.opc2;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 3u16);
        // C s_166_2: const #0u : u8
        let s_166_2: u8 = 0;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 3u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#142459 <= s_166_4
        fn_state.gs_142459 = s_166_4;
        // N s_166_6: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var CRn:u8
        let s_167_0: u8 = fn_state.CRn;
        // D s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 4u16);
        // C s_167_2: const #0u : u8
        let s_167_2: u8 = 0;
        // C s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 4u16);
        // D s_167_4: cmp-eq s_167_1 s_167_3
        let s_167_4: bool = ((s_167_1) == (s_167_3));
        // D s_167_5: write-var gs#142458 <= s_167_4
        fn_state.gs_142458 = s_167_4;
        // N s_167_6: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var opc1:u8
        let s_168_0: u8 = fn_state.opc1;
        // D s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 3u16);
        // C s_168_2: const #0u : u8
        let s_168_2: u8 = 0;
        // C s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 3u16);
        // D s_168_4: cmp-eq s_168_1 s_168_3
        let s_168_4: bool = ((s_168_1) == (s_168_3));
        // D s_168_5: write-var gs#142457 <= s_168_4
        fn_state.gs_142457 = s_168_4;
        // N s_168_6: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var temprt:u32
        let s_169_0: u32 = fn_state.temprt;
        // D s_169_1: call Mk_MIDR_Type(s_169_0)
        let s_169_1: ProductType700c18a878c5601b = Mk_MIDR_Type(state, tracer, s_169_0);
        // D s_169_2: call MIDR_write(s_169_1)
        let s_169_2: () = MIDR_write(state, tracer, s_169_1);
        // N s_169_3: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var CRm:u8
        let s_170_0: u8 = fn_state.CRm;
        // D s_170_1: cast zx s_170_0 -> bv
        let s_170_1: Bits = Bits::new(s_170_0 as u128, 4u16);
        // C s_170_2: const #0u : u8
        let s_170_2: u8 = 0;
        // C s_170_3: cast zx s_170_2 -> bv
        let s_170_3: Bits = Bits::new(s_170_2 as u128, 4u16);
        // D s_170_4: cmp-eq s_170_1 s_170_3
        let s_170_4: bool = ((s_170_1) == (s_170_3));
        // D s_170_5: write-var gs#142474 <= s_170_4
        fn_state.gs_142474 = s_170_4;
        // N s_170_6: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var CRn:u8
        let s_171_0: u8 = fn_state.CRn;
        // D s_171_1: cast zx s_171_0 -> bv
        let s_171_1: Bits = Bits::new(s_171_0 as u128, 4u16);
        // C s_171_2: const #0u : u8
        let s_171_2: u8 = 0;
        // C s_171_3: cast zx s_171_2 -> bv
        let s_171_3: Bits = Bits::new(s_171_2 as u128, 4u16);
        // D s_171_4: cmp-eq s_171_1 s_171_3
        let s_171_4: bool = ((s_171_1) == (s_171_3));
        // D s_171_5: write-var gs#142473 <= s_171_4
        fn_state.gs_142473 = s_171_4;
        // N s_171_6: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var opc2:u8
        let s_172_0: u8 = fn_state.opc2;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 3u16);
        // C s_172_2: const #4u : u8
        let s_172_2: u8 = 4;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 3u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // N s_172_5: branch s_172_4 b175 b173
        if s_172_4 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var opc2:u8
        let s_173_0: u8 = fn_state.opc2;
        // D s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 3u16);
        // C s_173_2: const #7u : u8
        let s_173_2: u8 = 7;
        // C s_173_3: cast zx s_173_2 -> bv
        let s_173_3: Bits = Bits::new(s_173_2 as u128, 3u16);
        // D s_173_4: cmp-eq s_173_1 s_173_3
        let s_173_4: bool = ((s_173_1) == (s_173_3));
        // D s_173_5: write-var gs#142471 <= s_173_4
        fn_state.gs_142471 = s_173_4;
        // N s_173_6: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#142471:u8
        let s_174_0: bool = fn_state.gs_142471;
        // D s_174_1: write-var gs#142472 <= s_174_0
        fn_state.gs_142472 = s_174_0;
        // N s_174_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #1u : u8
        let s_175_0: bool = true;
        // D s_175_1: write-var gs#142471 <= s_175_0
        fn_state.gs_142471 = s_175_0;
        // N s_175_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var opc1:u8
        let s_176_0: u8 = fn_state.opc1;
        // D s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 3u16);
        // C s_176_2: const #0u : u8
        let s_176_2: u8 = 0;
        // C s_176_3: cast zx s_176_2 -> bv
        let s_176_3: Bits = Bits::new(s_176_2 as u128, 3u16);
        // D s_176_4: cmp-eq s_176_1 s_176_3
        let s_176_4: bool = ((s_176_1) == (s_176_3));
        // D s_176_5: write-var gs#142470 <= s_176_4
        fn_state.gs_142470 = s_176_4;
        // N s_176_6: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #0u : u8
        let s_177_0: bool = false;
        // S s_177_1: call TakeReset(s_177_0)
        let s_177_1: () = TakeReset(state, tracer, s_177_0);
        // N s_177_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #1s : i
        let s_178_0: i128 = 1;
        // D s_178_1: read-var temprt:u32
        let s_178_1: u32 = fn_state.temprt;
        // D s_178_2: cast zx s_178_1 -> bv
        let s_178_2: Bits = Bits::new(s_178_1 as u128, 32u16);
        // C s_178_3: const #1u : u64
        let s_178_3: u64 = 1;
        // D s_178_4: bit-extract s_178_2 s_178_0 s_178_3
        let s_178_4: Bits = (Bits::new(
            ((s_178_2) >> (s_178_0)).value(),
            u16::try_from(s_178_3).unwrap(),
        ));
        // D s_178_5: cast reint s_178_4 -> u8
        let s_178_5: bool = ((s_178_4.value()) != 0);
        // C s_178_6: const #0s : i
        let s_178_6: i128 = 0;
        // C s_178_7: const #0u : u64
        let s_178_7: u64 = 0;
        // D s_178_8: cast zx s_178_5 -> u64
        let s_178_8: u64 = (s_178_5 as u64);
        // C s_178_9: const #1u : u64
        let s_178_9: u64 = 1;
        // D s_178_10: and s_178_8 s_178_9
        let s_178_10: u64 = ((s_178_8) & (s_178_9));
        // D s_178_11: cmp-eq s_178_10 s_178_9
        let s_178_11: bool = ((s_178_10) == (s_178_9));
        // D s_178_12: lsl s_178_8 s_178_6
        let s_178_12: u64 = s_178_8 << s_178_6;
        // D s_178_13: or s_178_7 s_178_12
        let s_178_13: u64 = ((s_178_7) | (s_178_12));
        // D s_178_14: cmpl s_178_12
        let s_178_14: u64 = !s_178_12;
        // D s_178_15: and s_178_7 s_178_14
        let s_178_15: u64 = ((s_178_7) & (s_178_14));
        // D s_178_16: select s_178_11 s_178_13 s_178_15
        let s_178_16: u64 = if s_178_11 { s_178_13 } else { s_178_15 };
        // D s_178_17: cast trunc s_178_16 -> u8
        let s_178_17: bool = ((s_178_16) != 0);
        // D s_178_18: cast zx s_178_17 -> bv
        let s_178_18: Bits = Bits::new(s_178_17 as u128, 1u16);
        // C s_178_19: const #1u : u8
        let s_178_19: bool = true;
        // C s_178_20: cast zx s_178_19 -> bv
        let s_178_20: Bits = Bits::new(s_178_19 as u128, 1u16);
        // D s_178_21: cmp-eq s_178_18 s_178_20
        let s_178_21: bool = ((s_178_18) == (s_178_20));
        // D s_178_22: write-var gs#142455 <= s_178_21
        fn_state.gs_142455 = s_178_21;
        // N s_178_23: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var CRm:u8
        let s_179_0: u8 = fn_state.CRm;
        // D s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 4u16);
        // C s_179_2: const #0u : u8
        let s_179_2: u8 = 0;
        // C s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 4u16);
        // D s_179_4: cmp-eq s_179_1 s_179_3
        let s_179_4: bool = ((s_179_1) == (s_179_3));
        // D s_179_5: write-var gs#142452 <= s_179_4
        fn_state.gs_142452 = s_179_4;
        // N s_179_6: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var opc2:u8
        let s_180_0: u8 = fn_state.opc2;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 3u16);
        // C s_180_2: const #2u : u8
        let s_180_2: u8 = 2;
        // C s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 3u16);
        // D s_180_4: cmp-eq s_180_1 s_180_3
        let s_180_4: bool = ((s_180_1) == (s_180_3));
        // D s_180_5: write-var gs#142451 <= s_180_4
        fn_state.gs_142451 = s_180_4;
        // N s_180_6: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var CRn:u8
        let s_181_0: u8 = fn_state.CRn;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 4u16);
        // C s_181_2: const #12u : u8
        let s_181_2: u8 = 12;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 4u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#142450 <= s_181_4
        fn_state.gs_142450 = s_181_4;
        // N s_181_6: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var opc1:u8
        let s_182_0: u8 = fn_state.opc1;
        // D s_182_1: cast zx s_182_0 -> bv
        let s_182_1: Bits = Bits::new(s_182_0 as u128, 3u16);
        // C s_182_2: const #0u : u8
        let s_182_2: u8 = 0;
        // C s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 3u16);
        // D s_182_4: cmp-eq s_182_1 s_182_3
        let s_182_4: bool = ((s_182_1) == (s_182_3));
        // N s_182_5: branch s_182_4 b185 b183
        if s_182_4 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var opc1:u8
        let s_183_0: u8 = fn_state.opc1;
        // D s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 3u16);
        // C s_183_2: const #4u : u8
        let s_183_2: u8 = 4;
        // C s_183_3: cast zx s_183_2 -> bv
        let s_183_3: Bits = Bits::new(s_183_2 as u128, 3u16);
        // D s_183_4: cmp-eq s_183_1 s_183_3
        let s_183_4: bool = ((s_183_1) == (s_183_3));
        // D s_183_5: write-var gs#142448 <= s_183_4
        fn_state.gs_142448 = s_183_4;
        // N s_183_6: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#142448:u8
        let s_184_0: bool = fn_state.gs_142448;
        // D s_184_1: write-var gs#142449 <= s_184_0
        fn_state.gs_142449 = s_184_0;
        // N s_184_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #1u : u8
        let s_185_0: bool = true;
        // D s_185_1: write-var gs#142448 <= s_185_0
        fn_state.gs_142448 = s_185_0;
        // N s_185_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var temprt:u32
        let s_186_0: u32 = fn_state.temprt;
        // D s_186_1: call AArch32_PMUSwIncrement(s_186_0)
        let s_186_1: () = AArch32_PMUSwIncrement(state, tracer, s_186_0);
        // N s_186_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_187_0: read-var CRm:u8
        let s_187_0: u8 = fn_state.CRm;
        // D s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 4u16);
        // C s_187_2: const #12u : u8
        let s_187_2: u8 = 12;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 4u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // D s_187_5: write-var gs#142446 <= s_187_4
        fn_state.gs_142446 = s_187_4;
        // N s_187_6: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var CRn:u8
        let s_188_0: u8 = fn_state.CRn;
        // D s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 4u16);
        // C s_188_2: const #9u : u8
        let s_188_2: u8 = 9;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 4u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // D s_188_5: write-var gs#142445 <= s_188_4
        fn_state.gs_142445 = s_188_4;
        // N s_188_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var opc2:u8
        let s_189_0: u8 = fn_state.opc2;
        // D s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 3u16);
        // C s_189_2: const #4u : u8
        let s_189_2: u8 = 4;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 3u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // D s_189_5: write-var gs#142444 <= s_189_4
        fn_state.gs_142444 = s_189_4;
        // N s_189_6: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var opc1:u8
        let s_190_0: u8 = fn_state.opc1;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 3u16);
        // C s_190_2: const #0u : u8
        let s_190_2: u8 = 0;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 3u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#142443 <= s_190_4
        fn_state.gs_142443 = s_190_4;
        // N s_190_6: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #0s : i
        let s_191_0: i128 = 0;
        // D s_191_1: read-var CRm:u8
        let s_191_1: u8 = fn_state.CRm;
        // D s_191_2: cast zx s_191_1 -> bv
        let s_191_2: Bits = Bits::new(s_191_1 as u128, 4u16);
        // C s_191_3: const #1s : i64
        let s_191_3: i64 = 1;
        // C s_191_4: cast zx s_191_3 -> i
        let s_191_4: i128 = (i128::try_from(s_191_3).unwrap());
        // C s_191_5: const #1s : i
        let s_191_5: i128 = 1;
        // C s_191_6: add s_191_5 s_191_4
        let s_191_6: i128 = (s_191_5 + s_191_4);
        // D s_191_7: bit-extract s_191_2 s_191_0 s_191_6
        let s_191_7: Bits = (Bits::new(
            ((s_191_2) >> (s_191_0)).value(),
            u16::try_from(s_191_6).unwrap(),
        ));
        // D s_191_8: cast reint s_191_7 -> u8
        let s_191_8: u8 = (s_191_7.value() as u8);
        // C s_191_9: const #0s : i
        let s_191_9: i128 = 0;
        // D s_191_10: read-var opc2:u8
        let s_191_10: u8 = fn_state.opc2;
        // D s_191_11: cast zx s_191_10 -> bv
        let s_191_11: Bits = Bits::new(s_191_10 as u128, 3u16);
        // C s_191_12: const #1s : i64
        let s_191_12: i64 = 1;
        // C s_191_13: cast zx s_191_12 -> i
        let s_191_13: i128 = (i128::try_from(s_191_12).unwrap());
        // C s_191_14: const #2s : i
        let s_191_14: i128 = 2;
        // C s_191_15: add s_191_14 s_191_13
        let s_191_15: i128 = (s_191_14 + s_191_13);
        // D s_191_16: bit-extract s_191_11 s_191_9 s_191_15
        let s_191_16: Bits = (Bits::new(
            ((s_191_11) >> (s_191_9)).value(),
            u16::try_from(s_191_15).unwrap(),
        ));
        // D s_191_17: cast reint s_191_16 -> u8
        let s_191_17: u8 = (s_191_16.value() as u8);
        // D s_191_18: cast zx s_191_8 -> bv
        let s_191_18: Bits = Bits::new(s_191_8 as u128, 2u16);
        // D s_191_19: cast zx s_191_17 -> bv
        let s_191_19: Bits = Bits::new(s_191_17 as u128, 3u16);
        // D s_191_20: cast reint s_191_18 -> u128
        let s_191_20: u128 = (s_191_18.value() as u128);
        // D s_191_21: size-of s_191_18
        let s_191_21: u16 = s_191_18.length();
        // D s_191_22: cast reint s_191_19 -> u128
        let s_191_22: u128 = (s_191_19.value() as u128);
        // D s_191_23: size-of s_191_19
        let s_191_23: u16 = s_191_19.length();
        // D s_191_24: lsl s_191_20 s_191_23
        let s_191_24: u128 = s_191_20 << s_191_23;
        // D s_191_25: or s_191_24 s_191_22
        let s_191_25: u128 = ((s_191_24) | (s_191_22));
        // D s_191_26: add s_191_21 s_191_23
        let s_191_26: u16 = (s_191_21 + s_191_23);
        // D s_191_27: create-bits s_191_25 s_191_26
        let s_191_27: Bits = Bits::new(s_191_25, s_191_26);
        // D s_191_28: cast reint s_191_27 -> u8
        let s_191_28: u8 = (s_191_27.value() as u8);
        // D s_191_29: cast zx s_191_28 -> bv
        let s_191_29: Bits = Bits::new(s_191_28 as u128, 5u16);
        // D s_191_30: cast zx s_191_29 -> i
        let s_191_30: i128 = (s_191_29.value() as i128);
        // D s_191_31: cast reint s_191_30 -> i64
        let s_191_31: i64 = (s_191_30 as i64);
        // D s_191_32: write-var index <= s_191_31
        fn_state.index = s_191_31;
        // C s_191_33: const #31s : i
        let s_191_33: i128 = 31;
        // D s_191_34: read-var index:i64
        let s_191_34: i64 = fn_state.index;
        // D s_191_35: cast zx s_191_34 -> i
        let s_191_35: i128 = (i128::try_from(s_191_34).unwrap());
        // D s_191_36: cmp-eq s_191_35 s_191_33
        let s_191_36: bool = ((s_191_35) == (s_191_33));
        // N s_191_37: branch s_191_36 b194 b192
        if s_191_36 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var temprt:u32
        let s_192_0: u32 = fn_state.temprt;
        // D s_192_1: call Mk_PMEVTYPER_Type(s_192_0)
        let s_192_1: ProductType700c18a878c5601b = Mk_PMEVTYPER_Type(
            state,
            tracer,
            s_192_0,
        );
        // D s_192_2: call __get_PMEVTYPER(s_192_1)
        let s_192_2: ProductType700c18a878c5601b = u__get_PMEVTYPER(
            state,
            tracer,
            s_192_1,
        );
        // D s_192_3: read-var index:i64
        let s_192_3: i64 = fn_state.index;
        // D s_192_4: call PMEVTYPER_set(s_192_3, s_192_2)
        let s_192_4: () = PMEVTYPER_set(state, tracer, s_192_3, s_192_2);
        // N s_192_5: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_193_0: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var temprt:u32
        let s_194_0: u32 = fn_state.temprt;
        // D s_194_1: call Mk_PMCCFILTR_Type(s_194_0)
        let s_194_1: ProductType700c18a878c5601b = Mk_PMCCFILTR_Type(
            state,
            tracer,
            s_194_0,
        );
        // D s_194_2: call __get_PMCCFILTR(s_194_1)
        let s_194_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_194_1,
        );
        // D s_194_3: call PMCCFILTR_write(s_194_2)
        let s_194_3: () = PMCCFILTR_write(state, tracer, s_194_2);
        // N s_194_4: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #2s : i
        let s_195_0: i128 = 2;
        // D s_195_1: read-var CRm:u8
        let s_195_1: u8 = fn_state.CRm;
        // D s_195_2: cast zx s_195_1 -> bv
        let s_195_2: Bits = Bits::new(s_195_1 as u128, 4u16);
        // C s_195_3: const #1s : i64
        let s_195_3: i64 = 1;
        // C s_195_4: cast zx s_195_3 -> i
        let s_195_4: i128 = (i128::try_from(s_195_3).unwrap());
        // C s_195_5: const #1s : i
        let s_195_5: i128 = 1;
        // C s_195_6: add s_195_5 s_195_4
        let s_195_6: i128 = (s_195_5 + s_195_4);
        // D s_195_7: bit-extract s_195_2 s_195_0 s_195_6
        let s_195_7: Bits = (Bits::new(
            ((s_195_2) >> (s_195_0)).value(),
            u16::try_from(s_195_6).unwrap(),
        ));
        // D s_195_8: cast reint s_195_7 -> u8
        let s_195_8: u8 = (s_195_7.value() as u8);
        // D s_195_9: cast zx s_195_8 -> bv
        let s_195_9: Bits = Bits::new(s_195_8 as u128, 2u16);
        // C s_195_10: const #3u : u8
        let s_195_10: u8 = 3;
        // C s_195_11: cast zx s_195_10 -> bv
        let s_195_11: Bits = Bits::new(s_195_10 as u128, 2u16);
        // D s_195_12: cmp-eq s_195_9 s_195_11
        let s_195_12: bool = ((s_195_9) == (s_195_11));
        // D s_195_13: write-var gs#142441 <= s_195_12
        fn_state.gs_142441 = s_195_12;
        // N s_195_14: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var CRn:u8
        let s_196_0: u8 = fn_state.CRn;
        // D s_196_1: cast zx s_196_0 -> bv
        let s_196_1: Bits = Bits::new(s_196_0 as u128, 4u16);
        // C s_196_2: const #14u : u8
        let s_196_2: u8 = 14;
        // C s_196_3: cast zx s_196_2 -> bv
        let s_196_3: Bits = Bits::new(s_196_2 as u128, 4u16);
        // D s_196_4: cmp-eq s_196_1 s_196_3
        let s_196_4: bool = ((s_196_1) == (s_196_3));
        // D s_196_5: write-var gs#142438 <= s_196_4
        fn_state.gs_142438 = s_196_4;
        // N s_196_6: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var opc1:u8
        let s_197_0: u8 = fn_state.opc1;
        // D s_197_1: cast zx s_197_0 -> bv
        let s_197_1: Bits = Bits::new(s_197_0 as u128, 3u16);
        // C s_197_2: const #0u : u8
        let s_197_2: u8 = 0;
        // C s_197_3: cast zx s_197_2 -> bv
        let s_197_3: Bits = Bits::new(s_197_2 as u128, 3u16);
        // D s_197_4: cmp-eq s_197_1 s_197_3
        let s_197_4: bool = ((s_197_1) == (s_197_3));
        // D s_197_5: write-var gs#142437 <= s_197_4
        fn_state.gs_142437 = s_197_4;
        // N s_197_6: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call PMSELR_read(s_198_0)
        let s_198_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_198_0);
        // S s_198_2: call _get_PMSELR_Type_SEL(s_198_1)
        let s_198_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_198_1);
        // S s_198_3: cast zx s_198_2 -> bv
        let s_198_3: Bits = Bits::new(s_198_2 as u128, 5u16);
        // S s_198_4: cast zx s_198_3 -> i
        let s_198_4: i128 = (s_198_3.value() as i128);
        // S s_198_5: cast reint s_198_4 -> i64
        let s_198_5: i64 = (s_198_4 as i64);
        // D s_198_6: write-var pmselr <= s_198_5
        fn_state.pmselr = s_198_5;
        // D s_198_7: read-var opc2:u8
        let s_198_7: u8 = fn_state.opc2;
        // D s_198_8: cast zx s_198_7 -> bv
        let s_198_8: Bits = Bits::new(s_198_7 as u128, 3u16);
        // C s_198_9: const #1u : u8
        let s_198_9: u8 = 1;
        // C s_198_10: cast zx s_198_9 -> bv
        let s_198_10: Bits = Bits::new(s_198_9 as u128, 3u16);
        // D s_198_11: cmp-eq s_198_8 s_198_10
        let s_198_11: bool = ((s_198_8) == (s_198_10));
        // N s_198_12: branch s_198_11 b204 b199
        if s_198_11 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_199_0: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var opc2:u8
        let s_200_0: u8 = fn_state.opc2;
        // D s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 3u16);
        // C s_200_2: const #2u : u8
        let s_200_2: u8 = 2;
        // C s_200_3: cast zx s_200_2 -> bv
        let s_200_3: Bits = Bits::new(s_200_2 as u128, 3u16);
        // D s_200_4: cmp-eq s_200_1 s_200_3
        let s_200_4: bool = ((s_200_1) == (s_200_3));
        // N s_200_5: branch s_200_4 b203 b201
        if s_200_4 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_201_0: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_202_0: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #31s : i
        let s_203_0: i128 = 31;
        // D s_203_1: read-var pmselr:i64
        let s_203_1: i64 = fn_state.pmselr;
        // D s_203_2: cast zx s_203_1 -> i
        let s_203_2: i128 = (i128::try_from(s_203_1).unwrap());
        // D s_203_3: cmp-lt s_203_2 s_203_0
        let s_203_3: bool = ((s_203_2) < (s_203_0));
        // N s_203_4: assert s_203_3
        let s_203_4: () = assert!(s_203_3);
        // D s_203_5: read-var pmselr:i64
        let s_203_5: i64 = fn_state.pmselr;
        // D s_203_6: read-var temprt:u32
        let s_203_6: u32 = fn_state.temprt;
        // D s_203_7: call PMEVCNTR_set(s_203_5, s_203_6)
        let s_203_7: () = PMEVCNTR_set(state, tracer, s_203_5, s_203_6);
        // N s_203_8: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #31s : i
        let s_204_0: i128 = 31;
        // D s_204_1: read-var pmselr:i64
        let s_204_1: i64 = fn_state.pmselr;
        // D s_204_2: cast zx s_204_1 -> i
        let s_204_2: i128 = (i128::try_from(s_204_1).unwrap());
        // D s_204_3: cmp-eq s_204_2 s_204_0
        let s_204_3: bool = ((s_204_2) == (s_204_0));
        // N s_204_4: branch s_204_3 b206 b205
        if s_204_3 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var temprt:u32
        let s_205_0: u32 = fn_state.temprt;
        // D s_205_1: call Mk_PMEVTYPER_Type(s_205_0)
        let s_205_1: ProductType700c18a878c5601b = Mk_PMEVTYPER_Type(
            state,
            tracer,
            s_205_0,
        );
        // D s_205_2: call __get_PMEVTYPER(s_205_1)
        let s_205_2: ProductType700c18a878c5601b = u__get_PMEVTYPER(
            state,
            tracer,
            s_205_1,
        );
        // D s_205_3: read-var pmselr:i64
        let s_205_3: i64 = fn_state.pmselr;
        // D s_205_4: call PMEVTYPER_set(s_205_3, s_205_2)
        let s_205_4: () = PMEVTYPER_set(state, tracer, s_205_3, s_205_2);
        // N s_205_5: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var temprt:u32
        let s_206_0: u32 = fn_state.temprt;
        // D s_206_1: call Mk_PMCCFILTR_Type(s_206_0)
        let s_206_1: ProductType700c18a878c5601b = Mk_PMCCFILTR_Type(
            state,
            tracer,
            s_206_0,
        );
        // D s_206_2: call __get_PMCCFILTR(s_206_1)
        let s_206_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_206_1,
        );
        // D s_206_3: call PMCCFILTR_write(s_206_2)
        let s_206_3: () = PMCCFILTR_write(state, tracer, s_206_2);
        // N s_206_4: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var opc2:u8
        let s_207_0: u8 = fn_state.opc2;
        // D s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 3u16);
        // C s_207_2: const #1u : u8
        let s_207_2: u8 = 1;
        // C s_207_3: cast zx s_207_2 -> bv
        let s_207_3: Bits = Bits::new(s_207_2 as u128, 3u16);
        // D s_207_4: cmp-eq s_207_1 s_207_3
        let s_207_4: bool = ((s_207_1) == (s_207_3));
        // N s_207_5: branch s_207_4 b210 b208
        if s_207_4 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var opc2:u8
        let s_208_0: u8 = fn_state.opc2;
        // D s_208_1: cast zx s_208_0 -> bv
        let s_208_1: Bits = Bits::new(s_208_0 as u128, 3u16);
        // C s_208_2: const #2u : u8
        let s_208_2: u8 = 2;
        // C s_208_3: cast zx s_208_2 -> bv
        let s_208_3: Bits = Bits::new(s_208_2 as u128, 3u16);
        // D s_208_4: cmp-eq s_208_1 s_208_3
        let s_208_4: bool = ((s_208_1) == (s_208_3));
        // D s_208_5: write-var gs#142434 <= s_208_4
        fn_state.gs_142434 = s_208_4;
        // N s_208_6: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var gs#142434:u8
        let s_209_0: bool = fn_state.gs_142434;
        // D s_209_1: write-var gs#142435 <= s_209_0
        fn_state.gs_142435 = s_209_0;
        // N s_209_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #1u : u8
        let s_210_0: bool = true;
        // D s_210_1: write-var gs#142434 <= s_210_0
        fn_state.gs_142434 = s_210_0;
        // N s_210_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var CRm:u8
        let s_211_0: u8 = fn_state.CRm;
        // D s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 4u16);
        // C s_211_2: const #13u : u8
        let s_211_2: u8 = 13;
        // C s_211_3: cast zx s_211_2 -> bv
        let s_211_3: Bits = Bits::new(s_211_2 as u128, 4u16);
        // D s_211_4: cmp-eq s_211_1 s_211_3
        let s_211_4: bool = ((s_211_1) == (s_211_3));
        // D s_211_5: write-var gs#142433 <= s_211_4
        fn_state.gs_142433 = s_211_4;
        // N s_211_6: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var CRn:u8
        let s_212_0: u8 = fn_state.CRn;
        // D s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 4u16);
        // C s_212_2: const #9u : u8
        let s_212_2: u8 = 9;
        // C s_212_3: cast zx s_212_2 -> bv
        let s_212_3: Bits = Bits::new(s_212_2 as u128, 4u16);
        // D s_212_4: cmp-eq s_212_1 s_212_3
        let s_212_4: bool = ((s_212_1) == (s_212_3));
        // D s_212_5: write-var gs#142432 <= s_212_4
        fn_state.gs_142432 = s_212_4;
        // N s_212_6: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_213_0: read-var opc1:u8
        let s_213_0: u8 = fn_state.opc1;
        // D s_213_1: cast zx s_213_0 -> bv
        let s_213_1: Bits = Bits::new(s_213_0 as u128, 3u16);
        // C s_213_2: const #0u : u8
        let s_213_2: u8 = 0;
        // C s_213_3: cast zx s_213_2 -> bv
        let s_213_3: Bits = Bits::new(s_213_2 as u128, 3u16);
        // D s_213_4: cmp-eq s_213_1 s_213_3
        let s_213_4: bool = ((s_213_1) == (s_213_3));
        // D s_213_5: write-var gs#142431 <= s_213_4
        fn_state.gs_142431 = s_213_4;
        // N s_213_6: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #2s : i
        let s_214_0: i128 = 2;
        // D s_214_1: read-var temprt:u32
        let s_214_1: u32 = fn_state.temprt;
        // D s_214_2: cast zx s_214_1 -> bv
        let s_214_2: Bits = Bits::new(s_214_1 as u128, 32u16);
        // C s_214_3: const #1u : u64
        let s_214_3: u64 = 1;
        // D s_214_4: bit-extract s_214_2 s_214_0 s_214_3
        let s_214_4: Bits = (Bits::new(
            ((s_214_2) >> (s_214_0)).value(),
            u16::try_from(s_214_3).unwrap(),
        ));
        // D s_214_5: cast reint s_214_4 -> u8
        let s_214_5: bool = ((s_214_4.value()) != 0);
        // C s_214_6: const #0s : i
        let s_214_6: i128 = 0;
        // C s_214_7: const #0u : u64
        let s_214_7: u64 = 0;
        // D s_214_8: cast zx s_214_5 -> u64
        let s_214_8: u64 = (s_214_5 as u64);
        // C s_214_9: const #1u : u64
        let s_214_9: u64 = 1;
        // D s_214_10: and s_214_8 s_214_9
        let s_214_10: u64 = ((s_214_8) & (s_214_9));
        // D s_214_11: cmp-eq s_214_10 s_214_9
        let s_214_11: bool = ((s_214_10) == (s_214_9));
        // D s_214_12: lsl s_214_8 s_214_6
        let s_214_12: u64 = s_214_8 << s_214_6;
        // D s_214_13: or s_214_7 s_214_12
        let s_214_13: u64 = ((s_214_7) | (s_214_12));
        // D s_214_14: cmpl s_214_12
        let s_214_14: u64 = !s_214_12;
        // D s_214_15: and s_214_7 s_214_14
        let s_214_15: u64 = ((s_214_7) & (s_214_14));
        // D s_214_16: select s_214_11 s_214_13 s_214_15
        let s_214_16: u64 = if s_214_11 { s_214_13 } else { s_214_15 };
        // D s_214_17: cast trunc s_214_16 -> u8
        let s_214_17: bool = ((s_214_16) != 0);
        // D s_214_18: cast zx s_214_17 -> bv
        let s_214_18: Bits = Bits::new(s_214_17 as u128, 1u16);
        // C s_214_19: const #1u : u8
        let s_214_19: bool = true;
        // C s_214_20: cast zx s_214_19 -> bv
        let s_214_20: Bits = Bits::new(s_214_19 as u128, 1u16);
        // D s_214_21: cmp-eq s_214_18 s_214_20
        let s_214_21: bool = ((s_214_18) == (s_214_20));
        // N s_214_22: branch s_214_21 b220 b215
        if s_214_21 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_215_0: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #1s : i
        let s_216_0: i128 = 1;
        // D s_216_1: read-var temprt:u32
        let s_216_1: u32 = fn_state.temprt;
        // D s_216_2: cast zx s_216_1 -> bv
        let s_216_2: Bits = Bits::new(s_216_1 as u128, 32u16);
        // C s_216_3: const #1u : u64
        let s_216_3: u64 = 1;
        // D s_216_4: bit-extract s_216_2 s_216_0 s_216_3
        let s_216_4: Bits = (Bits::new(
            ((s_216_2) >> (s_216_0)).value(),
            u16::try_from(s_216_3).unwrap(),
        ));
        // D s_216_5: cast reint s_216_4 -> u8
        let s_216_5: bool = ((s_216_4.value()) != 0);
        // C s_216_6: const #0s : i
        let s_216_6: i128 = 0;
        // C s_216_7: const #0u : u64
        let s_216_7: u64 = 0;
        // D s_216_8: cast zx s_216_5 -> u64
        let s_216_8: u64 = (s_216_5 as u64);
        // C s_216_9: const #1u : u64
        let s_216_9: u64 = 1;
        // D s_216_10: and s_216_8 s_216_9
        let s_216_10: u64 = ((s_216_8) & (s_216_9));
        // D s_216_11: cmp-eq s_216_10 s_216_9
        let s_216_11: bool = ((s_216_10) == (s_216_9));
        // D s_216_12: lsl s_216_8 s_216_6
        let s_216_12: u64 = s_216_8 << s_216_6;
        // D s_216_13: or s_216_7 s_216_12
        let s_216_13: u64 = ((s_216_7) | (s_216_12));
        // D s_216_14: cmpl s_216_12
        let s_216_14: u64 = !s_216_12;
        // D s_216_15: and s_216_7 s_216_14
        let s_216_15: u64 = ((s_216_7) & (s_216_14));
        // D s_216_16: select s_216_11 s_216_13 s_216_15
        let s_216_16: u64 = if s_216_11 { s_216_13 } else { s_216_15 };
        // D s_216_17: cast trunc s_216_16 -> u8
        let s_216_17: bool = ((s_216_16) != 0);
        // D s_216_18: cast zx s_216_17 -> bv
        let s_216_18: Bits = Bits::new(s_216_17 as u128, 1u16);
        // C s_216_19: const #1u : u8
        let s_216_19: bool = true;
        // C s_216_20: cast zx s_216_19 -> bv
        let s_216_20: Bits = Bits::new(s_216_19 as u128, 1u16);
        // D s_216_21: cmp-eq s_216_18 s_216_20
        let s_216_21: bool = ((s_216_18) == (s_216_20));
        // N s_216_22: branch s_216_21 b219 b217
        if s_216_21 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_217(state, tracer, fn_state);
        };
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_217_0: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_218_0: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #() : ()
        let s_219_0: () = ();
        // S s_219_1: call AArch32_ClearEventCounters(s_219_0)
        let s_219_1: () = AArch32_ClearEventCounters(state, tracer, s_219_0);
        // N s_219_2: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #64s : i
        let s_220_0: i128 = 64;
        // S s_220_1: call Zeros(s_220_0)
        let s_220_1: Bits = Zeros(state, tracer, s_220_0);
        // S s_220_2: cast reint s_220_1 -> u64
        let s_220_2: u64 = (s_220_1.value() as u64);
        // S s_220_3: call Mk_PMCCNTR_Type(s_220_2)
        let s_220_3: ProductType5c790c8ef59cc8b2 = Mk_PMCCNTR_Type(
            state,
            tracer,
            s_220_2,
        );
        // S s_220_4: call PMCCNTR_write(s_220_3)
        let s_220_4: () = PMCCNTR_write(state, tracer, s_220_3);
        // N s_220_5: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_221_0: read-var CRm:u8
        let s_221_0: u8 = fn_state.CRm;
        // D s_221_1: cast zx s_221_0 -> bv
        let s_221_1: Bits = Bits::new(s_221_0 as u128, 4u16);
        // C s_221_2: const #12u : u8
        let s_221_2: u8 = 12;
        // C s_221_3: cast zx s_221_2 -> bv
        let s_221_3: Bits = Bits::new(s_221_2 as u128, 4u16);
        // D s_221_4: cmp-eq s_221_1 s_221_3
        let s_221_4: bool = ((s_221_1) == (s_221_3));
        // D s_221_5: write-var gs#142429 <= s_221_4
        fn_state.gs_142429 = s_221_4;
        // N s_221_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var CRn:u8
        let s_222_0: u8 = fn_state.CRn;
        // D s_222_1: cast zx s_222_0 -> bv
        let s_222_1: Bits = Bits::new(s_222_0 as u128, 4u16);
        // C s_222_2: const #9u : u8
        let s_222_2: u8 = 9;
        // C s_222_3: cast zx s_222_2 -> bv
        let s_222_3: Bits = Bits::new(s_222_2 as u128, 4u16);
        // D s_222_4: cmp-eq s_222_1 s_222_3
        let s_222_4: bool = ((s_222_1) == (s_222_3));
        // D s_222_5: write-var gs#142428 <= s_222_4
        fn_state.gs_142428 = s_222_4;
        // N s_222_6: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var opc2:u8
        let s_223_0: u8 = fn_state.opc2;
        // D s_223_1: cast zx s_223_0 -> bv
        let s_223_1: Bits = Bits::new(s_223_0 as u128, 3u16);
        // C s_223_2: const #0u : u8
        let s_223_2: u8 = 0;
        // C s_223_3: cast zx s_223_2 -> bv
        let s_223_3: Bits = Bits::new(s_223_2 as u128, 3u16);
        // D s_223_4: cmp-eq s_223_1 s_223_3
        let s_223_4: bool = ((s_223_1) == (s_223_3));
        // D s_223_5: write-var gs#142427 <= s_223_4
        fn_state.gs_142427 = s_223_4;
        // N s_223_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var opc1:u8
        let s_224_0: u8 = fn_state.opc1;
        // D s_224_1: cast zx s_224_0 -> bv
        let s_224_1: Bits = Bits::new(s_224_0 as u128, 3u16);
        // C s_224_2: const #0u : u8
        let s_224_2: u8 = 0;
        // C s_224_3: cast zx s_224_2 -> bv
        let s_224_3: Bits = Bits::new(s_224_2 as u128, 3u16);
        // D s_224_4: cmp-eq s_224_1 s_224_3
        let s_224_4: bool = ((s_224_1) == (s_224_3));
        // D s_224_5: write-var gs#142426 <= s_224_4
        fn_state.gs_142426 = s_224_4;
        // N s_224_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_225_0: read-var opc2:u8
        let s_225_0: u8 = fn_state.opc2;
        // D s_225_1: cast zx s_225_0 -> bv
        let s_225_1: Bits = Bits::new(s_225_0 as u128, 3u16);
        // C s_225_2: const #3u : u8
        let s_225_2: u8 = 3;
        // C s_225_3: cast zx s_225_2 -> bv
        let s_225_3: Bits = Bits::new(s_225_2 as u128, 3u16);
        // D s_225_4: cmp-eq s_225_1 s_225_3
        let s_225_4: bool = ((s_225_1) == (s_225_3));
        // N s_225_5: branch s_225_4 b261 b226
        if s_225_4 {
            return block_261(state, tracer, fn_state);
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
        // D s_226_1: write-var gs#142501 <= s_226_0
        fn_state.gs_142501 = s_226_0;
        // N s_226_2: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var gs#142501:u8
        let s_227_0: bool = fn_state.gs_142501;
        // N s_227_1: branch s_227_0 b260 b228
        if s_227_0 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_228(state, tracer, fn_state);
        };
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_228_0: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_229_0: read-var opc2:u8
        let s_229_0: u8 = fn_state.opc2;
        // D s_229_1: cast zx s_229_0 -> bv
        let s_229_1: Bits = Bits::new(s_229_0 as u128, 3u16);
        // C s_229_2: const #3u : u8
        let s_229_2: u8 = 3;
        // C s_229_3: cast zx s_229_2 -> bv
        let s_229_3: Bits = Bits::new(s_229_2 as u128, 3u16);
        // D s_229_4: cmp-eq s_229_1 s_229_3
        let s_229_4: bool = ((s_229_1) == (s_229_3));
        // N s_229_5: branch s_229_4 b259 b230
        if s_229_4 {
            return block_259(state, tracer, fn_state);
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
        // D s_230_1: write-var gs#142502 <= s_230_0
        fn_state.gs_142502 = s_230_0;
        // N s_230_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_231_0: read-var gs#142502:u8
        let s_231_0: bool = fn_state.gs_142502;
        // N s_231_1: branch s_231_0 b258 b232
        if s_231_0 {
            return block_258(state, tracer, fn_state);
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
        // D s_233_0: read-var opc2:u8
        let s_233_0: u8 = fn_state.opc2;
        // D s_233_1: cast zx s_233_0 -> bv
        let s_233_1: Bits = Bits::new(s_233_0 as u128, 3u16);
        // C s_233_2: const #2u : u8
        let s_233_2: u8 = 2;
        // C s_233_3: cast zx s_233_2 -> bv
        let s_233_3: Bits = Bits::new(s_233_2 as u128, 3u16);
        // D s_233_4: cmp-eq s_233_1 s_233_3
        let s_233_4: bool = ((s_233_1) == (s_233_3));
        // N s_233_5: branch s_233_4 b257 b234
        if s_233_4 {
            return block_257(state, tracer, fn_state);
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
        // D s_234_1: write-var gs#142503 <= s_234_0
        fn_state.gs_142503 = s_234_0;
        // N s_234_2: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_235_0: read-var gs#142503:u8
        let s_235_0: bool = fn_state.gs_142503;
        // N s_235_1: branch s_235_0 b256 b236
        if s_235_0 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_236(state, tracer, fn_state);
        };
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_236_0: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_237_0: read-var opc2:u8
        let s_237_0: u8 = fn_state.opc2;
        // D s_237_1: cast zx s_237_0 -> bv
        let s_237_1: Bits = Bits::new(s_237_0 as u128, 3u16);
        // C s_237_2: const #1u : u8
        let s_237_2: u8 = 1;
        // C s_237_3: cast zx s_237_2 -> bv
        let s_237_3: Bits = Bits::new(s_237_2 as u128, 3u16);
        // D s_237_4: cmp-eq s_237_1 s_237_3
        let s_237_4: bool = ((s_237_1) == (s_237_3));
        // N s_237_5: branch s_237_4 b255 b238
        if s_237_4 {
            return block_255(state, tracer, fn_state);
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
        // D s_238_1: write-var gs#142504 <= s_238_0
        fn_state.gs_142504 = s_238_0;
        // N s_238_2: jump b239
        return block_239(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_239_0: read-var gs#142504:u8
        let s_239_0: bool = fn_state.gs_142504;
        // N s_239_1: branch s_239_0 b254 b240
        if s_239_0 {
            return block_254(state, tracer, fn_state);
        } else {
            return block_240(state, tracer, fn_state);
        };
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_240_0: jump b241
        return block_241(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_241_0: read-var opc2:u8
        let s_241_0: u8 = fn_state.opc2;
        // D s_241_1: cast zx s_241_0 -> bv
        let s_241_1: Bits = Bits::new(s_241_0 as u128, 3u16);
        // C s_241_2: const #2u : u8
        let s_241_2: u8 = 2;
        // C s_241_3: cast zx s_241_2 -> bv
        let s_241_3: Bits = Bits::new(s_241_2 as u128, 3u16);
        // D s_241_4: cmp-eq s_241_1 s_241_3
        let s_241_4: bool = ((s_241_1) == (s_241_3));
        // N s_241_5: branch s_241_4 b253 b242
        if s_241_4 {
            return block_253(state, tracer, fn_state);
        } else {
            return block_242(state, tracer, fn_state);
        };
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_242_0: const #0u : u8
        let s_242_0: bool = false;
        // D s_242_1: write-var gs#142505 <= s_242_0
        fn_state.gs_142505 = s_242_0;
        // N s_242_2: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_243_0: read-var gs#142505:u8
        let s_243_0: bool = fn_state.gs_142505;
        // N s_243_1: branch s_243_0 b252 b244
        if s_243_0 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_244_0: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_245_0: read-var opc2:u8
        let s_245_0: u8 = fn_state.opc2;
        // D s_245_1: cast zx s_245_0 -> bv
        let s_245_1: Bits = Bits::new(s_245_0 as u128, 3u16);
        // C s_245_2: const #1u : u8
        let s_245_2: u8 = 1;
        // C s_245_3: cast zx s_245_2 -> bv
        let s_245_3: Bits = Bits::new(s_245_2 as u128, 3u16);
        // D s_245_4: cmp-eq s_245_1 s_245_3
        let s_245_4: bool = ((s_245_1) == (s_245_3));
        // N s_245_5: branch s_245_4 b251 b246
        if s_245_4 {
            return block_251(state, tracer, fn_state);
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
        // D s_246_1: write-var gs#142506 <= s_246_0
        fn_state.gs_142506 = s_246_0;
        // N s_246_2: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_247_0: read-var gs#142506:u8
        let s_247_0: bool = fn_state.gs_142506;
        // N s_247_1: branch s_247_0 b250 b248
        if s_247_0 {
            return block_250(state, tracer, fn_state);
        } else {
            return block_248(state, tracer, fn_state);
        };
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_248_0: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_249_0: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var temprt:u32
        let s_250_0: u32 = fn_state.temprt;
        // D s_250_1: call Mk_PMINTENCLR_Type(s_250_0)
        let s_250_1: ProductType700c18a878c5601b = Mk_PMINTENCLR_Type(
            state,
            tracer,
            s_250_0,
        );
        // D s_250_2: call PMINTENCLR_write(s_250_1)
        let s_250_2: () = PMINTENCLR_write(state, tracer, s_250_1);
        // N s_250_3: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_251_0: read-var CRm:u8
        let s_251_0: u8 = fn_state.CRm;
        // D s_251_1: cast zx s_251_0 -> bv
        let s_251_1: Bits = Bits::new(s_251_0 as u128, 4u16);
        // C s_251_2: const #14u : u8
        let s_251_2: u8 = 14;
        // C s_251_3: cast zx s_251_2 -> bv
        let s_251_3: Bits = Bits::new(s_251_2 as u128, 4u16);
        // D s_251_4: cmp-eq s_251_1 s_251_3
        let s_251_4: bool = ((s_251_1) == (s_251_3));
        // D s_251_5: write-var gs#142506 <= s_251_4
        fn_state.gs_142506 = s_251_4;
        // N s_251_6: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_252_0: read-var temprt:u32
        let s_252_0: u32 = fn_state.temprt;
        // D s_252_1: call Mk_PMINTENSET_Type(s_252_0)
        let s_252_1: ProductType700c18a878c5601b = Mk_PMINTENSET_Type(
            state,
            tracer,
            s_252_0,
        );
        // D s_252_2: call PMINTENSET_write(s_252_1)
        let s_252_2: () = PMINTENSET_write(state, tracer, s_252_1);
        // N s_252_3: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_253_0: read-var CRm:u8
        let s_253_0: u8 = fn_state.CRm;
        // D s_253_1: cast zx s_253_0 -> bv
        let s_253_1: Bits = Bits::new(s_253_0 as u128, 4u16);
        // C s_253_2: const #14u : u8
        let s_253_2: u8 = 14;
        // C s_253_3: cast zx s_253_2 -> bv
        let s_253_3: Bits = Bits::new(s_253_2 as u128, 4u16);
        // D s_253_4: cmp-eq s_253_1 s_253_3
        let s_253_4: bool = ((s_253_1) == (s_253_3));
        // D s_253_5: write-var gs#142505 <= s_253_4
        fn_state.gs_142505 = s_253_4;
        // N s_253_6: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_254_0: read-var temprt:u32
        let s_254_0: u32 = fn_state.temprt;
        // D s_254_1: call Mk_PMCNTENCLR_Type(s_254_0)
        let s_254_1: ProductType700c18a878c5601b = Mk_PMCNTENCLR_Type(
            state,
            tracer,
            s_254_0,
        );
        // D s_254_2: call PMCNTENCLR_write(s_254_1)
        let s_254_2: () = PMCNTENCLR_write(state, tracer, s_254_1);
        // N s_254_3: jump b241
        return block_241(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_255_0: read-var CRm:u8
        let s_255_0: u8 = fn_state.CRm;
        // D s_255_1: cast zx s_255_0 -> bv
        let s_255_1: Bits = Bits::new(s_255_0 as u128, 4u16);
        // C s_255_2: const #12u : u8
        let s_255_2: u8 = 12;
        // C s_255_3: cast zx s_255_2 -> bv
        let s_255_3: Bits = Bits::new(s_255_2 as u128, 4u16);
        // D s_255_4: cmp-eq s_255_1 s_255_3
        let s_255_4: bool = ((s_255_1) == (s_255_3));
        // D s_255_5: write-var gs#142504 <= s_255_4
        fn_state.gs_142504 = s_255_4;
        // N s_255_6: jump b239
        return block_239(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var temprt:u32
        let s_256_0: u32 = fn_state.temprt;
        // D s_256_1: call Mk_PMCNTENSET_Type(s_256_0)
        let s_256_1: ProductType700c18a878c5601b = Mk_PMCNTENSET_Type(
            state,
            tracer,
            s_256_0,
        );
        // D s_256_2: call PMCNTENSET_write(s_256_1)
        let s_256_2: () = PMCNTENSET_write(state, tracer, s_256_1);
        // N s_256_3: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_257_0: read-var CRm:u8
        let s_257_0: u8 = fn_state.CRm;
        // D s_257_1: cast zx s_257_0 -> bv
        let s_257_1: Bits = Bits::new(s_257_0 as u128, 4u16);
        // C s_257_2: const #12u : u8
        let s_257_2: u8 = 12;
        // C s_257_3: cast zx s_257_2 -> bv
        let s_257_3: Bits = Bits::new(s_257_2 as u128, 4u16);
        // D s_257_4: cmp-eq s_257_1 s_257_3
        let s_257_4: bool = ((s_257_1) == (s_257_3));
        // D s_257_5: write-var gs#142503 <= s_257_4
        fn_state.gs_142503 = s_257_4;
        // N s_257_6: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var temprt:u32
        let s_258_0: u32 = fn_state.temprt;
        // D s_258_1: call Mk_PMOVSR_Type(s_258_0)
        let s_258_1: ProductType700c18a878c5601b = Mk_PMOVSR_Type(
            state,
            tracer,
            s_258_0,
        );
        // D s_258_2: call PMOVSR_write(s_258_1)
        let s_258_2: () = PMOVSR_write(state, tracer, s_258_1);
        // N s_258_3: jump b233
        return block_233(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_259_0: read-var CRm:u8
        let s_259_0: u8 = fn_state.CRm;
        // D s_259_1: cast zx s_259_0 -> bv
        let s_259_1: Bits = Bits::new(s_259_0 as u128, 4u16);
        // C s_259_2: const #14u : u8
        let s_259_2: u8 = 14;
        // C s_259_3: cast zx s_259_2 -> bv
        let s_259_3: Bits = Bits::new(s_259_2 as u128, 4u16);
        // D s_259_4: cmp-eq s_259_1 s_259_3
        let s_259_4: bool = ((s_259_1) == (s_259_3));
        // D s_259_5: write-var gs#142502 <= s_259_4
        fn_state.gs_142502 = s_259_4;
        // N s_259_6: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var temprt:u32
        let s_260_0: u32 = fn_state.temprt;
        // D s_260_1: call Mk_PMOVSSET_Type(s_260_0)
        let s_260_1: ProductType700c18a878c5601b = Mk_PMOVSSET_Type(
            state,
            tracer,
            s_260_0,
        );
        // D s_260_2: call PMOVSSET_write(s_260_1)
        let s_260_2: () = PMOVSSET_write(state, tracer, s_260_1);
        // N s_260_3: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_261_0: read-var CRm:u8
        let s_261_0: u8 = fn_state.CRm;
        // D s_261_1: cast zx s_261_0 -> bv
        let s_261_1: Bits = Bits::new(s_261_0 as u128, 4u16);
        // C s_261_2: const #12u : u8
        let s_261_2: u8 = 12;
        // C s_261_3: cast zx s_261_2 -> bv
        let s_261_3: Bits = Bits::new(s_261_2 as u128, 4u16);
        // D s_261_4: cmp-eq s_261_1 s_261_3
        let s_261_4: bool = ((s_261_1) == (s_261_3));
        // D s_261_5: write-var gs#142501 <= s_261_4
        fn_state.gs_142501 = s_261_4;
        // N s_261_6: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_262_0: read-var opc2:u8
        let s_262_0: u8 = fn_state.opc2;
        // D s_262_1: cast zx s_262_0 -> bv
        let s_262_1: Bits = Bits::new(s_262_0 as u128, 3u16);
        // C s_262_2: const #1u : u8
        let s_262_2: u8 = 1;
        // C s_262_3: cast zx s_262_2 -> bv
        let s_262_3: Bits = Bits::new(s_262_2 as u128, 3u16);
        // D s_262_4: cmp-eq s_262_1 s_262_3
        let s_262_4: bool = ((s_262_1) == (s_262_3));
        // N s_262_5: branch s_262_4 b268 b263
        if s_262_4 {
            return block_268(state, tracer, fn_state);
        } else {
            return block_263(state, tracer, fn_state);
        };
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_263_0: read-var opc2:u8
        let s_263_0: u8 = fn_state.opc2;
        // D s_263_1: cast zx s_263_0 -> bv
        let s_263_1: Bits = Bits::new(s_263_0 as u128, 3u16);
        // C s_263_2: const #2u : u8
        let s_263_2: u8 = 2;
        // C s_263_3: cast zx s_263_2 -> bv
        let s_263_3: Bits = Bits::new(s_263_2 as u128, 3u16);
        // D s_263_4: cmp-eq s_263_1 s_263_3
        let s_263_4: bool = ((s_263_1) == (s_263_3));
        // N s_263_5: branch s_263_4 b267 b264
        if s_263_4 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_264(state, tracer, fn_state);
        };
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_264_0: read-var opc2:u8
        let s_264_0: u8 = fn_state.opc2;
        // D s_264_1: cast zx s_264_0 -> bv
        let s_264_1: Bits = Bits::new(s_264_0 as u128, 3u16);
        // C s_264_2: const #3u : u8
        let s_264_2: u8 = 3;
        // C s_264_3: cast zx s_264_2 -> bv
        let s_264_3: Bits = Bits::new(s_264_2 as u128, 3u16);
        // D s_264_4: cmp-eq s_264_1 s_264_3
        let s_264_4: bool = ((s_264_1) == (s_264_3));
        // D s_264_5: write-var gs#142422 <= s_264_4
        fn_state.gs_142422 = s_264_4;
        // N s_264_6: jump b265
        return block_265(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_265_0: read-var gs#142422:u8
        let s_265_0: bool = fn_state.gs_142422;
        // D s_265_1: write-var gs#142423 <= s_265_0
        fn_state.gs_142423 = s_265_0;
        // N s_265_2: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_266_0: read-var gs#142423:u8
        let s_266_0: bool = fn_state.gs_142423;
        // D s_266_1: write-var gs#142424 <= s_266_0
        fn_state.gs_142424 = s_266_0;
        // N s_266_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #1u : u8
        let s_267_0: bool = true;
        // D s_267_1: write-var gs#142422 <= s_267_0
        fn_state.gs_142422 = s_267_0;
        // N s_267_2: jump b265
        return block_265(state, tracer, fn_state);
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_268_0: const #1u : u8
        let s_268_0: bool = true;
        // D s_268_1: write-var gs#142423 <= s_268_0
        fn_state.gs_142423 = s_268_0;
        // N s_268_2: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_269_0: read-var CRm:u8
        let s_269_0: u8 = fn_state.CRm;
        // D s_269_1: cast zx s_269_0 -> bv
        let s_269_1: Bits = Bits::new(s_269_0 as u128, 4u16);
        // C s_269_2: const #12u : u8
        let s_269_2: u8 = 12;
        // C s_269_3: cast zx s_269_2 -> bv
        let s_269_3: Bits = Bits::new(s_269_2 as u128, 4u16);
        // D s_269_4: cmp-eq s_269_1 s_269_3
        let s_269_4: bool = ((s_269_1) == (s_269_3));
        // N s_269_5: branch s_269_4 b272 b270
        if s_269_4 {
            return block_272(state, tracer, fn_state);
        } else {
            return block_270(state, tracer, fn_state);
        };
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_270_0: read-var CRm:u8
        let s_270_0: u8 = fn_state.CRm;
        // D s_270_1: cast zx s_270_0 -> bv
        let s_270_1: Bits = Bits::new(s_270_0 as u128, 4u16);
        // C s_270_2: const #14u : u8
        let s_270_2: u8 = 14;
        // C s_270_3: cast zx s_270_2 -> bv
        let s_270_3: Bits = Bits::new(s_270_2 as u128, 4u16);
        // D s_270_4: cmp-eq s_270_1 s_270_3
        let s_270_4: bool = ((s_270_1) == (s_270_3));
        // D s_270_5: write-var gs#142420 <= s_270_4
        fn_state.gs_142420 = s_270_4;
        // N s_270_6: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_271_0: read-var gs#142420:u8
        let s_271_0: bool = fn_state.gs_142420;
        // D s_271_1: write-var gs#142421 <= s_271_0
        fn_state.gs_142421 = s_271_0;
        // N s_271_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_272_0: const #1u : u8
        let s_272_0: bool = true;
        // D s_272_1: write-var gs#142420 <= s_272_0
        fn_state.gs_142420 = s_272_0;
        // N s_272_2: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_273_0: read-var CRn:u8
        let s_273_0: u8 = fn_state.CRn;
        // D s_273_1: cast zx s_273_0 -> bv
        let s_273_1: Bits = Bits::new(s_273_0 as u128, 4u16);
        // C s_273_2: const #9u : u8
        let s_273_2: u8 = 9;
        // C s_273_3: cast zx s_273_2 -> bv
        let s_273_3: Bits = Bits::new(s_273_2 as u128, 4u16);
        // D s_273_4: cmp-eq s_273_1 s_273_3
        let s_273_4: bool = ((s_273_1) == (s_273_3));
        // D s_273_5: write-var gs#142419 <= s_273_4
        fn_state.gs_142419 = s_273_4;
        // N s_273_6: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_274_0: read-var opc1:u8
        let s_274_0: u8 = fn_state.opc1;
        // D s_274_1: cast zx s_274_0 -> bv
        let s_274_1: Bits = Bits::new(s_274_0 as u128, 3u16);
        // C s_274_2: const #0u : u8
        let s_274_2: u8 = 0;
        // C s_274_3: cast zx s_274_2 -> bv
        let s_274_3: Bits = Bits::new(s_274_2 as u128, 3u16);
        // D s_274_4: cmp-eq s_274_1 s_274_3
        let s_274_4: bool = ((s_274_1) == (s_274_3));
        // D s_274_5: write-var gs#142418 <= s_274_4
        fn_state.gs_142418 = s_274_4;
        // N s_274_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_275_0: read-var temprt:u32
        let s_275_0: u32 = fn_state.temprt;
        // D s_275_1: cast zx s_275_0 -> bv
        let s_275_1: Bits = Bits::new(s_275_0 as u128, 32u16);
        // D s_275_2: cast zx s_275_1 -> i
        let s_275_2: i128 = (s_275_1.value() as i128);
        // D s_275_3: cast reint s_275_2 -> i64
        let s_275_3: i64 = (s_275_2 as i64);
        // C s_275_4: const #3316436565u : u32
        let s_275_4: u32 = 3316436565;
        // C s_275_5: cast zx s_275_4 -> bv
        let s_275_5: Bits = Bits::new(s_275_4 as u128, 32u16);
        // C s_275_6: cast zx s_275_5 -> i
        let s_275_6: i128 = (s_275_5.value() as i128);
        // C s_275_7: cast reint s_275_6 -> i64
        let s_275_7: i64 = (s_275_6 as i64);
        // D s_275_8: cast zx s_275_3 -> i
        let s_275_8: i128 = (i128::try_from(s_275_3).unwrap());
        // C s_275_9: cast zx s_275_7 -> i
        let s_275_9: i128 = (i128::try_from(s_275_7).unwrap());
        // D s_275_10: cmp-eq s_275_8 s_275_9
        let s_275_10: bool = ((s_275_8) == (s_275_9));
        // N s_275_11: branch s_275_10 b280 b276
        if s_275_10 {
            return block_280(state, tracer, fn_state);
        } else {
            return block_276(state, tracer, fn_state);
        };
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_276_0: const #() : ()
        let s_276_0: () = ();
        // S s_276_1: call DBGOSLSR_read(s_276_0)
        let s_276_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_276_0);
        // S s_276_2: call _get_DBGOSLSR_Type_OSLK(s_276_1)
        let s_276_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_276_1);
        // S s_276_3: cast zx s_276_2 -> bv
        let s_276_3: Bits = Bits::new(s_276_2 as u128, 1u16);
        // C s_276_4: const #1u : u8
        let s_276_4: bool = true;
        // C s_276_5: cast zx s_276_4 -> bv
        let s_276_5: Bits = Bits::new(s_276_4 as u128, 1u16);
        // S s_276_6: cmp-eq s_276_3 s_276_5
        let s_276_6: bool = ((s_276_3) == (s_276_5));
        // N s_276_7: branch s_276_6 b279 b277
        if s_276_6 {
            return block_279(state, tracer, fn_state);
        } else {
            return block_277(state, tracer, fn_state);
        };
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_277_0: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_278_0: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_279_0: const #() : ()
        let s_279_0: () = ();
        // S s_279_1: call DBGOSLSR_read(s_279_0)
        let s_279_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_279_0);
        // C s_279_2: const #0u : u8
        let s_279_2: bool = false;
        // S s_279_3: call _update_DBGOSLSR_Type_OSLK(s_279_1, s_279_2)
        let s_279_3: ProductType700c18a878c5601b = u_update_DBGOSLSR_Type_OSLK(
            state,
            tracer,
            s_279_1,
            s_279_2,
        );
        // S s_279_4: call DBGOSLSR_write(s_279_3)
        let s_279_4: () = DBGOSLSR_write(state, tracer, s_279_3);
        // C s_279_5: const #14040u : u32
        let s_279_5: u32 = 14040;
        // D s_279_6: read-reg s_279_5:struct
        let s_279_6: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_279_5 as isize);
            tracer.read_register(s_279_5 as isize, value);
            value
        };
        // C s_279_7: const #14040u : u32
        let s_279_7: u32 = 14040;
        // N s_279_8: write-reg s_279_7 <= s_279_6
        let s_279_8: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_279_7 as isize, s_279_6);
            tracer.write_register(s_279_7 as isize, s_279_6);
        };
        // C s_279_9: const #10128u : u32
        let s_279_9: u32 = 10128;
        // D s_279_10: read-reg s_279_9:struct
        let s_279_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_279_9 as isize);
            tracer.read_register(s_279_9 as isize, value);
            value
        };
        // C s_279_11: const #10128u : u32
        let s_279_11: u32 = 10128;
        // N s_279_12: write-reg s_279_11 <= s_279_10
        let s_279_12: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_279_11 as isize, s_279_10);
            tracer.write_register(s_279_11 as isize, s_279_10);
        };
        // C s_279_13: const #() : ()
        let s_279_13: () = ();
        // S s_279_14: call CheckOSUnlockCatch(s_279_13)
        let s_279_14: () = CheckOSUnlockCatch(state, tracer, s_279_13);
        // N s_279_15: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_280_0: const #() : ()
        let s_280_0: () = ();
        // S s_280_1: call DBGOSLSR_read(s_280_0)
        let s_280_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_280_0);
        // C s_280_2: const #1u : u8
        let s_280_2: bool = true;
        // S s_280_3: call _update_DBGOSLSR_Type_OSLK(s_280_1, s_280_2)
        let s_280_3: ProductType700c18a878c5601b = u_update_DBGOSLSR_Type_OSLK(
            state,
            tracer,
            s_280_1,
            s_280_2,
        );
        // S s_280_4: call DBGOSLSR_write(s_280_3)
        let s_280_4: () = DBGOSLSR_write(state, tracer, s_280_3);
        // C s_280_5: const #14040u : u32
        let s_280_5: u32 = 14040;
        // D s_280_6: read-reg s_280_5:struct
        let s_280_6: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_280_5 as isize);
            tracer.read_register(s_280_5 as isize, value);
            value
        };
        // C s_280_7: const #14040u : u32
        let s_280_7: u32 = 14040;
        // N s_280_8: write-reg s_280_7 <= s_280_6
        let s_280_8: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_280_7 as isize, s_280_6);
            tracer.write_register(s_280_7 as isize, s_280_6);
        };
        // C s_280_9: const #10128u : u32
        let s_280_9: u32 = 10128;
        // D s_280_10: read-reg s_280_9:struct
        let s_280_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_280_9 as isize);
            tracer.read_register(s_280_9 as isize, value);
            value
        };
        // C s_280_11: const #10128u : u32
        let s_280_11: u32 = 10128;
        // N s_280_12: write-reg s_280_11 <= s_280_10
        let s_280_12: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_280_11 as isize, s_280_10);
            tracer.write_register(s_280_11 as isize, s_280_10);
        };
        // N s_280_13: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_281_0: read-var CRm:u8
        let s_281_0: u8 = fn_state.CRm;
        // D s_281_1: cast zx s_281_0 -> bv
        let s_281_1: Bits = Bits::new(s_281_0 as u128, 4u16);
        // C s_281_2: const #0u : u8
        let s_281_2: u8 = 0;
        // C s_281_3: cast zx s_281_2 -> bv
        let s_281_3: Bits = Bits::new(s_281_2 as u128, 4u16);
        // D s_281_4: cmp-eq s_281_1 s_281_3
        let s_281_4: bool = ((s_281_1) == (s_281_3));
        // D s_281_5: write-var gs#142416 <= s_281_4
        fn_state.gs_142416 = s_281_4;
        // N s_281_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_282_0: read-var CRn:u8
        let s_282_0: u8 = fn_state.CRn;
        // D s_282_1: cast zx s_282_0 -> bv
        let s_282_1: Bits = Bits::new(s_282_0 as u128, 4u16);
        // C s_282_2: const #1u : u8
        let s_282_2: u8 = 1;
        // C s_282_3: cast zx s_282_2 -> bv
        let s_282_3: Bits = Bits::new(s_282_2 as u128, 4u16);
        // D s_282_4: cmp-eq s_282_1 s_282_3
        let s_282_4: bool = ((s_282_1) == (s_282_3));
        // D s_282_5: write-var gs#142415 <= s_282_4
        fn_state.gs_142415 = s_282_4;
        // N s_282_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_283_0: read-var opc2:u8
        let s_283_0: u8 = fn_state.opc2;
        // D s_283_1: cast zx s_283_0 -> bv
        let s_283_1: Bits = Bits::new(s_283_0 as u128, 3u16);
        // C s_283_2: const #4u : u8
        let s_283_2: u8 = 4;
        // C s_283_3: cast zx s_283_2 -> bv
        let s_283_3: Bits = Bits::new(s_283_2 as u128, 3u16);
        // D s_283_4: cmp-eq s_283_1 s_283_3
        let s_283_4: bool = ((s_283_1) == (s_283_3));
        // D s_283_5: write-var gs#142414 <= s_283_4
        fn_state.gs_142414 = s_283_4;
        // N s_283_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_284_0: read-var opc1:u8
        let s_284_0: u8 = fn_state.opc1;
        // D s_284_1: cast zx s_284_0 -> bv
        let s_284_1: Bits = Bits::new(s_284_0 as u128, 3u16);
        // C s_284_2: const #0u : u8
        let s_284_2: u8 = 0;
        // C s_284_3: cast zx s_284_2 -> bv
        let s_284_3: Bits = Bits::new(s_284_2 as u128, 3u16);
        // D s_284_4: cmp-eq s_284_1 s_284_3
        let s_284_4: bool = ((s_284_1) == (s_284_3));
        // D s_284_5: write-var gs#142413 <= s_284_4
        fn_state.gs_142413 = s_284_4;
        // N s_284_6: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_285_0: const #() : ()
        let s_285_0: () = ();
        // S s_285_1: call DBGCLAIMCLR_read(s_285_0)
        let s_285_1: ProductType700c18a878c5601b = DBGCLAIMCLR_read(
            state,
            tracer,
            s_285_0,
        );
        // D s_285_2: write-var ga#248661 <= s_285_1
        fn_state.ga_248661 = s_285_1;
        // D s_285_3: read-var ga#248661.0:struct
        let s_285_3: u32 = fn_state.ga_248661._0;
        // C s_285_4: const #0s : i
        let s_285_4: i128 = 0;
        // D s_285_5: read-var temprt:u32
        let s_285_5: u32 = fn_state.temprt;
        // D s_285_6: cast zx s_285_5 -> bv
        let s_285_6: Bits = Bits::new(s_285_5 as u128, 32u16);
        // C s_285_7: const #1s : i64
        let s_285_7: i64 = 1;
        // C s_285_8: cast zx s_285_7 -> i
        let s_285_8: i128 = (i128::try_from(s_285_7).unwrap());
        // C s_285_9: const #7s : i
        let s_285_9: i128 = 7;
        // C s_285_10: add s_285_9 s_285_8
        let s_285_10: i128 = (s_285_9 + s_285_8);
        // D s_285_11: bit-extract s_285_6 s_285_4 s_285_10
        let s_285_11: Bits = (Bits::new(
            ((s_285_6) >> (s_285_4)).value(),
            u16::try_from(s_285_10).unwrap(),
        ));
        // D s_285_12: cast reint s_285_11 -> u8
        let s_285_12: u8 = (s_285_11.value() as u8);
        // C s_285_13: const #0s : i
        let s_285_13: i128 = 0;
        // D s_285_14: cast zx s_285_3 -> bv
        let s_285_14: Bits = Bits::new(s_285_3 as u128, 32u16);
        // D s_285_15: cast zx s_285_12 -> bv
        let s_285_15: Bits = Bits::new(s_285_12 as u128, 8u16);
        // C s_285_16: const #7s : i
        let s_285_16: i128 = 7;
        // C s_285_17: const #1u : u64
        let s_285_17: u64 = 1;
        // C s_285_18: cast zx s_285_17 -> bv
        let s_285_18: Bits = Bits::new(s_285_17 as u128, 64u16);
        // C s_285_19: lsl s_285_18 s_285_16
        let s_285_19: Bits = s_285_18 << s_285_16;
        // C s_285_20: sub s_285_19 s_285_18
        let s_285_20: Bits = ((s_285_19) - (s_285_18));
        // D s_285_21: and s_285_15 s_285_20
        let s_285_21: Bits = ((s_285_15) & (s_285_20));
        // D s_285_22: lsl s_285_21 s_285_13
        let s_285_22: Bits = s_285_21 << s_285_13;
        // C s_285_23: lsl s_285_20 s_285_13
        let s_285_23: Bits = s_285_20 << s_285_13;
        // C s_285_24: cmpl s_285_23
        let s_285_24: Bits = !s_285_23;
        // D s_285_25: and s_285_14 s_285_24
        let s_285_25: Bits = ((s_285_14) & (s_285_24));
        // D s_285_26: or s_285_25 s_285_22
        let s_285_26: Bits = ((s_285_25) | (s_285_22));
        // D s_285_27: cast reint s_285_26 -> u32
        let s_285_27: u32 = (s_285_26.value() as u32);
        // D s_285_28: call Mk_DBGCLAIMCLR_Type(s_285_27)
        let s_285_28: ProductType700c18a878c5601b = Mk_DBGCLAIMCLR_Type(
            state,
            tracer,
            s_285_27,
        );
        // D s_285_29: call DBGCLAIMCLR_write(s_285_28)
        let s_285_29: () = DBGCLAIMCLR_write(state, tracer, s_285_28);
        // N s_285_30: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_286_0: read-var CRm:u8
        let s_286_0: u8 = fn_state.CRm;
        // D s_286_1: cast zx s_286_0 -> bv
        let s_286_1: Bits = Bits::new(s_286_0 as u128, 4u16);
        // C s_286_2: const #8u : u8
        let s_286_2: u8 = 8;
        // C s_286_3: cast zx s_286_2 -> bv
        let s_286_3: Bits = Bits::new(s_286_2 as u128, 4u16);
        // D s_286_4: cmp-eq s_286_1 s_286_3
        let s_286_4: bool = ((s_286_1) == (s_286_3));
        // D s_286_5: write-var gs#142411 <= s_286_4
        fn_state.gs_142411 = s_286_4;
        // N s_286_6: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_287_0: read-var CRn:u8
        let s_287_0: u8 = fn_state.CRn;
        // D s_287_1: cast zx s_287_0 -> bv
        let s_287_1: Bits = Bits::new(s_287_0 as u128, 4u16);
        // C s_287_2: const #7u : u8
        let s_287_2: u8 = 7;
        // C s_287_3: cast zx s_287_2 -> bv
        let s_287_3: Bits = Bits::new(s_287_2 as u128, 4u16);
        // D s_287_4: cmp-eq s_287_1 s_287_3
        let s_287_4: bool = ((s_287_1) == (s_287_3));
        // D s_287_5: write-var gs#142410 <= s_287_4
        fn_state.gs_142410 = s_287_4;
        // N s_287_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_288_0: read-var opc2:u8
        let s_288_0: u8 = fn_state.opc2;
        // D s_288_1: cast zx s_288_0 -> bv
        let s_288_1: Bits = Bits::new(s_288_0 as u128, 3u16);
        // C s_288_2: const #6u : u8
        let s_288_2: u8 = 6;
        // C s_288_3: cast zx s_288_2 -> bv
        let s_288_3: Bits = Bits::new(s_288_2 as u128, 3u16);
        // D s_288_4: cmp-eq s_288_1 s_288_3
        let s_288_4: bool = ((s_288_1) == (s_288_3));
        // D s_288_5: write-var gs#142409 <= s_288_4
        fn_state.gs_142409 = s_288_4;
        // N s_288_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_289_0: read-var opc1:u8
        let s_289_0: u8 = fn_state.opc1;
        // D s_289_1: cast zx s_289_0 -> bv
        let s_289_1: Bits = Bits::new(s_289_0 as u128, 3u16);
        // C s_289_2: const #0u : u8
        let s_289_2: u8 = 0;
        // C s_289_3: cast zx s_289_2 -> bv
        let s_289_3: Bits = Bits::new(s_289_2 as u128, 3u16);
        // D s_289_4: cmp-eq s_289_1 s_289_3
        let s_289_4: bool = ((s_289_1) == (s_289_3));
        // D s_289_5: write-var gs#142408 <= s_289_4
        fn_state.gs_142408 = s_289_4;
        // N s_289_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_290_0: read-var CRm:u8
        let s_290_0: u8 = fn_state.CRm;
        // D s_290_1: cast zx s_290_0 -> bv
        let s_290_1: Bits = Bits::new(s_290_0 as u128, 4u16);
        // C s_290_2: const #8u : u8
        let s_290_2: u8 = 8;
        // C s_290_3: cast zx s_290_2 -> bv
        let s_290_3: Bits = Bits::new(s_290_2 as u128, 4u16);
        // D s_290_4: cmp-eq s_290_1 s_290_3
        let s_290_4: bool = ((s_290_1) == (s_290_3));
        // N s_290_5: branch s_290_4 b296 b291
        if s_290_4 {
            return block_296(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_291_0: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_292_0: read-var CRm:u8
        let s_292_0: u8 = fn_state.CRm;
        // D s_292_1: cast zx s_292_0 -> bv
        let s_292_1: Bits = Bits::new(s_292_0 as u128, 4u16);
        // C s_292_2: const #9u : u8
        let s_292_2: u8 = 9;
        // C s_292_3: cast zx s_292_2 -> bv
        let s_292_3: Bits = Bits::new(s_292_2 as u128, 4u16);
        // D s_292_4: cmp-eq s_292_1 s_292_3
        let s_292_4: bool = ((s_292_1) == (s_292_3));
        // N s_292_5: branch s_292_4 b295 b293
        if s_292_4 {
            return block_295(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_293_0: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_294_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_295_0: const #() : ()
        let s_295_0: () = ();
        // S s_295_1: call DBGCLAIMCLR_read(s_295_0)
        let s_295_1: ProductType700c18a878c5601b = DBGCLAIMCLR_read(
            state,
            tracer,
            s_295_0,
        );
        // D s_295_2: write-var ga#248649 <= s_295_1
        fn_state.ga_248649 = s_295_1;
        // D s_295_3: read-var ga#248649.0:struct
        let s_295_3: u32 = fn_state.ga_248649._0;
        // C s_295_4: const #0s : i
        let s_295_4: i128 = 0;
        // D s_295_5: cast zx s_295_3 -> bv
        let s_295_5: Bits = Bits::new(s_295_3 as u128, 32u16);
        // C s_295_6: const #1s : i64
        let s_295_6: i64 = 1;
        // C s_295_7: cast zx s_295_6 -> i
        let s_295_7: i128 = (i128::try_from(s_295_6).unwrap());
        // C s_295_8: const #7s : i
        let s_295_8: i128 = 7;
        // C s_295_9: add s_295_8 s_295_7
        let s_295_9: i128 = (s_295_8 + s_295_7);
        // D s_295_10: bit-extract s_295_5 s_295_4 s_295_9
        let s_295_10: Bits = (Bits::new(
            ((s_295_5) >> (s_295_4)).value(),
            u16::try_from(s_295_9).unwrap(),
        ));
        // D s_295_11: cast reint s_295_10 -> u8
        let s_295_11: u8 = (s_295_10.value() as u8);
        // C s_295_12: const #0s : i
        let s_295_12: i128 = 0;
        // D s_295_13: read-var temprt:u32
        let s_295_13: u32 = fn_state.temprt;
        // D s_295_14: cast zx s_295_13 -> bv
        let s_295_14: Bits = Bits::new(s_295_13 as u128, 32u16);
        // C s_295_15: const #1s : i64
        let s_295_15: i64 = 1;
        // C s_295_16: cast zx s_295_15 -> i
        let s_295_16: i128 = (i128::try_from(s_295_15).unwrap());
        // C s_295_17: const #7s : i
        let s_295_17: i128 = 7;
        // C s_295_18: add s_295_17 s_295_16
        let s_295_18: i128 = (s_295_17 + s_295_16);
        // D s_295_19: bit-extract s_295_14 s_295_12 s_295_18
        let s_295_19: Bits = (Bits::new(
            ((s_295_14) >> (s_295_12)).value(),
            u16::try_from(s_295_18).unwrap(),
        ));
        // D s_295_20: cast reint s_295_19 -> u8
        let s_295_20: u8 = (s_295_19.value() as u8);
        // D s_295_21: cast zx s_295_20 -> bv
        let s_295_21: Bits = Bits::new(s_295_20 as u128, 8u16);
        // D s_295_22: not s_295_21
        let s_295_22: Bits = !s_295_21;
        // D s_295_23: cast reint s_295_22 -> u8
        let s_295_23: u8 = (s_295_22.value() as u8);
        // D s_295_24: cast zx s_295_11 -> bv
        let s_295_24: Bits = Bits::new(s_295_11 as u128, 8u16);
        // D s_295_25: cast zx s_295_23 -> bv
        let s_295_25: Bits = Bits::new(s_295_23 as u128, 8u16);
        // D s_295_26: and s_295_24 s_295_25
        let s_295_26: Bits = ((s_295_24) & (s_295_25));
        // D s_295_27: cast reint s_295_26 -> u8
        let s_295_27: u8 = (s_295_26.value() as u8);
        // C s_295_28: const #0s : i
        let s_295_28: i128 = 0;
        // D s_295_29: read-var temprt:u32
        let s_295_29: u32 = fn_state.temprt;
        // D s_295_30: cast zx s_295_29 -> bv
        let s_295_30: Bits = Bits::new(s_295_29 as u128, 32u16);
        // D s_295_31: cast zx s_295_27 -> bv
        let s_295_31: Bits = Bits::new(s_295_27 as u128, 8u16);
        // C s_295_32: const #7s : i
        let s_295_32: i128 = 7;
        // C s_295_33: const #1u : u64
        let s_295_33: u64 = 1;
        // C s_295_34: cast zx s_295_33 -> bv
        let s_295_34: Bits = Bits::new(s_295_33 as u128, 64u16);
        // C s_295_35: lsl s_295_34 s_295_32
        let s_295_35: Bits = s_295_34 << s_295_32;
        // C s_295_36: sub s_295_35 s_295_34
        let s_295_36: Bits = ((s_295_35) - (s_295_34));
        // D s_295_37: and s_295_31 s_295_36
        let s_295_37: Bits = ((s_295_31) & (s_295_36));
        // D s_295_38: lsl s_295_37 s_295_28
        let s_295_38: Bits = s_295_37 << s_295_28;
        // C s_295_39: lsl s_295_36 s_295_28
        let s_295_39: Bits = s_295_36 << s_295_28;
        // C s_295_40: cmpl s_295_39
        let s_295_40: Bits = !s_295_39;
        // D s_295_41: and s_295_30 s_295_40
        let s_295_41: Bits = ((s_295_30) & (s_295_40));
        // D s_295_42: or s_295_41 s_295_38
        let s_295_42: Bits = ((s_295_41) | (s_295_38));
        // D s_295_43: cast reint s_295_42 -> u32
        let s_295_43: u32 = (s_295_42.value() as u32);
        // D s_295_44: write-var temprt <= s_295_43
        fn_state.temprt = s_295_43;
        // N s_295_45: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_296_0: const #() : ()
        let s_296_0: () = ();
        // S s_296_1: call DBGCLAIMCLR_read(s_296_0)
        let s_296_1: ProductType700c18a878c5601b = DBGCLAIMCLR_read(
            state,
            tracer,
            s_296_0,
        );
        // D s_296_2: write-var ga#248643 <= s_296_1
        fn_state.ga_248643 = s_296_1;
        // D s_296_3: read-var ga#248643.0:struct
        let s_296_3: u32 = fn_state.ga_248643._0;
        // C s_296_4: const #0s : i
        let s_296_4: i128 = 0;
        // D s_296_5: cast zx s_296_3 -> bv
        let s_296_5: Bits = Bits::new(s_296_3 as u128, 32u16);
        // C s_296_6: const #1s : i64
        let s_296_6: i64 = 1;
        // C s_296_7: cast zx s_296_6 -> i
        let s_296_7: i128 = (i128::try_from(s_296_6).unwrap());
        // C s_296_8: const #7s : i
        let s_296_8: i128 = 7;
        // C s_296_9: add s_296_8 s_296_7
        let s_296_9: i128 = (s_296_8 + s_296_7);
        // D s_296_10: bit-extract s_296_5 s_296_4 s_296_9
        let s_296_10: Bits = (Bits::new(
            ((s_296_5) >> (s_296_4)).value(),
            u16::try_from(s_296_9).unwrap(),
        ));
        // D s_296_11: cast reint s_296_10 -> u8
        let s_296_11: u8 = (s_296_10.value() as u8);
        // C s_296_12: const #0s : i
        let s_296_12: i128 = 0;
        // D s_296_13: read-var temprt:u32
        let s_296_13: u32 = fn_state.temprt;
        // D s_296_14: cast zx s_296_13 -> bv
        let s_296_14: Bits = Bits::new(s_296_13 as u128, 32u16);
        // C s_296_15: const #1s : i64
        let s_296_15: i64 = 1;
        // C s_296_16: cast zx s_296_15 -> i
        let s_296_16: i128 = (i128::try_from(s_296_15).unwrap());
        // C s_296_17: const #7s : i
        let s_296_17: i128 = 7;
        // C s_296_18: add s_296_17 s_296_16
        let s_296_18: i128 = (s_296_17 + s_296_16);
        // D s_296_19: bit-extract s_296_14 s_296_12 s_296_18
        let s_296_19: Bits = (Bits::new(
            ((s_296_14) >> (s_296_12)).value(),
            u16::try_from(s_296_18).unwrap(),
        ));
        // D s_296_20: cast reint s_296_19 -> u8
        let s_296_20: u8 = (s_296_19.value() as u8);
        // D s_296_21: cast zx s_296_11 -> bv
        let s_296_21: Bits = Bits::new(s_296_11 as u128, 8u16);
        // D s_296_22: cast zx s_296_20 -> bv
        let s_296_22: Bits = Bits::new(s_296_20 as u128, 8u16);
        // D s_296_23: or s_296_21 s_296_22
        let s_296_23: Bits = ((s_296_21) | (s_296_22));
        // D s_296_24: cast reint s_296_23 -> u8
        let s_296_24: u8 = (s_296_23.value() as u8);
        // C s_296_25: const #0s : i
        let s_296_25: i128 = 0;
        // D s_296_26: read-var temprt:u32
        let s_296_26: u32 = fn_state.temprt;
        // D s_296_27: cast zx s_296_26 -> bv
        let s_296_27: Bits = Bits::new(s_296_26 as u128, 32u16);
        // D s_296_28: cast zx s_296_24 -> bv
        let s_296_28: Bits = Bits::new(s_296_24 as u128, 8u16);
        // C s_296_29: const #7s : i
        let s_296_29: i128 = 7;
        // C s_296_30: const #1u : u64
        let s_296_30: u64 = 1;
        // C s_296_31: cast zx s_296_30 -> bv
        let s_296_31: Bits = Bits::new(s_296_30 as u128, 64u16);
        // C s_296_32: lsl s_296_31 s_296_29
        let s_296_32: Bits = s_296_31 << s_296_29;
        // C s_296_33: sub s_296_32 s_296_31
        let s_296_33: Bits = ((s_296_32) - (s_296_31));
        // D s_296_34: and s_296_28 s_296_33
        let s_296_34: Bits = ((s_296_28) & (s_296_33));
        // D s_296_35: lsl s_296_34 s_296_25
        let s_296_35: Bits = s_296_34 << s_296_25;
        // C s_296_36: lsl s_296_33 s_296_25
        let s_296_36: Bits = s_296_33 << s_296_25;
        // C s_296_37: cmpl s_296_36
        let s_296_37: Bits = !s_296_36;
        // D s_296_38: and s_296_27 s_296_37
        let s_296_38: Bits = ((s_296_27) & (s_296_37));
        // D s_296_39: or s_296_38 s_296_35
        let s_296_39: Bits = ((s_296_38) | (s_296_35));
        // D s_296_40: cast reint s_296_39 -> u32
        let s_296_40: u32 = (s_296_39.value() as u32);
        // D s_296_41: write-var temprt <= s_296_40
        fn_state.temprt = s_296_40;
        // N s_296_42: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_297_0: read-var CRn:u8
        let s_297_0: u8 = fn_state.CRn;
        // D s_297_1: cast zx s_297_0 -> bv
        let s_297_1: Bits = Bits::new(s_297_0 as u128, 4u16);
        // C s_297_2: const #7u : u8
        let s_297_2: u8 = 7;
        // C s_297_3: cast zx s_297_2 -> bv
        let s_297_3: Bits = Bits::new(s_297_2 as u128, 4u16);
        // D s_297_4: cmp-eq s_297_1 s_297_3
        let s_297_4: bool = ((s_297_1) == (s_297_3));
        // D s_297_5: write-var gs#142312 <= s_297_4
        fn_state.gs_142312 = s_297_4;
        // N s_297_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_298_0: read-var opc2:u8
        let s_298_0: u8 = fn_state.opc2;
        // D s_298_1: cast zx s_298_0 -> bv
        let s_298_1: Bits = Bits::new(s_298_0 as u128, 3u16);
        // C s_298_2: const #6u : u8
        let s_298_2: u8 = 6;
        // C s_298_3: cast zx s_298_2 -> bv
        let s_298_3: Bits = Bits::new(s_298_2 as u128, 3u16);
        // D s_298_4: cmp-eq s_298_1 s_298_3
        let s_298_4: bool = ((s_298_1) == (s_298_3));
        // D s_298_5: write-var gs#142311 <= s_298_4
        fn_state.gs_142311 = s_298_4;
        // N s_298_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_299_0: read-var opc1:u8
        let s_299_0: u8 = fn_state.opc1;
        // D s_299_1: cast zx s_299_0 -> bv
        let s_299_1: Bits = Bits::new(s_299_0 as u128, 3u16);
        // C s_299_2: const #0u : u8
        let s_299_2: u8 = 0;
        // C s_299_3: cast zx s_299_2 -> bv
        let s_299_3: Bits = Bits::new(s_299_2 as u128, 3u16);
        // D s_299_4: cmp-eq s_299_1 s_299_3
        let s_299_4: bool = ((s_299_1) == (s_299_3));
        // D s_299_5: write-var gs#142310 <= s_299_4
        fn_state.gs_142310 = s_299_4;
        // N s_299_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_300_0: const #() : ()
        let s_300_0: () = ();
        // S s_300_1: call PMSELR_read(s_300_0)
        let s_300_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_300_0);
        // S s_300_2: call _get_PMSELR_Type_SEL(s_300_1)
        let s_300_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_300_1);
        // S s_300_3: cast zx s_300_2 -> bv
        let s_300_3: Bits = Bits::new(s_300_2 as u128, 5u16);
        // S s_300_4: cast zx s_300_3 -> i
        let s_300_4: i128 = (s_300_3.value() as i128);
        // S s_300_5: cast reint s_300_4 -> i64
        let s_300_5: i64 = (s_300_4 as i64);
        // C s_300_6: const #() : ()
        let s_300_6: () = ();
        // S s_300_7: call GetNumEventCounters(s_300_6)
        let s_300_7: i128 = GetNumEventCounters(state, tracer, s_300_6);
        // C s_300_8: const #1s : i
        let s_300_8: i128 = 1;
        // S s_300_9: sub s_300_7 s_300_8
        let s_300_9: i128 = ((s_300_7) - (s_300_8));
        // S s_300_10: cast zx s_300_5 -> i
        let s_300_10: i128 = (i128::try_from(s_300_5).unwrap());
        // S s_300_11: cmp-gt s_300_10 s_300_9
        let s_300_11: bool = ((s_300_10) > (s_300_9));
        // N s_300_12: branch s_300_11 b325 b301
        if s_300_11 {
            return block_325(state, tracer, fn_state);
        } else {
            return block_301(state, tracer, fn_state);
        };
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_301_0: const #0u : u8
        let s_301_0: bool = false;
        // D s_301_1: write-var gs#142330 <= s_301_0
        fn_state.gs_142330 = s_301_0;
        // N s_301_2: jump b302
        return block_302(state, tracer, fn_state);
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_302_0: read-var gs#142330:u8
        let s_302_0: bool = fn_state.gs_142330;
        // N s_302_1: branch s_302_0 b324 b303
        if s_302_0 {
            return block_324(state, tracer, fn_state);
        } else {
            return block_303(state, tracer, fn_state);
        };
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_303_0: const #() : ()
        let s_303_0: () = ();
        // S s_303_1: call EL2Enabled(s_303_0)
        let s_303_1: bool = EL2Enabled(state, tracer, s_303_0);
        // N s_303_2: branch s_303_1 b320 b304
        if s_303_1 {
            return block_320(state, tracer, fn_state);
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
        // D s_304_1: write-var gs#142332 <= s_304_0
        fn_state.gs_142332 = s_304_0;
        // N s_304_2: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_305_0: read-var gs#142332:u8
        let s_305_0: bool = fn_state.gs_142332;
        // N s_305_1: branch s_305_0 b319 b306
        if s_305_0 {
            return block_319(state, tracer, fn_state);
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
        // D s_306_1: write-var gs#142333 <= s_306_0
        fn_state.gs_142333 = s_306_0;
        // N s_306_2: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_307_0: read-var gs#142333:u8
        let s_307_0: bool = fn_state.gs_142333;
        // N s_307_1: branch s_307_0 b318 b308
        if s_307_0 {
            return block_318(state, tracer, fn_state);
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
        // D s_308_1: write-var gs#142335 <= s_308_0
        fn_state.gs_142335 = s_308_0;
        // N s_308_2: jump b309
        return block_309(state, tracer, fn_state);
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_309_0: read-var gs#142335:u8
        let s_309_0: bool = fn_state.gs_142335;
        // D s_309_1: write-var gs#142336 <= s_309_0
        fn_state.gs_142336 = s_309_0;
        // N s_309_2: jump b310
        return block_310(state, tracer, fn_state);
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_310_0: read-var gs#142336:u8
        let s_310_0: bool = fn_state.gs_142336;
        // N s_310_1: branch s_310_0 b313 b311
        if s_310_0 {
            return block_313(state, tracer, fn_state);
        } else {
            return block_311(state, tracer, fn_state);
        };
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_311_0: jump b312
        return block_312(state, tracer, fn_state);
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_312_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_313_0: const #() : ()
        let s_313_0: () = ();
        // S s_313_1: call PMSELR_read(s_313_0)
        let s_313_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_313_0);
        // S s_313_2: call _get_PMSELR_Type_SEL(s_313_1)
        let s_313_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_313_1);
        // S s_313_3: cast zx s_313_2 -> bv
        let s_313_3: Bits = Bits::new(s_313_2 as u128, 5u16);
        // S s_313_4: cast zx s_313_3 -> i
        let s_313_4: i128 = (s_313_3.value() as i128);
        // S s_313_5: cast reint s_313_4 -> i64
        let s_313_5: i64 = (s_313_4 as i64);
        // C s_313_6: const #() : ()
        let s_313_6: () = ();
        // S s_313_7: call GetNumEventCounters(s_313_6)
        let s_313_7: i128 = GetNumEventCounters(state, tracer, s_313_6);
        // C s_313_8: const #1s : i
        let s_313_8: i128 = 1;
        // S s_313_9: sub s_313_7 s_313_8
        let s_313_9: i128 = ((s_313_7) - (s_313_8));
        // S s_313_10: cast zx s_313_5 -> i
        let s_313_10: i128 = (i128::try_from(s_313_5).unwrap());
        // S s_313_11: cmp-gt s_313_10 s_313_9
        let s_313_11: bool = ((s_313_10) > (s_313_9));
        // N s_313_12: branch s_313_11 b317 b314
        if s_313_11 {
            return block_317(state, tracer, fn_state);
        } else {
            return block_314(state, tracer, fn_state);
        };
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_314_0: const #432u : u32
        let s_314_0: u32 = 432;
        // D s_314_1: read-reg s_314_0:u8
        let s_314_1: u8 = {
            let value = state.read_register::<u8>(s_314_0 as isize);
            tracer.read_register(s_314_0 as isize, value);
            value
        };
        // D s_314_2: call ELUsingAArch32(s_314_1)
        let s_314_2: bool = ELUsingAArch32(state, tracer, s_314_1);
        // N s_314_3: branch s_314_2 b316 b315
        if s_314_2 {
            return block_316(state, tracer, fn_state);
        } else {
            return block_315(state, tracer, fn_state);
        };
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_315_0: const #3u : u8
        let s_315_0: u8 = 3;
        // C s_315_1: cast zx s_315_0 -> bv
        let s_315_1: Bits = Bits::new(s_315_0 as u128, 8u16);
        // C s_315_2: cast zx s_315_1 -> i
        let s_315_2: i128 = (s_315_1.value() as i128);
        // C s_315_3: cast reint s_315_2 -> i64
        let s_315_3: i64 = (s_315_2 as i64);
        // C s_315_4: cast zx s_315_3 -> i
        let s_315_4: i128 = (i128::try_from(s_315_3).unwrap());
        // C s_315_5: const #432u : u32
        let s_315_5: u32 = 432;
        // D s_315_6: read-reg s_315_5:u8
        let s_315_6: u8 = {
            let value = state.read_register::<u8>(s_315_5 as isize);
            tracer.read_register(s_315_5 as isize, value);
            value
        };
        // D s_315_7: call AArch64_AArch32SystemAccessTrap(s_315_6, s_315_4)
        let s_315_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_315_6,
            s_315_4,
        );
        // N s_315_8: jump b312
        return block_312(state, tracer, fn_state);
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_316_0: const #3u : u8
        let s_316_0: u8 = 3;
        // C s_316_1: cast zx s_316_0 -> bv
        let s_316_1: Bits = Bits::new(s_316_0 as u128, 8u16);
        // C s_316_2: cast zx s_316_1 -> i
        let s_316_2: i128 = (s_316_1.value() as i128);
        // C s_316_3: cast reint s_316_2 -> i64
        let s_316_3: i64 = (s_316_2 as i64);
        // C s_316_4: cast zx s_316_3 -> i
        let s_316_4: i128 = (i128::try_from(s_316_3).unwrap());
        // S s_316_5: call AArch32_TakeHypTrapException(s_316_4)
        let s_316_5: () = AArch32_TakeHypTrapException(state, tracer, s_316_4);
        // N s_316_6: jump b312
        return block_312(state, tracer, fn_state);
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_317_0: panic
        panic!("{:?}", ());
        // N s_317_1: return
        return;
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_318_0: const #() : ()
        let s_318_0: () = ();
        // S s_318_1: call PMSELR_read(s_318_0)
        let s_318_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_318_0);
        // S s_318_2: call _get_PMSELR_Type_SEL(s_318_1)
        let s_318_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_318_1);
        // S s_318_3: cast zx s_318_2 -> bv
        let s_318_3: Bits = Bits::new(s_318_2 as u128, 5u16);
        // S s_318_4: cast zx s_318_3 -> i
        let s_318_4: i128 = (s_318_3.value() as i128);
        // S s_318_5: cast reint s_318_4 -> i64
        let s_318_5: i64 = (s_318_4 as i64);
        // C s_318_6: const #() : ()
        let s_318_6: () = ();
        // S s_318_7: call AArch32_GetNumEventCountersAccessible(s_318_6)
        let s_318_7: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_318_6,
        );
        // C s_318_8: const #1s : i
        let s_318_8: i128 = 1;
        // S s_318_9: sub s_318_7 s_318_8
        let s_318_9: i128 = ((s_318_7) - (s_318_8));
        // S s_318_10: cast zx s_318_5 -> i
        let s_318_10: i128 = (i128::try_from(s_318_5).unwrap());
        // S s_318_11: cmp-gt s_318_10 s_318_9
        let s_318_11: bool = ((s_318_10) > (s_318_9));
        // D s_318_12: write-var gs#142335 <= s_318_11
        fn_state.gs_142335 = s_318_11;
        // N s_318_13: jump b309
        return block_309(state, tracer, fn_state);
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_319_0: const #102624u : u32
        let s_319_0: u32 = 102624;
        // D s_319_1: read-reg s_319_0:struct
        let s_319_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_319_0 as isize);
            tracer.read_register(s_319_0 as isize, value);
            value
        };
        // D s_319_2: call _get_PMUSERENR_EL0_Type_EN(s_319_1)
        let s_319_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_319_1);
        // D s_319_3: cast zx s_319_2 -> bv
        let s_319_3: Bits = Bits::new(s_319_2 as u128, 1u16);
        // C s_319_4: const #1u : u8
        let s_319_4: bool = true;
        // C s_319_5: cast zx s_319_4 -> bv
        let s_319_5: Bits = Bits::new(s_319_4 as u128, 1u16);
        // D s_319_6: cmp-eq s_319_3 s_319_5
        let s_319_6: bool = ((s_319_3) == (s_319_5));
        // D s_319_7: write-var gs#142333 <= s_319_6
        fn_state.gs_142333 = s_319_6;
        // N s_319_8: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_320_0: const #16975u : u32
        let s_320_0: u32 = 16975;
        // D s_320_1: read-reg s_320_0:u8
        let s_320_1: u8 = {
            let value = state.read_register::<u8>(s_320_0 as isize);
            tracer.read_register(s_320_0 as isize, value);
            value
        };
        // D s_320_2: cast zx s_320_1 -> bv
        let s_320_2: Bits = Bits::new(s_320_1 as u128, 2u16);
        // C s_320_3: const #448u : u32
        let s_320_3: u32 = 448;
        // D s_320_4: read-reg s_320_3:u8
        let s_320_4: u8 = {
            let value = state.read_register::<u8>(s_320_3 as isize);
            tracer.read_register(s_320_3 as isize, value);
            value
        };
        // D s_320_5: cast zx s_320_4 -> bv
        let s_320_5: Bits = Bits::new(s_320_4 as u128, 2u16);
        // D s_320_6: cmp-eq s_320_2 s_320_5
        let s_320_6: bool = ((s_320_2) == (s_320_5));
        // N s_320_7: branch s_320_6 b323 b321
        if s_320_6 {
            return block_323(state, tracer, fn_state);
        } else {
            return block_321(state, tracer, fn_state);
        };
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_321_0: const #16975u : u32
        let s_321_0: u32 = 16975;
        // D s_321_1: read-reg s_321_0:u8
        let s_321_1: u8 = {
            let value = state.read_register::<u8>(s_321_0 as isize);
            tracer.read_register(s_321_0 as isize, value);
            value
        };
        // D s_321_2: cast zx s_321_1 -> bv
        let s_321_2: Bits = Bits::new(s_321_1 as u128, 2u16);
        // C s_321_3: const #440u : u32
        let s_321_3: u32 = 440;
        // D s_321_4: read-reg s_321_3:u8
        let s_321_4: u8 = {
            let value = state.read_register::<u8>(s_321_3 as isize);
            tracer.read_register(s_321_3 as isize, value);
            value
        };
        // D s_321_5: cast zx s_321_4 -> bv
        let s_321_5: Bits = Bits::new(s_321_4 as u128, 2u16);
        // D s_321_6: cmp-eq s_321_2 s_321_5
        let s_321_6: bool = ((s_321_2) == (s_321_5));
        // D s_321_7: write-var gs#142331 <= s_321_6
        fn_state.gs_142331 = s_321_6;
        // N s_321_8: jump b322
        return block_322(state, tracer, fn_state);
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_322_0: read-var gs#142331:u8
        let s_322_0: bool = fn_state.gs_142331;
        // D s_322_1: write-var gs#142332 <= s_322_0
        fn_state.gs_142332 = s_322_0;
        // N s_322_2: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_323_0: const #1u : u8
        let s_323_0: bool = true;
        // D s_323_1: write-var gs#142331 <= s_323_0
        fn_state.gs_142331 = s_323_0;
        // N s_323_2: jump b322
        return block_322(state, tracer, fn_state);
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_324_0: const #1u : u8
        let s_324_0: bool = true;
        // D s_324_1: write-var gs#142336 <= s_324_0
        fn_state.gs_142336 = s_324_0;
        // N s_324_2: jump b310
        return block_310(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_325_0: const #() : ()
        let s_325_0: () = ();
        // S s_325_1: call PMSELR_read(s_325_0)
        let s_325_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_325_0);
        // S s_325_2: call _get_PMSELR_Type_SEL(s_325_1)
        let s_325_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_325_1);
        // S s_325_3: cast zx s_325_2 -> bv
        let s_325_3: Bits = Bits::new(s_325_2 as u128, 5u16);
        // C s_325_4: const #31u : u8
        let s_325_4: u8 = 31;
        // C s_325_5: cast zx s_325_4 -> bv
        let s_325_5: Bits = Bits::new(s_325_4 as u128, 5u16);
        // S s_325_6: cmp-ne s_325_3 s_325_5
        let s_325_6: bool = ((s_325_3) != (s_325_5));
        // D s_325_7: write-var gs#142330 <= s_325_6
        fn_state.gs_142330 = s_325_6;
        // N s_325_8: jump b302
        return block_302(state, tracer, fn_state);
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_326_0: read-var opc2:u8
        let s_326_0: u8 = fn_state.opc2;
        // D s_326_1: cast zx s_326_0 -> bv
        let s_326_1: Bits = Bits::new(s_326_0 as u128, 3u16);
        // C s_326_2: const #1u : u8
        let s_326_2: u8 = 1;
        // C s_326_3: cast zx s_326_2 -> bv
        let s_326_3: Bits = Bits::new(s_326_2 as u128, 3u16);
        // D s_326_4: cmp-eq s_326_1 s_326_3
        let s_326_4: bool = ((s_326_1) == (s_326_3));
        // N s_326_5: branch s_326_4 b332 b327
        if s_326_4 {
            return block_332(state, tracer, fn_state);
        } else {
            return block_327(state, tracer, fn_state);
        };
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_327_0: const #0u : u8
        let s_327_0: bool = false;
        // D s_327_1: write-var gs#142306 <= s_327_0
        fn_state.gs_142306 = s_327_0;
        // N s_327_2: jump b328
        return block_328(state, tracer, fn_state);
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_328_0: read-var gs#142306:u8
        let s_328_0: bool = fn_state.gs_142306;
        // N s_328_1: branch s_328_0 b331 b329
        if s_328_0 {
            return block_331(state, tracer, fn_state);
        } else {
            return block_329(state, tracer, fn_state);
        };
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_329_0: read-var opc2:u8
        let s_329_0: u8 = fn_state.opc2;
        // D s_329_1: cast zx s_329_0 -> bv
        let s_329_1: Bits = Bits::new(s_329_0 as u128, 3u16);
        // C s_329_2: const #2u : u8
        let s_329_2: u8 = 2;
        // C s_329_3: cast zx s_329_2 -> bv
        let s_329_3: Bits = Bits::new(s_329_2 as u128, 3u16);
        // D s_329_4: cmp-eq s_329_1 s_329_3
        let s_329_4: bool = ((s_329_1) == (s_329_3));
        // D s_329_5: write-var gs#142307 <= s_329_4
        fn_state.gs_142307 = s_329_4;
        // N s_329_6: jump b330
        return block_330(state, tracer, fn_state);
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_330_0: read-var gs#142307:u8
        let s_330_0: bool = fn_state.gs_142307;
        // D s_330_1: write-var gs#142308 <= s_330_0
        fn_state.gs_142308 = s_330_0;
        // N s_330_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_331_0: const #1u : u8
        let s_331_0: bool = true;
        // D s_331_1: write-var gs#142307 <= s_331_0
        fn_state.gs_142307 = s_331_0;
        // N s_331_2: jump b330
        return block_330(state, tracer, fn_state);
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_332_0: const #() : ()
        let s_332_0: () = ();
        // S s_332_1: call PMSELR_read(s_332_0)
        let s_332_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_332_0);
        // S s_332_2: call _get_PMSELR_Type_SEL(s_332_1)
        let s_332_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_332_1);
        // S s_332_3: cast zx s_332_2 -> bv
        let s_332_3: Bits = Bits::new(s_332_2 as u128, 5u16);
        // C s_332_4: const #31u : u8
        let s_332_4: u8 = 31;
        // C s_332_5: cast zx s_332_4 -> bv
        let s_332_5: Bits = Bits::new(s_332_4 as u128, 5u16);
        // S s_332_6: cmp-ne s_332_3 s_332_5
        let s_332_6: bool = ((s_332_3) != (s_332_5));
        // D s_332_7: write-var gs#142306 <= s_332_6
        fn_state.gs_142306 = s_332_6;
        // N s_332_8: jump b328
        return block_328(state, tracer, fn_state);
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_333_0: read-var CRm:u8
        let s_333_0: u8 = fn_state.CRm;
        // D s_333_1: cast zx s_333_0 -> bv
        let s_333_1: Bits = Bits::new(s_333_0 as u128, 4u16);
        // C s_333_2: const #13u : u8
        let s_333_2: u8 = 13;
        // C s_333_3: cast zx s_333_2 -> bv
        let s_333_3: Bits = Bits::new(s_333_2 as u128, 4u16);
        // D s_333_4: cmp-eq s_333_1 s_333_3
        let s_333_4: bool = ((s_333_1) == (s_333_3));
        // D s_333_5: write-var gs#142305 <= s_333_4
        fn_state.gs_142305 = s_333_4;
        // N s_333_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_334_0: read-var CRn:u8
        let s_334_0: u8 = fn_state.CRn;
        // D s_334_1: cast zx s_334_0 -> bv
        let s_334_1: Bits = Bits::new(s_334_0 as u128, 4u16);
        // C s_334_2: const #9u : u8
        let s_334_2: u8 = 9;
        // C s_334_3: cast zx s_334_2 -> bv
        let s_334_3: Bits = Bits::new(s_334_2 as u128, 4u16);
        // D s_334_4: cmp-eq s_334_1 s_334_3
        let s_334_4: bool = ((s_334_1) == (s_334_3));
        // D s_334_5: write-var gs#142304 <= s_334_4
        fn_state.gs_142304 = s_334_4;
        // N s_334_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_335_0: read-var opc1:u8
        let s_335_0: u8 = fn_state.opc1;
        // D s_335_1: cast zx s_335_0 -> bv
        let s_335_1: Bits = Bits::new(s_335_0 as u128, 3u16);
        // C s_335_2: const #0u : u8
        let s_335_2: u8 = 0;
        // C s_335_3: cast zx s_335_2 -> bv
        let s_335_3: Bits = Bits::new(s_335_2 as u128, 3u16);
        // D s_335_4: cmp-eq s_335_1 s_335_3
        let s_335_4: bool = ((s_335_1) == (s_335_3));
        // D s_335_5: write-var gs#142303 <= s_335_4
        fn_state.gs_142303 = s_335_4;
        // N s_335_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_336_0: const #0s : i
        let s_336_0: i128 = 0;
        // D s_336_1: read-var CRm:u8
        let s_336_1: u8 = fn_state.CRm;
        // D s_336_2: cast zx s_336_1 -> bv
        let s_336_2: Bits = Bits::new(s_336_1 as u128, 4u16);
        // C s_336_3: const #1s : i64
        let s_336_3: i64 = 1;
        // C s_336_4: cast zx s_336_3 -> i
        let s_336_4: i128 = (i128::try_from(s_336_3).unwrap());
        // C s_336_5: const #1s : i
        let s_336_5: i128 = 1;
        // C s_336_6: add s_336_5 s_336_4
        let s_336_6: i128 = (s_336_5 + s_336_4);
        // D s_336_7: bit-extract s_336_2 s_336_0 s_336_6
        let s_336_7: Bits = (Bits::new(
            ((s_336_2) >> (s_336_0)).value(),
            u16::try_from(s_336_6).unwrap(),
        ));
        // D s_336_8: cast reint s_336_7 -> u8
        let s_336_8: u8 = (s_336_7.value() as u8);
        // C s_336_9: const #0s : i
        let s_336_9: i128 = 0;
        // D s_336_10: read-var opc2:u8
        let s_336_10: u8 = fn_state.opc2;
        // D s_336_11: cast zx s_336_10 -> bv
        let s_336_11: Bits = Bits::new(s_336_10 as u128, 3u16);
        // C s_336_12: const #1s : i64
        let s_336_12: i64 = 1;
        // C s_336_13: cast zx s_336_12 -> i
        let s_336_13: i128 = (i128::try_from(s_336_12).unwrap());
        // C s_336_14: const #2s : i
        let s_336_14: i128 = 2;
        // C s_336_15: add s_336_14 s_336_13
        let s_336_15: i128 = (s_336_14 + s_336_13);
        // D s_336_16: bit-extract s_336_11 s_336_9 s_336_15
        let s_336_16: Bits = (Bits::new(
            ((s_336_11) >> (s_336_9)).value(),
            u16::try_from(s_336_15).unwrap(),
        ));
        // D s_336_17: cast reint s_336_16 -> u8
        let s_336_17: u8 = (s_336_16.value() as u8);
        // D s_336_18: cast zx s_336_8 -> bv
        let s_336_18: Bits = Bits::new(s_336_8 as u128, 2u16);
        // D s_336_19: cast zx s_336_17 -> bv
        let s_336_19: Bits = Bits::new(s_336_17 as u128, 3u16);
        // D s_336_20: cast reint s_336_18 -> u128
        let s_336_20: u128 = (s_336_18.value() as u128);
        // D s_336_21: size-of s_336_18
        let s_336_21: u16 = s_336_18.length();
        // D s_336_22: cast reint s_336_19 -> u128
        let s_336_22: u128 = (s_336_19.value() as u128);
        // D s_336_23: size-of s_336_19
        let s_336_23: u16 = s_336_19.length();
        // D s_336_24: lsl s_336_20 s_336_23
        let s_336_24: u128 = s_336_20 << s_336_23;
        // D s_336_25: or s_336_24 s_336_22
        let s_336_25: u128 = ((s_336_24) | (s_336_22));
        // D s_336_26: add s_336_21 s_336_23
        let s_336_26: u16 = (s_336_21 + s_336_23);
        // D s_336_27: create-bits s_336_25 s_336_26
        let s_336_27: Bits = Bits::new(s_336_25, s_336_26);
        // D s_336_28: cast reint s_336_27 -> u8
        let s_336_28: u8 = (s_336_27.value() as u8);
        // D s_336_29: cast zx s_336_28 -> bv
        let s_336_29: Bits = Bits::new(s_336_28 as u128, 5u16);
        // D s_336_30: cast zx s_336_29 -> i
        let s_336_30: i128 = (s_336_29.value() as i128);
        // D s_336_31: cast reint s_336_30 -> i64
        let s_336_31: i64 = (s_336_30 as i64);
        // C s_336_32: const #() : ()
        let s_336_32: () = ();
        // S s_336_33: call GetNumEventCounters(s_336_32)
        let s_336_33: i128 = GetNumEventCounters(state, tracer, s_336_32);
        // C s_336_34: const #1s : i
        let s_336_34: i128 = 1;
        // S s_336_35: sub s_336_33 s_336_34
        let s_336_35: i128 = ((s_336_33) - (s_336_34));
        // D s_336_36: cast zx s_336_31 -> i
        let s_336_36: i128 = (i128::try_from(s_336_31).unwrap());
        // D s_336_37: cmp-gt s_336_36 s_336_35
        let s_336_37: bool = ((s_336_36) > (s_336_35));
        // N s_336_38: branch s_336_37 b361 b337
        if s_336_37 {
            return block_361(state, tracer, fn_state);
        } else {
            return block_337(state, tracer, fn_state);
        };
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_337_0: const #0u : u8
        let s_337_0: bool = false;
        // D s_337_1: write-var gs#142349 <= s_337_0
        fn_state.gs_142349 = s_337_0;
        // N s_337_2: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_338_0: read-var gs#142349:u8
        let s_338_0: bool = fn_state.gs_142349;
        // N s_338_1: branch s_338_0 b360 b339
        if s_338_0 {
            return block_360(state, tracer, fn_state);
        } else {
            return block_339(state, tracer, fn_state);
        };
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #() : ()
        let s_339_0: () = ();
        // S s_339_1: call EL2Enabled(s_339_0)
        let s_339_1: bool = EL2Enabled(state, tracer, s_339_0);
        // N s_339_2: branch s_339_1 b356 b340
        if s_339_1 {
            return block_356(state, tracer, fn_state);
        } else {
            return block_340(state, tracer, fn_state);
        };
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_340_0: const #0u : u8
        let s_340_0: bool = false;
        // D s_340_1: write-var gs#142351 <= s_340_0
        fn_state.gs_142351 = s_340_0;
        // N s_340_2: jump b341
        return block_341(state, tracer, fn_state);
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_341_0: read-var gs#142351:u8
        let s_341_0: bool = fn_state.gs_142351;
        // N s_341_1: branch s_341_0 b355 b342
        if s_341_0 {
            return block_355(state, tracer, fn_state);
        } else {
            return block_342(state, tracer, fn_state);
        };
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_342_0: const #0u : u8
        let s_342_0: bool = false;
        // D s_342_1: write-var gs#142352 <= s_342_0
        fn_state.gs_142352 = s_342_0;
        // N s_342_2: jump b343
        return block_343(state, tracer, fn_state);
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_343_0: read-var gs#142352:u8
        let s_343_0: bool = fn_state.gs_142352;
        // N s_343_1: branch s_343_0 b354 b344
        if s_343_0 {
            return block_354(state, tracer, fn_state);
        } else {
            return block_344(state, tracer, fn_state);
        };
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_344_0: const #0u : u8
        let s_344_0: bool = false;
        // D s_344_1: write-var gs#142358 <= s_344_0
        fn_state.gs_142358 = s_344_0;
        // N s_344_2: jump b345
        return block_345(state, tracer, fn_state);
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_345_0: read-var gs#142358:u8
        let s_345_0: bool = fn_state.gs_142358;
        // D s_345_1: write-var gs#142359 <= s_345_0
        fn_state.gs_142359 = s_345_0;
        // N s_345_2: jump b346
        return block_346(state, tracer, fn_state);
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_346_0: read-var gs#142359:u8
        let s_346_0: bool = fn_state.gs_142359;
        // N s_346_1: branch s_346_0 b349 b347
        if s_346_0 {
            return block_349(state, tracer, fn_state);
        } else {
            return block_347(state, tracer, fn_state);
        };
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_347_0: jump b348
        return block_348(state, tracer, fn_state);
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_348_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_349_0: const #0s : i
        let s_349_0: i128 = 0;
        // D s_349_1: read-var CRm:u8
        let s_349_1: u8 = fn_state.CRm;
        // D s_349_2: cast zx s_349_1 -> bv
        let s_349_2: Bits = Bits::new(s_349_1 as u128, 4u16);
        // C s_349_3: const #1s : i64
        let s_349_3: i64 = 1;
        // C s_349_4: cast zx s_349_3 -> i
        let s_349_4: i128 = (i128::try_from(s_349_3).unwrap());
        // C s_349_5: const #1s : i
        let s_349_5: i128 = 1;
        // C s_349_6: add s_349_5 s_349_4
        let s_349_6: i128 = (s_349_5 + s_349_4);
        // D s_349_7: bit-extract s_349_2 s_349_0 s_349_6
        let s_349_7: Bits = (Bits::new(
            ((s_349_2) >> (s_349_0)).value(),
            u16::try_from(s_349_6).unwrap(),
        ));
        // D s_349_8: cast reint s_349_7 -> u8
        let s_349_8: u8 = (s_349_7.value() as u8);
        // C s_349_9: const #0s : i
        let s_349_9: i128 = 0;
        // D s_349_10: read-var opc2:u8
        let s_349_10: u8 = fn_state.opc2;
        // D s_349_11: cast zx s_349_10 -> bv
        let s_349_11: Bits = Bits::new(s_349_10 as u128, 3u16);
        // C s_349_12: const #1s : i64
        let s_349_12: i64 = 1;
        // C s_349_13: cast zx s_349_12 -> i
        let s_349_13: i128 = (i128::try_from(s_349_12).unwrap());
        // C s_349_14: const #2s : i
        let s_349_14: i128 = 2;
        // C s_349_15: add s_349_14 s_349_13
        let s_349_15: i128 = (s_349_14 + s_349_13);
        // D s_349_16: bit-extract s_349_11 s_349_9 s_349_15
        let s_349_16: Bits = (Bits::new(
            ((s_349_11) >> (s_349_9)).value(),
            u16::try_from(s_349_15).unwrap(),
        ));
        // D s_349_17: cast reint s_349_16 -> u8
        let s_349_17: u8 = (s_349_16.value() as u8);
        // D s_349_18: cast zx s_349_8 -> bv
        let s_349_18: Bits = Bits::new(s_349_8 as u128, 2u16);
        // D s_349_19: cast zx s_349_17 -> bv
        let s_349_19: Bits = Bits::new(s_349_17 as u128, 3u16);
        // D s_349_20: cast reint s_349_18 -> u128
        let s_349_20: u128 = (s_349_18.value() as u128);
        // D s_349_21: size-of s_349_18
        let s_349_21: u16 = s_349_18.length();
        // D s_349_22: cast reint s_349_19 -> u128
        let s_349_22: u128 = (s_349_19.value() as u128);
        // D s_349_23: size-of s_349_19
        let s_349_23: u16 = s_349_19.length();
        // D s_349_24: lsl s_349_20 s_349_23
        let s_349_24: u128 = s_349_20 << s_349_23;
        // D s_349_25: or s_349_24 s_349_22
        let s_349_25: u128 = ((s_349_24) | (s_349_22));
        // D s_349_26: add s_349_21 s_349_23
        let s_349_26: u16 = (s_349_21 + s_349_23);
        // D s_349_27: create-bits s_349_25 s_349_26
        let s_349_27: Bits = Bits::new(s_349_25, s_349_26);
        // D s_349_28: cast reint s_349_27 -> u8
        let s_349_28: u8 = (s_349_27.value() as u8);
        // D s_349_29: cast zx s_349_28 -> bv
        let s_349_29: Bits = Bits::new(s_349_28 as u128, 5u16);
        // D s_349_30: cast zx s_349_29 -> i
        let s_349_30: i128 = (s_349_29.value() as i128);
        // D s_349_31: cast reint s_349_30 -> i64
        let s_349_31: i64 = (s_349_30 as i64);
        // C s_349_32: const #() : ()
        let s_349_32: () = ();
        // S s_349_33: call GetNumEventCounters(s_349_32)
        let s_349_33: i128 = GetNumEventCounters(state, tracer, s_349_32);
        // C s_349_34: const #1s : i
        let s_349_34: i128 = 1;
        // S s_349_35: sub s_349_33 s_349_34
        let s_349_35: i128 = ((s_349_33) - (s_349_34));
        // D s_349_36: cast zx s_349_31 -> i
        let s_349_36: i128 = (i128::try_from(s_349_31).unwrap());
        // D s_349_37: cmp-gt s_349_36 s_349_35
        let s_349_37: bool = ((s_349_36) > (s_349_35));
        // N s_349_38: branch s_349_37 b353 b350
        if s_349_37 {
            return block_353(state, tracer, fn_state);
        } else {
            return block_350(state, tracer, fn_state);
        };
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_350_0: const #432u : u32
        let s_350_0: u32 = 432;
        // D s_350_1: read-reg s_350_0:u8
        let s_350_1: u8 = {
            let value = state.read_register::<u8>(s_350_0 as isize);
            tracer.read_register(s_350_0 as isize, value);
            value
        };
        // D s_350_2: call ELUsingAArch32(s_350_1)
        let s_350_2: bool = ELUsingAArch32(state, tracer, s_350_1);
        // N s_350_3: branch s_350_2 b352 b351
        if s_350_2 {
            return block_352(state, tracer, fn_state);
        } else {
            return block_351(state, tracer, fn_state);
        };
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_351_0: const #3u : u8
        let s_351_0: u8 = 3;
        // C s_351_1: cast zx s_351_0 -> bv
        let s_351_1: Bits = Bits::new(s_351_0 as u128, 8u16);
        // C s_351_2: cast zx s_351_1 -> i
        let s_351_2: i128 = (s_351_1.value() as i128);
        // C s_351_3: cast reint s_351_2 -> i64
        let s_351_3: i64 = (s_351_2 as i64);
        // C s_351_4: cast zx s_351_3 -> i
        let s_351_4: i128 = (i128::try_from(s_351_3).unwrap());
        // C s_351_5: const #432u : u32
        let s_351_5: u32 = 432;
        // D s_351_6: read-reg s_351_5:u8
        let s_351_6: u8 = {
            let value = state.read_register::<u8>(s_351_5 as isize);
            tracer.read_register(s_351_5 as isize, value);
            value
        };
        // D s_351_7: call AArch64_AArch32SystemAccessTrap(s_351_6, s_351_4)
        let s_351_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_351_6,
            s_351_4,
        );
        // N s_351_8: jump b348
        return block_348(state, tracer, fn_state);
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_352_0: const #3u : u8
        let s_352_0: u8 = 3;
        // C s_352_1: cast zx s_352_0 -> bv
        let s_352_1: Bits = Bits::new(s_352_0 as u128, 8u16);
        // C s_352_2: cast zx s_352_1 -> i
        let s_352_2: i128 = (s_352_1.value() as i128);
        // C s_352_3: cast reint s_352_2 -> i64
        let s_352_3: i64 = (s_352_2 as i64);
        // C s_352_4: cast zx s_352_3 -> i
        let s_352_4: i128 = (i128::try_from(s_352_3).unwrap());
        // S s_352_5: call AArch32_TakeHypTrapException(s_352_4)
        let s_352_5: () = AArch32_TakeHypTrapException(state, tracer, s_352_4);
        // N s_352_6: jump b348
        return block_348(state, tracer, fn_state);
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_353_0: panic
        panic!("{:?}", ());
        // N s_353_1: return
        return;
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_354_0: const #0s : i
        let s_354_0: i128 = 0;
        // D s_354_1: read-var CRm:u8
        let s_354_1: u8 = fn_state.CRm;
        // D s_354_2: cast zx s_354_1 -> bv
        let s_354_2: Bits = Bits::new(s_354_1 as u128, 4u16);
        // C s_354_3: const #1s : i64
        let s_354_3: i64 = 1;
        // C s_354_4: cast zx s_354_3 -> i
        let s_354_4: i128 = (i128::try_from(s_354_3).unwrap());
        // C s_354_5: const #1s : i
        let s_354_5: i128 = 1;
        // C s_354_6: add s_354_5 s_354_4
        let s_354_6: i128 = (s_354_5 + s_354_4);
        // D s_354_7: bit-extract s_354_2 s_354_0 s_354_6
        let s_354_7: Bits = (Bits::new(
            ((s_354_2) >> (s_354_0)).value(),
            u16::try_from(s_354_6).unwrap(),
        ));
        // D s_354_8: cast reint s_354_7 -> u8
        let s_354_8: u8 = (s_354_7.value() as u8);
        // C s_354_9: const #0s : i
        let s_354_9: i128 = 0;
        // D s_354_10: read-var opc2:u8
        let s_354_10: u8 = fn_state.opc2;
        // D s_354_11: cast zx s_354_10 -> bv
        let s_354_11: Bits = Bits::new(s_354_10 as u128, 3u16);
        // C s_354_12: const #1s : i64
        let s_354_12: i64 = 1;
        // C s_354_13: cast zx s_354_12 -> i
        let s_354_13: i128 = (i128::try_from(s_354_12).unwrap());
        // C s_354_14: const #2s : i
        let s_354_14: i128 = 2;
        // C s_354_15: add s_354_14 s_354_13
        let s_354_15: i128 = (s_354_14 + s_354_13);
        // D s_354_16: bit-extract s_354_11 s_354_9 s_354_15
        let s_354_16: Bits = (Bits::new(
            ((s_354_11) >> (s_354_9)).value(),
            u16::try_from(s_354_15).unwrap(),
        ));
        // D s_354_17: cast reint s_354_16 -> u8
        let s_354_17: u8 = (s_354_16.value() as u8);
        // D s_354_18: cast zx s_354_8 -> bv
        let s_354_18: Bits = Bits::new(s_354_8 as u128, 2u16);
        // D s_354_19: cast zx s_354_17 -> bv
        let s_354_19: Bits = Bits::new(s_354_17 as u128, 3u16);
        // D s_354_20: cast reint s_354_18 -> u128
        let s_354_20: u128 = (s_354_18.value() as u128);
        // D s_354_21: size-of s_354_18
        let s_354_21: u16 = s_354_18.length();
        // D s_354_22: cast reint s_354_19 -> u128
        let s_354_22: u128 = (s_354_19.value() as u128);
        // D s_354_23: size-of s_354_19
        let s_354_23: u16 = s_354_19.length();
        // D s_354_24: lsl s_354_20 s_354_23
        let s_354_24: u128 = s_354_20 << s_354_23;
        // D s_354_25: or s_354_24 s_354_22
        let s_354_25: u128 = ((s_354_24) | (s_354_22));
        // D s_354_26: add s_354_21 s_354_23
        let s_354_26: u16 = (s_354_21 + s_354_23);
        // D s_354_27: create-bits s_354_25 s_354_26
        let s_354_27: Bits = Bits::new(s_354_25, s_354_26);
        // D s_354_28: cast reint s_354_27 -> u8
        let s_354_28: u8 = (s_354_27.value() as u8);
        // D s_354_29: cast zx s_354_28 -> bv
        let s_354_29: Bits = Bits::new(s_354_28 as u128, 5u16);
        // D s_354_30: cast zx s_354_29 -> i
        let s_354_30: i128 = (s_354_29.value() as i128);
        // D s_354_31: cast reint s_354_30 -> i64
        let s_354_31: i64 = (s_354_30 as i64);
        // C s_354_32: const #() : ()
        let s_354_32: () = ();
        // S s_354_33: call AArch32_GetNumEventCountersAccessible(s_354_32)
        let s_354_33: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_354_32,
        );
        // C s_354_34: const #1s : i
        let s_354_34: i128 = 1;
        // S s_354_35: sub s_354_33 s_354_34
        let s_354_35: i128 = ((s_354_33) - (s_354_34));
        // D s_354_36: cast zx s_354_31 -> i
        let s_354_36: i128 = (i128::try_from(s_354_31).unwrap());
        // D s_354_37: cmp-gt s_354_36 s_354_35
        let s_354_37: bool = ((s_354_36) > (s_354_35));
        // D s_354_38: write-var gs#142358 <= s_354_37
        fn_state.gs_142358 = s_354_37;
        // N s_354_39: jump b345
        return block_345(state, tracer, fn_state);
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_355_0: const #102624u : u32
        let s_355_0: u32 = 102624;
        // D s_355_1: read-reg s_355_0:struct
        let s_355_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_355_0 as isize);
            tracer.read_register(s_355_0 as isize, value);
            value
        };
        // D s_355_2: call _get_PMUSERENR_EL0_Type_EN(s_355_1)
        let s_355_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_355_1);
        // D s_355_3: cast zx s_355_2 -> bv
        let s_355_3: Bits = Bits::new(s_355_2 as u128, 1u16);
        // C s_355_4: const #1u : u8
        let s_355_4: bool = true;
        // C s_355_5: cast zx s_355_4 -> bv
        let s_355_5: Bits = Bits::new(s_355_4 as u128, 1u16);
        // D s_355_6: cmp-eq s_355_3 s_355_5
        let s_355_6: bool = ((s_355_3) == (s_355_5));
        // D s_355_7: write-var gs#142352 <= s_355_6
        fn_state.gs_142352 = s_355_6;
        // N s_355_8: jump b343
        return block_343(state, tracer, fn_state);
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_356_0: const #16975u : u32
        let s_356_0: u32 = 16975;
        // D s_356_1: read-reg s_356_0:u8
        let s_356_1: u8 = {
            let value = state.read_register::<u8>(s_356_0 as isize);
            tracer.read_register(s_356_0 as isize, value);
            value
        };
        // D s_356_2: cast zx s_356_1 -> bv
        let s_356_2: Bits = Bits::new(s_356_1 as u128, 2u16);
        // C s_356_3: const #448u : u32
        let s_356_3: u32 = 448;
        // D s_356_4: read-reg s_356_3:u8
        let s_356_4: u8 = {
            let value = state.read_register::<u8>(s_356_3 as isize);
            tracer.read_register(s_356_3 as isize, value);
            value
        };
        // D s_356_5: cast zx s_356_4 -> bv
        let s_356_5: Bits = Bits::new(s_356_4 as u128, 2u16);
        // D s_356_6: cmp-eq s_356_2 s_356_5
        let s_356_6: bool = ((s_356_2) == (s_356_5));
        // N s_356_7: branch s_356_6 b359 b357
        if s_356_6 {
            return block_359(state, tracer, fn_state);
        } else {
            return block_357(state, tracer, fn_state);
        };
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_357_0: const #16975u : u32
        let s_357_0: u32 = 16975;
        // D s_357_1: read-reg s_357_0:u8
        let s_357_1: u8 = {
            let value = state.read_register::<u8>(s_357_0 as isize);
            tracer.read_register(s_357_0 as isize, value);
            value
        };
        // D s_357_2: cast zx s_357_1 -> bv
        let s_357_2: Bits = Bits::new(s_357_1 as u128, 2u16);
        // C s_357_3: const #440u : u32
        let s_357_3: u32 = 440;
        // D s_357_4: read-reg s_357_3:u8
        let s_357_4: u8 = {
            let value = state.read_register::<u8>(s_357_3 as isize);
            tracer.read_register(s_357_3 as isize, value);
            value
        };
        // D s_357_5: cast zx s_357_4 -> bv
        let s_357_5: Bits = Bits::new(s_357_4 as u128, 2u16);
        // D s_357_6: cmp-eq s_357_2 s_357_5
        let s_357_6: bool = ((s_357_2) == (s_357_5));
        // D s_357_7: write-var gs#142350 <= s_357_6
        fn_state.gs_142350 = s_357_6;
        // N s_357_8: jump b358
        return block_358(state, tracer, fn_state);
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_358_0: read-var gs#142350:u8
        let s_358_0: bool = fn_state.gs_142350;
        // D s_358_1: write-var gs#142351 <= s_358_0
        fn_state.gs_142351 = s_358_0;
        // N s_358_2: jump b341
        return block_341(state, tracer, fn_state);
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_359_0: const #1u : u8
        let s_359_0: bool = true;
        // D s_359_1: write-var gs#142350 <= s_359_0
        fn_state.gs_142350 = s_359_0;
        // N s_359_2: jump b358
        return block_358(state, tracer, fn_state);
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_360_0: const #1u : u8
        let s_360_0: bool = true;
        // D s_360_1: write-var gs#142359 <= s_360_0
        fn_state.gs_142359 = s_360_0;
        // N s_360_2: jump b346
        return block_346(state, tracer, fn_state);
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_361_0: const #0s : i
        let s_361_0: i128 = 0;
        // D s_361_1: read-var CRm:u8
        let s_361_1: u8 = fn_state.CRm;
        // D s_361_2: cast zx s_361_1 -> bv
        let s_361_2: Bits = Bits::new(s_361_1 as u128, 4u16);
        // C s_361_3: const #1s : i64
        let s_361_3: i64 = 1;
        // C s_361_4: cast zx s_361_3 -> i
        let s_361_4: i128 = (i128::try_from(s_361_3).unwrap());
        // C s_361_5: const #1s : i
        let s_361_5: i128 = 1;
        // C s_361_6: add s_361_5 s_361_4
        let s_361_6: i128 = (s_361_5 + s_361_4);
        // D s_361_7: bit-extract s_361_2 s_361_0 s_361_6
        let s_361_7: Bits = (Bits::new(
            ((s_361_2) >> (s_361_0)).value(),
            u16::try_from(s_361_6).unwrap(),
        ));
        // D s_361_8: cast reint s_361_7 -> u8
        let s_361_8: u8 = (s_361_7.value() as u8);
        // C s_361_9: const #0s : i
        let s_361_9: i128 = 0;
        // D s_361_10: read-var opc2:u8
        let s_361_10: u8 = fn_state.opc2;
        // D s_361_11: cast zx s_361_10 -> bv
        let s_361_11: Bits = Bits::new(s_361_10 as u128, 3u16);
        // C s_361_12: const #1s : i64
        let s_361_12: i64 = 1;
        // C s_361_13: cast zx s_361_12 -> i
        let s_361_13: i128 = (i128::try_from(s_361_12).unwrap());
        // C s_361_14: const #2s : i
        let s_361_14: i128 = 2;
        // C s_361_15: add s_361_14 s_361_13
        let s_361_15: i128 = (s_361_14 + s_361_13);
        // D s_361_16: bit-extract s_361_11 s_361_9 s_361_15
        let s_361_16: Bits = (Bits::new(
            ((s_361_11) >> (s_361_9)).value(),
            u16::try_from(s_361_15).unwrap(),
        ));
        // D s_361_17: cast reint s_361_16 -> u8
        let s_361_17: u8 = (s_361_16.value() as u8);
        // D s_361_18: cast zx s_361_8 -> bv
        let s_361_18: Bits = Bits::new(s_361_8 as u128, 2u16);
        // D s_361_19: cast zx s_361_17 -> bv
        let s_361_19: Bits = Bits::new(s_361_17 as u128, 3u16);
        // D s_361_20: cast reint s_361_18 -> u128
        let s_361_20: u128 = (s_361_18.value() as u128);
        // D s_361_21: size-of s_361_18
        let s_361_21: u16 = s_361_18.length();
        // D s_361_22: cast reint s_361_19 -> u128
        let s_361_22: u128 = (s_361_19.value() as u128);
        // D s_361_23: size-of s_361_19
        let s_361_23: u16 = s_361_19.length();
        // D s_361_24: lsl s_361_20 s_361_23
        let s_361_24: u128 = s_361_20 << s_361_23;
        // D s_361_25: or s_361_24 s_361_22
        let s_361_25: u128 = ((s_361_24) | (s_361_22));
        // D s_361_26: add s_361_21 s_361_23
        let s_361_26: u16 = (s_361_21 + s_361_23);
        // D s_361_27: create-bits s_361_25 s_361_26
        let s_361_27: Bits = Bits::new(s_361_25, s_361_26);
        // D s_361_28: cast reint s_361_27 -> u8
        let s_361_28: u8 = (s_361_27.value() as u8);
        // D s_361_29: cast zx s_361_28 -> bv
        let s_361_29: Bits = Bits::new(s_361_28 as u128, 5u16);
        // C s_361_30: const #31u : u8
        let s_361_30: u8 = 31;
        // C s_361_31: cast zx s_361_30 -> bv
        let s_361_31: Bits = Bits::new(s_361_30 as u128, 5u16);
        // D s_361_32: cmp-ne s_361_29 s_361_31
        let s_361_32: bool = ((s_361_29) != (s_361_31));
        // D s_361_33: write-var gs#142349 <= s_361_32
        fn_state.gs_142349 = s_361_32;
        // N s_361_34: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_362_0: const #2s : i
        let s_362_0: i128 = 2;
        // D s_362_1: read-var CRm:u8
        let s_362_1: u8 = fn_state.CRm;
        // D s_362_2: cast zx s_362_1 -> bv
        let s_362_2: Bits = Bits::new(s_362_1 as u128, 4u16);
        // C s_362_3: const #1s : i64
        let s_362_3: i64 = 1;
        // C s_362_4: cast zx s_362_3 -> i
        let s_362_4: i128 = (i128::try_from(s_362_3).unwrap());
        // C s_362_5: const #1s : i
        let s_362_5: i128 = 1;
        // C s_362_6: add s_362_5 s_362_4
        let s_362_6: i128 = (s_362_5 + s_362_4);
        // D s_362_7: bit-extract s_362_2 s_362_0 s_362_6
        let s_362_7: Bits = (Bits::new(
            ((s_362_2) >> (s_362_0)).value(),
            u16::try_from(s_362_6).unwrap(),
        ));
        // D s_362_8: cast reint s_362_7 -> u8
        let s_362_8: u8 = (s_362_7.value() as u8);
        // D s_362_9: cast zx s_362_8 -> bv
        let s_362_9: Bits = Bits::new(s_362_8 as u128, 2u16);
        // C s_362_10: const #2u : u8
        let s_362_10: u8 = 2;
        // C s_362_11: cast zx s_362_10 -> bv
        let s_362_11: Bits = Bits::new(s_362_10 as u128, 2u16);
        // D s_362_12: cmp-eq s_362_9 s_362_11
        let s_362_12: bool = ((s_362_9) == (s_362_11));
        // N s_362_13: branch s_362_12 b368 b363
        if s_362_12 {
            return block_368(state, tracer, fn_state);
        } else {
            return block_363(state, tracer, fn_state);
        };
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_363_0: const #2s : i
        let s_363_0: i128 = 2;
        // D s_363_1: read-var CRm:u8
        let s_363_1: u8 = fn_state.CRm;
        // D s_363_2: cast zx s_363_1 -> bv
        let s_363_2: Bits = Bits::new(s_363_1 as u128, 4u16);
        // C s_363_3: const #1s : i64
        let s_363_3: i64 = 1;
        // C s_363_4: cast zx s_363_3 -> i
        let s_363_4: i128 = (i128::try_from(s_363_3).unwrap());
        // C s_363_5: const #1s : i
        let s_363_5: i128 = 1;
        // C s_363_6: add s_363_5 s_363_4
        let s_363_6: i128 = (s_363_5 + s_363_4);
        // D s_363_7: bit-extract s_363_2 s_363_0 s_363_6
        let s_363_7: Bits = (Bits::new(
            ((s_363_2) >> (s_363_0)).value(),
            u16::try_from(s_363_6).unwrap(),
        ));
        // D s_363_8: cast reint s_363_7 -> u8
        let s_363_8: u8 = (s_363_7.value() as u8);
        // D s_363_9: cast zx s_363_8 -> bv
        let s_363_9: Bits = Bits::new(s_363_8 as u128, 2u16);
        // C s_363_10: const #3u : u8
        let s_363_10: u8 = 3;
        // C s_363_11: cast zx s_363_10 -> bv
        let s_363_11: Bits = Bits::new(s_363_10 as u128, 2u16);
        // D s_363_12: cmp-eq s_363_9 s_363_11
        let s_363_12: bool = ((s_363_9) == (s_363_11));
        // N s_363_13: branch s_363_12 b367 b364
        if s_363_12 {
            return block_367(state, tracer, fn_state);
        } else {
            return block_364(state, tracer, fn_state);
        };
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_364_0: const #0u : u8
        let s_364_0: bool = false;
        // D s_364_1: write-var gs#142299 <= s_364_0
        fn_state.gs_142299 = s_364_0;
        // N s_364_2: jump b365
        return block_365(state, tracer, fn_state);
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_365_0: read-var gs#142299:u8
        let s_365_0: bool = fn_state.gs_142299;
        // D s_365_1: write-var gs#142300 <= s_365_0
        fn_state.gs_142300 = s_365_0;
        // N s_365_2: jump b366
        return block_366(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_366_0: read-var gs#142300:u8
        let s_366_0: bool = fn_state.gs_142300;
        // D s_366_1: write-var gs#142301 <= s_366_0
        fn_state.gs_142301 = s_366_0;
        // N s_366_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_367_0: const #0s : i
        let s_367_0: i128 = 0;
        // D s_367_1: read-var CRm:u8
        let s_367_1: u8 = fn_state.CRm;
        // D s_367_2: cast zx s_367_1 -> bv
        let s_367_2: Bits = Bits::new(s_367_1 as u128, 4u16);
        // C s_367_3: const #1s : i64
        let s_367_3: i64 = 1;
        // C s_367_4: cast zx s_367_3 -> i
        let s_367_4: i128 = (i128::try_from(s_367_3).unwrap());
        // C s_367_5: const #1s : i
        let s_367_5: i128 = 1;
        // C s_367_6: add s_367_5 s_367_4
        let s_367_6: i128 = (s_367_5 + s_367_4);
        // D s_367_7: bit-extract s_367_2 s_367_0 s_367_6
        let s_367_7: Bits = (Bits::new(
            ((s_367_2) >> (s_367_0)).value(),
            u16::try_from(s_367_6).unwrap(),
        ));
        // D s_367_8: cast reint s_367_7 -> u8
        let s_367_8: u8 = (s_367_7.value() as u8);
        // C s_367_9: const #0s : i
        let s_367_9: i128 = 0;
        // D s_367_10: read-var opc2:u8
        let s_367_10: u8 = fn_state.opc2;
        // D s_367_11: cast zx s_367_10 -> bv
        let s_367_11: Bits = Bits::new(s_367_10 as u128, 3u16);
        // C s_367_12: const #1s : i64
        let s_367_12: i64 = 1;
        // C s_367_13: cast zx s_367_12 -> i
        let s_367_13: i128 = (i128::try_from(s_367_12).unwrap());
        // C s_367_14: const #2s : i
        let s_367_14: i128 = 2;
        // C s_367_15: add s_367_14 s_367_13
        let s_367_15: i128 = (s_367_14 + s_367_13);
        // D s_367_16: bit-extract s_367_11 s_367_9 s_367_15
        let s_367_16: Bits = (Bits::new(
            ((s_367_11) >> (s_367_9)).value(),
            u16::try_from(s_367_15).unwrap(),
        ));
        // D s_367_17: cast reint s_367_16 -> u8
        let s_367_17: u8 = (s_367_16.value() as u8);
        // D s_367_18: cast zx s_367_8 -> bv
        let s_367_18: Bits = Bits::new(s_367_8 as u128, 2u16);
        // D s_367_19: cast zx s_367_17 -> bv
        let s_367_19: Bits = Bits::new(s_367_17 as u128, 3u16);
        // D s_367_20: cast reint s_367_18 -> u128
        let s_367_20: u128 = (s_367_18.value() as u128);
        // D s_367_21: size-of s_367_18
        let s_367_21: u16 = s_367_18.length();
        // D s_367_22: cast reint s_367_19 -> u128
        let s_367_22: u128 = (s_367_19.value() as u128);
        // D s_367_23: size-of s_367_19
        let s_367_23: u16 = s_367_19.length();
        // D s_367_24: lsl s_367_20 s_367_23
        let s_367_24: u128 = s_367_20 << s_367_23;
        // D s_367_25: or s_367_24 s_367_22
        let s_367_25: u128 = ((s_367_24) | (s_367_22));
        // D s_367_26: add s_367_21 s_367_23
        let s_367_26: u16 = (s_367_21 + s_367_23);
        // D s_367_27: create-bits s_367_25 s_367_26
        let s_367_27: Bits = Bits::new(s_367_25, s_367_26);
        // D s_367_28: cast reint s_367_27 -> u8
        let s_367_28: u8 = (s_367_27.value() as u8);
        // D s_367_29: cast zx s_367_28 -> bv
        let s_367_29: Bits = Bits::new(s_367_28 as u128, 5u16);
        // C s_367_30: const #31u : u8
        let s_367_30: u8 = 31;
        // C s_367_31: cast zx s_367_30 -> bv
        let s_367_31: Bits = Bits::new(s_367_30 as u128, 5u16);
        // D s_367_32: cmp-ne s_367_29 s_367_31
        let s_367_32: bool = ((s_367_29) != (s_367_31));
        // D s_367_33: write-var gs#142299 <= s_367_32
        fn_state.gs_142299 = s_367_32;
        // N s_367_34: jump b365
        return block_365(state, tracer, fn_state);
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_368_0: const #1u : u8
        let s_368_0: bool = true;
        // D s_368_1: write-var gs#142300 <= s_368_0
        fn_state.gs_142300 = s_368_0;
        // N s_368_2: jump b366
        return block_366(state, tracer, fn_state);
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_369_0: read-var CRn:u8
        let s_369_0: u8 = fn_state.CRn;
        // D s_369_1: cast zx s_369_0 -> bv
        let s_369_1: Bits = Bits::new(s_369_0 as u128, 4u16);
        // C s_369_2: const #14u : u8
        let s_369_2: u8 = 14;
        // C s_369_3: cast zx s_369_2 -> bv
        let s_369_3: Bits = Bits::new(s_369_2 as u128, 4u16);
        // D s_369_4: cmp-eq s_369_1 s_369_3
        let s_369_4: bool = ((s_369_1) == (s_369_3));
        // D s_369_5: write-var gs#142290 <= s_369_4
        fn_state.gs_142290 = s_369_4;
        // N s_369_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_370_0: read-var opc1:u8
        let s_370_0: u8 = fn_state.opc1;
        // D s_370_1: cast zx s_370_0 -> bv
        let s_370_1: Bits = Bits::new(s_370_0 as u128, 3u16);
        // C s_370_2: const #0u : u8
        let s_370_2: u8 = 0;
        // C s_370_3: cast zx s_370_2 -> bv
        let s_370_3: Bits = Bits::new(s_370_2 as u128, 3u16);
        // D s_370_4: cmp-eq s_370_1 s_370_3
        let s_370_4: bool = ((s_370_1) == (s_370_3));
        // D s_370_5: write-var gs#142289 <= s_370_4
        fn_state.gs_142289 = s_370_4;
        // N s_370_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_371_0: const #3s : i
        let s_371_0: i128 = 3;
        // D s_371_1: read-var temprt:u32
        let s_371_1: u32 = fn_state.temprt;
        // D s_371_2: cast zx s_371_1 -> bv
        let s_371_2: Bits = Bits::new(s_371_1 as u128, 32u16);
        // C s_371_3: const #1u : u64
        let s_371_3: u64 = 1;
        // D s_371_4: bit-extract s_371_2 s_371_0 s_371_3
        let s_371_4: Bits = (Bits::new(
            ((s_371_2) >> (s_371_0)).value(),
            u16::try_from(s_371_3).unwrap(),
        ));
        // D s_371_5: cast reint s_371_4 -> u8
        let s_371_5: bool = ((s_371_4.value()) != 0);
        // C s_371_6: const #0s : i
        let s_371_6: i128 = 0;
        // C s_371_7: const #0u : u64
        let s_371_7: u64 = 0;
        // D s_371_8: cast zx s_371_5 -> u64
        let s_371_8: u64 = (s_371_5 as u64);
        // C s_371_9: const #1u : u64
        let s_371_9: u64 = 1;
        // D s_371_10: and s_371_8 s_371_9
        let s_371_10: u64 = ((s_371_8) & (s_371_9));
        // D s_371_11: cmp-eq s_371_10 s_371_9
        let s_371_11: bool = ((s_371_10) == (s_371_9));
        // D s_371_12: lsl s_371_8 s_371_6
        let s_371_12: u64 = s_371_8 << s_371_6;
        // D s_371_13: or s_371_7 s_371_12
        let s_371_13: u64 = ((s_371_7) | (s_371_12));
        // D s_371_14: cmpl s_371_12
        let s_371_14: u64 = !s_371_12;
        // D s_371_15: and s_371_7 s_371_14
        let s_371_15: u64 = ((s_371_7) & (s_371_14));
        // D s_371_16: select s_371_11 s_371_13 s_371_15
        let s_371_16: u64 = if s_371_11 { s_371_13 } else { s_371_15 };
        // D s_371_17: cast trunc s_371_16 -> u8
        let s_371_17: bool = ((s_371_16) != 0);
        // D s_371_18: cast zx s_371_17 -> bv
        let s_371_18: Bits = Bits::new(s_371_17 as u128, 1u16);
        // C s_371_19: const #1u : u8
        let s_371_19: bool = true;
        // C s_371_20: cast zx s_371_19 -> bv
        let s_371_20: Bits = Bits::new(s_371_19 as u128, 1u16);
        // D s_371_21: cmp-eq s_371_18 s_371_20
        let s_371_21: bool = ((s_371_18) == (s_371_20));
        // N s_371_22: branch s_371_21 b377 b372
        if s_371_21 {
            return block_377(state, tracer, fn_state);
        } else {
            return block_372(state, tracer, fn_state);
        };
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_372_0: const #0u : u8
        let s_372_0: bool = false;
        // D s_372_1: write-var gs#142369 <= s_372_0
        fn_state.gs_142369 = s_372_0;
        // N s_372_2: jump b373
        return block_373(state, tracer, fn_state);
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_373_0: read-var gs#142369:u8
        let s_373_0: bool = fn_state.gs_142369;
        // N s_373_1: branch s_373_0 b376 b374
        if s_373_0 {
            return block_376(state, tracer, fn_state);
        } else {
            return block_374(state, tracer, fn_state);
        };
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_374_0: jump b375
        return block_375(state, tracer, fn_state);
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_375_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_376_0: const #63s : i
        let s_376_0: i128 = 63;
        // C s_376_1: const #15848u : u32
        let s_376_1: u32 = 15848;
        // N s_376_2: write-reg s_376_1 <= s_376_0
        let s_376_2: () = {
            state.write_register::<i128>(s_376_1 as isize, s_376_0);
            tracer.write_register(s_376_1 as isize, s_376_0);
        };
        // N s_376_3: jump b375
        return block_375(state, tracer, fn_state);
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_377_0: const #() : ()
        let s_377_0: () = ();
        // S s_377_1: call PMCR_read(s_377_0)
        let s_377_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_377_0);
        // S s_377_2: call _get_PMCR_Type_D(s_377_1)
        let s_377_2: bool = u_get_PMCR_Type_D(state, tracer, s_377_1);
        // S s_377_3: cast zx s_377_2 -> bv
        let s_377_3: Bits = Bits::new(s_377_2 as u128, 1u16);
        // C s_377_4: const #0u : u8
        let s_377_4: bool = false;
        // C s_377_5: cast zx s_377_4 -> bv
        let s_377_5: Bits = Bits::new(s_377_4 as u128, 1u16);
        // S s_377_6: cmp-eq s_377_3 s_377_5
        let s_377_6: bool = ((s_377_3) == (s_377_5));
        // D s_377_7: write-var gs#142369 <= s_377_6
        fn_state.gs_142369 = s_377_6;
        // N s_377_8: jump b373
        return block_373(state, tracer, fn_state);
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_378_0: read-var CRm:u8
        let s_378_0: u8 = fn_state.CRm;
        // D s_378_1: cast zx s_378_0 -> bv
        let s_378_1: Bits = Bits::new(s_378_0 as u128, 4u16);
        // C s_378_2: const #12u : u8
        let s_378_2: u8 = 12;
        // C s_378_3: cast zx s_378_2 -> bv
        let s_378_3: Bits = Bits::new(s_378_2 as u128, 4u16);
        // D s_378_4: cmp-eq s_378_1 s_378_3
        let s_378_4: bool = ((s_378_1) == (s_378_3));
        // D s_378_5: write-var gs#142287 <= s_378_4
        fn_state.gs_142287 = s_378_4;
        // N s_378_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_379_0: read-var CRn:u8
        let s_379_0: u8 = fn_state.CRn;
        // D s_379_1: cast zx s_379_0 -> bv
        let s_379_1: Bits = Bits::new(s_379_0 as u128, 4u16);
        // C s_379_2: const #9u : u8
        let s_379_2: u8 = 9;
        // C s_379_3: cast zx s_379_2 -> bv
        let s_379_3: Bits = Bits::new(s_379_2 as u128, 4u16);
        // D s_379_4: cmp-eq s_379_1 s_379_3
        let s_379_4: bool = ((s_379_1) == (s_379_3));
        // D s_379_5: write-var gs#142286 <= s_379_4
        fn_state.gs_142286 = s_379_4;
        // N s_379_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_380_0: read-var opc2:u8
        let s_380_0: u8 = fn_state.opc2;
        // D s_380_1: cast zx s_380_0 -> bv
        let s_380_1: Bits = Bits::new(s_380_0 as u128, 3u16);
        // C s_380_2: const #0u : u8
        let s_380_2: u8 = 0;
        // C s_380_3: cast zx s_380_2 -> bv
        let s_380_3: Bits = Bits::new(s_380_2 as u128, 3u16);
        // D s_380_4: cmp-eq s_380_1 s_380_3
        let s_380_4: bool = ((s_380_1) == (s_380_3));
        // D s_380_5: write-var gs#142285 <= s_380_4
        fn_state.gs_142285 = s_380_4;
        // N s_380_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_381_0: read-var opc1:u8
        let s_381_0: u8 = fn_state.opc1;
        // D s_381_1: cast zx s_381_0 -> bv
        let s_381_1: Bits = Bits::new(s_381_0 as u128, 3u16);
        // C s_381_2: const #0u : u8
        let s_381_2: u8 = 0;
        // C s_381_3: cast zx s_381_2 -> bv
        let s_381_3: Bits = Bits::new(s_381_2 as u128, 3u16);
        // D s_381_4: cmp-eq s_381_1 s_381_3
        let s_381_4: bool = ((s_381_1) == (s_381_3));
        // D s_381_5: write-var gs#142284 <= s_381_4
        fn_state.gs_142284 = s_381_4;
        // N s_381_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_382_0: const #() : ()
        let s_382_0: () = ();
        // S s_382_1: call PMUCounterMask(s_382_0)
        let s_382_1: u64 = PMUCounterMask(state, tracer, s_382_0);
        // C s_382_2: const #0s : i
        let s_382_2: i128 = 0;
        // S s_382_3: cast zx s_382_1 -> bv
        let s_382_3: Bits = Bits::new(s_382_1 as u128, 64u16);
        // C s_382_4: const #1s : i64
        let s_382_4: i64 = 1;
        // C s_382_5: cast zx s_382_4 -> i
        let s_382_5: i128 = (i128::try_from(s_382_4).unwrap());
        // C s_382_6: const #31s : i
        let s_382_6: i128 = 31;
        // C s_382_7: add s_382_6 s_382_5
        let s_382_7: i128 = (s_382_6 + s_382_5);
        // D s_382_8: bit-extract s_382_3 s_382_2 s_382_7
        let s_382_8: Bits = (Bits::new(
            ((s_382_3) >> (s_382_2)).value(),
            u16::try_from(s_382_7).unwrap(),
        ));
        // D s_382_9: cast reint s_382_8 -> u32
        let s_382_9: u32 = (s_382_8.value() as u32);
        // D s_382_10: write-var mask <= s_382_9
        fn_state.mask = s_382_9;
        // D s_382_11: read-var opc2:u8
        let s_382_11: u8 = fn_state.opc2;
        // D s_382_12: cast zx s_382_11 -> bv
        let s_382_12: Bits = Bits::new(s_382_11 as u128, 3u16);
        // C s_382_13: const #3u : u8
        let s_382_13: u8 = 3;
        // C s_382_14: cast zx s_382_13 -> bv
        let s_382_14: Bits = Bits::new(s_382_13 as u128, 3u16);
        // D s_382_15: cmp-eq s_382_12 s_382_14
        let s_382_15: bool = ((s_382_12) == (s_382_14));
        // N s_382_16: branch s_382_15 b418 b383
        if s_382_15 {
            return block_418(state, tracer, fn_state);
        } else {
            return block_383(state, tracer, fn_state);
        };
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_383_0: const #0u : u8
        let s_383_0: bool = false;
        // D s_383_1: write-var gs#142375 <= s_383_0
        fn_state.gs_142375 = s_383_0;
        // N s_383_2: jump b384
        return block_384(state, tracer, fn_state);
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_384_0: read-var gs#142375:u8
        let s_384_0: bool = fn_state.gs_142375;
        // N s_384_1: branch s_384_0 b417 b385
        if s_384_0 {
            return block_417(state, tracer, fn_state);
        } else {
            return block_385(state, tracer, fn_state);
        };
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_385_0: jump b386
        return block_386(state, tracer, fn_state);
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_386_0: read-var opc2:u8
        let s_386_0: u8 = fn_state.opc2;
        // D s_386_1: cast zx s_386_0 -> bv
        let s_386_1: Bits = Bits::new(s_386_0 as u128, 3u16);
        // C s_386_2: const #3u : u8
        let s_386_2: u8 = 3;
        // C s_386_3: cast zx s_386_2 -> bv
        let s_386_3: Bits = Bits::new(s_386_2 as u128, 3u16);
        // D s_386_4: cmp-eq s_386_1 s_386_3
        let s_386_4: bool = ((s_386_1) == (s_386_3));
        // N s_386_5: branch s_386_4 b416 b387
        if s_386_4 {
            return block_416(state, tracer, fn_state);
        } else {
            return block_387(state, tracer, fn_state);
        };
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_387_0: const #0u : u8
        let s_387_0: bool = false;
        // D s_387_1: write-var gs#142376 <= s_387_0
        fn_state.gs_142376 = s_387_0;
        // N s_387_2: jump b388
        return block_388(state, tracer, fn_state);
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_388_0: read-var gs#142376:u8
        let s_388_0: bool = fn_state.gs_142376;
        // N s_388_1: branch s_388_0 b415 b389
        if s_388_0 {
            return block_415(state, tracer, fn_state);
        } else {
            return block_389(state, tracer, fn_state);
        };
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_389_0: jump b390
        return block_390(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_390_0: read-var opc2:u8
        let s_390_0: u8 = fn_state.opc2;
        // D s_390_1: cast zx s_390_0 -> bv
        let s_390_1: Bits = Bits::new(s_390_0 as u128, 3u16);
        // C s_390_2: const #2u : u8
        let s_390_2: u8 = 2;
        // C s_390_3: cast zx s_390_2 -> bv
        let s_390_3: Bits = Bits::new(s_390_2 as u128, 3u16);
        // D s_390_4: cmp-eq s_390_1 s_390_3
        let s_390_4: bool = ((s_390_1) == (s_390_3));
        // N s_390_5: branch s_390_4 b414 b391
        if s_390_4 {
            return block_414(state, tracer, fn_state);
        } else {
            return block_391(state, tracer, fn_state);
        };
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_391_0: const #0u : u8
        let s_391_0: bool = false;
        // D s_391_1: write-var gs#142377 <= s_391_0
        fn_state.gs_142377 = s_391_0;
        // N s_391_2: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_392_0: read-var gs#142377:u8
        let s_392_0: bool = fn_state.gs_142377;
        // N s_392_1: branch s_392_0 b413 b393
        if s_392_0 {
            return block_413(state, tracer, fn_state);
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
        // D s_394_0: read-var opc2:u8
        let s_394_0: u8 = fn_state.opc2;
        // D s_394_1: cast zx s_394_0 -> bv
        let s_394_1: Bits = Bits::new(s_394_0 as u128, 3u16);
        // C s_394_2: const #1u : u8
        let s_394_2: u8 = 1;
        // C s_394_3: cast zx s_394_2 -> bv
        let s_394_3: Bits = Bits::new(s_394_2 as u128, 3u16);
        // D s_394_4: cmp-eq s_394_1 s_394_3
        let s_394_4: bool = ((s_394_1) == (s_394_3));
        // N s_394_5: branch s_394_4 b412 b395
        if s_394_4 {
            return block_412(state, tracer, fn_state);
        } else {
            return block_395(state, tracer, fn_state);
        };
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_395_0: const #0u : u8
        let s_395_0: bool = false;
        // D s_395_1: write-var gs#142378 <= s_395_0
        fn_state.gs_142378 = s_395_0;
        // N s_395_2: jump b396
        return block_396(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_396_0: read-var gs#142378:u8
        let s_396_0: bool = fn_state.gs_142378;
        // N s_396_1: branch s_396_0 b411 b397
        if s_396_0 {
            return block_411(state, tracer, fn_state);
        } else {
            return block_397(state, tracer, fn_state);
        };
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_397_0: jump b398
        return block_398(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_398_0: read-var opc2:u8
        let s_398_0: u8 = fn_state.opc2;
        // D s_398_1: cast zx s_398_0 -> bv
        let s_398_1: Bits = Bits::new(s_398_0 as u128, 3u16);
        // C s_398_2: const #2u : u8
        let s_398_2: u8 = 2;
        // C s_398_3: cast zx s_398_2 -> bv
        let s_398_3: Bits = Bits::new(s_398_2 as u128, 3u16);
        // D s_398_4: cmp-eq s_398_1 s_398_3
        let s_398_4: bool = ((s_398_1) == (s_398_3));
        // N s_398_5: branch s_398_4 b410 b399
        if s_398_4 {
            return block_410(state, tracer, fn_state);
        } else {
            return block_399(state, tracer, fn_state);
        };
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_399_0: const #0u : u8
        let s_399_0: bool = false;
        // D s_399_1: write-var gs#142379 <= s_399_0
        fn_state.gs_142379 = s_399_0;
        // N s_399_2: jump b400
        return block_400(state, tracer, fn_state);
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_400_0: read-var gs#142379:u8
        let s_400_0: bool = fn_state.gs_142379;
        // N s_400_1: branch s_400_0 b409 b401
        if s_400_0 {
            return block_409(state, tracer, fn_state);
        } else {
            return block_401(state, tracer, fn_state);
        };
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_401_0: jump b402
        return block_402(state, tracer, fn_state);
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_402_0: read-var opc2:u8
        let s_402_0: u8 = fn_state.opc2;
        // D s_402_1: cast zx s_402_0 -> bv
        let s_402_1: Bits = Bits::new(s_402_0 as u128, 3u16);
        // C s_402_2: const #1u : u8
        let s_402_2: u8 = 1;
        // C s_402_3: cast zx s_402_2 -> bv
        let s_402_3: Bits = Bits::new(s_402_2 as u128, 3u16);
        // D s_402_4: cmp-eq s_402_1 s_402_3
        let s_402_4: bool = ((s_402_1) == (s_402_3));
        // N s_402_5: branch s_402_4 b408 b403
        if s_402_4 {
            return block_408(state, tracer, fn_state);
        } else {
            return block_403(state, tracer, fn_state);
        };
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_403_0: const #0u : u8
        let s_403_0: bool = false;
        // D s_403_1: write-var gs#142380 <= s_403_0
        fn_state.gs_142380 = s_403_0;
        // N s_403_2: jump b404
        return block_404(state, tracer, fn_state);
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_404_0: read-var gs#142380:u8
        let s_404_0: bool = fn_state.gs_142380;
        // N s_404_1: branch s_404_0 b407 b405
        if s_404_0 {
            return block_407(state, tracer, fn_state);
        } else {
            return block_405(state, tracer, fn_state);
        };
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_405_0: jump b406
        return block_406(state, tracer, fn_state);
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_406_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_407_0: const #() : ()
        let s_407_0: () = ();
        // S s_407_1: call PMINTENSET_read(s_407_0)
        let s_407_1: ProductType700c18a878c5601b = PMINTENSET_read(
            state,
            tracer,
            s_407_0,
        );
        // D s_407_2: write-var ga#248535 <= s_407_1
        fn_state.ga_248535 = s_407_1;
        // D s_407_3: read-var ga#248535.0:struct
        let s_407_3: u32 = fn_state.ga_248535._0;
        // D s_407_4: read-var temprt:u32
        let s_407_4: u32 = fn_state.temprt;
        // D s_407_5: cast zx s_407_4 -> bv
        let s_407_5: Bits = Bits::new(s_407_4 as u128, 32u16);
        // D s_407_6: read-var mask:u32
        let s_407_6: u32 = fn_state.mask;
        // D s_407_7: cast zx s_407_6 -> bv
        let s_407_7: Bits = Bits::new(s_407_6 as u128, 32u16);
        // D s_407_8: and s_407_5 s_407_7
        let s_407_8: Bits = ((s_407_5) & (s_407_7));
        // D s_407_9: cast reint s_407_8 -> u32
        let s_407_9: u32 = (s_407_8.value() as u32);
        // D s_407_10: cast zx s_407_3 -> bv
        let s_407_10: Bits = Bits::new(s_407_3 as u128, 32u16);
        // D s_407_11: cast zx s_407_9 -> bv
        let s_407_11: Bits = Bits::new(s_407_9 as u128, 32u16);
        // D s_407_12: or s_407_10 s_407_11
        let s_407_12: Bits = ((s_407_10) | (s_407_11));
        // D s_407_13: cast reint s_407_12 -> u32
        let s_407_13: u32 = (s_407_12.value() as u32);
        // D s_407_14: write-var temprt <= s_407_13
        fn_state.temprt = s_407_13;
        // N s_407_15: jump b406
        return block_406(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_408_0: read-var CRm:u8
        let s_408_0: u8 = fn_state.CRm;
        // D s_408_1: cast zx s_408_0 -> bv
        let s_408_1: Bits = Bits::new(s_408_0 as u128, 4u16);
        // C s_408_2: const #14u : u8
        let s_408_2: u8 = 14;
        // C s_408_3: cast zx s_408_2 -> bv
        let s_408_3: Bits = Bits::new(s_408_2 as u128, 4u16);
        // D s_408_4: cmp-eq s_408_1 s_408_3
        let s_408_4: bool = ((s_408_1) == (s_408_3));
        // D s_408_5: write-var gs#142380 <= s_408_4
        fn_state.gs_142380 = s_408_4;
        // N s_408_6: jump b404
        return block_404(state, tracer, fn_state);
    }
    fn block_409<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_409_0: const #() : ()
        let s_409_0: () = ();
        // S s_409_1: call PMINTENSET_read(s_409_0)
        let s_409_1: ProductType700c18a878c5601b = PMINTENSET_read(
            state,
            tracer,
            s_409_0,
        );
        // D s_409_2: write-var ga#248529 <= s_409_1
        fn_state.ga_248529 = s_409_1;
        // D s_409_3: read-var ga#248529.0:struct
        let s_409_3: u32 = fn_state.ga_248529._0;
        // D s_409_4: read-var temprt:u32
        let s_409_4: u32 = fn_state.temprt;
        // D s_409_5: cast zx s_409_4 -> bv
        let s_409_5: Bits = Bits::new(s_409_4 as u128, 32u16);
        // D s_409_6: read-var mask:u32
        let s_409_6: u32 = fn_state.mask;
        // D s_409_7: cast zx s_409_6 -> bv
        let s_409_7: Bits = Bits::new(s_409_6 as u128, 32u16);
        // D s_409_8: and s_409_5 s_409_7
        let s_409_8: Bits = ((s_409_5) & (s_409_7));
        // D s_409_9: cast reint s_409_8 -> u32
        let s_409_9: u32 = (s_409_8.value() as u32);
        // D s_409_10: cast zx s_409_9 -> bv
        let s_409_10: Bits = Bits::new(s_409_9 as u128, 32u16);
        // D s_409_11: not s_409_10
        let s_409_11: Bits = !s_409_10;
        // D s_409_12: cast reint s_409_11 -> u32
        let s_409_12: u32 = (s_409_11.value() as u32);
        // D s_409_13: cast zx s_409_3 -> bv
        let s_409_13: Bits = Bits::new(s_409_3 as u128, 32u16);
        // D s_409_14: cast zx s_409_12 -> bv
        let s_409_14: Bits = Bits::new(s_409_12 as u128, 32u16);
        // D s_409_15: and s_409_13 s_409_14
        let s_409_15: Bits = ((s_409_13) & (s_409_14));
        // D s_409_16: cast reint s_409_15 -> u32
        let s_409_16: u32 = (s_409_15.value() as u32);
        // D s_409_17: write-var temprt <= s_409_16
        fn_state.temprt = s_409_16;
        // N s_409_18: jump b402
        return block_402(state, tracer, fn_state);
    }
    fn block_410<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_410_0: read-var CRm:u8
        let s_410_0: u8 = fn_state.CRm;
        // D s_410_1: cast zx s_410_0 -> bv
        let s_410_1: Bits = Bits::new(s_410_0 as u128, 4u16);
        // C s_410_2: const #14u : u8
        let s_410_2: u8 = 14;
        // C s_410_3: cast zx s_410_2 -> bv
        let s_410_3: Bits = Bits::new(s_410_2 as u128, 4u16);
        // D s_410_4: cmp-eq s_410_1 s_410_3
        let s_410_4: bool = ((s_410_1) == (s_410_3));
        // D s_410_5: write-var gs#142379 <= s_410_4
        fn_state.gs_142379 = s_410_4;
        // N s_410_6: jump b400
        return block_400(state, tracer, fn_state);
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_411_0: const #() : ()
        let s_411_0: () = ();
        // S s_411_1: call PMCNTENSET_read(s_411_0)
        let s_411_1: ProductType700c18a878c5601b = PMCNTENSET_read(
            state,
            tracer,
            s_411_0,
        );
        // D s_411_2: write-var ga#248524 <= s_411_1
        fn_state.ga_248524 = s_411_1;
        // D s_411_3: read-var ga#248524.0:struct
        let s_411_3: u32 = fn_state.ga_248524._0;
        // D s_411_4: read-var temprt:u32
        let s_411_4: u32 = fn_state.temprt;
        // D s_411_5: cast zx s_411_4 -> bv
        let s_411_5: Bits = Bits::new(s_411_4 as u128, 32u16);
        // D s_411_6: read-var mask:u32
        let s_411_6: u32 = fn_state.mask;
        // D s_411_7: cast zx s_411_6 -> bv
        let s_411_7: Bits = Bits::new(s_411_6 as u128, 32u16);
        // D s_411_8: and s_411_5 s_411_7
        let s_411_8: Bits = ((s_411_5) & (s_411_7));
        // D s_411_9: cast reint s_411_8 -> u32
        let s_411_9: u32 = (s_411_8.value() as u32);
        // D s_411_10: cast zx s_411_3 -> bv
        let s_411_10: Bits = Bits::new(s_411_3 as u128, 32u16);
        // D s_411_11: cast zx s_411_9 -> bv
        let s_411_11: Bits = Bits::new(s_411_9 as u128, 32u16);
        // D s_411_12: or s_411_10 s_411_11
        let s_411_12: Bits = ((s_411_10) | (s_411_11));
        // D s_411_13: cast reint s_411_12 -> u32
        let s_411_13: u32 = (s_411_12.value() as u32);
        // D s_411_14: write-var temprt <= s_411_13
        fn_state.temprt = s_411_13;
        // N s_411_15: jump b398
        return block_398(state, tracer, fn_state);
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_412_0: read-var CRm:u8
        let s_412_0: u8 = fn_state.CRm;
        // D s_412_1: cast zx s_412_0 -> bv
        let s_412_1: Bits = Bits::new(s_412_0 as u128, 4u16);
        // C s_412_2: const #12u : u8
        let s_412_2: u8 = 12;
        // C s_412_3: cast zx s_412_2 -> bv
        let s_412_3: Bits = Bits::new(s_412_2 as u128, 4u16);
        // D s_412_4: cmp-eq s_412_1 s_412_3
        let s_412_4: bool = ((s_412_1) == (s_412_3));
        // D s_412_5: write-var gs#142378 <= s_412_4
        fn_state.gs_142378 = s_412_4;
        // N s_412_6: jump b396
        return block_396(state, tracer, fn_state);
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_413_0: const #() : ()
        let s_413_0: () = ();
        // S s_413_1: call PMCNTENSET_read(s_413_0)
        let s_413_1: ProductType700c18a878c5601b = PMCNTENSET_read(
            state,
            tracer,
            s_413_0,
        );
        // D s_413_2: write-var ga#248518 <= s_413_1
        fn_state.ga_248518 = s_413_1;
        // D s_413_3: read-var ga#248518.0:struct
        let s_413_3: u32 = fn_state.ga_248518._0;
        // D s_413_4: read-var temprt:u32
        let s_413_4: u32 = fn_state.temprt;
        // D s_413_5: cast zx s_413_4 -> bv
        let s_413_5: Bits = Bits::new(s_413_4 as u128, 32u16);
        // D s_413_6: read-var mask:u32
        let s_413_6: u32 = fn_state.mask;
        // D s_413_7: cast zx s_413_6 -> bv
        let s_413_7: Bits = Bits::new(s_413_6 as u128, 32u16);
        // D s_413_8: and s_413_5 s_413_7
        let s_413_8: Bits = ((s_413_5) & (s_413_7));
        // D s_413_9: cast reint s_413_8 -> u32
        let s_413_9: u32 = (s_413_8.value() as u32);
        // D s_413_10: cast zx s_413_9 -> bv
        let s_413_10: Bits = Bits::new(s_413_9 as u128, 32u16);
        // D s_413_11: not s_413_10
        let s_413_11: Bits = !s_413_10;
        // D s_413_12: cast reint s_413_11 -> u32
        let s_413_12: u32 = (s_413_11.value() as u32);
        // D s_413_13: cast zx s_413_3 -> bv
        let s_413_13: Bits = Bits::new(s_413_3 as u128, 32u16);
        // D s_413_14: cast zx s_413_12 -> bv
        let s_413_14: Bits = Bits::new(s_413_12 as u128, 32u16);
        // D s_413_15: and s_413_13 s_413_14
        let s_413_15: Bits = ((s_413_13) & (s_413_14));
        // D s_413_16: cast reint s_413_15 -> u32
        let s_413_16: u32 = (s_413_15.value() as u32);
        // D s_413_17: write-var temprt <= s_413_16
        fn_state.temprt = s_413_16;
        // N s_413_18: jump b394
        return block_394(state, tracer, fn_state);
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_414_0: read-var CRm:u8
        let s_414_0: u8 = fn_state.CRm;
        // D s_414_1: cast zx s_414_0 -> bv
        let s_414_1: Bits = Bits::new(s_414_0 as u128, 4u16);
        // C s_414_2: const #12u : u8
        let s_414_2: u8 = 12;
        // C s_414_3: cast zx s_414_2 -> bv
        let s_414_3: Bits = Bits::new(s_414_2 as u128, 4u16);
        // D s_414_4: cmp-eq s_414_1 s_414_3
        let s_414_4: bool = ((s_414_1) == (s_414_3));
        // D s_414_5: write-var gs#142377 <= s_414_4
        fn_state.gs_142377 = s_414_4;
        // N s_414_6: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_415_0: const #() : ()
        let s_415_0: () = ();
        // S s_415_1: call PMOVSSET_read(s_415_0)
        let s_415_1: ProductType700c18a878c5601b = PMOVSSET_read(state, tracer, s_415_0);
        // D s_415_2: write-var ga#248513 <= s_415_1
        fn_state.ga_248513 = s_415_1;
        // D s_415_3: read-var ga#248513.0:struct
        let s_415_3: u32 = fn_state.ga_248513._0;
        // D s_415_4: read-var temprt:u32
        let s_415_4: u32 = fn_state.temprt;
        // D s_415_5: cast zx s_415_4 -> bv
        let s_415_5: Bits = Bits::new(s_415_4 as u128, 32u16);
        // D s_415_6: read-var mask:u32
        let s_415_6: u32 = fn_state.mask;
        // D s_415_7: cast zx s_415_6 -> bv
        let s_415_7: Bits = Bits::new(s_415_6 as u128, 32u16);
        // D s_415_8: and s_415_5 s_415_7
        let s_415_8: Bits = ((s_415_5) & (s_415_7));
        // D s_415_9: cast reint s_415_8 -> u32
        let s_415_9: u32 = (s_415_8.value() as u32);
        // D s_415_10: cast zx s_415_3 -> bv
        let s_415_10: Bits = Bits::new(s_415_3 as u128, 32u16);
        // D s_415_11: cast zx s_415_9 -> bv
        let s_415_11: Bits = Bits::new(s_415_9 as u128, 32u16);
        // D s_415_12: or s_415_10 s_415_11
        let s_415_12: Bits = ((s_415_10) | (s_415_11));
        // D s_415_13: cast reint s_415_12 -> u32
        let s_415_13: u32 = (s_415_12.value() as u32);
        // D s_415_14: write-var temprt <= s_415_13
        fn_state.temprt = s_415_13;
        // N s_415_15: jump b390
        return block_390(state, tracer, fn_state);
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_416_0: read-var CRm:u8
        let s_416_0: u8 = fn_state.CRm;
        // D s_416_1: cast zx s_416_0 -> bv
        let s_416_1: Bits = Bits::new(s_416_0 as u128, 4u16);
        // C s_416_2: const #14u : u8
        let s_416_2: u8 = 14;
        // C s_416_3: cast zx s_416_2 -> bv
        let s_416_3: Bits = Bits::new(s_416_2 as u128, 4u16);
        // D s_416_4: cmp-eq s_416_1 s_416_3
        let s_416_4: bool = ((s_416_1) == (s_416_3));
        // D s_416_5: write-var gs#142376 <= s_416_4
        fn_state.gs_142376 = s_416_4;
        // N s_416_6: jump b388
        return block_388(state, tracer, fn_state);
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_417_0: const #() : ()
        let s_417_0: () = ();
        // S s_417_1: call PMOVSSET_read(s_417_0)
        let s_417_1: ProductType700c18a878c5601b = PMOVSSET_read(state, tracer, s_417_0);
        // D s_417_2: write-var ga#248507 <= s_417_1
        fn_state.ga_248507 = s_417_1;
        // D s_417_3: read-var ga#248507.0:struct
        let s_417_3: u32 = fn_state.ga_248507._0;
        // D s_417_4: read-var temprt:u32
        let s_417_4: u32 = fn_state.temprt;
        // D s_417_5: cast zx s_417_4 -> bv
        let s_417_5: Bits = Bits::new(s_417_4 as u128, 32u16);
        // D s_417_6: read-var mask:u32
        let s_417_6: u32 = fn_state.mask;
        // D s_417_7: cast zx s_417_6 -> bv
        let s_417_7: Bits = Bits::new(s_417_6 as u128, 32u16);
        // D s_417_8: and s_417_5 s_417_7
        let s_417_8: Bits = ((s_417_5) & (s_417_7));
        // D s_417_9: cast reint s_417_8 -> u32
        let s_417_9: u32 = (s_417_8.value() as u32);
        // D s_417_10: cast zx s_417_9 -> bv
        let s_417_10: Bits = Bits::new(s_417_9 as u128, 32u16);
        // D s_417_11: not s_417_10
        let s_417_11: Bits = !s_417_10;
        // D s_417_12: cast reint s_417_11 -> u32
        let s_417_12: u32 = (s_417_11.value() as u32);
        // D s_417_13: cast zx s_417_3 -> bv
        let s_417_13: Bits = Bits::new(s_417_3 as u128, 32u16);
        // D s_417_14: cast zx s_417_12 -> bv
        let s_417_14: Bits = Bits::new(s_417_12 as u128, 32u16);
        // D s_417_15: and s_417_13 s_417_14
        let s_417_15: Bits = ((s_417_13) & (s_417_14));
        // D s_417_16: cast reint s_417_15 -> u32
        let s_417_16: u32 = (s_417_15.value() as u32);
        // D s_417_17: write-var temprt <= s_417_16
        fn_state.temprt = s_417_16;
        // N s_417_18: jump b386
        return block_386(state, tracer, fn_state);
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_418_0: read-var CRm:u8
        let s_418_0: u8 = fn_state.CRm;
        // D s_418_1: cast zx s_418_0 -> bv
        let s_418_1: Bits = Bits::new(s_418_0 as u128, 4u16);
        // C s_418_2: const #12u : u8
        let s_418_2: u8 = 12;
        // C s_418_3: cast zx s_418_2 -> bv
        let s_418_3: Bits = Bits::new(s_418_2 as u128, 4u16);
        // D s_418_4: cmp-eq s_418_1 s_418_3
        let s_418_4: bool = ((s_418_1) == (s_418_3));
        // D s_418_5: write-var gs#142375 <= s_418_4
        fn_state.gs_142375 = s_418_4;
        // N s_418_6: jump b384
        return block_384(state, tracer, fn_state);
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_419_0: read-var opc2:u8
        let s_419_0: u8 = fn_state.opc2;
        // D s_419_1: cast zx s_419_0 -> bv
        let s_419_1: Bits = Bits::new(s_419_0 as u128, 3u16);
        // C s_419_2: const #1u : u8
        let s_419_2: u8 = 1;
        // C s_419_3: cast zx s_419_2 -> bv
        let s_419_3: Bits = Bits::new(s_419_2 as u128, 3u16);
        // D s_419_4: cmp-eq s_419_1 s_419_3
        let s_419_4: bool = ((s_419_1) == (s_419_3));
        // N s_419_5: branch s_419_4 b425 b420
        if s_419_4 {
            return block_425(state, tracer, fn_state);
        } else {
            return block_420(state, tracer, fn_state);
        };
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_420_0: read-var opc2:u8
        let s_420_0: u8 = fn_state.opc2;
        // D s_420_1: cast zx s_420_0 -> bv
        let s_420_1: Bits = Bits::new(s_420_0 as u128, 3u16);
        // C s_420_2: const #2u : u8
        let s_420_2: u8 = 2;
        // C s_420_3: cast zx s_420_2 -> bv
        let s_420_3: Bits = Bits::new(s_420_2 as u128, 3u16);
        // D s_420_4: cmp-eq s_420_1 s_420_3
        let s_420_4: bool = ((s_420_1) == (s_420_3));
        // N s_420_5: branch s_420_4 b424 b421
        if s_420_4 {
            return block_424(state, tracer, fn_state);
        } else {
            return block_421(state, tracer, fn_state);
        };
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_421_0: read-var opc2:u8
        let s_421_0: u8 = fn_state.opc2;
        // D s_421_1: cast zx s_421_0 -> bv
        let s_421_1: Bits = Bits::new(s_421_0 as u128, 3u16);
        // C s_421_2: const #3u : u8
        let s_421_2: u8 = 3;
        // C s_421_3: cast zx s_421_2 -> bv
        let s_421_3: Bits = Bits::new(s_421_2 as u128, 3u16);
        // D s_421_4: cmp-eq s_421_1 s_421_3
        let s_421_4: bool = ((s_421_1) == (s_421_3));
        // D s_421_5: write-var gs#142280 <= s_421_4
        fn_state.gs_142280 = s_421_4;
        // N s_421_6: jump b422
        return block_422(state, tracer, fn_state);
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_422_0: read-var gs#142280:u8
        let s_422_0: bool = fn_state.gs_142280;
        // D s_422_1: write-var gs#142281 <= s_422_0
        fn_state.gs_142281 = s_422_0;
        // N s_422_2: jump b423
        return block_423(state, tracer, fn_state);
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_423_0: read-var gs#142281:u8
        let s_423_0: bool = fn_state.gs_142281;
        // D s_423_1: write-var gs#142282 <= s_423_0
        fn_state.gs_142282 = s_423_0;
        // N s_423_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_424_0: const #1u : u8
        let s_424_0: bool = true;
        // D s_424_1: write-var gs#142280 <= s_424_0
        fn_state.gs_142280 = s_424_0;
        // N s_424_2: jump b422
        return block_422(state, tracer, fn_state);
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_425_0: const #1u : u8
        let s_425_0: bool = true;
        // D s_425_1: write-var gs#142281 <= s_425_0
        fn_state.gs_142281 = s_425_0;
        // N s_425_2: jump b423
        return block_423(state, tracer, fn_state);
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_426_0: read-var CRm:u8
        let s_426_0: u8 = fn_state.CRm;
        // D s_426_1: cast zx s_426_0 -> bv
        let s_426_1: Bits = Bits::new(s_426_0 as u128, 4u16);
        // C s_426_2: const #12u : u8
        let s_426_2: u8 = 12;
        // C s_426_3: cast zx s_426_2 -> bv
        let s_426_3: Bits = Bits::new(s_426_2 as u128, 4u16);
        // D s_426_4: cmp-eq s_426_1 s_426_3
        let s_426_4: bool = ((s_426_1) == (s_426_3));
        // N s_426_5: branch s_426_4 b429 b427
        if s_426_4 {
            return block_429(state, tracer, fn_state);
        } else {
            return block_427(state, tracer, fn_state);
        };
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_427_0: read-var CRm:u8
        let s_427_0: u8 = fn_state.CRm;
        // D s_427_1: cast zx s_427_0 -> bv
        let s_427_1: Bits = Bits::new(s_427_0 as u128, 4u16);
        // C s_427_2: const #14u : u8
        let s_427_2: u8 = 14;
        // C s_427_3: cast zx s_427_2 -> bv
        let s_427_3: Bits = Bits::new(s_427_2 as u128, 4u16);
        // D s_427_4: cmp-eq s_427_1 s_427_3
        let s_427_4: bool = ((s_427_1) == (s_427_3));
        // D s_427_5: write-var gs#142278 <= s_427_4
        fn_state.gs_142278 = s_427_4;
        // N s_427_6: jump b428
        return block_428(state, tracer, fn_state);
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_428_0: read-var gs#142278:u8
        let s_428_0: bool = fn_state.gs_142278;
        // D s_428_1: write-var gs#142279 <= s_428_0
        fn_state.gs_142279 = s_428_0;
        // N s_428_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_429<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_429_0: const #1u : u8
        let s_429_0: bool = true;
        // D s_429_1: write-var gs#142278 <= s_429_0
        fn_state.gs_142278 = s_429_0;
        // N s_429_2: jump b428
        return block_428(state, tracer, fn_state);
    }
    fn block_430<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_430_0: read-var CRn:u8
        let s_430_0: u8 = fn_state.CRn;
        // D s_430_1: cast zx s_430_0 -> bv
        let s_430_1: Bits = Bits::new(s_430_0 as u128, 4u16);
        // C s_430_2: const #9u : u8
        let s_430_2: u8 = 9;
        // C s_430_3: cast zx s_430_2 -> bv
        let s_430_3: Bits = Bits::new(s_430_2 as u128, 4u16);
        // D s_430_4: cmp-eq s_430_1 s_430_3
        let s_430_4: bool = ((s_430_1) == (s_430_3));
        // D s_430_5: write-var gs#142277 <= s_430_4
        fn_state.gs_142277 = s_430_4;
        // N s_430_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_431<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_431_0: read-var opc1:u8
        let s_431_0: u8 = fn_state.opc1;
        // D s_431_1: cast zx s_431_0 -> bv
        let s_431_1: Bits = Bits::new(s_431_0 as u128, 3u16);
        // C s_431_2: const #0u : u8
        let s_431_2: u8 = 0;
        // C s_431_3: cast zx s_431_2 -> bv
        let s_431_3: Bits = Bits::new(s_431_2 as u128, 3u16);
        // D s_431_4: cmp-eq s_431_1 s_431_3
        let s_431_4: bool = ((s_431_1) == (s_431_3));
        // D s_431_5: write-var gs#142276 <= s_431_4
        fn_state.gs_142276 = s_431_4;
        // N s_431_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_432<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_432_0: const #() : ()
        let s_432_0: () = ();
        // S s_432_1: call CurrentSecurityState(s_432_0)
        let s_432_1: u32 = CurrentSecurityState(state, tracer, s_432_0);
        // C s_432_2: const #0u : u32
        let s_432_2: u32 = 0;
        // S s_432_3: cmp-eq s_432_1 s_432_2
        let s_432_3: bool = ((s_432_1) == (s_432_2));
        // N s_432_4: branch s_432_3 b444 b433
        if s_432_3 {
            return block_444(state, tracer, fn_state);
        } else {
            return block_433(state, tracer, fn_state);
        };
    }
    fn block_433<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_433_0: const #0u : u8
        let s_433_0: bool = false;
        // D s_433_1: write-var gs#142388 <= s_433_0
        fn_state.gs_142388 = s_433_0;
        // N s_433_2: jump b434
        return block_434(state, tracer, fn_state);
    }
    fn block_434<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_434_0: read-var gs#142388:u8
        let s_434_0: bool = fn_state.gs_142388;
        // N s_434_1: branch s_434_0 b443 b435
        if s_434_0 {
            return block_443(state, tracer, fn_state);
        } else {
            return block_435(state, tracer, fn_state);
        };
    }
    fn block_435<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_435_0: const #0u : u8
        let s_435_0: bool = false;
        // D s_435_1: write-var gs#142389 <= s_435_0
        fn_state.gs_142389 = s_435_0;
        // N s_435_2: jump b436
        return block_436(state, tracer, fn_state);
    }
    fn block_436<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_436_0: read-var gs#142389:u8
        let s_436_0: bool = fn_state.gs_142389;
        // N s_436_1: branch s_436_0 b442 b437
        if s_436_0 {
            return block_442(state, tracer, fn_state);
        } else {
            return block_437(state, tracer, fn_state);
        };
    }
    fn block_437<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_437_0: const #0u : u8
        let s_437_0: bool = false;
        // D s_437_1: write-var gs#142390 <= s_437_0
        fn_state.gs_142390 = s_437_0;
        // N s_437_2: jump b438
        return block_438(state, tracer, fn_state);
    }
    fn block_438<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_438_0: read-var gs#142390:u8
        let s_438_0: bool = fn_state.gs_142390;
        // N s_438_1: branch s_438_0 b441 b439
        if s_438_0 {
            return block_441(state, tracer, fn_state);
        } else {
            return block_439(state, tracer, fn_state);
        };
    }
    fn block_439<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_439_0: jump b440
        return block_440(state, tracer, fn_state);
    }
    fn block_440<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_440_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_441<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_441_0: const #() : ()
        let s_441_0: () = ();
        // S s_441_1: call CPACR_read__1(s_441_0)
        let s_441_1: ProductType700c18a878c5601b = CPACR_read__1(state, tracer, s_441_0);
        // S s_441_2: call _get_CPACR_Type_cp11(s_441_1)
        let s_441_2: u8 = u_get_CPACR_Type_cp11(state, tracer, s_441_1);
        // C s_441_3: const #22s : i
        let s_441_3: i128 = 22;
        // D s_441_4: read-var temprt:u32
        let s_441_4: u32 = fn_state.temprt;
        // D s_441_5: cast zx s_441_4 -> bv
        let s_441_5: Bits = Bits::new(s_441_4 as u128, 32u16);
        // S s_441_6: cast zx s_441_2 -> bv
        let s_441_6: Bits = Bits::new(s_441_2 as u128, 2u16);
        // C s_441_7: const #1s : i
        let s_441_7: i128 = 1;
        // C s_441_8: const #1u : u64
        let s_441_8: u64 = 1;
        // C s_441_9: cast zx s_441_8 -> bv
        let s_441_9: Bits = Bits::new(s_441_8 as u128, 64u16);
        // C s_441_10: lsl s_441_9 s_441_7
        let s_441_10: Bits = s_441_9 << s_441_7;
        // C s_441_11: sub s_441_10 s_441_9
        let s_441_11: Bits = ((s_441_10) - (s_441_9));
        // S s_441_12: and s_441_6 s_441_11
        let s_441_12: Bits = ((s_441_6) & (s_441_11));
        // S s_441_13: lsl s_441_12 s_441_3
        let s_441_13: Bits = s_441_12 << s_441_3;
        // C s_441_14: lsl s_441_11 s_441_3
        let s_441_14: Bits = s_441_11 << s_441_3;
        // C s_441_15: cmpl s_441_14
        let s_441_15: Bits = !s_441_14;
        // D s_441_16: and s_441_5 s_441_15
        let s_441_16: Bits = ((s_441_5) & (s_441_15));
        // D s_441_17: or s_441_16 s_441_13
        let s_441_17: Bits = ((s_441_16) | (s_441_13));
        // D s_441_18: cast reint s_441_17 -> u32
        let s_441_18: u32 = (s_441_17.value() as u32);
        // D s_441_19: write-var temprt <= s_441_18
        fn_state.temprt = s_441_18;
        // C s_441_20: const #() : ()
        let s_441_20: () = ();
        // S s_441_21: call CPACR_read__1(s_441_20)
        let s_441_21: ProductType700c18a878c5601b = CPACR_read__1(
            state,
            tracer,
            s_441_20,
        );
        // S s_441_22: call _get_CPACR_Type_cp10(s_441_21)
        let s_441_22: u8 = u_get_CPACR_Type_cp10(state, tracer, s_441_21);
        // C s_441_23: const #20s : i
        let s_441_23: i128 = 20;
        // D s_441_24: read-var temprt:u32
        let s_441_24: u32 = fn_state.temprt;
        // D s_441_25: cast zx s_441_24 -> bv
        let s_441_25: Bits = Bits::new(s_441_24 as u128, 32u16);
        // S s_441_26: cast zx s_441_22 -> bv
        let s_441_26: Bits = Bits::new(s_441_22 as u128, 2u16);
        // C s_441_27: const #1s : i
        let s_441_27: i128 = 1;
        // C s_441_28: const #1u : u64
        let s_441_28: u64 = 1;
        // C s_441_29: cast zx s_441_28 -> bv
        let s_441_29: Bits = Bits::new(s_441_28 as u128, 64u16);
        // C s_441_30: lsl s_441_29 s_441_27
        let s_441_30: Bits = s_441_29 << s_441_27;
        // C s_441_31: sub s_441_30 s_441_29
        let s_441_31: Bits = ((s_441_30) - (s_441_29));
        // S s_441_32: and s_441_26 s_441_31
        let s_441_32: Bits = ((s_441_26) & (s_441_31));
        // S s_441_33: lsl s_441_32 s_441_23
        let s_441_33: Bits = s_441_32 << s_441_23;
        // C s_441_34: lsl s_441_31 s_441_23
        let s_441_34: Bits = s_441_31 << s_441_23;
        // C s_441_35: cmpl s_441_34
        let s_441_35: Bits = !s_441_34;
        // D s_441_36: and s_441_25 s_441_35
        let s_441_36: Bits = ((s_441_25) & (s_441_35));
        // D s_441_37: or s_441_36 s_441_33
        let s_441_37: Bits = ((s_441_36) | (s_441_33));
        // D s_441_38: cast reint s_441_37 -> u32
        let s_441_38: u32 = (s_441_37.value() as u32);
        // D s_441_39: write-var temprt <= s_441_38
        fn_state.temprt = s_441_38;
        // N s_441_40: jump b440
        return block_440(state, tracer, fn_state);
    }
    fn block_442<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_442_0: const #102488u : u32
        let s_442_0: u32 = 102488;
        // D s_442_1: read-reg s_442_0:struct
        let s_442_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_442_0 as isize);
            tracer.read_register(s_442_0 as isize, value);
            value
        };
        // D s_442_2: call _get_NSACR_Type_cp10(s_442_1)
        let s_442_2: bool = u_get_NSACR_Type_cp10(state, tracer, s_442_1);
        // D s_442_3: cast zx s_442_2 -> bv
        let s_442_3: Bits = Bits::new(s_442_2 as u128, 1u16);
        // C s_442_4: const #0u : u8
        let s_442_4: bool = false;
        // C s_442_5: cast zx s_442_4 -> bv
        let s_442_5: Bits = Bits::new(s_442_4 as u128, 1u16);
        // D s_442_6: cmp-eq s_442_3 s_442_5
        let s_442_6: bool = ((s_442_3) == (s_442_5));
        // D s_442_7: write-var gs#142390 <= s_442_6
        fn_state.gs_142390 = s_442_6;
        // N s_442_8: jump b438
        return block_438(state, tracer, fn_state);
    }
    fn block_443<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_443_0: const #424u : u32
        let s_443_0: u32 = 424;
        // D s_443_1: read-reg s_443_0:u8
        let s_443_1: u8 = {
            let value = state.read_register::<u8>(s_443_0 as isize);
            tracer.read_register(s_443_0 as isize, value);
            value
        };
        // D s_443_2: call ELUsingAArch32(s_443_1)
        let s_443_2: bool = ELUsingAArch32(state, tracer, s_443_1);
        // D s_443_3: write-var gs#142389 <= s_443_2
        fn_state.gs_142389 = s_443_2;
        // N s_443_4: jump b436
        return block_436(state, tracer, fn_state);
    }
    fn block_444<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_444_0: const #424u : u32
        let s_444_0: u32 = 424;
        // D s_444_1: read-reg s_444_0:u8
        let s_444_1: u8 = {
            let value = state.read_register::<u8>(s_444_0 as isize);
            tracer.read_register(s_444_0 as isize, value);
            value
        };
        // C s_444_2: const #2u : u8
        let s_444_2: u8 = 2;
        // D s_444_3: cmp-lt s_444_1 s_444_2
        let s_444_3: bool = ((s_444_1) < (s_444_2));
        // D s_444_4: write-var gs#142388 <= s_444_3
        fn_state.gs_142388 = s_444_3;
        // N s_444_5: jump b434
        return block_434(state, tracer, fn_state);
    }
    fn block_445<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_445_0: read-var opc2:u8
        let s_445_0: u8 = fn_state.opc2;
        // D s_445_1: cast zx s_445_0 -> bv
        let s_445_1: Bits = Bits::new(s_445_0 as u128, 3u16);
        // C s_445_2: const #2u : u8
        let s_445_2: u8 = 2;
        // C s_445_3: cast zx s_445_2 -> bv
        let s_445_3: Bits = Bits::new(s_445_2 as u128, 3u16);
        // D s_445_4: cmp-eq s_445_1 s_445_3
        let s_445_4: bool = ((s_445_1) == (s_445_3));
        // D s_445_5: write-var gs#142274 <= s_445_4
        fn_state.gs_142274 = s_445_4;
        // N s_445_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_446<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_446_0: read-var CRm:u8
        let s_446_0: u8 = fn_state.CRm;
        // D s_446_1: cast zx s_446_0 -> bv
        let s_446_1: Bits = Bits::new(s_446_0 as u128, 4u16);
        // C s_446_2: const #0u : u8
        let s_446_2: u8 = 0;
        // C s_446_3: cast zx s_446_2 -> bv
        let s_446_3: Bits = Bits::new(s_446_2 as u128, 4u16);
        // D s_446_4: cmp-eq s_446_1 s_446_3
        let s_446_4: bool = ((s_446_1) == (s_446_3));
        // D s_446_5: write-var gs#142273 <= s_446_4
        fn_state.gs_142273 = s_446_4;
        // N s_446_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_447<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_447_0: read-var CRn:u8
        let s_447_0: u8 = fn_state.CRn;
        // D s_447_1: cast zx s_447_0 -> bv
        let s_447_1: Bits = Bits::new(s_447_0 as u128, 4u16);
        // C s_447_2: const #1u : u8
        let s_447_2: u8 = 1;
        // C s_447_3: cast zx s_447_2 -> bv
        let s_447_3: Bits = Bits::new(s_447_2 as u128, 4u16);
        // D s_447_4: cmp-eq s_447_1 s_447_3
        let s_447_4: bool = ((s_447_1) == (s_447_3));
        // D s_447_5: write-var gs#142272 <= s_447_4
        fn_state.gs_142272 = s_447_4;
        // N s_447_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_448<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_448_0: read-var opc1:u8
        let s_448_0: u8 = fn_state.opc1;
        // D s_448_1: cast zx s_448_0 -> bv
        let s_448_1: Bits = Bits::new(s_448_0 as u128, 3u16);
        // C s_448_2: const #0u : u8
        let s_448_2: u8 = 0;
        // C s_448_3: cast zx s_448_2 -> bv
        let s_448_3: Bits = Bits::new(s_448_2 as u128, 3u16);
        // D s_448_4: cmp-eq s_448_1 s_448_3
        let s_448_4: bool = ((s_448_1) == (s_448_3));
        // D s_448_5: write-var gs#142271 <= s_448_4
        fn_state.gs_142271 = s_448_4;
        // N s_448_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_449<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_449_0: const #7s : i
        let s_449_0: i128 = 7;
        // D s_449_1: read-var temprt:u32
        let s_449_1: u32 = fn_state.temprt;
        // D s_449_2: cast zx s_449_1 -> bv
        let s_449_2: Bits = Bits::new(s_449_1 as u128, 32u16);
        // C s_449_3: const #1u : u64
        let s_449_3: u64 = 1;
        // D s_449_4: bit-extract s_449_2 s_449_0 s_449_3
        let s_449_4: Bits = (Bits::new(
            ((s_449_2) >> (s_449_0)).value(),
            u16::try_from(s_449_3).unwrap(),
        ));
        // D s_449_5: cast reint s_449_4 -> u8
        let s_449_5: bool = ((s_449_4.value()) != 0);
        // C s_449_6: const #0s : i
        let s_449_6: i128 = 0;
        // C s_449_7: const #0u : u64
        let s_449_7: u64 = 0;
        // D s_449_8: cast zx s_449_5 -> u64
        let s_449_8: u64 = (s_449_5 as u64);
        // C s_449_9: const #1u : u64
        let s_449_9: u64 = 1;
        // D s_449_10: and s_449_8 s_449_9
        let s_449_10: u64 = ((s_449_8) & (s_449_9));
        // D s_449_11: cmp-eq s_449_10 s_449_9
        let s_449_11: bool = ((s_449_10) == (s_449_9));
        // D s_449_12: lsl s_449_8 s_449_6
        let s_449_12: u64 = s_449_8 << s_449_6;
        // D s_449_13: or s_449_7 s_449_12
        let s_449_13: u64 = ((s_449_7) | (s_449_12));
        // D s_449_14: cmpl s_449_12
        let s_449_14: u64 = !s_449_12;
        // D s_449_15: and s_449_7 s_449_14
        let s_449_15: u64 = ((s_449_7) & (s_449_14));
        // D s_449_16: select s_449_11 s_449_13 s_449_15
        let s_449_16: u64 = if s_449_11 { s_449_13 } else { s_449_15 };
        // D s_449_17: cast trunc s_449_16 -> u8
        let s_449_17: bool = ((s_449_16) != 0);
        // D s_449_18: call Bit(s_449_17)
        let s_449_18: bool = Bit(state, tracer, s_449_17);
        // C s_449_19: const #8s : i
        let s_449_19: i128 = 8;
        // D s_449_20: read-var temprt:u32
        let s_449_20: u32 = fn_state.temprt;
        // D s_449_21: cast zx s_449_20 -> bv
        let s_449_21: Bits = Bits::new(s_449_20 as u128, 32u16);
        // C s_449_22: const #1u : u64
        let s_449_22: u64 = 1;
        // D s_449_23: bit-insert s_449_21 s_449_21 s_449_19 s_449_22
        let s_449_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_449_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_449_21.length(),
            );
            (s_449_21 & mask) | (s_449_21 << s_449_19)
        };
        // D s_449_24: cast reint s_449_23 -> u32
        let s_449_24: u32 = (s_449_23.value() as u32);
        // D s_449_25: write-var temprt <= s_449_24
        fn_state.temprt = s_449_24;
        // C s_449_26: const #5s : i
        let s_449_26: i128 = 5;
        // D s_449_27: read-var temprt:u32
        let s_449_27: u32 = fn_state.temprt;
        // D s_449_28: cast zx s_449_27 -> bv
        let s_449_28: Bits = Bits::new(s_449_27 as u128, 32u16);
        // C s_449_29: const #1u : u64
        let s_449_29: u64 = 1;
        // D s_449_30: bit-extract s_449_28 s_449_26 s_449_29
        let s_449_30: Bits = (Bits::new(
            ((s_449_28) >> (s_449_26)).value(),
            u16::try_from(s_449_29).unwrap(),
        ));
        // D s_449_31: cast reint s_449_30 -> u8
        let s_449_31: bool = ((s_449_30.value()) != 0);
        // C s_449_32: const #0s : i
        let s_449_32: i128 = 0;
        // C s_449_33: const #0u : u64
        let s_449_33: u64 = 0;
        // D s_449_34: cast zx s_449_31 -> u64
        let s_449_34: u64 = (s_449_31 as u64);
        // C s_449_35: const #1u : u64
        let s_449_35: u64 = 1;
        // D s_449_36: and s_449_34 s_449_35
        let s_449_36: u64 = ((s_449_34) & (s_449_35));
        // D s_449_37: cmp-eq s_449_36 s_449_35
        let s_449_37: bool = ((s_449_36) == (s_449_35));
        // D s_449_38: lsl s_449_34 s_449_32
        let s_449_38: u64 = s_449_34 << s_449_32;
        // D s_449_39: or s_449_33 s_449_38
        let s_449_39: u64 = ((s_449_33) | (s_449_38));
        // D s_449_40: cmpl s_449_38
        let s_449_40: u64 = !s_449_38;
        // D s_449_41: and s_449_33 s_449_40
        let s_449_41: u64 = ((s_449_33) & (s_449_40));
        // D s_449_42: select s_449_37 s_449_39 s_449_41
        let s_449_42: u64 = if s_449_37 { s_449_39 } else { s_449_41 };
        // D s_449_43: cast trunc s_449_42 -> u8
        let s_449_43: bool = ((s_449_42) != 0);
        // D s_449_44: call Bit(s_449_43)
        let s_449_44: bool = Bit(state, tracer, s_449_43);
        // C s_449_45: const #6s : i
        let s_449_45: i128 = 6;
        // D s_449_46: read-var temprt:u32
        let s_449_46: u32 = fn_state.temprt;
        // D s_449_47: cast zx s_449_46 -> bv
        let s_449_47: Bits = Bits::new(s_449_46 as u128, 32u16);
        // C s_449_48: const #1u : u64
        let s_449_48: u64 = 1;
        // D s_449_49: bit-insert s_449_47 s_449_47 s_449_45 s_449_48
        let s_449_49: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_449_48 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_449_47.length(),
            );
            (s_449_47 & mask) | (s_449_47 << s_449_45)
        };
        // D s_449_50: cast reint s_449_49 -> u32
        let s_449_50: u32 = (s_449_49.value() as u32);
        // D s_449_51: write-var temprt <= s_449_50
        fn_state.temprt = s_449_50;
        // N s_449_52: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_450<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_450_0: read-var opc2:u8
        let s_450_0: u8 = fn_state.opc2;
        // D s_450_1: cast zx s_450_0 -> bv
        let s_450_1: Bits = Bits::new(s_450_0 as u128, 3u16);
        // C s_450_2: const #5u : u8
        let s_450_2: u8 = 5;
        // C s_450_3: cast zx s_450_2 -> bv
        let s_450_3: Bits = Bits::new(s_450_2 as u128, 3u16);
        // D s_450_4: cmp-eq s_450_1 s_450_3
        let s_450_4: bool = ((s_450_1) == (s_450_3));
        // D s_450_5: write-var gs#142269 <= s_450_4
        fn_state.gs_142269 = s_450_4;
        // N s_450_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_451<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_451_0: read-var CRn:u8
        let s_451_0: u8 = fn_state.CRn;
        // D s_451_1: cast zx s_451_0 -> bv
        let s_451_1: Bits = Bits::new(s_451_0 as u128, 4u16);
        // C s_451_2: const #0u : u8
        let s_451_2: u8 = 0;
        // C s_451_3: cast zx s_451_2 -> bv
        let s_451_3: Bits = Bits::new(s_451_2 as u128, 4u16);
        // D s_451_4: cmp-eq s_451_1 s_451_3
        let s_451_4: bool = ((s_451_1) == (s_451_3));
        // D s_451_5: write-var gs#142268 <= s_451_4
        fn_state.gs_142268 = s_451_4;
        // N s_451_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_452<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_452_0: read-var opc1:u8
        let s_452_0: u8 = fn_state.opc1;
        // D s_452_1: cast zx s_452_0 -> bv
        let s_452_1: Bits = Bits::new(s_452_0 as u128, 3u16);
        // C s_452_2: const #0u : u8
        let s_452_2: u8 = 0;
        // C s_452_3: cast zx s_452_2 -> bv
        let s_452_3: Bits = Bits::new(s_452_2 as u128, 3u16);
        // D s_452_4: cmp-eq s_452_1 s_452_3
        let s_452_4: bool = ((s_452_1) == (s_452_3));
        // D s_452_5: write-var gs#142267 <= s_452_4
        fn_state.gs_142267 = s_452_4;
        // N s_452_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
