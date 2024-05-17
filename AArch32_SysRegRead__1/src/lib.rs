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
use u_update_DBGDSCRint_Type_RXfull::*;
use EDSCR_write::*;
use CacheConfigRead::*;
use DBGDSCRext_read::*;
use getISR::*;
use DBGDSCRext_write::*;
use ERRSELR_read::*;
use GetNumEventCounters::*;
use AArch32_TakeHypTrapException::*;
use PMSELR_read::*;
use PMEVTYPER_read::*;
use Zeros::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use AArch32_GetNumEventCountersAccessible::*;
use u_get_MDCR_EL3_Type_SPME::*;
use R_read::*;
use u_get_AMCGCR_Type_CG1NC::*;
use PMUCounterMask::*;
use u_update_DBGDSCRext_Type_RXfull::*;
use u_update_EDSCR_Type_RXfull::*;
use EL2Enabled::*;
use PMEVCNTR_read::*;
use u_get_ERRIDR_Type_NUM::*;
use ERRIDR_read::*;
use AMCGCR_read::*;
use PMCCFILTR_read::*;
use u_get_AMCGCR_Type_CG0NC::*;
use CSSELR_read::*;
use CurrentSecurityState::*;
use u_get_PMSELR_Type_SEL::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_PMUSERENR_EL0_Type_ER::*;
use u_get_PMSELR_EL0_Type_SEL::*;
use u__id::*;
use u_get_SDCR_Type_SPME::*;
use R_set::*;
use ELFromM32::*;
use integer_subrange::*;
use ELUsingAArch32::*;
use DBGDSCRint_read::*;
use AArch32_AutoGen_SysRegRead32::*;
use u_get_NSACR_Type_cp10::*;
use DBGDSCRint_write::*;
use u_get_ERRSELR_Type_SEL::*;
use EDSCR_read::*;
use DBGCLAIMCLR_read::*;
use common::*;
pub fn AArch32_SysRegRead__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cp_num: i128,
    instr: u32,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_142079: bool,
        ga_248406: ProductType5c790c8ef59cc8b2,
        gs_142198: bool,
        gs_141953: bool,
        gs_142027: bool,
        gs_142050: bool,
        gs_142201: bool,
        gs_142069: bool,
        gs_142019: bool,
        temp: u32,
        gs_141943: bool,
        ga_248332: u32,
        gs_142017: bool,
        gs_141934: bool,
        gs_141984: bool,
        gs_142052: bool,
        CRn: u8,
        gs_142064: bool,
        gs_142073: bool,
        gs_142053: bool,
        ga_248056: ProductTypea5cc8de4daab131c,
        gs_141980: bool,
        gs_141965: bool,
        gs_142055: bool,
        gs_141960: bool,
        gs_141963: bool,
        gs_142065: bool,
        gs_142131: bool,
        gs_142059: bool,
        gs_142034: bool,
        gs_142068: bool,
        gs_141958: bool,
        gs_141928: bool,
        ga_248226: ProductType700c18a878c5601b,
        gs_142103: bool,
        gs_142181: bool,
        opc1: u8,
        gs_142026: bool,
        gs_142057: bool,
        gs_142075: bool,
        gs_142183: bool,
        gs_142155: bool,
        gs_141936: bool,
        gs_142016: bool,
        gs_141981: bool,
        gs_142130: bool,
        gs_142015: bool,
        gs_141983: bool,
        gs_142021: bool,
        gs_142032: bool,
        gs_142035: bool,
        ga_248362: ProductType700c18a878c5601b,
        gs_141990: bool,
        gs_142162: bool,
        ga_248367: ProductType700c18a878c5601b,
        gs_142025: bool,
        gs_141959: bool,
        gs_142070: bool,
        gs_142071: bool,
        gs_142024: bool,
        gs_142074: bool,
        indexshadow_1012: i64,
        ga_248333: bool,
        gs_142018: bool,
        gs_142080: bool,
        gs_141955: bool,
        gs_142054: bool,
        gs_142180: bool,
        gs_142062: bool,
        gs_142110: bool,
        gs_142063: bool,
        ga_248320: ProductType700c18a878c5601b,
        gs_142060: bool,
        ga_248224: ProductType700c18a878c5601b,
        gs_142113: bool,
        el: u8,
        gs_142048: bool,
        gs_141948: bool,
        gs_142092: bool,
        gs_142111: bool,
        gs_141931: bool,
        gs_142045: bool,
        gs_142030: bool,
        opc2: u8,
        gs_141929: bool,
        gs_142020: bool,
        gs_142112: bool,
        gs_142029: bool,
        nshadow_1013: i64,
        gs_141966: bool,
        gs_141954: bool,
        gs_142078: bool,
        pmselr: i64,
        gs_141942: bool,
        gs_141952: bool,
        gs_141977: bool,
        gs_142044: bool,
        gs_142097: bool,
        gs_142161: bool,
        gs_141927: bool,
        gs_142154: bool,
        gs_141982: bool,
        gs_141930: bool,
        ga_248399: ProductType5c790c8ef59cc8b2,
        gs_141951: bool,
        gs_142066: bool,
        gs_142184: bool,
        gs_141950: bool,
        gs_142023: bool,
        gs_142197: bool,
        gs_141962: bool,
        gs_142114: bool,
        gs_141937: bool,
        CRm: u8,
        gs_142058: bool,
        gs_141961: bool,
        gs_142046: bool,
        gs_141991: bool,
        gs_142049: bool,
        gs_142199: bool,
        gs_142031: bool,
        gs_141933: bool,
        gs_142076: bool,
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
        // C s_0_0: const #16983u : u32
        let s_0_0: u32 = 16983;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call ELFromM32(s_0_1)
        let s_0_2: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_0_1);
        // D s_0_3: write-var ga#248056 <= s_0_2
        fn_state.ga_248056 = s_0_2;
        // D s_0_4: read-var ga#248056.1:struct
        let s_0_4: u8 = fn_state.ga_248056._1;
        // D s_0_5: write-var el <= s_0_4
        fn_state.el = s_0_4;
        // C s_0_6: const #21s : i
        let s_0_6: i128 = 21;
        // C s_0_7: const #3s : i
        let s_0_7: i128 = 3;
        // D s_0_8: read-var instr:u32
        let s_0_8: u32 = fn_state.instr;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_6 s_0_7
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_6)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: u8 = (s_0_10.value() as u8);
        // D s_0_12: write-var opc1 <= s_0_11
        fn_state.opc1 = s_0_11;
        // C s_0_13: const #16s : i
        let s_0_13: i128 = 16;
        // C s_0_14: const #4s : i
        let s_0_14: i128 = 4;
        // D s_0_15: read-var instr:u32
        let s_0_15: u32 = fn_state.instr;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 32u16);
        // D s_0_17: bit-extract s_0_16 s_0_13 s_0_14
        let s_0_17: Bits = (Bits::new(
            ((s_0_16) >> (s_0_13)).value(),
            u16::try_from(s_0_14).unwrap(),
        ));
        // D s_0_18: cast reint s_0_17 -> u8
        let s_0_18: u8 = (s_0_17.value() as u8);
        // D s_0_19: write-var CRn <= s_0_18
        fn_state.CRn = s_0_18;
        // C s_0_20: const #0s : i
        let s_0_20: i128 = 0;
        // C s_0_21: const #4s : i
        let s_0_21: i128 = 4;
        // D s_0_22: read-var instr:u32
        let s_0_22: u32 = fn_state.instr;
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 32u16);
        // D s_0_24: bit-extract s_0_23 s_0_20 s_0_21
        let s_0_24: Bits = (Bits::new(
            ((s_0_23) >> (s_0_20)).value(),
            u16::try_from(s_0_21).unwrap(),
        ));
        // D s_0_25: cast reint s_0_24 -> u8
        let s_0_25: u8 = (s_0_24.value() as u8);
        // D s_0_26: write-var CRm <= s_0_25
        fn_state.CRm = s_0_25;
        // C s_0_27: const #5s : i
        let s_0_27: i128 = 5;
        // C s_0_28: const #3s : i
        let s_0_28: i128 = 3;
        // D s_0_29: read-var instr:u32
        let s_0_29: u32 = fn_state.instr;
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 32u16);
        // D s_0_31: bit-extract s_0_30 s_0_27 s_0_28
        let s_0_31: Bits = (Bits::new(
            ((s_0_30) >> (s_0_27)).value(),
            u16::try_from(s_0_28).unwrap(),
        ));
        // D s_0_32: cast reint s_0_31 -> u8
        let s_0_32: u8 = (s_0_31.value() as u8);
        // D s_0_33: write-var opc2 <= s_0_32
        fn_state.opc2 = s_0_32;
        // C s_0_34: const #15s : i
        let s_0_34: i128 = 15;
        // D s_0_35: read-var cp_num:i
        let s_0_35: i128 = fn_state.cp_num;
        // D s_0_36: cmp-eq s_0_35 s_0_34
        let s_0_36: bool = ((s_0_35) == (s_0_34));
        // N s_0_37: branch s_0_36 b428 b1
        if s_0_36 {
            return block_428(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#141927 <= s_1_0
        fn_state.gs_141927 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#141927:u8
        let s_2_0: bool = fn_state.gs_141927;
        // N s_2_1: branch s_2_0 b424 b3
        if s_2_0 {
            return block_424(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#141929 <= s_3_0
        fn_state.gs_141929 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#141929:u8
        let s_4_0: bool = fn_state.gs_141929;
        // N s_4_1: branch s_4_0 b423 b5
        if s_4_0 {
            return block_423(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#141930 <= s_5_0
        fn_state.gs_141930 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#141930:u8
        let s_6_0: bool = fn_state.gs_141930;
        // N s_6_1: branch s_6_0 b422 b7
        if s_6_0 {
            return block_422(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#141931 <= s_7_0
        fn_state.gs_141931 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#141931:u8
        let s_8_0: bool = fn_state.gs_141931;
        // N s_8_1: branch s_8_0 b421 b9
        if s_8_0 {
            return block_421(state, tracer, fn_state);
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
        // C s_10_0: const #15s : i
        let s_10_0: i128 = 15;
        // D s_10_1: read-var cp_num:i
        let s_10_1: i128 = fn_state.cp_num;
        // D s_10_2: cmp-eq s_10_1 s_10_0
        let s_10_2: bool = ((s_10_1) == (s_10_0));
        // N s_10_3: branch s_10_2 b420 b11
        if s_10_2 {
            return block_420(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#141933 <= s_11_0
        fn_state.gs_141933 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#141933:u8
        let s_12_0: bool = fn_state.gs_141933;
        // N s_12_1: branch s_12_0 b419 b13
        if s_12_0 {
            return block_419(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#141934 <= s_13_0
        fn_state.gs_141934 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#141934:u8
        let s_14_0: bool = fn_state.gs_141934;
        // N s_14_1: branch s_14_0 b408 b15
        if s_14_0 {
            return block_408(state, tracer, fn_state);
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
        // C s_16_0: const #15s : i
        let s_16_0: i128 = 15;
        // D s_16_1: read-var cp_num:i
        let s_16_1: i128 = fn_state.cp_num;
        // D s_16_2: cmp-eq s_16_1 s_16_0
        let s_16_2: bool = ((s_16_1) == (s_16_0));
        // N s_16_3: branch s_16_2 b407 b17
        if s_16_2 {
            return block_407(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#141936 <= s_17_0
        fn_state.gs_141936 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#141936:u8
        let s_18_0: bool = fn_state.gs_141936;
        // N s_18_1: branch s_18_0 b406 b19
        if s_18_0 {
            return block_406(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#141937 <= s_19_0
        fn_state.gs_141937 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#141937:u8
        let s_20_0: bool = fn_state.gs_141937;
        // N s_20_1: branch s_20_0 b402 b21
        if s_20_0 {
            return block_402(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#141943 <= s_21_0
        fn_state.gs_141943 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#141943:u8
        let s_22_0: bool = fn_state.gs_141943;
        // N s_22_1: branch s_22_0 b401 b23
        if s_22_0 {
            return block_401(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#141948 <= s_23_0
        fn_state.gs_141948 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#141948:u8
        let s_24_0: bool = fn_state.gs_141948;
        // N s_24_1: branch s_24_0 b369 b25
        if s_24_0 {
            return block_369(state, tracer, fn_state);
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
        // C s_26_0: const #15s : i
        let s_26_0: i128 = 15;
        // D s_26_1: read-var cp_num:i
        let s_26_1: i128 = fn_state.cp_num;
        // D s_26_2: cmp-eq s_26_1 s_26_0
        let s_26_2: bool = ((s_26_1) == (s_26_0));
        // N s_26_3: branch s_26_2 b368 b27
        if s_26_2 {
            return block_368(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#141950 <= s_27_0
        fn_state.gs_141950 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#141950:u8
        let s_28_0: bool = fn_state.gs_141950;
        // N s_28_1: branch s_28_0 b367 b29
        if s_28_0 {
            return block_367(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#141951 <= s_29_0
        fn_state.gs_141951 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#141951:u8
        let s_30_0: bool = fn_state.gs_141951;
        // N s_30_1: branch s_30_0 b366 b31
        if s_30_0 {
            return block_366(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#141952 <= s_31_0
        fn_state.gs_141952 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#141952:u8
        let s_32_0: bool = fn_state.gs_141952;
        // N s_32_1: branch s_32_0 b362 b33
        if s_32_0 {
            return block_362(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#141954 <= s_33_0
        fn_state.gs_141954 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#141954:u8
        let s_34_0: bool = fn_state.gs_141954;
        // N s_34_1: branch s_34_0 b361 b35
        if s_34_0 {
            return block_361(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#141955 <= s_35_0
        fn_state.gs_141955 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#141955:u8
        let s_36_0: bool = fn_state.gs_141955;
        // N s_36_1: branch s_36_0 b329 b37
        if s_36_0 {
            return block_329(state, tracer, fn_state);
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
        // D s_38_0: read-var t:i
        let s_38_0: i128 = fn_state.t;
        // D s_38_1: call R_read(s_38_0)
        let s_38_1: u32 = R_read(state, tracer, s_38_0);
        // D s_38_2: write-var temp <= s_38_1
        fn_state.temp = s_38_1;
        // C s_38_3: const #3s : i
        let s_38_3: i128 = 3;
        // C s_38_4: const #0s : i
        let s_38_4: i128 = 0;
        // D s_38_5: read-var cp_num:i
        let s_38_5: i128 = fn_state.cp_num;
        // D s_38_6: call integer_subrange(s_38_5, s_38_3, s_38_4)
        let s_38_6: Bits = integer_subrange(state, tracer, s_38_5, s_38_3, s_38_4);
        // D s_38_7: cast reint s_38_6 -> u8
        let s_38_7: u8 = (s_38_6.value() as u8);
        // D s_38_8: read-var el:u8
        let s_38_8: u8 = fn_state.el;
        // D s_38_9: read-var opc1:u8
        let s_38_9: u8 = fn_state.opc1;
        // D s_38_10: read-var CRn:u8
        let s_38_10: u8 = fn_state.CRn;
        // D s_38_11: read-var opc2:u8
        let s_38_11: u8 = fn_state.opc2;
        // D s_38_12: read-var CRm:u8
        let s_38_12: u8 = fn_state.CRm;
        // D s_38_13: read-var t:i
        let s_38_13: i128 = fn_state.t;
        // D s_38_14: call AArch32_AutoGen_SysRegRead32(s_38_8, s_38_7, s_38_9, s_38_10, s_38_11, s_38_12, s_38_13)
        let s_38_14: () = AArch32_AutoGen_SysRegRead32(
            state,
            tracer,
            s_38_8,
            s_38_7,
            s_38_9,
            s_38_10,
            s_38_11,
            s_38_12,
            s_38_13,
        );
        // N s_38_15: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #15s : i
        let s_39_0: i128 = 15;
        // D s_39_1: read-var cp_num:i
        let s_39_1: i128 = fn_state.cp_num;
        // D s_39_2: cmp-eq s_39_1 s_39_0
        let s_39_2: bool = ((s_39_1) == (s_39_0));
        // N s_39_3: branch s_39_2 b328 b40
        if s_39_2 {
            return block_328(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#142015 <= s_40_0
        fn_state.gs_142015 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#142015:u8
        let s_41_0: bool = fn_state.gs_142015;
        // N s_41_1: branch s_41_0 b327 b42
        if s_41_0 {
            return block_327(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#142016 <= s_42_0
        fn_state.gs_142016 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#142016:u8
        let s_43_0: bool = fn_state.gs_142016;
        // N s_43_1: branch s_43_0 b323 b44
        if s_43_0 {
            return block_323(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#142018 <= s_44_0
        fn_state.gs_142018 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#142018:u8
        let s_45_0: bool = fn_state.gs_142018;
        // N s_45_1: branch s_45_0 b316 b46
        if s_45_0 {
            return block_316(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#142021 <= s_46_0
        fn_state.gs_142021 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#142021:u8
        let s_47_0: bool = fn_state.gs_142021;
        // N s_47_1: branch s_47_0 b315 b48
        if s_47_0 {
            return block_315(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #15s : i
        let s_49_0: i128 = 15;
        // D s_49_1: read-var cp_num:i
        let s_49_1: i128 = fn_state.cp_num;
        // D s_49_2: cmp-eq s_49_1 s_49_0
        let s_49_2: bool = ((s_49_1) == (s_49_0));
        // N s_49_3: branch s_49_2 b314 b50
        if s_49_2 {
            return block_314(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#142023 <= s_50_0
        fn_state.gs_142023 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#142023:u8
        let s_51_0: bool = fn_state.gs_142023;
        // N s_51_1: branch s_51_0 b313 b52
        if s_51_0 {
            return block_313(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#142024 <= s_52_0
        fn_state.gs_142024 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#142024:u8
        let s_53_0: bool = fn_state.gs_142024;
        // N s_53_1: branch s_53_0 b312 b54
        if s_53_0 {
            return block_312(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#142025 <= s_54_0
        fn_state.gs_142025 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#142025:u8
        let s_55_0: bool = fn_state.gs_142025;
        // N s_55_1: branch s_55_0 b308 b56
        if s_55_0 {
            return block_308(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#142027 <= s_56_0
        fn_state.gs_142027 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#142027:u8
        let s_57_0: bool = fn_state.gs_142027;
        // N s_57_1: branch s_57_0 b262 b58
        if s_57_0 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #15s : i
        let s_59_0: i128 = 15;
        // D s_59_1: read-var cp_num:i
        let s_59_1: i128 = fn_state.cp_num;
        // D s_59_2: cmp-eq s_59_1 s_59_0
        let s_59_2: bool = ((s_59_1) == (s_59_0));
        // N s_59_3: branch s_59_2 b261 b60
        if s_59_2 {
            return block_261(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#142029 <= s_60_0
        fn_state.gs_142029 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#142029:u8
        let s_61_0: bool = fn_state.gs_142029;
        // N s_61_1: branch s_61_0 b260 b62
        if s_61_0 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#142030 <= s_62_0
        fn_state.gs_142030 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#142030:u8
        let s_63_0: bool = fn_state.gs_142030;
        // N s_63_1: branch s_63_0 b259 b64
        if s_63_0 {
            return block_259(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#142031 <= s_64_0
        fn_state.gs_142031 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#142031:u8
        let s_65_0: bool = fn_state.gs_142031;
        // N s_65_1: branch s_65_0 b258 b66
        if s_65_0 {
            return block_258(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#142032 <= s_66_0
        fn_state.gs_142032 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#142032:u8
        let s_67_0: bool = fn_state.gs_142032;
        // N s_67_1: branch s_67_0 b257 b68
        if s_67_0 {
            return block_257(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #15s : i
        let s_69_0: i128 = 15;
        // D s_69_1: read-var cp_num:i
        let s_69_1: i128 = fn_state.cp_num;
        // D s_69_2: cmp-eq s_69_1 s_69_0
        let s_69_2: bool = ((s_69_1) == (s_69_0));
        // N s_69_3: branch s_69_2 b256 b70
        if s_69_2 {
            return block_256(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#142034 <= s_70_0
        fn_state.gs_142034 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#142034:u8
        let s_71_0: bool = fn_state.gs_142034;
        // N s_71_1: branch s_71_0 b255 b72
        if s_71_0 {
            return block_255(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#142035 <= s_72_0
        fn_state.gs_142035 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#142035:u8
        let s_73_0: bool = fn_state.gs_142035;
        // N s_73_1: branch s_73_0 b248 b74
        if s_73_0 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#142046 <= s_74_0
        fn_state.gs_142046 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#142046:u8
        let s_75_0: bool = fn_state.gs_142046;
        // N s_75_1: branch s_75_0 b228 b76
        if s_75_0 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #14s : i
        let s_77_0: i128 = 14;
        // D s_77_1: read-var cp_num:i
        let s_77_1: i128 = fn_state.cp_num;
        // D s_77_2: cmp-eq s_77_1 s_77_0
        let s_77_2: bool = ((s_77_1) == (s_77_0));
        // N s_77_3: branch s_77_2 b227 b78
        if s_77_2 {
            return block_227(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#142048 <= s_78_0
        fn_state.gs_142048 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#142048:u8
        let s_79_0: bool = fn_state.gs_142048;
        // N s_79_1: branch s_79_0 b226 b80
        if s_79_0 {
            return block_226(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#142049 <= s_80_0
        fn_state.gs_142049 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#142049:u8
        let s_81_0: bool = fn_state.gs_142049;
        // N s_81_1: branch s_81_0 b225 b82
        if s_81_0 {
            return block_225(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#142050 <= s_82_0
        fn_state.gs_142050 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#142050:u8
        let s_83_0: bool = fn_state.gs_142050;
        // N s_83_1: branch s_83_0 b219 b84
        if s_83_0 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #14s : i
        let s_84_0: i128 = 14;
        // D s_84_1: read-var cp_num:i
        let s_84_1: i128 = fn_state.cp_num;
        // D s_84_2: cmp-eq s_84_1 s_84_0
        let s_84_2: bool = ((s_84_1) == (s_84_0));
        // N s_84_3: branch s_84_2 b218 b85
        if s_84_2 {
            return block_218(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#142052 <= s_85_0
        fn_state.gs_142052 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#142052:u8
        let s_86_0: bool = fn_state.gs_142052;
        // N s_86_1: branch s_86_0 b217 b87
        if s_86_0 {
            return block_217(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#142053 <= s_87_0
        fn_state.gs_142053 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#142053:u8
        let s_88_0: bool = fn_state.gs_142053;
        // N s_88_1: branch s_88_0 b216 b89
        if s_88_0 {
            return block_216(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#142054 <= s_89_0
        fn_state.gs_142054 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#142054:u8
        let s_90_0: bool = fn_state.gs_142054;
        // N s_90_1: branch s_90_0 b215 b91
        if s_90_0 {
            return block_215(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#142055 <= s_91_0
        fn_state.gs_142055 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#142055:u8
        let s_92_0: bool = fn_state.gs_142055;
        // N s_92_1: branch s_92_0 b199 b93
        if s_92_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #15s : i
        let s_93_0: i128 = 15;
        // D s_93_1: read-var cp_num:i
        let s_93_1: i128 = fn_state.cp_num;
        // D s_93_2: cmp-eq s_93_1 s_93_0
        let s_93_2: bool = ((s_93_1) == (s_93_0));
        // N s_93_3: branch s_93_2 b198 b94
        if s_93_2 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#142057 <= s_94_0
        fn_state.gs_142057 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#142057:u8
        let s_95_0: bool = fn_state.gs_142057;
        // N s_95_1: branch s_95_0 b197 b96
        if s_95_0 {
            return block_197(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#142058 <= s_96_0
        fn_state.gs_142058 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#142058:u8
        let s_97_0: bool = fn_state.gs_142058;
        // N s_97_1: branch s_97_0 b196 b98
        if s_97_0 {
            return block_196(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#142059 <= s_98_0
        fn_state.gs_142059 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#142059:u8
        let s_99_0: bool = fn_state.gs_142059;
        // N s_99_1: branch s_99_0 b195 b100
        if s_99_0 {
            return block_195(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#142060 <= s_100_0
        fn_state.gs_142060 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#142060:u8
        let s_101_0: bool = fn_state.gs_142060;
        // N s_101_1: branch s_101_0 b194 b102
        if s_101_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #15s : i
        let s_102_0: i128 = 15;
        // D s_102_1: read-var cp_num:i
        let s_102_1: i128 = fn_state.cp_num;
        // D s_102_2: cmp-eq s_102_1 s_102_0
        let s_102_2: bool = ((s_102_1) == (s_102_0));
        // N s_102_3: branch s_102_2 b193 b103
        if s_102_2 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#142110 <= s_103_0
        fn_state.gs_142110 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#142110:u8
        let s_104_0: bool = fn_state.gs_142110;
        // N s_104_1: branch s_104_0 b189 b105
        if s_104_0 {
            return block_189(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#142112 <= s_105_0
        fn_state.gs_142112 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#142112:u8
        let s_106_0: bool = fn_state.gs_142112;
        // N s_106_1: branch s_106_0 b188 b107
        if s_106_0 {
            return block_188(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#142113 <= s_107_0
        fn_state.gs_142113 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#142113:u8
        let s_108_0: bool = fn_state.gs_142113;
        // N s_108_1: branch s_108_0 b187 b109
        if s_108_0 {
            return block_187(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#142114 <= s_109_0
        fn_state.gs_142114 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#142114:u8
        let s_110_0: bool = fn_state.gs_142114;
        // N s_110_1: branch s_110_0 b184 b111
        if s_110_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #15s : i
        let s_111_0: i128 = 15;
        // D s_111_1: read-var cp_num:i
        let s_111_1: i128 = fn_state.cp_num;
        // D s_111_2: cmp-eq s_111_1 s_111_0
        let s_111_2: bool = ((s_111_1) == (s_111_0));
        // N s_111_3: branch s_111_2 b183 b112
        if s_111_2 {
            return block_183(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#142062 <= s_112_0
        fn_state.gs_142062 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#142062:u8
        let s_113_0: bool = fn_state.gs_142062;
        // N s_113_1: branch s_113_0 b182 b114
        if s_113_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#142063 <= s_114_0
        fn_state.gs_142063 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#142063:u8
        let s_115_0: bool = fn_state.gs_142063;
        // N s_115_1: branch s_115_0 b181 b116
        if s_115_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_116_1: write-var gs#142064 <= s_116_0
        fn_state.gs_142064 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#142064:u8
        let s_117_0: bool = fn_state.gs_142064;
        // N s_117_1: branch s_117_0 b177 b118
        if s_117_0 {
            return block_177(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#142066 <= s_118_0
        fn_state.gs_142066 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#142066:u8
        let s_119_0: bool = fn_state.gs_142066;
        // N s_119_1: branch s_119_0 b162 b120
        if s_119_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_120_0: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #14s : i
        let s_121_0: i128 = 14;
        // D s_121_1: read-var cp_num:i
        let s_121_1: i128 = fn_state.cp_num;
        // D s_121_2: cmp-eq s_121_1 s_121_0
        let s_121_2: bool = ((s_121_1) == (s_121_0));
        // N s_121_3: branch s_121_2 b161 b122
        if s_121_2 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#142068 <= s_122_0
        fn_state.gs_142068 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#142068:u8
        let s_123_0: bool = fn_state.gs_142068;
        // N s_123_1: branch s_123_0 b160 b124
        if s_123_0 {
            return block_160(state, tracer, fn_state);
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
        // D s_124_1: write-var gs#142069 <= s_124_0
        fn_state.gs_142069 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#142069:u8
        let s_125_0: bool = fn_state.gs_142069;
        // N s_125_1: branch s_125_0 b159 b126
        if s_125_0 {
            return block_159(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#142070 <= s_126_0
        fn_state.gs_142070 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#142070:u8
        let s_127_0: bool = fn_state.gs_142070;
        // N s_127_1: branch s_127_0 b158 b128
        if s_127_0 {
            return block_158(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#142071 <= s_128_0
        fn_state.gs_142071 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#142071:u8
        let s_129_0: bool = fn_state.gs_142071;
        // N s_129_1: branch s_129_0 b157 b130
        if s_129_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_130_0: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #15s : i
        let s_131_0: i128 = 15;
        // D s_131_1: read-var cp_num:i
        let s_131_1: i128 = fn_state.cp_num;
        // D s_131_2: cmp-eq s_131_1 s_131_0
        let s_131_2: bool = ((s_131_1) == (s_131_0));
        // N s_131_3: branch s_131_2 b156 b132
        if s_131_2 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#142073 <= s_132_0
        fn_state.gs_142073 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#142073:u8
        let s_133_0: bool = fn_state.gs_142073;
        // N s_133_1: branch s_133_0 b155 b134
        if s_133_0 {
            return block_155(state, tracer, fn_state);
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
        // D s_134_1: write-var gs#142074 <= s_134_0
        fn_state.gs_142074 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#142074:u8
        let s_135_0: bool = fn_state.gs_142074;
        // N s_135_1: branch s_135_0 b154 b136
        if s_135_0 {
            return block_154(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#142075 <= s_136_0
        fn_state.gs_142075 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#142075:u8
        let s_137_0: bool = fn_state.gs_142075;
        // N s_137_1: branch s_137_0 b153 b138
        if s_137_0 {
            return block_153(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#142076 <= s_138_0
        fn_state.gs_142076 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#142076:u8
        let s_139_0: bool = fn_state.gs_142076;
        // N s_139_1: branch s_139_0 b141 b140
        if s_139_0 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_140_0: return
        return;
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #() : ()
        let s_141_0: () = ();
        // S s_141_1: call CurrentSecurityState(s_141_0)
        let s_141_1: u32 = CurrentSecurityState(state, tracer, s_141_0);
        // C s_141_2: const #0u : u32
        let s_141_2: u32 = 0;
        // S s_141_3: cmp-eq s_141_1 s_141_2
        let s_141_3: bool = ((s_141_1) == (s_141_2));
        // N s_141_4: branch s_141_3 b152 b142
        if s_141_3 {
            return block_152(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#142078 <= s_142_0
        fn_state.gs_142078 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#142078:u8
        let s_143_0: bool = fn_state.gs_142078;
        // N s_143_1: branch s_143_0 b151 b144
        if s_143_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#142079 <= s_144_0
        fn_state.gs_142079 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#142079:u8
        let s_145_0: bool = fn_state.gs_142079;
        // N s_145_1: branch s_145_0 b150 b146
        if s_145_0 {
            return block_150(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#142080 <= s_146_0
        fn_state.gs_142080 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#142080:u8
        let s_147_0: bool = fn_state.gs_142080;
        // N s_147_1: branch s_147_0 b149 b148
        if s_147_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_148_0: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var t:i
        let s_149_0: i128 = fn_state.t;
        // D s_149_1: call R_read(s_149_0)
        let s_149_1: u32 = R_read(state, tracer, s_149_0);
        // C s_149_2: const #20s : i
        let s_149_2: i128 = 20;
        // D s_149_3: cast zx s_149_1 -> bv
        let s_149_3: Bits = Bits::new(s_149_1 as u128, 32u16);
        // C s_149_4: const #0u : u8
        let s_149_4: u8 = 0;
        // C s_149_5: cast zx s_149_4 -> bv
        let s_149_5: Bits = Bits::new(s_149_4 as u128, 4u16);
        // C s_149_6: const #3s : i
        let s_149_6: i128 = 3;
        // C s_149_7: const #1u : u64
        let s_149_7: u64 = 1;
        // C s_149_8: cast zx s_149_7 -> bv
        let s_149_8: Bits = Bits::new(s_149_7 as u128, 64u16);
        // C s_149_9: lsl s_149_8 s_149_6
        let s_149_9: Bits = s_149_8 << s_149_6;
        // C s_149_10: sub s_149_9 s_149_8
        let s_149_10: Bits = ((s_149_9) - (s_149_8));
        // C s_149_11: and s_149_5 s_149_10
        let s_149_11: Bits = ((s_149_5) & (s_149_10));
        // C s_149_12: lsl s_149_11 s_149_2
        let s_149_12: Bits = s_149_11 << s_149_2;
        // C s_149_13: lsl s_149_10 s_149_2
        let s_149_13: Bits = s_149_10 << s_149_2;
        // C s_149_14: cmpl s_149_13
        let s_149_14: Bits = !s_149_13;
        // D s_149_15: and s_149_3 s_149_14
        let s_149_15: Bits = ((s_149_3) & (s_149_14));
        // D s_149_16: or s_149_15 s_149_12
        let s_149_16: Bits = ((s_149_15) | (s_149_12));
        // D s_149_17: cast reint s_149_16 -> u32
        let s_149_17: u32 = (s_149_16.value() as u32);
        // D s_149_18: read-var t:i
        let s_149_18: i128 = fn_state.t;
        // D s_149_19: call R_set(s_149_18, s_149_17)
        let s_149_19: () = R_set(state, tracer, s_149_18, s_149_17);
        // N s_149_20: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #102488u : u32
        let s_150_0: u32 = 102488;
        // D s_150_1: read-reg s_150_0:struct
        let s_150_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // D s_150_2: call _get_NSACR_Type_cp10(s_150_1)
        let s_150_2: bool = u_get_NSACR_Type_cp10(state, tracer, s_150_1);
        // D s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // C s_150_4: const #0u : u8
        let s_150_4: bool = false;
        // C s_150_5: cast zx s_150_4 -> bv
        let s_150_5: Bits = Bits::new(s_150_4 as u128, 1u16);
        // D s_150_6: cmp-eq s_150_3 s_150_5
        let s_150_6: bool = ((s_150_3) == (s_150_5));
        // D s_150_7: write-var gs#142080 <= s_150_6
        fn_state.gs_142080 = s_150_6;
        // N s_150_8: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #424u : u32
        let s_151_0: u32 = 424;
        // D s_151_1: read-reg s_151_0:u8
        let s_151_1: u8 = {
            let value = state.read_register::<u8>(s_151_0 as isize);
            tracer.read_register(s_151_0 as isize, value);
            value
        };
        // D s_151_2: call ELUsingAArch32(s_151_1)
        let s_151_2: bool = ELUsingAArch32(state, tracer, s_151_1);
        // D s_151_3: write-var gs#142079 <= s_151_2
        fn_state.gs_142079 = s_151_2;
        // N s_151_4: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #424u : u32
        let s_152_0: u32 = 424;
        // D s_152_1: read-reg s_152_0:u8
        let s_152_1: u8 = {
            let value = state.read_register::<u8>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // C s_152_2: const #2u : u8
        let s_152_2: u8 = 2;
        // D s_152_3: cmp-lt s_152_1 s_152_2
        let s_152_3: bool = ((s_152_1) < (s_152_2));
        // D s_152_4: write-var gs#142078 <= s_152_3
        fn_state.gs_142078 = s_152_3;
        // N s_152_5: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var opc2:u8
        let s_153_0: u8 = fn_state.opc2;
        // D s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 3u16);
        // C s_153_2: const #2u : u8
        let s_153_2: u8 = 2;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 3u16);
        // D s_153_4: cmp-eq s_153_1 s_153_3
        let s_153_4: bool = ((s_153_1) == (s_153_3));
        // D s_153_5: write-var gs#142076 <= s_153_4
        fn_state.gs_142076 = s_153_4;
        // N s_153_6: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var CRm:u8
        let s_154_0: u8 = fn_state.CRm;
        // D s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 4u16);
        // C s_154_2: const #0u : u8
        let s_154_2: u8 = 0;
        // C s_154_3: cast zx s_154_2 -> bv
        let s_154_3: Bits = Bits::new(s_154_2 as u128, 4u16);
        // D s_154_4: cmp-eq s_154_1 s_154_3
        let s_154_4: bool = ((s_154_1) == (s_154_3));
        // D s_154_5: write-var gs#142075 <= s_154_4
        fn_state.gs_142075 = s_154_4;
        // N s_154_6: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var CRn:u8
        let s_155_0: u8 = fn_state.CRn;
        // D s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 4u16);
        // C s_155_2: const #1u : u8
        let s_155_2: u8 = 1;
        // C s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 4u16);
        // D s_155_4: cmp-eq s_155_1 s_155_3
        let s_155_4: bool = ((s_155_1) == (s_155_3));
        // D s_155_5: write-var gs#142074 <= s_155_4
        fn_state.gs_142074 = s_155_4;
        // N s_155_6: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var opc1:u8
        let s_156_0: u8 = fn_state.opc1;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 3u16);
        // C s_156_2: const #0u : u8
        let s_156_2: u8 = 0;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 3u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // D s_156_5: write-var gs#142073 <= s_156_4
        fn_state.gs_142073 = s_156_4;
        // N s_156_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #() : ()
        let s_157_0: () = ();
        // S s_157_1: call DBGDSCRint_read(s_157_0)
        let s_157_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_157_0,
        );
        // C s_157_2: const #0u : u8
        let s_157_2: bool = false;
        // S s_157_3: call _update_DBGDSCRint_Type_RXfull(s_157_1, s_157_2)
        let s_157_3: ProductType700c18a878c5601b = u_update_DBGDSCRint_Type_RXfull(
            state,
            tracer,
            s_157_1,
            s_157_2,
        );
        // S s_157_4: call DBGDSCRint_write(s_157_3)
        let s_157_4: () = DBGDSCRint_write(state, tracer, s_157_3);
        // C s_157_5: const #16832u : u32
        let s_157_5: u32 = 16832;
        // D s_157_6: read-reg s_157_5:struct
        let s_157_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_157_5 as isize);
            tracer.read_register(s_157_5 as isize, value);
            value
        };
        // C s_157_7: const #16832u : u32
        let s_157_7: u32 = 16832;
        // N s_157_8: write-reg s_157_7 <= s_157_6
        let s_157_8: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_157_7 as isize, s_157_6);
            tracer.write_register(s_157_7 as isize, s_157_6);
        };
        // C s_157_9: const #() : ()
        let s_157_9: () = ();
        // S s_157_10: call DBGDSCRext_read(s_157_9)
        let s_157_10: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_157_9,
        );
        // C s_157_11: const #0u : u8
        let s_157_11: bool = false;
        // S s_157_12: call _update_DBGDSCRext_Type_RXfull(s_157_10, s_157_11)
        let s_157_12: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_RXfull(
            state,
            tracer,
            s_157_10,
            s_157_11,
        );
        // S s_157_13: call DBGDSCRext_write(s_157_12)
        let s_157_13: () = DBGDSCRext_write(state, tracer, s_157_12);
        // C s_157_14: const #() : ()
        let s_157_14: () = ();
        // S s_157_15: call EDSCR_read(s_157_14)
        let s_157_15: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_157_14);
        // C s_157_16: const #0u : u8
        let s_157_16: bool = false;
        // S s_157_17: call _update_EDSCR_Type_RXfull(s_157_15, s_157_16)
        let s_157_17: ProductType700c18a878c5601b = u_update_EDSCR_Type_RXfull(
            state,
            tracer,
            s_157_15,
            s_157_16,
        );
        // S s_157_18: call EDSCR_write(s_157_17)
        let s_157_18: () = EDSCR_write(state, tracer, s_157_17);
        // N s_157_19: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var opc2:u8
        let s_158_0: u8 = fn_state.opc2;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 3u16);
        // C s_158_2: const #0u : u8
        let s_158_2: u8 = 0;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 3u16);
        // D s_158_4: cmp-eq s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) == (s_158_3));
        // D s_158_5: write-var gs#142071 <= s_158_4
        fn_state.gs_142071 = s_158_4;
        // N s_158_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var CRm:u8
        let s_159_0: u8 = fn_state.CRm;
        // D s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 4u16);
        // C s_159_2: const #5u : u8
        let s_159_2: u8 = 5;
        // C s_159_3: cast zx s_159_2 -> bv
        let s_159_3: Bits = Bits::new(s_159_2 as u128, 4u16);
        // D s_159_4: cmp-eq s_159_1 s_159_3
        let s_159_4: bool = ((s_159_1) == (s_159_3));
        // D s_159_5: write-var gs#142070 <= s_159_4
        fn_state.gs_142070 = s_159_4;
        // N s_159_6: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var CRn:u8
        let s_160_0: u8 = fn_state.CRn;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 4u16);
        // C s_160_2: const #0u : u8
        let s_160_2: u8 = 0;
        // C s_160_3: cast zx s_160_2 -> bv
        let s_160_3: Bits = Bits::new(s_160_2 as u128, 4u16);
        // D s_160_4: cmp-eq s_160_1 s_160_3
        let s_160_4: bool = ((s_160_1) == (s_160_3));
        // D s_160_5: write-var gs#142069 <= s_160_4
        fn_state.gs_142069 = s_160_4;
        // N s_160_6: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var opc1:u8
        let s_161_0: u8 = fn_state.opc1;
        // D s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 3u16);
        // C s_161_2: const #0u : u8
        let s_161_2: u8 = 0;
        // C s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 3u16);
        // D s_161_4: cmp-eq s_161_1 s_161_3
        let s_161_4: bool = ((s_161_1) == (s_161_3));
        // D s_161_5: write-var gs#142068 <= s_161_4
        fn_state.gs_142068 = s_161_4;
        // N s_161_6: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #() : ()
        let s_162_0: () = ();
        // S s_162_1: call ERRIDR_read(s_162_0)
        let s_162_1: ProductType700c18a878c5601b = ERRIDR_read(state, tracer, s_162_0);
        // S s_162_2: call _get_ERRIDR_Type_NUM(s_162_1)
        let s_162_2: u16 = u_get_ERRIDR_Type_NUM(state, tracer, s_162_1);
        // S s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 16u16);
        // S s_162_4: cast zx s_162_3 -> i
        let s_162_4: i128 = (s_162_3.value() as i128);
        // S s_162_5: cast reint s_162_4 -> i64
        let s_162_5: i64 = (s_162_4 as i64);
        // C s_162_6: const #0u : u8
        let s_162_6: u8 = 0;
        // C s_162_7: cast zx s_162_6 -> bv
        let s_162_7: Bits = Bits::new(s_162_6 as u128, 4u16);
        // C s_162_8: cast zx s_162_7 -> i
        let s_162_8: i128 = (s_162_7.value() as i128);
        // C s_162_9: cast reint s_162_8 -> i64
        let s_162_9: i64 = (s_162_8 as i64);
        // S s_162_10: cast zx s_162_5 -> i
        let s_162_10: i128 = (i128::try_from(s_162_5).unwrap());
        // C s_162_11: cast zx s_162_9 -> i
        let s_162_11: i128 = (i128::try_from(s_162_9).unwrap());
        // S s_162_12: cmp-eq s_162_10 s_162_11
        let s_162_12: bool = ((s_162_10) == (s_162_11));
        // N s_162_13: branch s_162_12 b176 b163
        if s_162_12 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call ERRSELR_read(s_163_0)
        let s_163_1: ProductType700c18a878c5601b = ERRSELR_read(state, tracer, s_163_0);
        // S s_163_2: call _get_ERRSELR_Type_SEL(s_163_1)
        let s_163_2: u16 = u_get_ERRSELR_Type_SEL(state, tracer, s_163_1);
        // S s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 16u16);
        // S s_163_4: cast zx s_163_3 -> i
        let s_163_4: i128 = (s_163_3.value() as i128);
        // S s_163_5: cast reint s_163_4 -> i64
        let s_163_5: i64 = (s_163_4 as i64);
        // C s_163_6: const #() : ()
        let s_163_6: () = ();
        // S s_163_7: call ERRIDR_read(s_163_6)
        let s_163_7: ProductType700c18a878c5601b = ERRIDR_read(state, tracer, s_163_6);
        // S s_163_8: call _get_ERRIDR_Type_NUM(s_163_7)
        let s_163_8: u16 = u_get_ERRIDR_Type_NUM(state, tracer, s_163_7);
        // S s_163_9: cast zx s_163_8 -> bv
        let s_163_9: Bits = Bits::new(s_163_8 as u128, 16u16);
        // S s_163_10: cast zx s_163_9 -> i
        let s_163_10: i128 = (s_163_9.value() as i128);
        // S s_163_11: cast reint s_163_10 -> i64
        let s_163_11: i64 = (s_163_10 as i64);
        // S s_163_12: cast zx s_163_5 -> i
        let s_163_12: i128 = (i128::try_from(s_163_5).unwrap());
        // S s_163_13: cast zx s_163_11 -> i
        let s_163_13: i128 = (i128::try_from(s_163_11).unwrap());
        // S s_163_14: cmp-ge s_163_12 s_163_13
        let s_163_14: bool = ((s_163_12) >= (s_163_13));
        // D s_163_15: write-var gs#142092 <= s_163_14
        fn_state.gs_142092 = s_163_14;
        // N s_163_16: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#142092:u8
        let s_164_0: bool = fn_state.gs_142092;
        // N s_164_1: branch s_164_0 b175 b165
        if s_164_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #() : ()
        let s_165_0: () = ();
        // S s_165_1: call ERRSELR_read(s_165_0)
        let s_165_1: ProductType700c18a878c5601b = ERRSELR_read(state, tracer, s_165_0);
        // S s_165_2: call _get_ERRSELR_Type_SEL(s_165_1)
        let s_165_2: u16 = u_get_ERRSELR_Type_SEL(state, tracer, s_165_1);
        // S s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 16u16);
        // S s_165_4: cast zx s_165_3 -> i
        let s_165_4: i128 = (s_165_3.value() as i128);
        // S s_165_5: cast reint s_165_4 -> i64
        let s_165_5: i64 = (s_165_4 as i64);
        // D s_165_6: write-var indexshadow#1012 <= s_165_5
        fn_state.indexshadow_1012 = s_165_5;
        // D s_165_7: read-var opc2:u8
        let s_165_7: u8 = fn_state.opc2;
        // D s_165_8: cast zx s_165_7 -> bv
        let s_165_8: Bits = Bits::new(s_165_7 as u128, 3u16);
        // C s_165_9: const #0u : u8
        let s_165_9: u8 = 0;
        // C s_165_10: cast zx s_165_9 -> bv
        let s_165_10: Bits = Bits::new(s_165_9 as u128, 3u16);
        // D s_165_11: cmp-eq s_165_8 s_165_10
        let s_165_11: bool = ((s_165_8) == (s_165_10));
        // N s_165_12: branch s_165_11 b171 b166
        if s_165_11 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var indexshadow#1012:i64
        let s_166_0: i64 = fn_state.indexshadow_1012;
        // D s_166_1: cast zx s_166_0 -> i
        let s_166_1: i128 = (i128::try_from(s_166_0).unwrap());
        // D s_166_2: call __id(s_166_1)
        let s_166_2: i128 = u__id(state, tracer, s_166_1);
        // D s_166_3: cast reint s_166_2 -> i64
        let s_166_3: i64 = (s_166_2 as i64);
        // C s_166_4: const #0s : i
        let s_166_4: i128 = 0;
        // D s_166_5: cast zx s_166_3 -> i
        let s_166_5: i128 = (i128::try_from(s_166_3).unwrap());
        // D s_166_6: cmp-le s_166_4 s_166_5
        let s_166_6: bool = ((s_166_4) <= (s_166_5));
        // N s_166_7: branch s_166_6 b170 b167
        if s_166_6 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0u : u8
        let s_167_0: bool = false;
        // D s_167_1: write-var gs#142097 <= s_167_0
        fn_state.gs_142097 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#142097:u8
        let s_168_0: bool = fn_state.gs_142097;
        // N s_168_1: assert s_168_0
        let s_168_1: () = assert!(s_168_0);
        // C s_168_2: const #16912u : u32
        let s_168_2: u32 = 16912;
        // D s_168_3: read-reg s_168_2:[struct; 4]
        let s_168_3: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_168_2 as isize);
            tracer.read_register(s_168_2 as isize, value);
            value
        };
        // D s_168_4: read-var indexshadow#1012:i64
        let s_168_4: i64 = fn_state.indexshadow_1012;
        // D s_168_5: cast zx s_168_4 -> i
        let s_168_5: i128 = (i128::try_from(s_168_4).unwrap());
        // D s_168_6: read-element s_168_3[s_168_5]
        let s_168_6: ProductType5c790c8ef59cc8b2 = s_168_3[(s_168_5) as usize];
        // D s_168_7: write-var ga#248406 <= s_168_6
        fn_state.ga_248406 = s_168_6;
        // D s_168_8: read-var ga#248406.0:struct
        let s_168_8: u64 = fn_state.ga_248406._0;
        // C s_168_9: const #32s : i
        let s_168_9: i128 = 32;
        // D s_168_10: cast zx s_168_8 -> bv
        let s_168_10: Bits = Bits::new(s_168_8 as u128, 64u16);
        // C s_168_11: const #1s : i64
        let s_168_11: i64 = 1;
        // C s_168_12: cast zx s_168_11 -> i
        let s_168_12: i128 = (i128::try_from(s_168_11).unwrap());
        // C s_168_13: const #31s : i
        let s_168_13: i128 = 31;
        // C s_168_14: add s_168_13 s_168_12
        let s_168_14: i128 = (s_168_13 + s_168_12);
        // D s_168_15: bit-extract s_168_10 s_168_9 s_168_14
        let s_168_15: Bits = (Bits::new(
            ((s_168_10) >> (s_168_9)).value(),
            u16::try_from(s_168_14).unwrap(),
        ));
        // D s_168_16: cast reint s_168_15 -> u32
        let s_168_16: u32 = (s_168_15.value() as u32);
        // D s_168_17: read-var t:i
        let s_168_17: i128 = fn_state.t;
        // D s_168_18: call R_set(s_168_17, s_168_16)
        let s_168_18: () = R_set(state, tracer, s_168_17, s_168_16);
        // N s_168_19: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_169_0: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var indexshadow#1012:i64
        let s_170_0: i64 = fn_state.indexshadow_1012;
        // D s_170_1: cast zx s_170_0 -> i
        let s_170_1: i128 = (i128::try_from(s_170_0).unwrap());
        // D s_170_2: call __id(s_170_1)
        let s_170_2: i128 = u__id(state, tracer, s_170_1);
        // D s_170_3: cast reint s_170_2 -> i64
        let s_170_3: i64 = (s_170_2 as i64);
        // C s_170_4: const #4s : i
        let s_170_4: i128 = 4;
        // D s_170_5: cast zx s_170_3 -> i
        let s_170_5: i128 = (i128::try_from(s_170_3).unwrap());
        // D s_170_6: cmp-lt s_170_5 s_170_4
        let s_170_6: bool = ((s_170_5) < (s_170_4));
        // D s_170_7: write-var gs#142097 <= s_170_6
        fn_state.gs_142097 = s_170_6;
        // N s_170_8: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var indexshadow#1012:i64
        let s_171_0: i64 = fn_state.indexshadow_1012;
        // D s_171_1: cast zx s_171_0 -> i
        let s_171_1: i128 = (i128::try_from(s_171_0).unwrap());
        // D s_171_2: call __id(s_171_1)
        let s_171_2: i128 = u__id(state, tracer, s_171_1);
        // D s_171_3: cast reint s_171_2 -> i64
        let s_171_3: i64 = (s_171_2 as i64);
        // C s_171_4: const #0s : i
        let s_171_4: i128 = 0;
        // D s_171_5: cast zx s_171_3 -> i
        let s_171_5: i128 = (i128::try_from(s_171_3).unwrap());
        // D s_171_6: cmp-le s_171_4 s_171_5
        let s_171_6: bool = ((s_171_4) <= (s_171_5));
        // N s_171_7: branch s_171_6 b174 b172
        if s_171_6 {
            return block_174(state, tracer, fn_state);
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
        // D s_172_1: write-var gs#142103 <= s_172_0
        fn_state.gs_142103 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#142103:u8
        let s_173_0: bool = fn_state.gs_142103;
        // N s_173_1: assert s_173_0
        let s_173_1: () = assert!(s_173_0);
        // C s_173_2: const #16912u : u32
        let s_173_2: u32 = 16912;
        // D s_173_3: read-reg s_173_2:[struct; 4]
        let s_173_3: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_173_2 as isize);
            tracer.read_register(s_173_2 as isize, value);
            value
        };
        // D s_173_4: read-var indexshadow#1012:i64
        let s_173_4: i64 = fn_state.indexshadow_1012;
        // D s_173_5: cast zx s_173_4 -> i
        let s_173_5: i128 = (i128::try_from(s_173_4).unwrap());
        // D s_173_6: read-element s_173_3[s_173_5]
        let s_173_6: ProductType5c790c8ef59cc8b2 = s_173_3[(s_173_5) as usize];
        // D s_173_7: write-var ga#248399 <= s_173_6
        fn_state.ga_248399 = s_173_6;
        // D s_173_8: read-var ga#248399.0:struct
        let s_173_8: u64 = fn_state.ga_248399._0;
        // C s_173_9: const #0s : i
        let s_173_9: i128 = 0;
        // D s_173_10: cast zx s_173_8 -> bv
        let s_173_10: Bits = Bits::new(s_173_8 as u128, 64u16);
        // C s_173_11: const #1s : i64
        let s_173_11: i64 = 1;
        // C s_173_12: cast zx s_173_11 -> i
        let s_173_12: i128 = (i128::try_from(s_173_11).unwrap());
        // C s_173_13: const #31s : i
        let s_173_13: i128 = 31;
        // C s_173_14: add s_173_13 s_173_12
        let s_173_14: i128 = (s_173_13 + s_173_12);
        // D s_173_15: bit-extract s_173_10 s_173_9 s_173_14
        let s_173_15: Bits = (Bits::new(
            ((s_173_10) >> (s_173_9)).value(),
            u16::try_from(s_173_14).unwrap(),
        ));
        // D s_173_16: cast reint s_173_15 -> u32
        let s_173_16: u32 = (s_173_15.value() as u32);
        // D s_173_17: read-var t:i
        let s_173_17: i128 = fn_state.t;
        // D s_173_18: call R_set(s_173_17, s_173_16)
        let s_173_18: () = R_set(state, tracer, s_173_17, s_173_16);
        // N s_173_19: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var indexshadow#1012:i64
        let s_174_0: i64 = fn_state.indexshadow_1012;
        // D s_174_1: cast zx s_174_0 -> i
        let s_174_1: i128 = (i128::try_from(s_174_0).unwrap());
        // D s_174_2: call __id(s_174_1)
        let s_174_2: i128 = u__id(state, tracer, s_174_1);
        // D s_174_3: cast reint s_174_2 -> i64
        let s_174_3: i64 = (s_174_2 as i64);
        // C s_174_4: const #4s : i
        let s_174_4: i128 = 4;
        // D s_174_5: cast zx s_174_3 -> i
        let s_174_5: i128 = (i128::try_from(s_174_3).unwrap());
        // D s_174_6: cmp-lt s_174_5 s_174_4
        let s_174_6: bool = ((s_174_5) < (s_174_4));
        // D s_174_7: write-var gs#142103 <= s_174_6
        fn_state.gs_142103 = s_174_6;
        // N s_174_8: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #32s : i
        let s_175_0: i128 = 32;
        // S s_175_1: call Zeros(s_175_0)
        let s_175_1: Bits = Zeros(state, tracer, s_175_0);
        // S s_175_2: cast reint s_175_1 -> u32
        let s_175_2: u32 = (s_175_1.value() as u32);
        // D s_175_3: read-var t:i
        let s_175_3: i128 = fn_state.t;
        // D s_175_4: call R_set(s_175_3, s_175_2)
        let s_175_4: () = R_set(state, tracer, s_175_3, s_175_2);
        // N s_175_5: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #1u : u8
        let s_176_0: bool = true;
        // D s_176_1: write-var gs#142092 <= s_176_0
        fn_state.gs_142092 = s_176_0;
        // N s_176_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var opc2:u8
        let s_177_0: u8 = fn_state.opc2;
        // D s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 3u16);
        // C s_177_2: const #0u : u8
        let s_177_2: u8 = 0;
        // C s_177_3: cast zx s_177_2 -> bv
        let s_177_3: Bits = Bits::new(s_177_2 as u128, 3u16);
        // D s_177_4: cmp-eq s_177_1 s_177_3
        let s_177_4: bool = ((s_177_1) == (s_177_3));
        // N s_177_5: branch s_177_4 b180 b178
        if s_177_4 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var opc2:u8
        let s_178_0: u8 = fn_state.opc2;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 3u16);
        // C s_178_2: const #4u : u8
        let s_178_2: u8 = 4;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 3u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#142065 <= s_178_4
        fn_state.gs_142065 = s_178_4;
        // N s_178_6: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var gs#142065:u8
        let s_179_0: bool = fn_state.gs_142065;
        // D s_179_1: write-var gs#142066 <= s_179_0
        fn_state.gs_142066 = s_179_0;
        // N s_179_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #1u : u8
        let s_180_0: bool = true;
        // D s_180_1: write-var gs#142065 <= s_180_0
        fn_state.gs_142065 = s_180_0;
        // N s_180_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var CRm:u8
        let s_181_0: u8 = fn_state.CRm;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 4u16);
        // C s_181_2: const #4u : u8
        let s_181_2: u8 = 4;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 4u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#142064 <= s_181_4
        fn_state.gs_142064 = s_181_4;
        // N s_181_6: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var CRn:u8
        let s_182_0: u8 = fn_state.CRn;
        // D s_182_1: cast zx s_182_0 -> bv
        let s_182_1: Bits = Bits::new(s_182_0 as u128, 4u16);
        // C s_182_2: const #5u : u8
        let s_182_2: u8 = 5;
        // C s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 4u16);
        // D s_182_4: cmp-eq s_182_1 s_182_3
        let s_182_4: bool = ((s_182_1) == (s_182_3));
        // D s_182_5: write-var gs#142063 <= s_182_4
        fn_state.gs_142063 = s_182_4;
        // N s_182_6: jump b115
        return block_115(state, tracer, fn_state);
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
        // C s_183_2: const #0u : u8
        let s_183_2: u8 = 0;
        // C s_183_3: cast zx s_183_2 -> bv
        let s_183_3: Bits = Bits::new(s_183_2 as u128, 3u16);
        // D s_183_4: cmp-eq s_183_1 s_183_3
        let s_183_4: bool = ((s_183_1) == (s_183_3));
        // D s_183_5: write-var gs#142062 <= s_183_4
        fn_state.gs_142062 = s_183_4;
        // N s_183_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var opc2:u8
        let s_184_0: u8 = fn_state.opc2;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 3u16);
        // C s_184_2: const #0u : u8
        let s_184_2: u8 = 0;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 3u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // N s_184_5: branch s_184_4 b186 b185
        if s_184_4 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #() : ()
        let s_185_0: () = ();
        // S s_185_1: call CSSELR_read(s_185_0)
        let s_185_1: ProductType700c18a878c5601b = CSSELR_read(state, tracer, s_185_0);
        // D s_185_2: write-var ga#248367 <= s_185_1
        fn_state.ga_248367 = s_185_1;
        // D s_185_3: read-var ga#248367.0:struct
        let s_185_3: u32 = fn_state.ga_248367._0;
        // C s_185_4: const #0s : i
        let s_185_4: i128 = 0;
        // D s_185_5: cast zx s_185_3 -> bv
        let s_185_5: Bits = Bits::new(s_185_3 as u128, 32u16);
        // C s_185_6: const #1s : i64
        let s_185_6: i64 = 1;
        // C s_185_7: cast zx s_185_6 -> i
        let s_185_7: i128 = (i128::try_from(s_185_6).unwrap());
        // C s_185_8: const #3s : i
        let s_185_8: i128 = 3;
        // C s_185_9: add s_185_8 s_185_7
        let s_185_9: i128 = (s_185_8 + s_185_7);
        // D s_185_10: bit-extract s_185_5 s_185_4 s_185_9
        let s_185_10: Bits = (Bits::new(
            ((s_185_5) >> (s_185_4)).value(),
            u16::try_from(s_185_9).unwrap(),
        ));
        // D s_185_11: cast reint s_185_10 -> u8
        let s_185_11: u8 = (s_185_10.value() as u8);
        // D s_185_12: call CacheConfigRead(s_185_11)
        let s_185_12: u64 = CacheConfigRead(state, tracer, s_185_11);
        // C s_185_13: const #32s : i
        let s_185_13: i128 = 32;
        // D s_185_14: cast zx s_185_12 -> bv
        let s_185_14: Bits = Bits::new(s_185_12 as u128, 64u16);
        // C s_185_15: const #1s : i64
        let s_185_15: i64 = 1;
        // C s_185_16: cast zx s_185_15 -> i
        let s_185_16: i128 = (i128::try_from(s_185_15).unwrap());
        // C s_185_17: const #31s : i
        let s_185_17: i128 = 31;
        // C s_185_18: add s_185_17 s_185_16
        let s_185_18: i128 = (s_185_17 + s_185_16);
        // D s_185_19: bit-extract s_185_14 s_185_13 s_185_18
        let s_185_19: Bits = (Bits::new(
            ((s_185_14) >> (s_185_13)).value(),
            u16::try_from(s_185_18).unwrap(),
        ));
        // D s_185_20: cast reint s_185_19 -> u32
        let s_185_20: u32 = (s_185_19.value() as u32);
        // D s_185_21: read-var t:i
        let s_185_21: i128 = fn_state.t;
        // D s_185_22: call R_set(s_185_21, s_185_20)
        let s_185_22: () = R_set(state, tracer, s_185_21, s_185_20);
        // N s_185_23: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #() : ()
        let s_186_0: () = ();
        // S s_186_1: call CSSELR_read(s_186_0)
        let s_186_1: ProductType700c18a878c5601b = CSSELR_read(state, tracer, s_186_0);
        // D s_186_2: write-var ga#248362 <= s_186_1
        fn_state.ga_248362 = s_186_1;
        // D s_186_3: read-var ga#248362.0:struct
        let s_186_3: u32 = fn_state.ga_248362._0;
        // C s_186_4: const #0s : i
        let s_186_4: i128 = 0;
        // D s_186_5: cast zx s_186_3 -> bv
        let s_186_5: Bits = Bits::new(s_186_3 as u128, 32u16);
        // C s_186_6: const #1s : i64
        let s_186_6: i64 = 1;
        // C s_186_7: cast zx s_186_6 -> i
        let s_186_7: i128 = (i128::try_from(s_186_6).unwrap());
        // C s_186_8: const #3s : i
        let s_186_8: i128 = 3;
        // C s_186_9: add s_186_8 s_186_7
        let s_186_9: i128 = (s_186_8 + s_186_7);
        // D s_186_10: bit-extract s_186_5 s_186_4 s_186_9
        let s_186_10: Bits = (Bits::new(
            ((s_186_5) >> (s_186_4)).value(),
            u16::try_from(s_186_9).unwrap(),
        ));
        // D s_186_11: cast reint s_186_10 -> u8
        let s_186_11: u8 = (s_186_10.value() as u8);
        // D s_186_12: call CacheConfigRead(s_186_11)
        let s_186_12: u64 = CacheConfigRead(state, tracer, s_186_11);
        // C s_186_13: const #0s : i
        let s_186_13: i128 = 0;
        // D s_186_14: cast zx s_186_12 -> bv
        let s_186_14: Bits = Bits::new(s_186_12 as u128, 64u16);
        // C s_186_15: const #1s : i64
        let s_186_15: i64 = 1;
        // C s_186_16: cast zx s_186_15 -> i
        let s_186_16: i128 = (i128::try_from(s_186_15).unwrap());
        // C s_186_17: const #31s : i
        let s_186_17: i128 = 31;
        // C s_186_18: add s_186_17 s_186_16
        let s_186_18: i128 = (s_186_17 + s_186_16);
        // D s_186_19: bit-extract s_186_14 s_186_13 s_186_18
        let s_186_19: Bits = (Bits::new(
            ((s_186_14) >> (s_186_13)).value(),
            u16::try_from(s_186_18).unwrap(),
        ));
        // D s_186_20: cast reint s_186_19 -> u32
        let s_186_20: u32 = (s_186_19.value() as u32);
        // D s_186_21: read-var t:i
        let s_186_21: i128 = fn_state.t;
        // D s_186_22: call R_set(s_186_21, s_186_20)
        let s_186_22: () = R_set(state, tracer, s_186_21, s_186_20);
        // N s_186_23: return
        return;
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
        // C s_187_2: const #0u : u8
        let s_187_2: u8 = 0;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 4u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // D s_187_5: write-var gs#142114 <= s_187_4
        fn_state.gs_142114 = s_187_4;
        // N s_187_6: jump b110
        return block_110(state, tracer, fn_state);
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
        // C s_188_2: const #0u : u8
        let s_188_2: u8 = 0;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 4u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // D s_188_5: write-var gs#142113 <= s_188_4
        fn_state.gs_142113 = s_188_4;
        // N s_188_6: jump b108
        return block_108(state, tracer, fn_state);
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
        // C s_189_2: const #0u : u8
        let s_189_2: u8 = 0;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 3u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // N s_189_5: branch s_189_4 b192 b190
        if s_189_4 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var opc2:u8
        let s_190_0: u8 = fn_state.opc2;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 3u16);
        // C s_190_2: const #2u : u8
        let s_190_2: u8 = 2;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 3u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#142111 <= s_190_4
        fn_state.gs_142111 = s_190_4;
        // N s_190_6: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var gs#142111:u8
        let s_191_0: bool = fn_state.gs_142111;
        // D s_191_1: write-var gs#142112 <= s_191_0
        fn_state.gs_142112 = s_191_0;
        // N s_191_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #1u : u8
        let s_192_0: bool = true;
        // D s_192_1: write-var gs#142111 <= s_192_0
        fn_state.gs_142111 = s_192_0;
        // N s_192_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var opc1:u8
        let s_193_0: u8 = fn_state.opc1;
        // D s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 3u16);
        // C s_193_2: const #1u : u8
        let s_193_2: u8 = 1;
        // C s_193_3: cast zx s_193_2 -> bv
        let s_193_3: Bits = Bits::new(s_193_2 as u128, 3u16);
        // D s_193_4: cmp-eq s_193_1 s_193_3
        let s_193_4: bool = ((s_193_1) == (s_193_3));
        // D s_193_5: write-var gs#142110 <= s_193_4
        fn_state.gs_142110 = s_193_4;
        // N s_193_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #() : ()
        let s_194_0: () = ();
        // S s_194_1: call getISR(s_194_0)
        let s_194_1: u32 = getISR(state, tracer, s_194_0);
        // D s_194_2: read-var t:i
        let s_194_2: i128 = fn_state.t;
        // D s_194_3: call R_set(s_194_2, s_194_1)
        let s_194_3: () = R_set(state, tracer, s_194_2, s_194_1);
        // N s_194_4: return
        return;
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var CRm:u8
        let s_195_0: u8 = fn_state.CRm;
        // D s_195_1: cast zx s_195_0 -> bv
        let s_195_1: Bits = Bits::new(s_195_0 as u128, 4u16);
        // C s_195_2: const #1u : u8
        let s_195_2: u8 = 1;
        // C s_195_3: cast zx s_195_2 -> bv
        let s_195_3: Bits = Bits::new(s_195_2 as u128, 4u16);
        // D s_195_4: cmp-eq s_195_1 s_195_3
        let s_195_4: bool = ((s_195_1) == (s_195_3));
        // D s_195_5: write-var gs#142060 <= s_195_4
        fn_state.gs_142060 = s_195_4;
        // N s_195_6: jump b101
        return block_101(state, tracer, fn_state);
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
        // C s_196_2: const #12u : u8
        let s_196_2: u8 = 12;
        // C s_196_3: cast zx s_196_2 -> bv
        let s_196_3: Bits = Bits::new(s_196_2 as u128, 4u16);
        // D s_196_4: cmp-eq s_196_1 s_196_3
        let s_196_4: bool = ((s_196_1) == (s_196_3));
        // D s_196_5: write-var gs#142059 <= s_196_4
        fn_state.gs_142059 = s_196_4;
        // N s_196_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var opc2:u8
        let s_197_0: u8 = fn_state.opc2;
        // D s_197_1: cast zx s_197_0 -> bv
        let s_197_1: Bits = Bits::new(s_197_0 as u128, 3u16);
        // C s_197_2: const #0u : u8
        let s_197_2: u8 = 0;
        // C s_197_3: cast zx s_197_2 -> bv
        let s_197_3: Bits = Bits::new(s_197_2 as u128, 3u16);
        // D s_197_4: cmp-eq s_197_1 s_197_3
        let s_197_4: bool = ((s_197_1) == (s_197_3));
        // D s_197_5: write-var gs#142058 <= s_197_4
        fn_state.gs_142058 = s_197_4;
        // N s_197_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var opc1:u8
        let s_198_0: u8 = fn_state.opc1;
        // D s_198_1: cast zx s_198_0 -> bv
        let s_198_1: Bits = Bits::new(s_198_0 as u128, 3u16);
        // C s_198_2: const #0u : u8
        let s_198_2: u8 = 0;
        // C s_198_3: cast zx s_198_2 -> bv
        let s_198_3: Bits = Bits::new(s_198_2 as u128, 3u16);
        // D s_198_4: cmp-eq s_198_1 s_198_3
        let s_198_4: bool = ((s_198_1) == (s_198_3));
        // D s_198_5: write-var gs#142057 <= s_198_4
        fn_state.gs_142057 = s_198_4;
        // N s_198_6: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_199_0: read-var t:i
        let s_199_0: i128 = fn_state.t;
        // D s_199_1: call R_read(s_199_0)
        let s_199_1: u32 = R_read(state, tracer, s_199_0);
        // D s_199_2: write-var ga#248332 <= s_199_1
        fn_state.ga_248332 = s_199_1;
        // C s_199_3: const #() : ()
        let s_199_3: () = ();
        // S s_199_4: call CurrentSecurityState(s_199_3)
        let s_199_4: u32 = CurrentSecurityState(state, tracer, s_199_3);
        // C s_199_5: const #0u : u32
        let s_199_5: u32 = 0;
        // S s_199_6: cmp-eq s_199_4 s_199_5
        let s_199_6: bool = ((s_199_4) == (s_199_5));
        // N s_199_7: branch s_199_6 b214 b200
        if s_199_6 {
            return block_214(state, tracer, fn_state);
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
        // D s_200_1: write-var ga#248333 <= s_200_0
        fn_state.ga_248333 = s_200_0;
        // N s_200_2: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #18s : i
        let s_201_0: i128 = 18;
        // D s_201_1: read-var ga#248332:u32
        let s_201_1: u32 = fn_state.ga_248332;
        // D s_201_2: cast zx s_201_1 -> bv
        let s_201_2: Bits = Bits::new(s_201_1 as u128, 32u16);
        // D s_201_3: read-var ga#248333:u8
        let s_201_3: bool = fn_state.ga_248333;
        // D s_201_4: cast zx s_201_3 -> bv
        let s_201_4: Bits = Bits::new(s_201_3 as u128, 1u16);
        // C s_201_5: const #0s : i
        let s_201_5: i128 = 0;
        // C s_201_6: const #1u : u64
        let s_201_6: u64 = 1;
        // C s_201_7: cast zx s_201_6 -> bv
        let s_201_7: Bits = Bits::new(s_201_6 as u128, 64u16);
        // C s_201_8: lsl s_201_7 s_201_5
        let s_201_8: Bits = s_201_7 << s_201_5;
        // C s_201_9: sub s_201_8 s_201_7
        let s_201_9: Bits = ((s_201_8) - (s_201_7));
        // D s_201_10: and s_201_4 s_201_9
        let s_201_10: Bits = ((s_201_4) & (s_201_9));
        // D s_201_11: lsl s_201_10 s_201_0
        let s_201_11: Bits = s_201_10 << s_201_0;
        // C s_201_12: lsl s_201_9 s_201_0
        let s_201_12: Bits = s_201_9 << s_201_0;
        // C s_201_13: cmpl s_201_12
        let s_201_13: Bits = !s_201_12;
        // D s_201_14: and s_201_2 s_201_13
        let s_201_14: Bits = ((s_201_2) & (s_201_13));
        // D s_201_15: or s_201_14 s_201_11
        let s_201_15: Bits = ((s_201_14) | (s_201_11));
        // D s_201_16: cast reint s_201_15 -> u32
        let s_201_16: u32 = (s_201_15.value() as u32);
        // D s_201_17: read-var t:i
        let s_201_17: i128 = fn_state.t;
        // D s_201_18: call R_set(s_201_17, s_201_16)
        let s_201_18: () = R_set(state, tracer, s_201_17, s_201_16);
        // C s_201_19: const #424u : u32
        let s_201_19: u32 = 424;
        // D s_201_20: read-reg s_201_19:u8
        let s_201_20: u8 = {
            let value = state.read_register::<u8>(s_201_19 as isize);
            tracer.read_register(s_201_19 as isize, value);
            value
        };
        // C s_201_21: const #2u : u8
        let s_201_21: u8 = 2;
        // D s_201_22: cmp-lt s_201_20 s_201_21
        let s_201_22: bool = ((s_201_20) < (s_201_21));
        // N s_201_23: branch s_201_22 b203 b202
        if s_201_22 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_202(state, tracer, fn_state);
        };
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_202_0: return
        return;
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #424u : u32
        let s_203_0: u32 = 424;
        // D s_203_1: read-reg s_203_0:u8
        let s_203_1: u8 = {
            let value = state.read_register::<u8>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call ELUsingAArch32(s_203_1)
        let s_203_2: bool = ELUsingAArch32(state, tracer, s_203_1);
        // N s_203_3: branch s_203_2 b213 b204
        if s_203_2 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_204(state, tracer, fn_state);
        };
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #0u : u8
        let s_204_0: bool = false;
        // D s_204_1: write-var gs#142130 <= s_204_0
        fn_state.gs_142130 = s_204_0;
        // N s_204_2: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var gs#142130:u8
        let s_205_0: bool = fn_state.gs_142130;
        // N s_205_1: branch s_205_0 b212 b206
        if s_205_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #424u : u32
        let s_206_0: u32 = 424;
        // D s_206_1: read-reg s_206_0:u8
        let s_206_1: u8 = {
            let value = state.read_register::<u8>(s_206_0 as isize);
            tracer.read_register(s_206_0 as isize, value);
            value
        };
        // D s_206_2: call ELUsingAArch32(s_206_1)
        let s_206_2: bool = ELUsingAArch32(state, tracer, s_206_1);
        // D s_206_3: not s_206_2
        let s_206_3: bool = !s_206_2;
        // N s_206_4: branch s_206_3 b211 b207
        if s_206_3 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #0u : u8
        let s_207_0: bool = false;
        // D s_207_1: write-var gs#142131 <= s_207_0
        fn_state.gs_142131 = s_207_0;
        // N s_207_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#142131:u8
        let s_208_0: bool = fn_state.gs_142131;
        // N s_208_1: branch s_208_0 b210 b209
        if s_208_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_209(state, tracer, fn_state);
        };
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var t:i
        let s_209_0: i128 = fn_state.t;
        // D s_209_1: call R_read(s_209_0)
        let s_209_1: u32 = R_read(state, tracer, s_209_0);
        // C s_209_2: const #17s : i
        let s_209_2: i128 = 17;
        // D s_209_3: cast zx s_209_1 -> bv
        let s_209_3: Bits = Bits::new(s_209_1 as u128, 32u16);
        // C s_209_4: const #1u : u8
        let s_209_4: bool = true;
        // C s_209_5: cast zx s_209_4 -> bv
        let s_209_5: Bits = Bits::new(s_209_4 as u128, 1u16);
        // C s_209_6: const #0s : i
        let s_209_6: i128 = 0;
        // C s_209_7: const #1u : u64
        let s_209_7: u64 = 1;
        // C s_209_8: cast zx s_209_7 -> bv
        let s_209_8: Bits = Bits::new(s_209_7 as u128, 64u16);
        // C s_209_9: lsl s_209_8 s_209_6
        let s_209_9: Bits = s_209_8 << s_209_6;
        // C s_209_10: sub s_209_9 s_209_8
        let s_209_10: Bits = ((s_209_9) - (s_209_8));
        // C s_209_11: and s_209_5 s_209_10
        let s_209_11: Bits = ((s_209_5) & (s_209_10));
        // C s_209_12: lsl s_209_11 s_209_2
        let s_209_12: Bits = s_209_11 << s_209_2;
        // C s_209_13: lsl s_209_10 s_209_2
        let s_209_13: Bits = s_209_10 << s_209_2;
        // C s_209_14: cmpl s_209_13
        let s_209_14: Bits = !s_209_13;
        // D s_209_15: and s_209_3 s_209_14
        let s_209_15: Bits = ((s_209_3) & (s_209_14));
        // D s_209_16: or s_209_15 s_209_12
        let s_209_16: Bits = ((s_209_15) | (s_209_12));
        // D s_209_17: cast reint s_209_16 -> u32
        let s_209_17: u32 = (s_209_16.value() as u32);
        // D s_209_18: read-var t:i
        let s_209_18: i128 = fn_state.t;
        // D s_209_19: call R_set(s_209_18, s_209_17)
        let s_209_19: () = R_set(state, tracer, s_209_18, s_209_17);
        // N s_209_20: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var t:i
        let s_210_0: i128 = fn_state.t;
        // D s_210_1: call R_read(s_210_0)
        let s_210_1: u32 = R_read(state, tracer, s_210_0);
        // C s_210_2: const #17s : i
        let s_210_2: i128 = 17;
        // D s_210_3: cast zx s_210_1 -> bv
        let s_210_3: Bits = Bits::new(s_210_1 as u128, 32u16);
        // C s_210_4: const #0u : u8
        let s_210_4: bool = false;
        // C s_210_5: cast zx s_210_4 -> bv
        let s_210_5: Bits = Bits::new(s_210_4 as u128, 1u16);
        // C s_210_6: const #0s : i
        let s_210_6: i128 = 0;
        // C s_210_7: const #1u : u64
        let s_210_7: u64 = 1;
        // C s_210_8: cast zx s_210_7 -> bv
        let s_210_8: Bits = Bits::new(s_210_7 as u128, 64u16);
        // C s_210_9: lsl s_210_8 s_210_6
        let s_210_9: Bits = s_210_8 << s_210_6;
        // C s_210_10: sub s_210_9 s_210_8
        let s_210_10: Bits = ((s_210_9) - (s_210_8));
        // C s_210_11: and s_210_5 s_210_10
        let s_210_11: Bits = ((s_210_5) & (s_210_10));
        // C s_210_12: lsl s_210_11 s_210_2
        let s_210_12: Bits = s_210_11 << s_210_2;
        // C s_210_13: lsl s_210_10 s_210_2
        let s_210_13: Bits = s_210_10 << s_210_2;
        // C s_210_14: cmpl s_210_13
        let s_210_14: Bits = !s_210_13;
        // D s_210_15: and s_210_3 s_210_14
        let s_210_15: Bits = ((s_210_3) & (s_210_14));
        // D s_210_16: or s_210_15 s_210_12
        let s_210_16: Bits = ((s_210_15) | (s_210_12));
        // D s_210_17: cast reint s_210_16 -> u32
        let s_210_17: u32 = (s_210_16.value() as u32);
        // D s_210_18: read-var t:i
        let s_210_18: i128 = fn_state.t;
        // D s_210_19: call R_set(s_210_18, s_210_17)
        let s_210_19: () = R_set(state, tracer, s_210_18, s_210_17);
        // N s_210_20: return
        return;
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #22712u : u32
        let s_211_0: u32 = 22712;
        // D s_211_1: read-reg s_211_0:struct
        let s_211_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_211_0 as isize);
            tracer.read_register(s_211_0 as isize, value);
            value
        };
        // D s_211_2: call _get_MDCR_EL3_Type_SPME(s_211_1)
        let s_211_2: bool = u_get_MDCR_EL3_Type_SPME(state, tracer, s_211_1);
        // D s_211_3: cast zx s_211_2 -> bv
        let s_211_3: Bits = Bits::new(s_211_2 as u128, 1u16);
        // C s_211_4: const #1u : u8
        let s_211_4: bool = true;
        // C s_211_5: cast zx s_211_4 -> bv
        let s_211_5: Bits = Bits::new(s_211_4 as u128, 1u16);
        // D s_211_6: cmp-eq s_211_3 s_211_5
        let s_211_6: bool = ((s_211_3) == (s_211_5));
        // D s_211_7: write-var gs#142131 <= s_211_6
        fn_state.gs_142131 = s_211_6;
        // N s_211_8: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var t:i
        let s_212_0: i128 = fn_state.t;
        // D s_212_1: call R_read(s_212_0)
        let s_212_1: u32 = R_read(state, tracer, s_212_0);
        // C s_212_2: const #17s : i
        let s_212_2: i128 = 17;
        // D s_212_3: cast zx s_212_1 -> bv
        let s_212_3: Bits = Bits::new(s_212_1 as u128, 32u16);
        // C s_212_4: const #0u : u8
        let s_212_4: bool = false;
        // C s_212_5: cast zx s_212_4 -> bv
        let s_212_5: Bits = Bits::new(s_212_4 as u128, 1u16);
        // C s_212_6: const #0s : i
        let s_212_6: i128 = 0;
        // C s_212_7: const #1u : u64
        let s_212_7: u64 = 1;
        // C s_212_8: cast zx s_212_7 -> bv
        let s_212_8: Bits = Bits::new(s_212_7 as u128, 64u16);
        // C s_212_9: lsl s_212_8 s_212_6
        let s_212_9: Bits = s_212_8 << s_212_6;
        // C s_212_10: sub s_212_9 s_212_8
        let s_212_10: Bits = ((s_212_9) - (s_212_8));
        // C s_212_11: and s_212_5 s_212_10
        let s_212_11: Bits = ((s_212_5) & (s_212_10));
        // C s_212_12: lsl s_212_11 s_212_2
        let s_212_12: Bits = s_212_11 << s_212_2;
        // C s_212_13: lsl s_212_10 s_212_2
        let s_212_13: Bits = s_212_10 << s_212_2;
        // C s_212_14: cmpl s_212_13
        let s_212_14: Bits = !s_212_13;
        // D s_212_15: and s_212_3 s_212_14
        let s_212_15: Bits = ((s_212_3) & (s_212_14));
        // D s_212_16: or s_212_15 s_212_12
        let s_212_16: Bits = ((s_212_15) | (s_212_12));
        // D s_212_17: cast reint s_212_16 -> u32
        let s_212_17: u32 = (s_212_16.value() as u32);
        // D s_212_18: read-var t:i
        let s_212_18: i128 = fn_state.t;
        // D s_212_19: call R_set(s_212_18, s_212_17)
        let s_212_19: () = R_set(state, tracer, s_212_18, s_212_17);
        // N s_212_20: return
        return;
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #15048u : u32
        let s_213_0: u32 = 15048;
        // D s_213_1: read-reg s_213_0:struct
        let s_213_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_213_0 as isize);
            tracer.read_register(s_213_0 as isize, value);
            value
        };
        // D s_213_2: call _get_SDCR_Type_SPME(s_213_1)
        let s_213_2: bool = u_get_SDCR_Type_SPME(state, tracer, s_213_1);
        // D s_213_3: cast zx s_213_2 -> bv
        let s_213_3: Bits = Bits::new(s_213_2 as u128, 1u16);
        // C s_213_4: const #1u : u8
        let s_213_4: bool = true;
        // C s_213_5: cast zx s_213_4 -> bv
        let s_213_5: Bits = Bits::new(s_213_4 as u128, 1u16);
        // D s_213_6: cmp-eq s_213_3 s_213_5
        let s_213_6: bool = ((s_213_3) == (s_213_5));
        // D s_213_7: write-var gs#142130 <= s_213_6
        fn_state.gs_142130 = s_213_6;
        // N s_213_8: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #1u : u8
        let s_214_0: bool = true;
        // D s_214_1: write-var ga#248333 <= s_214_0
        fn_state.ga_248333 = s_214_0;
        // N s_214_2: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var CRm:u8
        let s_215_0: u8 = fn_state.CRm;
        // D s_215_1: cast zx s_215_0 -> bv
        let s_215_1: Bits = Bits::new(s_215_0 as u128, 4u16);
        // C s_215_2: const #2u : u8
        let s_215_2: u8 = 2;
        // C s_215_3: cast zx s_215_2 -> bv
        let s_215_3: Bits = Bits::new(s_215_2 as u128, 4u16);
        // D s_215_4: cmp-eq s_215_1 s_215_3
        let s_215_4: bool = ((s_215_1) == (s_215_3));
        // D s_215_5: write-var gs#142055 <= s_215_4
        fn_state.gs_142055 = s_215_4;
        // N s_215_6: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var CRn:u8
        let s_216_0: u8 = fn_state.CRn;
        // D s_216_1: cast zx s_216_0 -> bv
        let s_216_1: Bits = Bits::new(s_216_0 as u128, 4u16);
        // C s_216_2: const #0u : u8
        let s_216_2: u8 = 0;
        // C s_216_3: cast zx s_216_2 -> bv
        let s_216_3: Bits = Bits::new(s_216_2 as u128, 4u16);
        // D s_216_4: cmp-eq s_216_1 s_216_3
        let s_216_4: bool = ((s_216_1) == (s_216_3));
        // D s_216_5: write-var gs#142054 <= s_216_4
        fn_state.gs_142054 = s_216_4;
        // N s_216_6: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var opc2:u8
        let s_217_0: u8 = fn_state.opc2;
        // D s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 3u16);
        // C s_217_2: const #2u : u8
        let s_217_2: u8 = 2;
        // C s_217_3: cast zx s_217_2 -> bv
        let s_217_3: Bits = Bits::new(s_217_2 as u128, 3u16);
        // D s_217_4: cmp-eq s_217_1 s_217_3
        let s_217_4: bool = ((s_217_1) == (s_217_3));
        // D s_217_5: write-var gs#142053 <= s_217_4
        fn_state.gs_142053 = s_217_4;
        // N s_217_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var opc1:u8
        let s_218_0: u8 = fn_state.opc1;
        // D s_218_1: cast zx s_218_0 -> bv
        let s_218_1: Bits = Bits::new(s_218_0 as u128, 3u16);
        // C s_218_2: const #0u : u8
        let s_218_2: u8 = 0;
        // C s_218_3: cast zx s_218_2 -> bv
        let s_218_3: Bits = Bits::new(s_218_2 as u128, 3u16);
        // D s_218_4: cmp-eq s_218_1 s_218_3
        let s_218_4: bool = ((s_218_1) == (s_218_3));
        // D s_218_5: write-var gs#142052 <= s_218_4
        fn_state.gs_142052 = s_218_4;
        // N s_218_6: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_219_0: read-var CRm:u8
        let s_219_0: u8 = fn_state.CRm;
        // D s_219_1: cast zx s_219_0 -> bv
        let s_219_1: Bits = Bits::new(s_219_0 as u128, 4u16);
        // C s_219_2: const #8u : u8
        let s_219_2: u8 = 8;
        // C s_219_3: cast zx s_219_2 -> bv
        let s_219_3: Bits = Bits::new(s_219_2 as u128, 4u16);
        // D s_219_4: cmp-eq s_219_1 s_219_3
        let s_219_4: bool = ((s_219_1) == (s_219_3));
        // N s_219_5: branch s_219_4 b224 b220
        if s_219_4 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_220_0: jump b221
        return block_221(state, tracer, fn_state);
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
        // C s_221_2: const #9u : u8
        let s_221_2: u8 = 9;
        // C s_221_3: cast zx s_221_2 -> bv
        let s_221_3: Bits = Bits::new(s_221_2 as u128, 4u16);
        // D s_221_4: cmp-eq s_221_1 s_221_3
        let s_221_4: bool = ((s_221_1) == (s_221_3));
        // N s_221_5: branch s_221_4 b223 b222
        if s_221_4 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_222(state, tracer, fn_state);
        };
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_222_0: return
        return;
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var t:i
        let s_223_0: i128 = fn_state.t;
        // D s_223_1: call R_read(s_223_0)
        let s_223_1: u32 = R_read(state, tracer, s_223_0);
        // C s_223_2: const #() : ()
        let s_223_2: () = ();
        // S s_223_3: call DBGCLAIMCLR_read(s_223_2)
        let s_223_3: ProductType700c18a878c5601b = DBGCLAIMCLR_read(
            state,
            tracer,
            s_223_2,
        );
        // D s_223_4: write-var ga#248320 <= s_223_3
        fn_state.ga_248320 = s_223_3;
        // D s_223_5: read-var ga#248320.0:struct
        let s_223_5: u32 = fn_state.ga_248320._0;
        // C s_223_6: const #0s : i
        let s_223_6: i128 = 0;
        // D s_223_7: cast zx s_223_5 -> bv
        let s_223_7: Bits = Bits::new(s_223_5 as u128, 32u16);
        // C s_223_8: const #1s : i64
        let s_223_8: i64 = 1;
        // C s_223_9: cast zx s_223_8 -> i
        let s_223_9: i128 = (i128::try_from(s_223_8).unwrap());
        // C s_223_10: const #7s : i
        let s_223_10: i128 = 7;
        // C s_223_11: add s_223_10 s_223_9
        let s_223_11: i128 = (s_223_10 + s_223_9);
        // D s_223_12: bit-extract s_223_7 s_223_6 s_223_11
        let s_223_12: Bits = (Bits::new(
            ((s_223_7) >> (s_223_6)).value(),
            u16::try_from(s_223_11).unwrap(),
        ));
        // D s_223_13: cast reint s_223_12 -> u8
        let s_223_13: u8 = (s_223_12.value() as u8);
        // C s_223_14: const #0s : i
        let s_223_14: i128 = 0;
        // D s_223_15: cast zx s_223_1 -> bv
        let s_223_15: Bits = Bits::new(s_223_1 as u128, 32u16);
        // D s_223_16: cast zx s_223_13 -> bv
        let s_223_16: Bits = Bits::new(s_223_13 as u128, 8u16);
        // C s_223_17: const #7s : i
        let s_223_17: i128 = 7;
        // C s_223_18: const #1u : u64
        let s_223_18: u64 = 1;
        // C s_223_19: cast zx s_223_18 -> bv
        let s_223_19: Bits = Bits::new(s_223_18 as u128, 64u16);
        // C s_223_20: lsl s_223_19 s_223_17
        let s_223_20: Bits = s_223_19 << s_223_17;
        // C s_223_21: sub s_223_20 s_223_19
        let s_223_21: Bits = ((s_223_20) - (s_223_19));
        // D s_223_22: and s_223_16 s_223_21
        let s_223_22: Bits = ((s_223_16) & (s_223_21));
        // D s_223_23: lsl s_223_22 s_223_14
        let s_223_23: Bits = s_223_22 << s_223_14;
        // C s_223_24: lsl s_223_21 s_223_14
        let s_223_24: Bits = s_223_21 << s_223_14;
        // C s_223_25: cmpl s_223_24
        let s_223_25: Bits = !s_223_24;
        // D s_223_26: and s_223_15 s_223_25
        let s_223_26: Bits = ((s_223_15) & (s_223_25));
        // D s_223_27: or s_223_26 s_223_23
        let s_223_27: Bits = ((s_223_26) | (s_223_23));
        // D s_223_28: cast reint s_223_27 -> u32
        let s_223_28: u32 = (s_223_27.value() as u32);
        // D s_223_29: read-var t:i
        let s_223_29: i128 = fn_state.t;
        // D s_223_30: call R_set(s_223_29, s_223_28)
        let s_223_30: () = R_set(state, tracer, s_223_29, s_223_28);
        // N s_223_31: return
        return;
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var t:i
        let s_224_0: i128 = fn_state.t;
        // D s_224_1: call R_read(s_224_0)
        let s_224_1: u32 = R_read(state, tracer, s_224_0);
        // C s_224_2: const #0s : i
        let s_224_2: i128 = 0;
        // D s_224_3: cast zx s_224_1 -> bv
        let s_224_3: Bits = Bits::new(s_224_1 as u128, 32u16);
        // C s_224_4: const #255u : u8
        let s_224_4: u8 = 255;
        // C s_224_5: cast zx s_224_4 -> bv
        let s_224_5: Bits = Bits::new(s_224_4 as u128, 8u16);
        // C s_224_6: const #7s : i
        let s_224_6: i128 = 7;
        // C s_224_7: const #1u : u64
        let s_224_7: u64 = 1;
        // C s_224_8: cast zx s_224_7 -> bv
        let s_224_8: Bits = Bits::new(s_224_7 as u128, 64u16);
        // C s_224_9: lsl s_224_8 s_224_6
        let s_224_9: Bits = s_224_8 << s_224_6;
        // C s_224_10: sub s_224_9 s_224_8
        let s_224_10: Bits = ((s_224_9) - (s_224_8));
        // C s_224_11: and s_224_5 s_224_10
        let s_224_11: Bits = ((s_224_5) & (s_224_10));
        // C s_224_12: lsl s_224_11 s_224_2
        let s_224_12: Bits = s_224_11 << s_224_2;
        // C s_224_13: lsl s_224_10 s_224_2
        let s_224_13: Bits = s_224_10 << s_224_2;
        // C s_224_14: cmpl s_224_13
        let s_224_14: Bits = !s_224_13;
        // D s_224_15: and s_224_3 s_224_14
        let s_224_15: Bits = ((s_224_3) & (s_224_14));
        // D s_224_16: or s_224_15 s_224_12
        let s_224_16: Bits = ((s_224_15) | (s_224_12));
        // D s_224_17: cast reint s_224_16 -> u32
        let s_224_17: u32 = (s_224_16.value() as u32);
        // D s_224_18: read-var t:i
        let s_224_18: i128 = fn_state.t;
        // D s_224_19: call R_set(s_224_18, s_224_17)
        let s_224_19: () = R_set(state, tracer, s_224_18, s_224_17);
        // N s_224_20: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_225_0: read-var CRn:u8
        let s_225_0: u8 = fn_state.CRn;
        // D s_225_1: cast zx s_225_0 -> bv
        let s_225_1: Bits = Bits::new(s_225_0 as u128, 4u16);
        // C s_225_2: const #7u : u8
        let s_225_2: u8 = 7;
        // C s_225_3: cast zx s_225_2 -> bv
        let s_225_3: Bits = Bits::new(s_225_2 as u128, 4u16);
        // D s_225_4: cmp-eq s_225_1 s_225_3
        let s_225_4: bool = ((s_225_1) == (s_225_3));
        // D s_225_5: write-var gs#142050 <= s_225_4
        fn_state.gs_142050 = s_225_4;
        // N s_225_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var opc2:u8
        let s_226_0: u8 = fn_state.opc2;
        // D s_226_1: cast zx s_226_0 -> bv
        let s_226_1: Bits = Bits::new(s_226_0 as u128, 3u16);
        // C s_226_2: const #6u : u8
        let s_226_2: u8 = 6;
        // C s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 3u16);
        // D s_226_4: cmp-eq s_226_1 s_226_3
        let s_226_4: bool = ((s_226_1) == (s_226_3));
        // D s_226_5: write-var gs#142049 <= s_226_4
        fn_state.gs_142049 = s_226_4;
        // N s_226_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var opc1:u8
        let s_227_0: u8 = fn_state.opc1;
        // D s_227_1: cast zx s_227_0 -> bv
        let s_227_1: Bits = Bits::new(s_227_0 as u128, 3u16);
        // C s_227_2: const #0u : u8
        let s_227_2: u8 = 0;
        // C s_227_3: cast zx s_227_2 -> bv
        let s_227_3: Bits = Bits::new(s_227_2 as u128, 3u16);
        // D s_227_4: cmp-eq s_227_1 s_227_3
        let s_227_4: bool = ((s_227_1) == (s_227_3));
        // D s_227_5: write-var gs#142048 <= s_227_4
        fn_state.gs_142048 = s_227_4;
        // N s_227_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #0s : i
        let s_228_0: i128 = 0;
        // D s_228_1: read-var CRm:u8
        let s_228_1: u8 = fn_state.CRm;
        // D s_228_2: cast zx s_228_1 -> bv
        let s_228_2: Bits = Bits::new(s_228_1 as u128, 4u16);
        // C s_228_3: const #1s : i64
        let s_228_3: i64 = 1;
        // C s_228_4: cast zx s_228_3 -> i
        let s_228_4: i128 = (i128::try_from(s_228_3).unwrap());
        // C s_228_5: const #1s : i
        let s_228_5: i128 = 1;
        // C s_228_6: add s_228_5 s_228_4
        let s_228_6: i128 = (s_228_5 + s_228_4);
        // D s_228_7: bit-extract s_228_2 s_228_0 s_228_6
        let s_228_7: Bits = (Bits::new(
            ((s_228_2) >> (s_228_0)).value(),
            u16::try_from(s_228_6).unwrap(),
        ));
        // D s_228_8: cast reint s_228_7 -> u8
        let s_228_8: u8 = (s_228_7.value() as u8);
        // C s_228_9: const #0s : i
        let s_228_9: i128 = 0;
        // D s_228_10: read-var opc2:u8
        let s_228_10: u8 = fn_state.opc2;
        // D s_228_11: cast zx s_228_10 -> bv
        let s_228_11: Bits = Bits::new(s_228_10 as u128, 3u16);
        // C s_228_12: const #1s : i64
        let s_228_12: i64 = 1;
        // C s_228_13: cast zx s_228_12 -> i
        let s_228_13: i128 = (i128::try_from(s_228_12).unwrap());
        // C s_228_14: const #2s : i
        let s_228_14: i128 = 2;
        // C s_228_15: add s_228_14 s_228_13
        let s_228_15: i128 = (s_228_14 + s_228_13);
        // D s_228_16: bit-extract s_228_11 s_228_9 s_228_15
        let s_228_16: Bits = (Bits::new(
            ((s_228_11) >> (s_228_9)).value(),
            u16::try_from(s_228_15).unwrap(),
        ));
        // D s_228_17: cast reint s_228_16 -> u8
        let s_228_17: u8 = (s_228_16.value() as u8);
        // D s_228_18: cast zx s_228_8 -> bv
        let s_228_18: Bits = Bits::new(s_228_8 as u128, 2u16);
        // D s_228_19: cast zx s_228_17 -> bv
        let s_228_19: Bits = Bits::new(s_228_17 as u128, 3u16);
        // D s_228_20: cast reint s_228_18 -> u128
        let s_228_20: u128 = (s_228_18.value() as u128);
        // D s_228_21: size-of s_228_18
        let s_228_21: u16 = s_228_18.length();
        // D s_228_22: cast reint s_228_19 -> u128
        let s_228_22: u128 = (s_228_19.value() as u128);
        // D s_228_23: size-of s_228_19
        let s_228_23: u16 = s_228_19.length();
        // D s_228_24: lsl s_228_20 s_228_23
        let s_228_24: u128 = s_228_20 << s_228_23;
        // D s_228_25: or s_228_24 s_228_22
        let s_228_25: u128 = ((s_228_24) | (s_228_22));
        // D s_228_26: add s_228_21 s_228_23
        let s_228_26: u16 = (s_228_21 + s_228_23);
        // D s_228_27: create-bits s_228_25 s_228_26
        let s_228_27: Bits = Bits::new(s_228_25, s_228_26);
        // D s_228_28: cast reint s_228_27 -> u8
        let s_228_28: u8 = (s_228_27.value() as u8);
        // D s_228_29: cast zx s_228_28 -> bv
        let s_228_29: Bits = Bits::new(s_228_28 as u128, 5u16);
        // D s_228_30: cast zx s_228_29 -> i
        let s_228_30: i128 = (s_228_29.value() as i128);
        // D s_228_31: cast reint s_228_30 -> i64
        let s_228_31: i64 = (s_228_30 as i64);
        // C s_228_32: const #() : ()
        let s_228_32: () = ();
        // S s_228_33: call GetNumEventCounters(s_228_32)
        let s_228_33: i128 = GetNumEventCounters(state, tracer, s_228_32);
        // C s_228_34: const #1s : i
        let s_228_34: i128 = 1;
        // S s_228_35: sub s_228_33 s_228_34
        let s_228_35: i128 = ((s_228_33) - (s_228_34));
        // D s_228_36: cast zx s_228_31 -> i
        let s_228_36: i128 = (i128::try_from(s_228_31).unwrap());
        // D s_228_37: cmp-gt s_228_36 s_228_35
        let s_228_37: bool = ((s_228_36) > (s_228_35));
        // N s_228_38: branch s_228_37 b247 b229
        if s_228_37 {
            return block_247(state, tracer, fn_state);
        } else {
            return block_229(state, tracer, fn_state);
        };
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #() : ()
        let s_229_0: () = ();
        // S s_229_1: call EL2Enabled(s_229_0)
        let s_229_1: bool = EL2Enabled(state, tracer, s_229_0);
        // N s_229_2: branch s_229_1 b243 b230
        if s_229_1 {
            return block_243(state, tracer, fn_state);
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
        // D s_230_1: write-var gs#142155 <= s_230_0
        fn_state.gs_142155 = s_230_0;
        // N s_230_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_231_0: read-var gs#142155:u8
        let s_231_0: bool = fn_state.gs_142155;
        // N s_231_1: branch s_231_0 b242 b232
        if s_231_0 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_232(state, tracer, fn_state);
        };
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #0u : u8
        let s_232_0: bool = false;
        // D s_232_1: write-var gs#142161 <= s_232_0
        fn_state.gs_142161 = s_232_0;
        // N s_232_2: jump b233
        return block_233(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_233_0: read-var gs#142161:u8
        let s_233_0: bool = fn_state.gs_142161;
        // D s_233_1: write-var gs#142162 <= s_233_0
        fn_state.gs_142162 = s_233_0;
        // N s_233_2: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_234_0: read-var gs#142162:u8
        let s_234_0: bool = fn_state.gs_142162;
        // N s_234_1: branch s_234_0 b237 b235
        if s_234_0 {
            return block_237(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_235_0: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_236_0: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #0s : i
        let s_237_0: i128 = 0;
        // D s_237_1: read-var CRm:u8
        let s_237_1: u8 = fn_state.CRm;
        // D s_237_2: cast zx s_237_1 -> bv
        let s_237_2: Bits = Bits::new(s_237_1 as u128, 4u16);
        // C s_237_3: const #1s : i64
        let s_237_3: i64 = 1;
        // C s_237_4: cast zx s_237_3 -> i
        let s_237_4: i128 = (i128::try_from(s_237_3).unwrap());
        // C s_237_5: const #1s : i
        let s_237_5: i128 = 1;
        // C s_237_6: add s_237_5 s_237_4
        let s_237_6: i128 = (s_237_5 + s_237_4);
        // D s_237_7: bit-extract s_237_2 s_237_0 s_237_6
        let s_237_7: Bits = (Bits::new(
            ((s_237_2) >> (s_237_0)).value(),
            u16::try_from(s_237_6).unwrap(),
        ));
        // D s_237_8: cast reint s_237_7 -> u8
        let s_237_8: u8 = (s_237_7.value() as u8);
        // C s_237_9: const #0s : i
        let s_237_9: i128 = 0;
        // D s_237_10: read-var opc2:u8
        let s_237_10: u8 = fn_state.opc2;
        // D s_237_11: cast zx s_237_10 -> bv
        let s_237_11: Bits = Bits::new(s_237_10 as u128, 3u16);
        // C s_237_12: const #1s : i64
        let s_237_12: i64 = 1;
        // C s_237_13: cast zx s_237_12 -> i
        let s_237_13: i128 = (i128::try_from(s_237_12).unwrap());
        // C s_237_14: const #2s : i
        let s_237_14: i128 = 2;
        // C s_237_15: add s_237_14 s_237_13
        let s_237_15: i128 = (s_237_14 + s_237_13);
        // D s_237_16: bit-extract s_237_11 s_237_9 s_237_15
        let s_237_16: Bits = (Bits::new(
            ((s_237_11) >> (s_237_9)).value(),
            u16::try_from(s_237_15).unwrap(),
        ));
        // D s_237_17: cast reint s_237_16 -> u8
        let s_237_17: u8 = (s_237_16.value() as u8);
        // D s_237_18: cast zx s_237_8 -> bv
        let s_237_18: Bits = Bits::new(s_237_8 as u128, 2u16);
        // D s_237_19: cast zx s_237_17 -> bv
        let s_237_19: Bits = Bits::new(s_237_17 as u128, 3u16);
        // D s_237_20: cast reint s_237_18 -> u128
        let s_237_20: u128 = (s_237_18.value() as u128);
        // D s_237_21: size-of s_237_18
        let s_237_21: u16 = s_237_18.length();
        // D s_237_22: cast reint s_237_19 -> u128
        let s_237_22: u128 = (s_237_19.value() as u128);
        // D s_237_23: size-of s_237_19
        let s_237_23: u16 = s_237_19.length();
        // D s_237_24: lsl s_237_20 s_237_23
        let s_237_24: u128 = s_237_20 << s_237_23;
        // D s_237_25: or s_237_24 s_237_22
        let s_237_25: u128 = ((s_237_24) | (s_237_22));
        // D s_237_26: add s_237_21 s_237_23
        let s_237_26: u16 = (s_237_21 + s_237_23);
        // D s_237_27: create-bits s_237_25 s_237_26
        let s_237_27: Bits = Bits::new(s_237_25, s_237_26);
        // D s_237_28: cast reint s_237_27 -> u8
        let s_237_28: u8 = (s_237_27.value() as u8);
        // D s_237_29: cast zx s_237_28 -> bv
        let s_237_29: Bits = Bits::new(s_237_28 as u128, 5u16);
        // D s_237_30: cast zx s_237_29 -> i
        let s_237_30: i128 = (s_237_29.value() as i128);
        // D s_237_31: cast reint s_237_30 -> i64
        let s_237_31: i64 = (s_237_30 as i64);
        // C s_237_32: const #() : ()
        let s_237_32: () = ();
        // S s_237_33: call GetNumEventCounters(s_237_32)
        let s_237_33: i128 = GetNumEventCounters(state, tracer, s_237_32);
        // C s_237_34: const #1s : i
        let s_237_34: i128 = 1;
        // S s_237_35: sub s_237_33 s_237_34
        let s_237_35: i128 = ((s_237_33) - (s_237_34));
        // D s_237_36: cast zx s_237_31 -> i
        let s_237_36: i128 = (i128::try_from(s_237_31).unwrap());
        // D s_237_37: cmp-gt s_237_36 s_237_35
        let s_237_37: bool = ((s_237_36) > (s_237_35));
        // N s_237_38: branch s_237_37 b241 b238
        if s_237_37 {
            return block_241(state, tracer, fn_state);
        } else {
            return block_238(state, tracer, fn_state);
        };
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #432u : u32
        let s_238_0: u32 = 432;
        // D s_238_1: read-reg s_238_0:u8
        let s_238_1: u8 = {
            let value = state.read_register::<u8>(s_238_0 as isize);
            tracer.read_register(s_238_0 as isize, value);
            value
        };
        // D s_238_2: call ELUsingAArch32(s_238_1)
        let s_238_2: bool = ELUsingAArch32(state, tracer, s_238_1);
        // N s_238_3: branch s_238_2 b240 b239
        if s_238_2 {
            return block_240(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_239_0: read-var t:i
        let s_239_0: i128 = fn_state.t;
        // D s_239_1: read-var temp:u32
        let s_239_1: u32 = fn_state.temp;
        // D s_239_2: call R_set(s_239_0, s_239_1)
        let s_239_2: () = R_set(state, tracer, s_239_0, s_239_1);
        // C s_239_3: const #3u : u8
        let s_239_3: u8 = 3;
        // C s_239_4: cast zx s_239_3 -> bv
        let s_239_4: Bits = Bits::new(s_239_3 as u128, 8u16);
        // C s_239_5: cast zx s_239_4 -> i
        let s_239_5: i128 = (s_239_4.value() as i128);
        // C s_239_6: cast reint s_239_5 -> i64
        let s_239_6: i64 = (s_239_5 as i64);
        // C s_239_7: cast zx s_239_6 -> i
        let s_239_7: i128 = (i128::try_from(s_239_6).unwrap());
        // C s_239_8: const #432u : u32
        let s_239_8: u32 = 432;
        // D s_239_9: read-reg s_239_8:u8
        let s_239_9: u8 = {
            let value = state.read_register::<u8>(s_239_8 as isize);
            tracer.read_register(s_239_8 as isize, value);
            value
        };
        // D s_239_10: call AArch64_AArch32SystemAccessTrap(s_239_9, s_239_7)
        let s_239_10: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_239_9,
            s_239_7,
        );
        // N s_239_11: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var t:i
        let s_240_0: i128 = fn_state.t;
        // D s_240_1: read-var temp:u32
        let s_240_1: u32 = fn_state.temp;
        // D s_240_2: call R_set(s_240_0, s_240_1)
        let s_240_2: () = R_set(state, tracer, s_240_0, s_240_1);
        // C s_240_3: const #3u : u8
        let s_240_3: u8 = 3;
        // C s_240_4: cast zx s_240_3 -> bv
        let s_240_4: Bits = Bits::new(s_240_3 as u128, 8u16);
        // C s_240_5: cast zx s_240_4 -> i
        let s_240_5: i128 = (s_240_4.value() as i128);
        // C s_240_6: cast reint s_240_5 -> i64
        let s_240_6: i64 = (s_240_5 as i64);
        // C s_240_7: cast zx s_240_6 -> i
        let s_240_7: i128 = (i128::try_from(s_240_6).unwrap());
        // S s_240_8: call AArch32_TakeHypTrapException(s_240_7)
        let s_240_8: () = AArch32_TakeHypTrapException(state, tracer, s_240_7);
        // N s_240_9: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_241_0: read-var t:i
        let s_241_0: i128 = fn_state.t;
        // D s_241_1: read-var temp:u32
        let s_241_1: u32 = fn_state.temp;
        // D s_241_2: call R_set(s_241_0, s_241_1)
        let s_241_2: () = R_set(state, tracer, s_241_0, s_241_1);
        // N s_241_3: panic
        panic!("{:?}", ());
        // N s_241_4: return
        return;
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_242_0: const #0s : i
        let s_242_0: i128 = 0;
        // D s_242_1: read-var CRm:u8
        let s_242_1: u8 = fn_state.CRm;
        // D s_242_2: cast zx s_242_1 -> bv
        let s_242_2: Bits = Bits::new(s_242_1 as u128, 4u16);
        // C s_242_3: const #1s : i64
        let s_242_3: i64 = 1;
        // C s_242_4: cast zx s_242_3 -> i
        let s_242_4: i128 = (i128::try_from(s_242_3).unwrap());
        // C s_242_5: const #1s : i
        let s_242_5: i128 = 1;
        // C s_242_6: add s_242_5 s_242_4
        let s_242_6: i128 = (s_242_5 + s_242_4);
        // D s_242_7: bit-extract s_242_2 s_242_0 s_242_6
        let s_242_7: Bits = (Bits::new(
            ((s_242_2) >> (s_242_0)).value(),
            u16::try_from(s_242_6).unwrap(),
        ));
        // D s_242_8: cast reint s_242_7 -> u8
        let s_242_8: u8 = (s_242_7.value() as u8);
        // C s_242_9: const #0s : i
        let s_242_9: i128 = 0;
        // D s_242_10: read-var opc2:u8
        let s_242_10: u8 = fn_state.opc2;
        // D s_242_11: cast zx s_242_10 -> bv
        let s_242_11: Bits = Bits::new(s_242_10 as u128, 3u16);
        // C s_242_12: const #1s : i64
        let s_242_12: i64 = 1;
        // C s_242_13: cast zx s_242_12 -> i
        let s_242_13: i128 = (i128::try_from(s_242_12).unwrap());
        // C s_242_14: const #2s : i
        let s_242_14: i128 = 2;
        // C s_242_15: add s_242_14 s_242_13
        let s_242_15: i128 = (s_242_14 + s_242_13);
        // D s_242_16: bit-extract s_242_11 s_242_9 s_242_15
        let s_242_16: Bits = (Bits::new(
            ((s_242_11) >> (s_242_9)).value(),
            u16::try_from(s_242_15).unwrap(),
        ));
        // D s_242_17: cast reint s_242_16 -> u8
        let s_242_17: u8 = (s_242_16.value() as u8);
        // D s_242_18: cast zx s_242_8 -> bv
        let s_242_18: Bits = Bits::new(s_242_8 as u128, 2u16);
        // D s_242_19: cast zx s_242_17 -> bv
        let s_242_19: Bits = Bits::new(s_242_17 as u128, 3u16);
        // D s_242_20: cast reint s_242_18 -> u128
        let s_242_20: u128 = (s_242_18.value() as u128);
        // D s_242_21: size-of s_242_18
        let s_242_21: u16 = s_242_18.length();
        // D s_242_22: cast reint s_242_19 -> u128
        let s_242_22: u128 = (s_242_19.value() as u128);
        // D s_242_23: size-of s_242_19
        let s_242_23: u16 = s_242_19.length();
        // D s_242_24: lsl s_242_20 s_242_23
        let s_242_24: u128 = s_242_20 << s_242_23;
        // D s_242_25: or s_242_24 s_242_22
        let s_242_25: u128 = ((s_242_24) | (s_242_22));
        // D s_242_26: add s_242_21 s_242_23
        let s_242_26: u16 = (s_242_21 + s_242_23);
        // D s_242_27: create-bits s_242_25 s_242_26
        let s_242_27: Bits = Bits::new(s_242_25, s_242_26);
        // D s_242_28: cast reint s_242_27 -> u8
        let s_242_28: u8 = (s_242_27.value() as u8);
        // D s_242_29: cast zx s_242_28 -> bv
        let s_242_29: Bits = Bits::new(s_242_28 as u128, 5u16);
        // D s_242_30: cast zx s_242_29 -> i
        let s_242_30: i128 = (s_242_29.value() as i128);
        // D s_242_31: cast reint s_242_30 -> i64
        let s_242_31: i64 = (s_242_30 as i64);
        // C s_242_32: const #() : ()
        let s_242_32: () = ();
        // S s_242_33: call AArch32_GetNumEventCountersAccessible(s_242_32)
        let s_242_33: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_242_32,
        );
        // C s_242_34: const #1s : i
        let s_242_34: i128 = 1;
        // S s_242_35: sub s_242_33 s_242_34
        let s_242_35: i128 = ((s_242_33) - (s_242_34));
        // D s_242_36: cast zx s_242_31 -> i
        let s_242_36: i128 = (i128::try_from(s_242_31).unwrap());
        // D s_242_37: cmp-gt s_242_36 s_242_35
        let s_242_37: bool = ((s_242_36) > (s_242_35));
        // D s_242_38: write-var gs#142161 <= s_242_37
        fn_state.gs_142161 = s_242_37;
        // N s_242_39: jump b233
        return block_233(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #16975u : u32
        let s_243_0: u32 = 16975;
        // D s_243_1: read-reg s_243_0:u8
        let s_243_1: u8 = {
            let value = state.read_register::<u8>(s_243_0 as isize);
            tracer.read_register(s_243_0 as isize, value);
            value
        };
        // D s_243_2: cast zx s_243_1 -> bv
        let s_243_2: Bits = Bits::new(s_243_1 as u128, 2u16);
        // C s_243_3: const #448u : u32
        let s_243_3: u32 = 448;
        // D s_243_4: read-reg s_243_3:u8
        let s_243_4: u8 = {
            let value = state.read_register::<u8>(s_243_3 as isize);
            tracer.read_register(s_243_3 as isize, value);
            value
        };
        // D s_243_5: cast zx s_243_4 -> bv
        let s_243_5: Bits = Bits::new(s_243_4 as u128, 2u16);
        // D s_243_6: cmp-eq s_243_2 s_243_5
        let s_243_6: bool = ((s_243_2) == (s_243_5));
        // N s_243_7: branch s_243_6 b246 b244
        if s_243_6 {
            return block_246(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #16975u : u32
        let s_244_0: u32 = 16975;
        // D s_244_1: read-reg s_244_0:u8
        let s_244_1: u8 = {
            let value = state.read_register::<u8>(s_244_0 as isize);
            tracer.read_register(s_244_0 as isize, value);
            value
        };
        // D s_244_2: cast zx s_244_1 -> bv
        let s_244_2: Bits = Bits::new(s_244_1 as u128, 2u16);
        // C s_244_3: const #440u : u32
        let s_244_3: u32 = 440;
        // D s_244_4: read-reg s_244_3:u8
        let s_244_4: u8 = {
            let value = state.read_register::<u8>(s_244_3 as isize);
            tracer.read_register(s_244_3 as isize, value);
            value
        };
        // D s_244_5: cast zx s_244_4 -> bv
        let s_244_5: Bits = Bits::new(s_244_4 as u128, 2u16);
        // D s_244_6: cmp-eq s_244_2 s_244_5
        let s_244_6: bool = ((s_244_2) == (s_244_5));
        // D s_244_7: write-var gs#142154 <= s_244_6
        fn_state.gs_142154 = s_244_6;
        // N s_244_8: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_245_0: read-var gs#142154:u8
        let s_245_0: bool = fn_state.gs_142154;
        // D s_245_1: write-var gs#142155 <= s_245_0
        fn_state.gs_142155 = s_245_0;
        // N s_245_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #1u : u8
        let s_246_0: bool = true;
        // D s_246_1: write-var gs#142154 <= s_246_0
        fn_state.gs_142154 = s_246_0;
        // N s_246_2: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #1u : u8
        let s_247_0: bool = true;
        // D s_247_1: write-var gs#142162 <= s_247_0
        fn_state.gs_142162 = s_247_0;
        // N s_247_2: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_248_0: const #2s : i
        let s_248_0: i128 = 2;
        // D s_248_1: read-var CRm:u8
        let s_248_1: u8 = fn_state.CRm;
        // D s_248_2: cast zx s_248_1 -> bv
        let s_248_2: Bits = Bits::new(s_248_1 as u128, 4u16);
        // C s_248_3: const #1s : i64
        let s_248_3: i64 = 1;
        // C s_248_4: cast zx s_248_3 -> i
        let s_248_4: i128 = (i128::try_from(s_248_3).unwrap());
        // C s_248_5: const #1s : i
        let s_248_5: i128 = 1;
        // C s_248_6: add s_248_5 s_248_4
        let s_248_6: i128 = (s_248_5 + s_248_4);
        // D s_248_7: bit-extract s_248_2 s_248_0 s_248_6
        let s_248_7: Bits = (Bits::new(
            ((s_248_2) >> (s_248_0)).value(),
            u16::try_from(s_248_6).unwrap(),
        ));
        // D s_248_8: cast reint s_248_7 -> u8
        let s_248_8: u8 = (s_248_7.value() as u8);
        // D s_248_9: cast zx s_248_8 -> bv
        let s_248_9: Bits = Bits::new(s_248_8 as u128, 2u16);
        // C s_248_10: const #2u : u8
        let s_248_10: u8 = 2;
        // C s_248_11: cast zx s_248_10 -> bv
        let s_248_11: Bits = Bits::new(s_248_10 as u128, 2u16);
        // D s_248_12: cmp-eq s_248_9 s_248_11
        let s_248_12: bool = ((s_248_9) == (s_248_11));
        // N s_248_13: branch s_248_12 b254 b249
        if s_248_12 {
            return block_254(state, tracer, fn_state);
        } else {
            return block_249(state, tracer, fn_state);
        };
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #2s : i
        let s_249_0: i128 = 2;
        // D s_249_1: read-var CRm:u8
        let s_249_1: u8 = fn_state.CRm;
        // D s_249_2: cast zx s_249_1 -> bv
        let s_249_2: Bits = Bits::new(s_249_1 as u128, 4u16);
        // C s_249_3: const #1s : i64
        let s_249_3: i64 = 1;
        // C s_249_4: cast zx s_249_3 -> i
        let s_249_4: i128 = (i128::try_from(s_249_3).unwrap());
        // C s_249_5: const #1s : i
        let s_249_5: i128 = 1;
        // C s_249_6: add s_249_5 s_249_4
        let s_249_6: i128 = (s_249_5 + s_249_4);
        // D s_249_7: bit-extract s_249_2 s_249_0 s_249_6
        let s_249_7: Bits = (Bits::new(
            ((s_249_2) >> (s_249_0)).value(),
            u16::try_from(s_249_6).unwrap(),
        ));
        // D s_249_8: cast reint s_249_7 -> u8
        let s_249_8: u8 = (s_249_7.value() as u8);
        // D s_249_9: cast zx s_249_8 -> bv
        let s_249_9: Bits = Bits::new(s_249_8 as u128, 2u16);
        // C s_249_10: const #3u : u8
        let s_249_10: u8 = 3;
        // C s_249_11: cast zx s_249_10 -> bv
        let s_249_11: Bits = Bits::new(s_249_10 as u128, 2u16);
        // D s_249_12: cmp-eq s_249_9 s_249_11
        let s_249_12: bool = ((s_249_9) == (s_249_11));
        // N s_249_13: branch s_249_12 b253 b250
        if s_249_12 {
            return block_253(state, tracer, fn_state);
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
        // D s_250_1: write-var gs#142044 <= s_250_0
        fn_state.gs_142044 = s_250_0;
        // N s_250_2: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_251_0: read-var gs#142044:u8
        let s_251_0: bool = fn_state.gs_142044;
        // D s_251_1: write-var gs#142045 <= s_251_0
        fn_state.gs_142045 = s_251_0;
        // N s_251_2: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_252_0: read-var gs#142045:u8
        let s_252_0: bool = fn_state.gs_142045;
        // D s_252_1: write-var gs#142046 <= s_252_0
        fn_state.gs_142046 = s_252_0;
        // N s_252_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #0s : i
        let s_253_0: i128 = 0;
        // D s_253_1: read-var CRm:u8
        let s_253_1: u8 = fn_state.CRm;
        // D s_253_2: cast zx s_253_1 -> bv
        let s_253_2: Bits = Bits::new(s_253_1 as u128, 4u16);
        // C s_253_3: const #1s : i64
        let s_253_3: i64 = 1;
        // C s_253_4: cast zx s_253_3 -> i
        let s_253_4: i128 = (i128::try_from(s_253_3).unwrap());
        // C s_253_5: const #1s : i
        let s_253_5: i128 = 1;
        // C s_253_6: add s_253_5 s_253_4
        let s_253_6: i128 = (s_253_5 + s_253_4);
        // D s_253_7: bit-extract s_253_2 s_253_0 s_253_6
        let s_253_7: Bits = (Bits::new(
            ((s_253_2) >> (s_253_0)).value(),
            u16::try_from(s_253_6).unwrap(),
        ));
        // D s_253_8: cast reint s_253_7 -> u8
        let s_253_8: u8 = (s_253_7.value() as u8);
        // C s_253_9: const #0s : i
        let s_253_9: i128 = 0;
        // D s_253_10: read-var opc2:u8
        let s_253_10: u8 = fn_state.opc2;
        // D s_253_11: cast zx s_253_10 -> bv
        let s_253_11: Bits = Bits::new(s_253_10 as u128, 3u16);
        // C s_253_12: const #1s : i64
        let s_253_12: i64 = 1;
        // C s_253_13: cast zx s_253_12 -> i
        let s_253_13: i128 = (i128::try_from(s_253_12).unwrap());
        // C s_253_14: const #2s : i
        let s_253_14: i128 = 2;
        // C s_253_15: add s_253_14 s_253_13
        let s_253_15: i128 = (s_253_14 + s_253_13);
        // D s_253_16: bit-extract s_253_11 s_253_9 s_253_15
        let s_253_16: Bits = (Bits::new(
            ((s_253_11) >> (s_253_9)).value(),
            u16::try_from(s_253_15).unwrap(),
        ));
        // D s_253_17: cast reint s_253_16 -> u8
        let s_253_17: u8 = (s_253_16.value() as u8);
        // D s_253_18: cast zx s_253_8 -> bv
        let s_253_18: Bits = Bits::new(s_253_8 as u128, 2u16);
        // D s_253_19: cast zx s_253_17 -> bv
        let s_253_19: Bits = Bits::new(s_253_17 as u128, 3u16);
        // D s_253_20: cast reint s_253_18 -> u128
        let s_253_20: u128 = (s_253_18.value() as u128);
        // D s_253_21: size-of s_253_18
        let s_253_21: u16 = s_253_18.length();
        // D s_253_22: cast reint s_253_19 -> u128
        let s_253_22: u128 = (s_253_19.value() as u128);
        // D s_253_23: size-of s_253_19
        let s_253_23: u16 = s_253_19.length();
        // D s_253_24: lsl s_253_20 s_253_23
        let s_253_24: u128 = s_253_20 << s_253_23;
        // D s_253_25: or s_253_24 s_253_22
        let s_253_25: u128 = ((s_253_24) | (s_253_22));
        // D s_253_26: add s_253_21 s_253_23
        let s_253_26: u16 = (s_253_21 + s_253_23);
        // D s_253_27: create-bits s_253_25 s_253_26
        let s_253_27: Bits = Bits::new(s_253_25, s_253_26);
        // D s_253_28: cast reint s_253_27 -> u8
        let s_253_28: u8 = (s_253_27.value() as u8);
        // D s_253_29: cast zx s_253_28 -> bv
        let s_253_29: Bits = Bits::new(s_253_28 as u128, 5u16);
        // C s_253_30: const #31u : u8
        let s_253_30: u8 = 31;
        // C s_253_31: cast zx s_253_30 -> bv
        let s_253_31: Bits = Bits::new(s_253_30 as u128, 5u16);
        // D s_253_32: cmp-ne s_253_29 s_253_31
        let s_253_32: bool = ((s_253_29) != (s_253_31));
        // D s_253_33: write-var gs#142044 <= s_253_32
        fn_state.gs_142044 = s_253_32;
        // N s_253_34: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_254_0: const #1u : u8
        let s_254_0: bool = true;
        // D s_254_1: write-var gs#142045 <= s_254_0
        fn_state.gs_142045 = s_254_0;
        // N s_254_2: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_255_0: read-var CRn:u8
        let s_255_0: u8 = fn_state.CRn;
        // D s_255_1: cast zx s_255_0 -> bv
        let s_255_1: Bits = Bits::new(s_255_0 as u128, 4u16);
        // C s_255_2: const #14u : u8
        let s_255_2: u8 = 14;
        // C s_255_3: cast zx s_255_2 -> bv
        let s_255_3: Bits = Bits::new(s_255_2 as u128, 4u16);
        // D s_255_4: cmp-eq s_255_1 s_255_3
        let s_255_4: bool = ((s_255_1) == (s_255_3));
        // D s_255_5: write-var gs#142035 <= s_255_4
        fn_state.gs_142035 = s_255_4;
        // N s_255_6: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var opc1:u8
        let s_256_0: u8 = fn_state.opc1;
        // D s_256_1: cast zx s_256_0 -> bv
        let s_256_1: Bits = Bits::new(s_256_0 as u128, 3u16);
        // C s_256_2: const #0u : u8
        let s_256_2: u8 = 0;
        // C s_256_3: cast zx s_256_2 -> bv
        let s_256_3: Bits = Bits::new(s_256_2 as u128, 3u16);
        // D s_256_4: cmp-eq s_256_1 s_256_3
        let s_256_4: bool = ((s_256_1) == (s_256_3));
        // D s_256_5: write-var gs#142034 <= s_256_4
        fn_state.gs_142034 = s_256_4;
        // N s_256_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_257_0: read-var t:i
        let s_257_0: i128 = fn_state.t;
        // D s_257_1: call R_read(s_257_0)
        let s_257_1: u32 = R_read(state, tracer, s_257_0);
        // C s_257_2: const #() : ()
        let s_257_2: () = ();
        // S s_257_3: call AArch32_GetNumEventCountersAccessible(s_257_2)
        let s_257_3: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_257_2,
        );
        // C s_257_4: const #4s : i
        let s_257_4: i128 = 4;
        // C s_257_5: const #0s : i
        let s_257_5: i128 = 0;
        // S s_257_6: call integer_subrange(s_257_3, s_257_4, s_257_5)
        let s_257_6: Bits = integer_subrange(state, tracer, s_257_3, s_257_4, s_257_5);
        // S s_257_7: cast reint s_257_6 -> u8
        let s_257_7: u8 = (s_257_6.value() as u8);
        // C s_257_8: const #11s : i
        let s_257_8: i128 = 11;
        // D s_257_9: cast zx s_257_1 -> bv
        let s_257_9: Bits = Bits::new(s_257_1 as u128, 32u16);
        // S s_257_10: cast zx s_257_7 -> bv
        let s_257_10: Bits = Bits::new(s_257_7 as u128, 5u16);
        // C s_257_11: const #4s : i
        let s_257_11: i128 = 4;
        // C s_257_12: const #1u : u64
        let s_257_12: u64 = 1;
        // C s_257_13: cast zx s_257_12 -> bv
        let s_257_13: Bits = Bits::new(s_257_12 as u128, 64u16);
        // C s_257_14: lsl s_257_13 s_257_11
        let s_257_14: Bits = s_257_13 << s_257_11;
        // C s_257_15: sub s_257_14 s_257_13
        let s_257_15: Bits = ((s_257_14) - (s_257_13));
        // S s_257_16: and s_257_10 s_257_15
        let s_257_16: Bits = ((s_257_10) & (s_257_15));
        // S s_257_17: lsl s_257_16 s_257_8
        let s_257_17: Bits = s_257_16 << s_257_8;
        // C s_257_18: lsl s_257_15 s_257_8
        let s_257_18: Bits = s_257_15 << s_257_8;
        // C s_257_19: cmpl s_257_18
        let s_257_19: Bits = !s_257_18;
        // D s_257_20: and s_257_9 s_257_19
        let s_257_20: Bits = ((s_257_9) & (s_257_19));
        // D s_257_21: or s_257_20 s_257_17
        let s_257_21: Bits = ((s_257_20) | (s_257_17));
        // D s_257_22: cast reint s_257_21 -> u32
        let s_257_22: u32 = (s_257_21.value() as u32);
        // D s_257_23: read-var t:i
        let s_257_23: i128 = fn_state.t;
        // D s_257_24: call R_set(s_257_23, s_257_22)
        let s_257_24: () = R_set(state, tracer, s_257_23, s_257_22);
        // N s_257_25: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var CRm:u8
        let s_258_0: u8 = fn_state.CRm;
        // D s_258_1: cast zx s_258_0 -> bv
        let s_258_1: Bits = Bits::new(s_258_0 as u128, 4u16);
        // C s_258_2: const #12u : u8
        let s_258_2: u8 = 12;
        // C s_258_3: cast zx s_258_2 -> bv
        let s_258_3: Bits = Bits::new(s_258_2 as u128, 4u16);
        // D s_258_4: cmp-eq s_258_1 s_258_3
        let s_258_4: bool = ((s_258_1) == (s_258_3));
        // D s_258_5: write-var gs#142032 <= s_258_4
        fn_state.gs_142032 = s_258_4;
        // N s_258_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_259_0: read-var CRn:u8
        let s_259_0: u8 = fn_state.CRn;
        // D s_259_1: cast zx s_259_0 -> bv
        let s_259_1: Bits = Bits::new(s_259_0 as u128, 4u16);
        // C s_259_2: const #9u : u8
        let s_259_2: u8 = 9;
        // C s_259_3: cast zx s_259_2 -> bv
        let s_259_3: Bits = Bits::new(s_259_2 as u128, 4u16);
        // D s_259_4: cmp-eq s_259_1 s_259_3
        let s_259_4: bool = ((s_259_1) == (s_259_3));
        // D s_259_5: write-var gs#142031 <= s_259_4
        fn_state.gs_142031 = s_259_4;
        // N s_259_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var opc2:u8
        let s_260_0: u8 = fn_state.opc2;
        // D s_260_1: cast zx s_260_0 -> bv
        let s_260_1: Bits = Bits::new(s_260_0 as u128, 3u16);
        // C s_260_2: const #0u : u8
        let s_260_2: u8 = 0;
        // C s_260_3: cast zx s_260_2 -> bv
        let s_260_3: Bits = Bits::new(s_260_2 as u128, 3u16);
        // D s_260_4: cmp-eq s_260_1 s_260_3
        let s_260_4: bool = ((s_260_1) == (s_260_3));
        // D s_260_5: write-var gs#142030 <= s_260_4
        fn_state.gs_142030 = s_260_4;
        // N s_260_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_261_0: read-var opc1:u8
        let s_261_0: u8 = fn_state.opc1;
        // D s_261_1: cast zx s_261_0 -> bv
        let s_261_1: Bits = Bits::new(s_261_0 as u128, 3u16);
        // C s_261_2: const #0u : u8
        let s_261_2: u8 = 0;
        // C s_261_3: cast zx s_261_2 -> bv
        let s_261_3: Bits = Bits::new(s_261_2 as u128, 3u16);
        // D s_261_4: cmp-eq s_261_1 s_261_3
        let s_261_4: bool = ((s_261_1) == (s_261_3));
        // D s_261_5: write-var gs#142029 <= s_261_4
        fn_state.gs_142029 = s_261_4;
        // N s_261_6: jump b61
        return block_61(state, tracer, fn_state);
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
        // N s_262_5: branch s_262_4 b286 b263
        if s_262_4 {
            return block_286(state, tracer, fn_state);
        } else {
            return block_263(state, tracer, fn_state);
        };
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_263_0: jump b264
        return block_264(state, tracer, fn_state);
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
        // C s_264_2: const #2u : u8
        let s_264_2: u8 = 2;
        // C s_264_3: cast zx s_264_2 -> bv
        let s_264_3: Bits = Bits::new(s_264_2 as u128, 3u16);
        // D s_264_4: cmp-eq s_264_1 s_264_3
        let s_264_4: bool = ((s_264_1) == (s_264_3));
        // N s_264_5: branch s_264_4 b267 b265
        if s_264_4 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_265(state, tracer, fn_state);
        };
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_265_0: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_266_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #() : ()
        let s_267_0: () = ();
        // S s_267_1: call PMSELR_read(s_267_0)
        let s_267_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_267_0);
        // S s_267_2: call _get_PMSELR_Type_SEL(s_267_1)
        let s_267_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_267_1);
        // S s_267_3: cast zx s_267_2 -> bv
        let s_267_3: Bits = Bits::new(s_267_2 as u128, 5u16);
        // S s_267_4: cast zx s_267_3 -> i
        let s_267_4: i128 = (s_267_3.value() as i128);
        // S s_267_5: cast reint s_267_4 -> i64
        let s_267_5: i64 = (s_267_4 as i64);
        // C s_267_6: const #() : ()
        let s_267_6: () = ();
        // S s_267_7: call GetNumEventCounters(s_267_6)
        let s_267_7: i128 = GetNumEventCounters(state, tracer, s_267_6);
        // C s_267_8: const #1s : i
        let s_267_8: i128 = 1;
        // S s_267_9: sub s_267_7 s_267_8
        let s_267_9: i128 = ((s_267_7) - (s_267_8));
        // S s_267_10: cast zx s_267_5 -> i
        let s_267_10: i128 = (i128::try_from(s_267_5).unwrap());
        // S s_267_11: cmp-gt s_267_10 s_267_9
        let s_267_11: bool = ((s_267_10) > (s_267_9));
        // N s_267_12: branch s_267_11 b285 b268
        if s_267_11 {
            return block_285(state, tracer, fn_state);
        } else {
            return block_268(state, tracer, fn_state);
        };
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_268_0: const #() : ()
        let s_268_0: () = ();
        // S s_268_1: call EL2Enabled(s_268_0)
        let s_268_1: bool = EL2Enabled(state, tracer, s_268_0);
        // N s_268_2: branch s_268_1 b281 b269
        if s_268_1 {
            return block_281(state, tracer, fn_state);
        } else {
            return block_269(state, tracer, fn_state);
        };
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_269_0: const #0u : u8
        let s_269_0: bool = false;
        // D s_269_1: write-var gs#142181 <= s_269_0
        fn_state.gs_142181 = s_269_0;
        // N s_269_2: jump b270
        return block_270(state, tracer, fn_state);
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_270_0: read-var gs#142181:u8
        let s_270_0: bool = fn_state.gs_142181;
        // N s_270_1: branch s_270_0 b280 b271
        if s_270_0 {
            return block_280(state, tracer, fn_state);
        } else {
            return block_271(state, tracer, fn_state);
        };
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_271_0: const #0u : u8
        let s_271_0: bool = false;
        // D s_271_1: write-var gs#142183 <= s_271_0
        fn_state.gs_142183 = s_271_0;
        // N s_271_2: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_272_0: read-var gs#142183:u8
        let s_272_0: bool = fn_state.gs_142183;
        // D s_272_1: write-var gs#142184 <= s_272_0
        fn_state.gs_142184 = s_272_0;
        // N s_272_2: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_273_0: read-var gs#142184:u8
        let s_273_0: bool = fn_state.gs_142184;
        // N s_273_1: branch s_273_0 b275 b274
        if s_273_0 {
            return block_275(state, tracer, fn_state);
        } else {
            return block_274(state, tracer, fn_state);
        };
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_274_0: const #() : ()
        let s_274_0: () = ();
        // S s_274_1: call PMSELR_read(s_274_0)
        let s_274_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_274_0);
        // S s_274_2: call _get_PMSELR_Type_SEL(s_274_1)
        let s_274_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_274_1);
        // S s_274_3: cast zx s_274_2 -> bv
        let s_274_3: Bits = Bits::new(s_274_2 as u128, 5u16);
        // S s_274_4: cast zx s_274_3 -> i
        let s_274_4: i128 = (s_274_3.value() as i128);
        // S s_274_5: cast reint s_274_4 -> i64
        let s_274_5: i64 = (s_274_4 as i64);
        // C s_274_6: const #31s : i
        let s_274_6: i128 = 31;
        // S s_274_7: cast zx s_274_5 -> i
        let s_274_7: i128 = (i128::try_from(s_274_5).unwrap());
        // S s_274_8: cmp-lt s_274_7 s_274_6
        let s_274_8: bool = ((s_274_7) < (s_274_6));
        // N s_274_9: assert s_274_8
        let s_274_9: () = assert!(s_274_8);
        // S s_274_10: call PMEVCNTR_read(s_274_5)
        let s_274_10: u32 = PMEVCNTR_read(state, tracer, s_274_5);
        // C s_274_11: const #0s : i
        let s_274_11: i128 = 0;
        // S s_274_12: cast zx s_274_10 -> bv
        let s_274_12: Bits = Bits::new(s_274_10 as u128, 32u16);
        // C s_274_13: const #1s : i64
        let s_274_13: i64 = 1;
        // C s_274_14: cast zx s_274_13 -> i
        let s_274_14: i128 = (i128::try_from(s_274_13).unwrap());
        // C s_274_15: const #31s : i
        let s_274_15: i128 = 31;
        // C s_274_16: add s_274_15 s_274_14
        let s_274_16: i128 = (s_274_15 + s_274_14);
        // D s_274_17: bit-extract s_274_12 s_274_11 s_274_16
        let s_274_17: Bits = (Bits::new(
            ((s_274_12) >> (s_274_11)).value(),
            u16::try_from(s_274_16).unwrap(),
        ));
        // D s_274_18: cast reint s_274_17 -> u32
        let s_274_18: u32 = (s_274_17.value() as u32);
        // D s_274_19: read-var t:i
        let s_274_19: i128 = fn_state.t;
        // D s_274_20: call R_set(s_274_19, s_274_18)
        let s_274_20: () = R_set(state, tracer, s_274_19, s_274_18);
        // N s_274_21: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_275_0: const #() : ()
        let s_275_0: () = ();
        // S s_275_1: call PMSELR_read(s_275_0)
        let s_275_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_275_0);
        // S s_275_2: call _get_PMSELR_Type_SEL(s_275_1)
        let s_275_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_275_1);
        // S s_275_3: cast zx s_275_2 -> bv
        let s_275_3: Bits = Bits::new(s_275_2 as u128, 5u16);
        // S s_275_4: cast zx s_275_3 -> i
        let s_275_4: i128 = (s_275_3.value() as i128);
        // S s_275_5: cast reint s_275_4 -> i64
        let s_275_5: i64 = (s_275_4 as i64);
        // C s_275_6: const #() : ()
        let s_275_6: () = ();
        // S s_275_7: call GetNumEventCounters(s_275_6)
        let s_275_7: i128 = GetNumEventCounters(state, tracer, s_275_6);
        // C s_275_8: const #1s : i
        let s_275_8: i128 = 1;
        // S s_275_9: sub s_275_7 s_275_8
        let s_275_9: i128 = ((s_275_7) - (s_275_8));
        // S s_275_10: cast zx s_275_5 -> i
        let s_275_10: i128 = (i128::try_from(s_275_5).unwrap());
        // S s_275_11: cmp-gt s_275_10 s_275_9
        let s_275_11: bool = ((s_275_10) > (s_275_9));
        // N s_275_12: branch s_275_11 b279 b276
        if s_275_11 {
            return block_279(state, tracer, fn_state);
        } else {
            return block_276(state, tracer, fn_state);
        };
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_276_0: const #432u : u32
        let s_276_0: u32 = 432;
        // D s_276_1: read-reg s_276_0:u8
        let s_276_1: u8 = {
            let value = state.read_register::<u8>(s_276_0 as isize);
            tracer.read_register(s_276_0 as isize, value);
            value
        };
        // D s_276_2: call ELUsingAArch32(s_276_1)
        let s_276_2: bool = ELUsingAArch32(state, tracer, s_276_1);
        // N s_276_3: branch s_276_2 b278 b277
        if s_276_2 {
            return block_278(state, tracer, fn_state);
        } else {
            return block_277(state, tracer, fn_state);
        };
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_277_0: read-var t:i
        let s_277_0: i128 = fn_state.t;
        // D s_277_1: read-var temp:u32
        let s_277_1: u32 = fn_state.temp;
        // D s_277_2: call R_set(s_277_0, s_277_1)
        let s_277_2: () = R_set(state, tracer, s_277_0, s_277_1);
        // C s_277_3: const #3u : u8
        let s_277_3: u8 = 3;
        // C s_277_4: cast zx s_277_3 -> bv
        let s_277_4: Bits = Bits::new(s_277_3 as u128, 8u16);
        // C s_277_5: cast zx s_277_4 -> i
        let s_277_5: i128 = (s_277_4.value() as i128);
        // C s_277_6: cast reint s_277_5 -> i64
        let s_277_6: i64 = (s_277_5 as i64);
        // C s_277_7: cast zx s_277_6 -> i
        let s_277_7: i128 = (i128::try_from(s_277_6).unwrap());
        // C s_277_8: const #432u : u32
        let s_277_8: u32 = 432;
        // D s_277_9: read-reg s_277_8:u8
        let s_277_9: u8 = {
            let value = state.read_register::<u8>(s_277_8 as isize);
            tracer.read_register(s_277_8 as isize, value);
            value
        };
        // D s_277_10: call AArch64_AArch32SystemAccessTrap(s_277_9, s_277_7)
        let s_277_10: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_277_9,
            s_277_7,
        );
        // N s_277_11: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_278_0: read-var t:i
        let s_278_0: i128 = fn_state.t;
        // D s_278_1: read-var temp:u32
        let s_278_1: u32 = fn_state.temp;
        // D s_278_2: call R_set(s_278_0, s_278_1)
        let s_278_2: () = R_set(state, tracer, s_278_0, s_278_1);
        // C s_278_3: const #3u : u8
        let s_278_3: u8 = 3;
        // C s_278_4: cast zx s_278_3 -> bv
        let s_278_4: Bits = Bits::new(s_278_3 as u128, 8u16);
        // C s_278_5: cast zx s_278_4 -> i
        let s_278_5: i128 = (s_278_4.value() as i128);
        // C s_278_6: cast reint s_278_5 -> i64
        let s_278_6: i64 = (s_278_5 as i64);
        // C s_278_7: cast zx s_278_6 -> i
        let s_278_7: i128 = (i128::try_from(s_278_6).unwrap());
        // S s_278_8: call AArch32_TakeHypTrapException(s_278_7)
        let s_278_8: () = AArch32_TakeHypTrapException(state, tracer, s_278_7);
        // N s_278_9: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_279_0: read-var t:i
        let s_279_0: i128 = fn_state.t;
        // D s_279_1: read-var temp:u32
        let s_279_1: u32 = fn_state.temp;
        // D s_279_2: call R_set(s_279_0, s_279_1)
        let s_279_2: () = R_set(state, tracer, s_279_0, s_279_1);
        // N s_279_3: panic
        panic!("{:?}", ());
        // N s_279_4: return
        return;
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_280_0: const #() : ()
        let s_280_0: () = ();
        // S s_280_1: call PMSELR_read(s_280_0)
        let s_280_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_280_0);
        // S s_280_2: call _get_PMSELR_Type_SEL(s_280_1)
        let s_280_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_280_1);
        // S s_280_3: cast zx s_280_2 -> bv
        let s_280_3: Bits = Bits::new(s_280_2 as u128, 5u16);
        // S s_280_4: cast zx s_280_3 -> i
        let s_280_4: i128 = (s_280_3.value() as i128);
        // S s_280_5: cast reint s_280_4 -> i64
        let s_280_5: i64 = (s_280_4 as i64);
        // C s_280_6: const #() : ()
        let s_280_6: () = ();
        // S s_280_7: call AArch32_GetNumEventCountersAccessible(s_280_6)
        let s_280_7: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_280_6,
        );
        // C s_280_8: const #1s : i
        let s_280_8: i128 = 1;
        // S s_280_9: sub s_280_7 s_280_8
        let s_280_9: i128 = ((s_280_7) - (s_280_8));
        // S s_280_10: cast zx s_280_5 -> i
        let s_280_10: i128 = (i128::try_from(s_280_5).unwrap());
        // S s_280_11: cmp-gt s_280_10 s_280_9
        let s_280_11: bool = ((s_280_10) > (s_280_9));
        // D s_280_12: write-var gs#142183 <= s_280_11
        fn_state.gs_142183 = s_280_11;
        // N s_280_13: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_281_0: const #16975u : u32
        let s_281_0: u32 = 16975;
        // D s_281_1: read-reg s_281_0:u8
        let s_281_1: u8 = {
            let value = state.read_register::<u8>(s_281_0 as isize);
            tracer.read_register(s_281_0 as isize, value);
            value
        };
        // D s_281_2: cast zx s_281_1 -> bv
        let s_281_2: Bits = Bits::new(s_281_1 as u128, 2u16);
        // C s_281_3: const #448u : u32
        let s_281_3: u32 = 448;
        // D s_281_4: read-reg s_281_3:u8
        let s_281_4: u8 = {
            let value = state.read_register::<u8>(s_281_3 as isize);
            tracer.read_register(s_281_3 as isize, value);
            value
        };
        // D s_281_5: cast zx s_281_4 -> bv
        let s_281_5: Bits = Bits::new(s_281_4 as u128, 2u16);
        // D s_281_6: cmp-eq s_281_2 s_281_5
        let s_281_6: bool = ((s_281_2) == (s_281_5));
        // N s_281_7: branch s_281_6 b284 b282
        if s_281_6 {
            return block_284(state, tracer, fn_state);
        } else {
            return block_282(state, tracer, fn_state);
        };
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_282_0: const #16975u : u32
        let s_282_0: u32 = 16975;
        // D s_282_1: read-reg s_282_0:u8
        let s_282_1: u8 = {
            let value = state.read_register::<u8>(s_282_0 as isize);
            tracer.read_register(s_282_0 as isize, value);
            value
        };
        // D s_282_2: cast zx s_282_1 -> bv
        let s_282_2: Bits = Bits::new(s_282_1 as u128, 2u16);
        // C s_282_3: const #440u : u32
        let s_282_3: u32 = 440;
        // D s_282_4: read-reg s_282_3:u8
        let s_282_4: u8 = {
            let value = state.read_register::<u8>(s_282_3 as isize);
            tracer.read_register(s_282_3 as isize, value);
            value
        };
        // D s_282_5: cast zx s_282_4 -> bv
        let s_282_5: Bits = Bits::new(s_282_4 as u128, 2u16);
        // D s_282_6: cmp-eq s_282_2 s_282_5
        let s_282_6: bool = ((s_282_2) == (s_282_5));
        // D s_282_7: write-var gs#142180 <= s_282_6
        fn_state.gs_142180 = s_282_6;
        // N s_282_8: jump b283
        return block_283(state, tracer, fn_state);
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_283_0: read-var gs#142180:u8
        let s_283_0: bool = fn_state.gs_142180;
        // D s_283_1: write-var gs#142181 <= s_283_0
        fn_state.gs_142181 = s_283_0;
        // N s_283_2: jump b270
        return block_270(state, tracer, fn_state);
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_284_0: const #1u : u8
        let s_284_0: bool = true;
        // D s_284_1: write-var gs#142180 <= s_284_0
        fn_state.gs_142180 = s_284_0;
        // N s_284_2: jump b283
        return block_283(state, tracer, fn_state);
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_285_0: const #1u : u8
        let s_285_0: bool = true;
        // D s_285_1: write-var gs#142184 <= s_285_0
        fn_state.gs_142184 = s_285_0;
        // N s_285_2: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_286_0: const #() : ()
        let s_286_0: () = ();
        // S s_286_1: call PMSELR_read(s_286_0)
        let s_286_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_286_0);
        // S s_286_2: call _get_PMSELR_Type_SEL(s_286_1)
        let s_286_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_286_1);
        // S s_286_3: cast zx s_286_2 -> bv
        let s_286_3: Bits = Bits::new(s_286_2 as u128, 5u16);
        // S s_286_4: cast zx s_286_3 -> i
        let s_286_4: i128 = (s_286_3.value() as i128);
        // S s_286_5: cast reint s_286_4 -> i64
        let s_286_5: i64 = (s_286_4 as i64);
        // D s_286_6: write-var pmselr <= s_286_5
        fn_state.pmselr = s_286_5;
        // C s_286_7: const #() : ()
        let s_286_7: () = ();
        // S s_286_8: call EL2Enabled(s_286_7)
        let s_286_8: bool = EL2Enabled(state, tracer, s_286_7);
        // N s_286_9: branch s_286_8 b304 b287
        if s_286_8 {
            return block_304(state, tracer, fn_state);
        } else {
            return block_287(state, tracer, fn_state);
        };
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_287_0: const #0u : u8
        let s_287_0: bool = false;
        // D s_287_1: write-var gs#142198 <= s_287_0
        fn_state.gs_142198 = s_287_0;
        // N s_287_2: jump b288
        return block_288(state, tracer, fn_state);
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_288_0: read-var gs#142198:u8
        let s_288_0: bool = fn_state.gs_142198;
        // N s_288_1: branch s_288_0 b303 b289
        if s_288_0 {
            return block_303(state, tracer, fn_state);
        } else {
            return block_289(state, tracer, fn_state);
        };
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_289_0: const #0u : u8
        let s_289_0: bool = false;
        // D s_289_1: write-var gs#142199 <= s_289_0
        fn_state.gs_142199 = s_289_0;
        // N s_289_2: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_290_0: read-var gs#142199:u8
        let s_290_0: bool = fn_state.gs_142199;
        // N s_290_1: branch s_290_0 b302 b291
        if s_290_0 {
            return block_302(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_291_0: const #0u : u8
        let s_291_0: bool = false;
        // D s_291_1: write-var gs#142201 <= s_291_0
        fn_state.gs_142201 = s_291_0;
        // N s_291_2: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_292_0: read-var gs#142201:u8
        let s_292_0: bool = fn_state.gs_142201;
        // N s_292_1: branch s_292_0 b297 b293
        if s_292_0 {
            return block_297(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_293_0: const #31s : i
        let s_293_0: i128 = 31;
        // D s_293_1: read-var pmselr:i64
        let s_293_1: i64 = fn_state.pmselr;
        // D s_293_2: cast zx s_293_1 -> i
        let s_293_2: i128 = (i128::try_from(s_293_1).unwrap());
        // D s_293_3: cmp-eq s_293_2 s_293_0
        let s_293_3: bool = ((s_293_2) == (s_293_0));
        // N s_293_4: branch s_293_3 b296 b294
        if s_293_3 {
            return block_296(state, tracer, fn_state);
        } else {
            return block_294(state, tracer, fn_state);
        };
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_294_0: read-var pmselr:i64
        let s_294_0: i64 = fn_state.pmselr;
        // D s_294_1: call PMEVTYPER_read(s_294_0)
        let s_294_1: ProductType700c18a878c5601b = PMEVTYPER_read(
            state,
            tracer,
            s_294_0,
        );
        // D s_294_2: write-var ga#248226 <= s_294_1
        fn_state.ga_248226 = s_294_1;
        // D s_294_3: read-var ga#248226.0:struct
        let s_294_3: u32 = fn_state.ga_248226._0;
        // D s_294_4: read-var t:i
        let s_294_4: i128 = fn_state.t;
        // D s_294_5: call R_set(s_294_4, s_294_3)
        let s_294_5: () = R_set(state, tracer, s_294_4, s_294_3);
        // N s_294_6: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_295_0: jump b264
        return block_264(state, tracer, fn_state);
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_296_0: const #() : ()
        let s_296_0: () = ();
        // S s_296_1: call PMCCFILTR_read(s_296_0)
        let s_296_1: ProductType700c18a878c5601b = PMCCFILTR_read(
            state,
            tracer,
            s_296_0,
        );
        // D s_296_2: write-var ga#248224 <= s_296_1
        fn_state.ga_248224 = s_296_1;
        // D s_296_3: read-var ga#248224.0:struct
        let s_296_3: u32 = fn_state.ga_248224._0;
        // D s_296_4: read-var t:i
        let s_296_4: i128 = fn_state.t;
        // D s_296_5: call R_set(s_296_4, s_296_3)
        let s_296_5: () = R_set(state, tracer, s_296_4, s_296_3);
        // N s_296_6: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_297_0: const #() : ()
        let s_297_0: () = ();
        // S s_297_1: call PMSELR_read(s_297_0)
        let s_297_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_297_0);
        // S s_297_2: call _get_PMSELR_Type_SEL(s_297_1)
        let s_297_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_297_1);
        // S s_297_3: cast zx s_297_2 -> bv
        let s_297_3: Bits = Bits::new(s_297_2 as u128, 5u16);
        // S s_297_4: cast zx s_297_3 -> i
        let s_297_4: i128 = (s_297_3.value() as i128);
        // S s_297_5: cast reint s_297_4 -> i64
        let s_297_5: i64 = (s_297_4 as i64);
        // C s_297_6: const #() : ()
        let s_297_6: () = ();
        // S s_297_7: call GetNumEventCounters(s_297_6)
        let s_297_7: i128 = GetNumEventCounters(state, tracer, s_297_6);
        // C s_297_8: const #1s : i
        let s_297_8: i128 = 1;
        // S s_297_9: sub s_297_7 s_297_8
        let s_297_9: i128 = ((s_297_7) - (s_297_8));
        // S s_297_10: cast zx s_297_5 -> i
        let s_297_10: i128 = (i128::try_from(s_297_5).unwrap());
        // S s_297_11: cmp-gt s_297_10 s_297_9
        let s_297_11: bool = ((s_297_10) > (s_297_9));
        // N s_297_12: branch s_297_11 b301 b298
        if s_297_11 {
            return block_301(state, tracer, fn_state);
        } else {
            return block_298(state, tracer, fn_state);
        };
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_298_0: const #432u : u32
        let s_298_0: u32 = 432;
        // D s_298_1: read-reg s_298_0:u8
        let s_298_1: u8 = {
            let value = state.read_register::<u8>(s_298_0 as isize);
            tracer.read_register(s_298_0 as isize, value);
            value
        };
        // D s_298_2: call ELUsingAArch32(s_298_1)
        let s_298_2: bool = ELUsingAArch32(state, tracer, s_298_1);
        // N s_298_3: branch s_298_2 b300 b299
        if s_298_2 {
            return block_300(state, tracer, fn_state);
        } else {
            return block_299(state, tracer, fn_state);
        };
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_299_0: read-var t:i
        let s_299_0: i128 = fn_state.t;
        // D s_299_1: read-var temp:u32
        let s_299_1: u32 = fn_state.temp;
        // D s_299_2: call R_set(s_299_0, s_299_1)
        let s_299_2: () = R_set(state, tracer, s_299_0, s_299_1);
        // C s_299_3: const #3u : u8
        let s_299_3: u8 = 3;
        // C s_299_4: cast zx s_299_3 -> bv
        let s_299_4: Bits = Bits::new(s_299_3 as u128, 8u16);
        // C s_299_5: cast zx s_299_4 -> i
        let s_299_5: i128 = (s_299_4.value() as i128);
        // C s_299_6: cast reint s_299_5 -> i64
        let s_299_6: i64 = (s_299_5 as i64);
        // C s_299_7: cast zx s_299_6 -> i
        let s_299_7: i128 = (i128::try_from(s_299_6).unwrap());
        // C s_299_8: const #432u : u32
        let s_299_8: u32 = 432;
        // D s_299_9: read-reg s_299_8:u8
        let s_299_9: u8 = {
            let value = state.read_register::<u8>(s_299_8 as isize);
            tracer.read_register(s_299_8 as isize, value);
            value
        };
        // D s_299_10: call AArch64_AArch32SystemAccessTrap(s_299_9, s_299_7)
        let s_299_10: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_299_9,
            s_299_7,
        );
        // N s_299_11: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_300_0: read-var t:i
        let s_300_0: i128 = fn_state.t;
        // D s_300_1: read-var temp:u32
        let s_300_1: u32 = fn_state.temp;
        // D s_300_2: call R_set(s_300_0, s_300_1)
        let s_300_2: () = R_set(state, tracer, s_300_0, s_300_1);
        // C s_300_3: const #3u : u8
        let s_300_3: u8 = 3;
        // C s_300_4: cast zx s_300_3 -> bv
        let s_300_4: Bits = Bits::new(s_300_3 as u128, 8u16);
        // C s_300_5: cast zx s_300_4 -> i
        let s_300_5: i128 = (s_300_4.value() as i128);
        // C s_300_6: cast reint s_300_5 -> i64
        let s_300_6: i64 = (s_300_5 as i64);
        // C s_300_7: cast zx s_300_6 -> i
        let s_300_7: i128 = (i128::try_from(s_300_6).unwrap());
        // S s_300_8: call AArch32_TakeHypTrapException(s_300_7)
        let s_300_8: () = AArch32_TakeHypTrapException(state, tracer, s_300_7);
        // N s_300_9: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_301_0: read-var t:i
        let s_301_0: i128 = fn_state.t;
        // D s_301_1: read-var temp:u32
        let s_301_1: u32 = fn_state.temp;
        // D s_301_2: call R_set(s_301_0, s_301_1)
        let s_301_2: () = R_set(state, tracer, s_301_0, s_301_1);
        // N s_301_3: panic
        panic!("{:?}", ());
        // N s_301_4: return
        return;
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_302_0: const #() : ()
        let s_302_0: () = ();
        // S s_302_1: call PMSELR_read(s_302_0)
        let s_302_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_302_0);
        // S s_302_2: call _get_PMSELR_Type_SEL(s_302_1)
        let s_302_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_302_1);
        // S s_302_3: cast zx s_302_2 -> bv
        let s_302_3: Bits = Bits::new(s_302_2 as u128, 5u16);
        // S s_302_4: cast zx s_302_3 -> i
        let s_302_4: i128 = (s_302_3.value() as i128);
        // S s_302_5: cast reint s_302_4 -> i64
        let s_302_5: i64 = (s_302_4 as i64);
        // C s_302_6: const #() : ()
        let s_302_6: () = ();
        // S s_302_7: call AArch32_GetNumEventCountersAccessible(s_302_6)
        let s_302_7: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_302_6,
        );
        // C s_302_8: const #1s : i
        let s_302_8: i128 = 1;
        // S s_302_9: sub s_302_7 s_302_8
        let s_302_9: i128 = ((s_302_7) - (s_302_8));
        // S s_302_10: cast zx s_302_5 -> i
        let s_302_10: i128 = (i128::try_from(s_302_5).unwrap());
        // S s_302_11: cmp-gt s_302_10 s_302_9
        let s_302_11: bool = ((s_302_10) > (s_302_9));
        // D s_302_12: write-var gs#142201 <= s_302_11
        fn_state.gs_142201 = s_302_11;
        // N s_302_13: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_303_0: const #() : ()
        let s_303_0: () = ();
        // S s_303_1: call PMSELR_read(s_303_0)
        let s_303_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_303_0);
        // S s_303_2: call _get_PMSELR_Type_SEL(s_303_1)
        let s_303_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_303_1);
        // S s_303_3: cast zx s_303_2 -> bv
        let s_303_3: Bits = Bits::new(s_303_2 as u128, 5u16);
        // C s_303_4: const #31u : u8
        let s_303_4: u8 = 31;
        // C s_303_5: cast zx s_303_4 -> bv
        let s_303_5: Bits = Bits::new(s_303_4 as u128, 5u16);
        // S s_303_6: cmp-ne s_303_3 s_303_5
        let s_303_6: bool = ((s_303_3) != (s_303_5));
        // D s_303_7: write-var gs#142199 <= s_303_6
        fn_state.gs_142199 = s_303_6;
        // N s_303_8: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_304_0: const #16975u : u32
        let s_304_0: u32 = 16975;
        // D s_304_1: read-reg s_304_0:u8
        let s_304_1: u8 = {
            let value = state.read_register::<u8>(s_304_0 as isize);
            tracer.read_register(s_304_0 as isize, value);
            value
        };
        // D s_304_2: cast zx s_304_1 -> bv
        let s_304_2: Bits = Bits::new(s_304_1 as u128, 2u16);
        // C s_304_3: const #448u : u32
        let s_304_3: u32 = 448;
        // D s_304_4: read-reg s_304_3:u8
        let s_304_4: u8 = {
            let value = state.read_register::<u8>(s_304_3 as isize);
            tracer.read_register(s_304_3 as isize, value);
            value
        };
        // D s_304_5: cast zx s_304_4 -> bv
        let s_304_5: Bits = Bits::new(s_304_4 as u128, 2u16);
        // D s_304_6: cmp-eq s_304_2 s_304_5
        let s_304_6: bool = ((s_304_2) == (s_304_5));
        // N s_304_7: branch s_304_6 b307 b305
        if s_304_6 {
            return block_307(state, tracer, fn_state);
        } else {
            return block_305(state, tracer, fn_state);
        };
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_305_0: const #16975u : u32
        let s_305_0: u32 = 16975;
        // D s_305_1: read-reg s_305_0:u8
        let s_305_1: u8 = {
            let value = state.read_register::<u8>(s_305_0 as isize);
            tracer.read_register(s_305_0 as isize, value);
            value
        };
        // D s_305_2: cast zx s_305_1 -> bv
        let s_305_2: Bits = Bits::new(s_305_1 as u128, 2u16);
        // C s_305_3: const #440u : u32
        let s_305_3: u32 = 440;
        // D s_305_4: read-reg s_305_3:u8
        let s_305_4: u8 = {
            let value = state.read_register::<u8>(s_305_3 as isize);
            tracer.read_register(s_305_3 as isize, value);
            value
        };
        // D s_305_5: cast zx s_305_4 -> bv
        let s_305_5: Bits = Bits::new(s_305_4 as u128, 2u16);
        // D s_305_6: cmp-eq s_305_2 s_305_5
        let s_305_6: bool = ((s_305_2) == (s_305_5));
        // D s_305_7: write-var gs#142197 <= s_305_6
        fn_state.gs_142197 = s_305_6;
        // N s_305_8: jump b306
        return block_306(state, tracer, fn_state);
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_306_0: read-var gs#142197:u8
        let s_306_0: bool = fn_state.gs_142197;
        // D s_306_1: write-var gs#142198 <= s_306_0
        fn_state.gs_142198 = s_306_0;
        // N s_306_2: jump b288
        return block_288(state, tracer, fn_state);
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_307_0: const #1u : u8
        let s_307_0: bool = true;
        // D s_307_1: write-var gs#142197 <= s_307_0
        fn_state.gs_142197 = s_307_0;
        // N s_307_2: jump b306
        return block_306(state, tracer, fn_state);
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_308_0: read-var opc2:u8
        let s_308_0: u8 = fn_state.opc2;
        // D s_308_1: cast zx s_308_0 -> bv
        let s_308_1: Bits = Bits::new(s_308_0 as u128, 3u16);
        // C s_308_2: const #1u : u8
        let s_308_2: u8 = 1;
        // C s_308_3: cast zx s_308_2 -> bv
        let s_308_3: Bits = Bits::new(s_308_2 as u128, 3u16);
        // D s_308_4: cmp-eq s_308_1 s_308_3
        let s_308_4: bool = ((s_308_1) == (s_308_3));
        // N s_308_5: branch s_308_4 b311 b309
        if s_308_4 {
            return block_311(state, tracer, fn_state);
        } else {
            return block_309(state, tracer, fn_state);
        };
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_309_0: read-var opc2:u8
        let s_309_0: u8 = fn_state.opc2;
        // D s_309_1: cast zx s_309_0 -> bv
        let s_309_1: Bits = Bits::new(s_309_0 as u128, 3u16);
        // C s_309_2: const #2u : u8
        let s_309_2: u8 = 2;
        // C s_309_3: cast zx s_309_2 -> bv
        let s_309_3: Bits = Bits::new(s_309_2 as u128, 3u16);
        // D s_309_4: cmp-eq s_309_1 s_309_3
        let s_309_4: bool = ((s_309_1) == (s_309_3));
        // D s_309_5: write-var gs#142026 <= s_309_4
        fn_state.gs_142026 = s_309_4;
        // N s_309_6: jump b310
        return block_310(state, tracer, fn_state);
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_310_0: read-var gs#142026:u8
        let s_310_0: bool = fn_state.gs_142026;
        // D s_310_1: write-var gs#142027 <= s_310_0
        fn_state.gs_142027 = s_310_0;
        // N s_310_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_311_0: const #1u : u8
        let s_311_0: bool = true;
        // D s_311_1: write-var gs#142026 <= s_311_0
        fn_state.gs_142026 = s_311_0;
        // N s_311_2: jump b310
        return block_310(state, tracer, fn_state);
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_312_0: read-var CRm:u8
        let s_312_0: u8 = fn_state.CRm;
        // D s_312_1: cast zx s_312_0 -> bv
        let s_312_1: Bits = Bits::new(s_312_0 as u128, 4u16);
        // C s_312_2: const #13u : u8
        let s_312_2: u8 = 13;
        // C s_312_3: cast zx s_312_2 -> bv
        let s_312_3: Bits = Bits::new(s_312_2 as u128, 4u16);
        // D s_312_4: cmp-eq s_312_1 s_312_3
        let s_312_4: bool = ((s_312_1) == (s_312_3));
        // D s_312_5: write-var gs#142025 <= s_312_4
        fn_state.gs_142025 = s_312_4;
        // N s_312_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_313_0: read-var CRn:u8
        let s_313_0: u8 = fn_state.CRn;
        // D s_313_1: cast zx s_313_0 -> bv
        let s_313_1: Bits = Bits::new(s_313_0 as u128, 4u16);
        // C s_313_2: const #9u : u8
        let s_313_2: u8 = 9;
        // C s_313_3: cast zx s_313_2 -> bv
        let s_313_3: Bits = Bits::new(s_313_2 as u128, 4u16);
        // D s_313_4: cmp-eq s_313_1 s_313_3
        let s_313_4: bool = ((s_313_1) == (s_313_3));
        // D s_313_5: write-var gs#142024 <= s_313_4
        fn_state.gs_142024 = s_313_4;
        // N s_313_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_314_0: read-var opc1:u8
        let s_314_0: u8 = fn_state.opc1;
        // D s_314_1: cast zx s_314_0 -> bv
        let s_314_1: Bits = Bits::new(s_314_0 as u128, 3u16);
        // C s_314_2: const #0u : u8
        let s_314_2: u8 = 0;
        // C s_314_3: cast zx s_314_2 -> bv
        let s_314_3: Bits = Bits::new(s_314_2 as u128, 3u16);
        // D s_314_4: cmp-eq s_314_1 s_314_3
        let s_314_4: bool = ((s_314_1) == (s_314_3));
        // D s_314_5: write-var gs#142023 <= s_314_4
        fn_state.gs_142023 = s_314_4;
        // N s_314_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_315_0: const #() : ()
        let s_315_0: () = ();
        // S s_315_1: call PMUCounterMask(s_315_0)
        let s_315_1: u64 = PMUCounterMask(state, tracer, s_315_0);
        // C s_315_2: const #0s : i
        let s_315_2: i128 = 0;
        // S s_315_3: cast zx s_315_1 -> bv
        let s_315_3: Bits = Bits::new(s_315_1 as u128, 64u16);
        // C s_315_4: const #1s : i64
        let s_315_4: i64 = 1;
        // C s_315_5: cast zx s_315_4 -> i
        let s_315_5: i128 = (i128::try_from(s_315_4).unwrap());
        // C s_315_6: const #31s : i
        let s_315_6: i128 = 31;
        // C s_315_7: add s_315_6 s_315_5
        let s_315_7: i128 = (s_315_6 + s_315_5);
        // D s_315_8: bit-extract s_315_3 s_315_2 s_315_7
        let s_315_8: Bits = (Bits::new(
            ((s_315_3) >> (s_315_2)).value(),
            u16::try_from(s_315_7).unwrap(),
        ));
        // D s_315_9: cast reint s_315_8 -> u32
        let s_315_9: u32 = (s_315_8.value() as u32);
        // D s_315_10: read-var t:i
        let s_315_10: i128 = fn_state.t;
        // D s_315_11: call R_read(s_315_10)
        let s_315_11: u32 = R_read(state, tracer, s_315_10);
        // D s_315_12: cast zx s_315_11 -> bv
        let s_315_12: Bits = Bits::new(s_315_11 as u128, 32u16);
        // D s_315_13: cast zx s_315_9 -> bv
        let s_315_13: Bits = Bits::new(s_315_9 as u128, 32u16);
        // D s_315_14: and s_315_12 s_315_13
        let s_315_14: Bits = ((s_315_12) & (s_315_13));
        // D s_315_15: cast reint s_315_14 -> u32
        let s_315_15: u32 = (s_315_14.value() as u32);
        // D s_315_16: read-var t:i
        let s_315_16: i128 = fn_state.t;
        // D s_315_17: call R_set(s_315_16, s_315_15)
        let s_315_17: () = R_set(state, tracer, s_315_16, s_315_15);
        // N s_315_18: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_316_0: read-var opc2:u8
        let s_316_0: u8 = fn_state.opc2;
        // D s_316_1: cast zx s_316_0 -> bv
        let s_316_1: Bits = Bits::new(s_316_0 as u128, 3u16);
        // C s_316_2: const #1u : u8
        let s_316_2: u8 = 1;
        // C s_316_3: cast zx s_316_2 -> bv
        let s_316_3: Bits = Bits::new(s_316_2 as u128, 3u16);
        // D s_316_4: cmp-eq s_316_1 s_316_3
        let s_316_4: bool = ((s_316_1) == (s_316_3));
        // N s_316_5: branch s_316_4 b322 b317
        if s_316_4 {
            return block_322(state, tracer, fn_state);
        } else {
            return block_317(state, tracer, fn_state);
        };
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_317_0: read-var opc2:u8
        let s_317_0: u8 = fn_state.opc2;
        // D s_317_1: cast zx s_317_0 -> bv
        let s_317_1: Bits = Bits::new(s_317_0 as u128, 3u16);
        // C s_317_2: const #2u : u8
        let s_317_2: u8 = 2;
        // C s_317_3: cast zx s_317_2 -> bv
        let s_317_3: Bits = Bits::new(s_317_2 as u128, 3u16);
        // D s_317_4: cmp-eq s_317_1 s_317_3
        let s_317_4: bool = ((s_317_1) == (s_317_3));
        // N s_317_5: branch s_317_4 b321 b318
        if s_317_4 {
            return block_321(state, tracer, fn_state);
        } else {
            return block_318(state, tracer, fn_state);
        };
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_318_0: read-var opc2:u8
        let s_318_0: u8 = fn_state.opc2;
        // D s_318_1: cast zx s_318_0 -> bv
        let s_318_1: Bits = Bits::new(s_318_0 as u128, 3u16);
        // C s_318_2: const #3u : u8
        let s_318_2: u8 = 3;
        // C s_318_3: cast zx s_318_2 -> bv
        let s_318_3: Bits = Bits::new(s_318_2 as u128, 3u16);
        // D s_318_4: cmp-eq s_318_1 s_318_3
        let s_318_4: bool = ((s_318_1) == (s_318_3));
        // D s_318_5: write-var gs#142019 <= s_318_4
        fn_state.gs_142019 = s_318_4;
        // N s_318_6: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_319_0: read-var gs#142019:u8
        let s_319_0: bool = fn_state.gs_142019;
        // D s_319_1: write-var gs#142020 <= s_319_0
        fn_state.gs_142020 = s_319_0;
        // N s_319_2: jump b320
        return block_320(state, tracer, fn_state);
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_320_0: read-var gs#142020:u8
        let s_320_0: bool = fn_state.gs_142020;
        // D s_320_1: write-var gs#142021 <= s_320_0
        fn_state.gs_142021 = s_320_0;
        // N s_320_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_321_0: const #1u : u8
        let s_321_0: bool = true;
        // D s_321_1: write-var gs#142019 <= s_321_0
        fn_state.gs_142019 = s_321_0;
        // N s_321_2: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_322_0: const #1u : u8
        let s_322_0: bool = true;
        // D s_322_1: write-var gs#142020 <= s_322_0
        fn_state.gs_142020 = s_322_0;
        // N s_322_2: jump b320
        return block_320(state, tracer, fn_state);
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_323_0: read-var CRm:u8
        let s_323_0: u8 = fn_state.CRm;
        // D s_323_1: cast zx s_323_0 -> bv
        let s_323_1: Bits = Bits::new(s_323_0 as u128, 4u16);
        // C s_323_2: const #12u : u8
        let s_323_2: u8 = 12;
        // C s_323_3: cast zx s_323_2 -> bv
        let s_323_3: Bits = Bits::new(s_323_2 as u128, 4u16);
        // D s_323_4: cmp-eq s_323_1 s_323_3
        let s_323_4: bool = ((s_323_1) == (s_323_3));
        // N s_323_5: branch s_323_4 b326 b324
        if s_323_4 {
            return block_326(state, tracer, fn_state);
        } else {
            return block_324(state, tracer, fn_state);
        };
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_324_0: read-var CRm:u8
        let s_324_0: u8 = fn_state.CRm;
        // D s_324_1: cast zx s_324_0 -> bv
        let s_324_1: Bits = Bits::new(s_324_0 as u128, 4u16);
        // C s_324_2: const #14u : u8
        let s_324_2: u8 = 14;
        // C s_324_3: cast zx s_324_2 -> bv
        let s_324_3: Bits = Bits::new(s_324_2 as u128, 4u16);
        // D s_324_4: cmp-eq s_324_1 s_324_3
        let s_324_4: bool = ((s_324_1) == (s_324_3));
        // D s_324_5: write-var gs#142017 <= s_324_4
        fn_state.gs_142017 = s_324_4;
        // N s_324_6: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_325_0: read-var gs#142017:u8
        let s_325_0: bool = fn_state.gs_142017;
        // D s_325_1: write-var gs#142018 <= s_325_0
        fn_state.gs_142018 = s_325_0;
        // N s_325_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_326_0: const #1u : u8
        let s_326_0: bool = true;
        // D s_326_1: write-var gs#142017 <= s_326_0
        fn_state.gs_142017 = s_326_0;
        // N s_326_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_327_0: read-var CRn:u8
        let s_327_0: u8 = fn_state.CRn;
        // D s_327_1: cast zx s_327_0 -> bv
        let s_327_1: Bits = Bits::new(s_327_0 as u128, 4u16);
        // C s_327_2: const #9u : u8
        let s_327_2: u8 = 9;
        // C s_327_3: cast zx s_327_2 -> bv
        let s_327_3: Bits = Bits::new(s_327_2 as u128, 4u16);
        // D s_327_4: cmp-eq s_327_1 s_327_3
        let s_327_4: bool = ((s_327_1) == (s_327_3));
        // D s_327_5: write-var gs#142016 <= s_327_4
        fn_state.gs_142016 = s_327_4;
        // N s_327_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_328_0: read-var opc1:u8
        let s_328_0: u8 = fn_state.opc1;
        // D s_328_1: cast zx s_328_0 -> bv
        let s_328_1: Bits = Bits::new(s_328_0 as u128, 3u16);
        // C s_328_2: const #0u : u8
        let s_328_2: u8 = 0;
        // C s_328_3: cast zx s_328_2 -> bv
        let s_328_3: Bits = Bits::new(s_328_2 as u128, 3u16);
        // D s_328_4: cmp-eq s_328_1 s_328_3
        let s_328_4: bool = ((s_328_1) == (s_328_3));
        // D s_328_5: write-var gs#142015 <= s_328_4
        fn_state.gs_142015 = s_328_4;
        // N s_328_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_329_0: const #19136u : u32
        let s_329_0: u32 = 19136;
        // D s_329_1: read-reg s_329_0:struct
        let s_329_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_329_0 as isize);
            tracer.read_register(s_329_0 as isize, value);
            value
        };
        // D s_329_2: call _get_PMSELR_EL0_Type_SEL(s_329_1)
        let s_329_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_329_1);
        // D s_329_3: cast zx s_329_2 -> bv
        let s_329_3: Bits = Bits::new(s_329_2 as u128, 5u16);
        // D s_329_4: cast zx s_329_3 -> i
        let s_329_4: i128 = (s_329_3.value() as i128);
        // D s_329_5: cast reint s_329_4 -> i64
        let s_329_5: i64 = (s_329_4 as i64);
        // C s_329_6: const #() : ()
        let s_329_6: () = ();
        // S s_329_7: call GetNumEventCounters(s_329_6)
        let s_329_7: i128 = GetNumEventCounters(state, tracer, s_329_6);
        // C s_329_8: const #1s : i
        let s_329_8: i128 = 1;
        // S s_329_9: sub s_329_7 s_329_8
        let s_329_9: i128 = ((s_329_7) - (s_329_8));
        // D s_329_10: cast zx s_329_5 -> i
        let s_329_10: i128 = (i128::try_from(s_329_5).unwrap());
        // D s_329_11: cmp-gt s_329_10 s_329_9
        let s_329_11: bool = ((s_329_10) > (s_329_9));
        // N s_329_12: branch s_329_11 b360 b330
        if s_329_11 {
            return block_360(state, tracer, fn_state);
        } else {
            return block_330(state, tracer, fn_state);
        };
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_330_0: const #() : ()
        let s_330_0: () = ();
        // S s_330_1: call EL2Enabled(s_330_0)
        let s_330_1: bool = EL2Enabled(state, tracer, s_330_0);
        // N s_330_2: branch s_330_1 b344 b331
        if s_330_1 {
            return block_344(state, tracer, fn_state);
        } else {
            return block_331(state, tracer, fn_state);
        };
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_331_0: const #0u : u8
        let s_331_0: bool = false;
        // D s_331_1: write-var gs#141963 <= s_331_0
        fn_state.gs_141963 = s_331_0;
        // N s_331_2: jump b332
        return block_332(state, tracer, fn_state);
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_332_0: read-var gs#141963:u8
        let s_332_0: bool = fn_state.gs_141963;
        // N s_332_1: branch s_332_0 b343 b333
        if s_332_0 {
            return block_343(state, tracer, fn_state);
        } else {
            return block_333(state, tracer, fn_state);
        };
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_333_0: const #0u : u8
        let s_333_0: bool = false;
        // D s_333_1: write-var gs#141965 <= s_333_0
        fn_state.gs_141965 = s_333_0;
        // N s_333_2: jump b334
        return block_334(state, tracer, fn_state);
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_334_0: read-var gs#141965:u8
        let s_334_0: bool = fn_state.gs_141965;
        // D s_334_1: write-var gs#141966 <= s_334_0
        fn_state.gs_141966 = s_334_0;
        // N s_334_2: jump b335
        return block_335(state, tracer, fn_state);
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_335_0: read-var gs#141966:u8
        let s_335_0: bool = fn_state.gs_141966;
        // N s_335_1: branch s_335_0 b338 b336
        if s_335_0 {
            return block_338(state, tracer, fn_state);
        } else {
            return block_336(state, tracer, fn_state);
        };
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_336_0: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_337_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_338_0: const #() : ()
        let s_338_0: () = ();
        // S s_338_1: call PMSELR_read(s_338_0)
        let s_338_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_338_0);
        // S s_338_2: call _get_PMSELR_Type_SEL(s_338_1)
        let s_338_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_338_1);
        // S s_338_3: cast zx s_338_2 -> bv
        let s_338_3: Bits = Bits::new(s_338_2 as u128, 5u16);
        // S s_338_4: cast zx s_338_3 -> i
        let s_338_4: i128 = (s_338_3.value() as i128);
        // S s_338_5: cast reint s_338_4 -> i64
        let s_338_5: i64 = (s_338_4 as i64);
        // C s_338_6: const #() : ()
        let s_338_6: () = ();
        // S s_338_7: call GetNumEventCounters(s_338_6)
        let s_338_7: i128 = GetNumEventCounters(state, tracer, s_338_6);
        // C s_338_8: const #1s : i
        let s_338_8: i128 = 1;
        // S s_338_9: sub s_338_7 s_338_8
        let s_338_9: i128 = ((s_338_7) - (s_338_8));
        // S s_338_10: cast zx s_338_5 -> i
        let s_338_10: i128 = (i128::try_from(s_338_5).unwrap());
        // S s_338_11: cmp-gt s_338_10 s_338_9
        let s_338_11: bool = ((s_338_10) > (s_338_9));
        // N s_338_12: branch s_338_11 b342 b339
        if s_338_11 {
            return block_342(state, tracer, fn_state);
        } else {
            return block_339(state, tracer, fn_state);
        };
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #432u : u32
        let s_339_0: u32 = 432;
        // D s_339_1: read-reg s_339_0:u8
        let s_339_1: u8 = {
            let value = state.read_register::<u8>(s_339_0 as isize);
            tracer.read_register(s_339_0 as isize, value);
            value
        };
        // D s_339_2: call ELUsingAArch32(s_339_1)
        let s_339_2: bool = ELUsingAArch32(state, tracer, s_339_1);
        // N s_339_3: branch s_339_2 b341 b340
        if s_339_2 {
            return block_341(state, tracer, fn_state);
        } else {
            return block_340(state, tracer, fn_state);
        };
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_340_0: const #3u : u8
        let s_340_0: u8 = 3;
        // C s_340_1: cast zx s_340_0 -> bv
        let s_340_1: Bits = Bits::new(s_340_0 as u128, 8u16);
        // C s_340_2: cast zx s_340_1 -> i
        let s_340_2: i128 = (s_340_1.value() as i128);
        // C s_340_3: cast reint s_340_2 -> i64
        let s_340_3: i64 = (s_340_2 as i64);
        // C s_340_4: cast zx s_340_3 -> i
        let s_340_4: i128 = (i128::try_from(s_340_3).unwrap());
        // C s_340_5: const #432u : u32
        let s_340_5: u32 = 432;
        // D s_340_6: read-reg s_340_5:u8
        let s_340_6: u8 = {
            let value = state.read_register::<u8>(s_340_5 as isize);
            tracer.read_register(s_340_5 as isize, value);
            value
        };
        // D s_340_7: call AArch64_AArch32SystemAccessTrap(s_340_6, s_340_4)
        let s_340_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_340_6,
            s_340_4,
        );
        // N s_340_8: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_341_0: const #3u : u8
        let s_341_0: u8 = 3;
        // C s_341_1: cast zx s_341_0 -> bv
        let s_341_1: Bits = Bits::new(s_341_0 as u128, 8u16);
        // C s_341_2: cast zx s_341_1 -> i
        let s_341_2: i128 = (s_341_1.value() as i128);
        // C s_341_3: cast reint s_341_2 -> i64
        let s_341_3: i64 = (s_341_2 as i64);
        // C s_341_4: cast zx s_341_3 -> i
        let s_341_4: i128 = (i128::try_from(s_341_3).unwrap());
        // S s_341_5: call AArch32_TakeHypTrapException(s_341_4)
        let s_341_5: () = AArch32_TakeHypTrapException(state, tracer, s_341_4);
        // N s_341_6: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_342_0: panic
        panic!("{:?}", ());
        // N s_342_1: return
        return;
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_343_0: const #() : ()
        let s_343_0: () = ();
        // S s_343_1: call PMSELR_read(s_343_0)
        let s_343_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_343_0);
        // S s_343_2: call _get_PMSELR_Type_SEL(s_343_1)
        let s_343_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_343_1);
        // S s_343_3: cast zx s_343_2 -> bv
        let s_343_3: Bits = Bits::new(s_343_2 as u128, 5u16);
        // S s_343_4: cast zx s_343_3 -> i
        let s_343_4: i128 = (s_343_3.value() as i128);
        // S s_343_5: cast reint s_343_4 -> i64
        let s_343_5: i64 = (s_343_4 as i64);
        // C s_343_6: const #() : ()
        let s_343_6: () = ();
        // S s_343_7: call AArch32_GetNumEventCountersAccessible(s_343_6)
        let s_343_7: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_343_6,
        );
        // C s_343_8: const #1s : i
        let s_343_8: i128 = 1;
        // S s_343_9: sub s_343_7 s_343_8
        let s_343_9: i128 = ((s_343_7) - (s_343_8));
        // S s_343_10: cast zx s_343_5 -> i
        let s_343_10: i128 = (i128::try_from(s_343_5).unwrap());
        // S s_343_11: cmp-gt s_343_10 s_343_9
        let s_343_11: bool = ((s_343_10) > (s_343_9));
        // D s_343_12: write-var gs#141965 <= s_343_11
        fn_state.gs_141965 = s_343_11;
        // N s_343_13: jump b334
        return block_334(state, tracer, fn_state);
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_344_0: const #16975u : u32
        let s_344_0: u32 = 16975;
        // D s_344_1: read-reg s_344_0:u8
        let s_344_1: u8 = {
            let value = state.read_register::<u8>(s_344_0 as isize);
            tracer.read_register(s_344_0 as isize, value);
            value
        };
        // D s_344_2: cast zx s_344_1 -> bv
        let s_344_2: Bits = Bits::new(s_344_1 as u128, 2u16);
        // C s_344_3: const #440u : u32
        let s_344_3: u32 = 440;
        // D s_344_4: read-reg s_344_3:u8
        let s_344_4: u8 = {
            let value = state.read_register::<u8>(s_344_3 as isize);
            tracer.read_register(s_344_3 as isize, value);
            value
        };
        // D s_344_5: cast zx s_344_4 -> bv
        let s_344_5: Bits = Bits::new(s_344_4 as u128, 2u16);
        // D s_344_6: cmp-eq s_344_2 s_344_5
        let s_344_6: bool = ((s_344_2) == (s_344_5));
        // N s_344_7: branch s_344_6 b359 b345
        if s_344_6 {
            return block_359(state, tracer, fn_state);
        } else {
            return block_345(state, tracer, fn_state);
        };
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_345_0: const #16975u : u32
        let s_345_0: u32 = 16975;
        // D s_345_1: read-reg s_345_0:u8
        let s_345_1: u8 = {
            let value = state.read_register::<u8>(s_345_0 as isize);
            tracer.read_register(s_345_0 as isize, value);
            value
        };
        // D s_345_2: cast zx s_345_1 -> bv
        let s_345_2: Bits = Bits::new(s_345_1 as u128, 2u16);
        // C s_345_3: const #448u : u32
        let s_345_3: u32 = 448;
        // D s_345_4: read-reg s_345_3:u8
        let s_345_4: u8 = {
            let value = state.read_register::<u8>(s_345_3 as isize);
            tracer.read_register(s_345_3 as isize, value);
            value
        };
        // D s_345_5: cast zx s_345_4 -> bv
        let s_345_5: Bits = Bits::new(s_345_4 as u128, 2u16);
        // D s_345_6: cmp-eq s_345_2 s_345_5
        let s_345_6: bool = ((s_345_2) == (s_345_5));
        // N s_345_7: branch s_345_6 b349 b346
        if s_345_6 {
            return block_349(state, tracer, fn_state);
        } else {
            return block_346(state, tracer, fn_state);
        };
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_346_0: const #0u : u8
        let s_346_0: bool = false;
        // D s_346_1: write-var gs#141961 <= s_346_0
        fn_state.gs_141961 = s_346_0;
        // N s_346_2: jump b347
        return block_347(state, tracer, fn_state);
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_347_0: read-var gs#141961:u8
        let s_347_0: bool = fn_state.gs_141961;
        // D s_347_1: write-var gs#141962 <= s_347_0
        fn_state.gs_141962 = s_347_0;
        // N s_347_2: jump b348
        return block_348(state, tracer, fn_state);
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_348_0: read-var gs#141962:u8
        let s_348_0: bool = fn_state.gs_141962;
        // D s_348_1: write-var gs#141963 <= s_348_0
        fn_state.gs_141963 = s_348_0;
        // N s_348_2: jump b332
        return block_332(state, tracer, fn_state);
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_349_0: read-var opc2:u8
        let s_349_0: u8 = fn_state.opc2;
        // D s_349_1: cast zx s_349_0 -> bv
        let s_349_1: Bits = Bits::new(s_349_0 as u128, 3u16);
        // C s_349_2: const #2u : u8
        let s_349_2: u8 = 2;
        // C s_349_3: cast zx s_349_2 -> bv
        let s_349_3: Bits = Bits::new(s_349_2 as u128, 3u16);
        // D s_349_4: cmp-eq s_349_1 s_349_3
        let s_349_4: bool = ((s_349_1) == (s_349_3));
        // N s_349_5: branch s_349_4 b358 b350
        if s_349_4 {
            return block_358(state, tracer, fn_state);
        } else {
            return block_350(state, tracer, fn_state);
        };
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_350_0: const #0u : u8
        let s_350_0: bool = false;
        // D s_350_1: write-var gs#141958 <= s_350_0
        fn_state.gs_141958 = s_350_0;
        // N s_350_2: jump b351
        return block_351(state, tracer, fn_state);
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_351_0: read-var gs#141958:u8
        let s_351_0: bool = fn_state.gs_141958;
        // N s_351_1: branch s_351_0 b357 b352
        if s_351_0 {
            return block_357(state, tracer, fn_state);
        } else {
            return block_352(state, tracer, fn_state);
        };
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_352_0: read-var opc2:u8
        let s_352_0: u8 = fn_state.opc2;
        // D s_352_1: cast zx s_352_0 -> bv
        let s_352_1: Bits = Bits::new(s_352_0 as u128, 3u16);
        // C s_352_2: const #1u : u8
        let s_352_2: u8 = 1;
        // C s_352_3: cast zx s_352_2 -> bv
        let s_352_3: Bits = Bits::new(s_352_2 as u128, 3u16);
        // D s_352_4: cmp-eq s_352_1 s_352_3
        let s_352_4: bool = ((s_352_1) == (s_352_3));
        // N s_352_5: branch s_352_4 b356 b353
        if s_352_4 {
            return block_356(state, tracer, fn_state);
        } else {
            return block_353(state, tracer, fn_state);
        };
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_353_0: const #0u : u8
        let s_353_0: bool = false;
        // D s_353_1: write-var gs#141959 <= s_353_0
        fn_state.gs_141959 = s_353_0;
        // N s_353_2: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_354_0: read-var gs#141959:u8
        let s_354_0: bool = fn_state.gs_141959;
        // D s_354_1: write-var gs#141960 <= s_354_0
        fn_state.gs_141960 = s_354_0;
        // N s_354_2: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_355_0: read-var gs#141960:u8
        let s_355_0: bool = fn_state.gs_141960;
        // D s_355_1: write-var gs#141961 <= s_355_0
        fn_state.gs_141961 = s_355_0;
        // N s_355_2: jump b347
        return block_347(state, tracer, fn_state);
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_356_0: const #102624u : u32
        let s_356_0: u32 = 102624;
        // D s_356_1: read-reg s_356_0:struct
        let s_356_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_356_0 as isize);
            tracer.read_register(s_356_0 as isize, value);
            value
        };
        // D s_356_2: call _get_PMUSERENR_EL0_Type_EN(s_356_1)
        let s_356_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_356_1);
        // D s_356_3: cast zx s_356_2 -> bv
        let s_356_3: Bits = Bits::new(s_356_2 as u128, 1u16);
        // C s_356_4: const #1u : u8
        let s_356_4: bool = true;
        // C s_356_5: cast zx s_356_4 -> bv
        let s_356_5: Bits = Bits::new(s_356_4 as u128, 1u16);
        // D s_356_6: cmp-eq s_356_3 s_356_5
        let s_356_6: bool = ((s_356_3) == (s_356_5));
        // D s_356_7: write-var gs#141959 <= s_356_6
        fn_state.gs_141959 = s_356_6;
        // N s_356_8: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_357_0: const #1u : u8
        let s_357_0: bool = true;
        // D s_357_1: write-var gs#141960 <= s_357_0
        fn_state.gs_141960 = s_357_0;
        // N s_357_2: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_358_0: const #102624u : u32
        let s_358_0: u32 = 102624;
        // D s_358_1: read-reg s_358_0:struct
        let s_358_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_358_0 as isize);
            tracer.read_register(s_358_0 as isize, value);
            value
        };
        // D s_358_2: call _get_PMUSERENR_EL0_Type_ER(s_358_1)
        let s_358_2: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_358_1);
        // C s_358_3: const #102624u : u32
        let s_358_3: u32 = 102624;
        // D s_358_4: read-reg s_358_3:struct
        let s_358_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_358_3 as isize);
            tracer.read_register(s_358_3 as isize, value);
            value
        };
        // D s_358_5: call _get_PMUSERENR_EL0_Type_EN(s_358_4)
        let s_358_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_358_4);
        // D s_358_6: cast zx s_358_2 -> bv
        let s_358_6: Bits = Bits::new(s_358_2 as u128, 1u16);
        // D s_358_7: cast zx s_358_5 -> bv
        let s_358_7: Bits = Bits::new(s_358_5 as u128, 1u16);
        // D s_358_8: cast reint s_358_6 -> u128
        let s_358_8: u128 = (s_358_6.value() as u128);
        // D s_358_9: size-of s_358_6
        let s_358_9: u16 = s_358_6.length();
        // D s_358_10: cast reint s_358_7 -> u128
        let s_358_10: u128 = (s_358_7.value() as u128);
        // D s_358_11: size-of s_358_7
        let s_358_11: u16 = s_358_7.length();
        // D s_358_12: lsl s_358_8 s_358_11
        let s_358_12: u128 = s_358_8 << s_358_11;
        // D s_358_13: or s_358_12 s_358_10
        let s_358_13: u128 = ((s_358_12) | (s_358_10));
        // D s_358_14: add s_358_9 s_358_11
        let s_358_14: u16 = (s_358_9 + s_358_11);
        // D s_358_15: create-bits s_358_13 s_358_14
        let s_358_15: Bits = Bits::new(s_358_13, s_358_14);
        // D s_358_16: cast reint s_358_15 -> u8
        let s_358_16: u8 = (s_358_15.value() as u8);
        // D s_358_17: cast zx s_358_16 -> bv
        let s_358_17: Bits = Bits::new(s_358_16 as u128, 2u16);
        // C s_358_18: const #0u : u8
        let s_358_18: u8 = 0;
        // C s_358_19: cast zx s_358_18 -> bv
        let s_358_19: Bits = Bits::new(s_358_18 as u128, 2u16);
        // D s_358_20: cmp-ne s_358_17 s_358_19
        let s_358_20: bool = ((s_358_17) != (s_358_19));
        // D s_358_21: write-var gs#141958 <= s_358_20
        fn_state.gs_141958 = s_358_20;
        // N s_358_22: jump b351
        return block_351(state, tracer, fn_state);
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_359_0: const #1u : u8
        let s_359_0: bool = true;
        // D s_359_1: write-var gs#141962 <= s_359_0
        fn_state.gs_141962 = s_359_0;
        // N s_359_2: jump b348
        return block_348(state, tracer, fn_state);
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_360_0: const #1u : u8
        let s_360_0: bool = true;
        // D s_360_1: write-var gs#141966 <= s_360_0
        fn_state.gs_141966 = s_360_0;
        // N s_360_2: jump b335
        return block_335(state, tracer, fn_state);
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_361_0: const #19136u : u32
        let s_361_0: u32 = 19136;
        // D s_361_1: read-reg s_361_0:struct
        let s_361_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_361_0 as isize);
            tracer.read_register(s_361_0 as isize, value);
            value
        };
        // D s_361_2: call _get_PMSELR_EL0_Type_SEL(s_361_1)
        let s_361_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_361_1);
        // D s_361_3: cast zx s_361_2 -> bv
        let s_361_3: Bits = Bits::new(s_361_2 as u128, 5u16);
        // C s_361_4: const #31u : u8
        let s_361_4: u8 = 31;
        // C s_361_5: cast zx s_361_4 -> bv
        let s_361_5: Bits = Bits::new(s_361_4 as u128, 5u16);
        // D s_361_6: cmp-ne s_361_3 s_361_5
        let s_361_6: bool = ((s_361_3) != (s_361_5));
        // D s_361_7: write-var gs#141955 <= s_361_6
        fn_state.gs_141955 = s_361_6;
        // N s_361_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_362_0: read-var opc2:u8
        let s_362_0: u8 = fn_state.opc2;
        // D s_362_1: cast zx s_362_0 -> bv
        let s_362_1: Bits = Bits::new(s_362_0 as u128, 3u16);
        // C s_362_2: const #1u : u8
        let s_362_2: u8 = 1;
        // C s_362_3: cast zx s_362_2 -> bv
        let s_362_3: Bits = Bits::new(s_362_2 as u128, 3u16);
        // D s_362_4: cmp-eq s_362_1 s_362_3
        let s_362_4: bool = ((s_362_1) == (s_362_3));
        // N s_362_5: branch s_362_4 b365 b363
        if s_362_4 {
            return block_365(state, tracer, fn_state);
        } else {
            return block_363(state, tracer, fn_state);
        };
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_363_0: read-var opc2:u8
        let s_363_0: u8 = fn_state.opc2;
        // D s_363_1: cast zx s_363_0 -> bv
        let s_363_1: Bits = Bits::new(s_363_0 as u128, 3u16);
        // C s_363_2: const #2u : u8
        let s_363_2: u8 = 2;
        // C s_363_3: cast zx s_363_2 -> bv
        let s_363_3: Bits = Bits::new(s_363_2 as u128, 3u16);
        // D s_363_4: cmp-eq s_363_1 s_363_3
        let s_363_4: bool = ((s_363_1) == (s_363_3));
        // D s_363_5: write-var gs#141953 <= s_363_4
        fn_state.gs_141953 = s_363_4;
        // N s_363_6: jump b364
        return block_364(state, tracer, fn_state);
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_364_0: read-var gs#141953:u8
        let s_364_0: bool = fn_state.gs_141953;
        // D s_364_1: write-var gs#141954 <= s_364_0
        fn_state.gs_141954 = s_364_0;
        // N s_364_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_365_0: const #1u : u8
        let s_365_0: bool = true;
        // D s_365_1: write-var gs#141953 <= s_365_0
        fn_state.gs_141953 = s_365_0;
        // N s_365_2: jump b364
        return block_364(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_366_0: read-var CRm:u8
        let s_366_0: u8 = fn_state.CRm;
        // D s_366_1: cast zx s_366_0 -> bv
        let s_366_1: Bits = Bits::new(s_366_0 as u128, 4u16);
        // C s_366_2: const #13u : u8
        let s_366_2: u8 = 13;
        // C s_366_3: cast zx s_366_2 -> bv
        let s_366_3: Bits = Bits::new(s_366_2 as u128, 4u16);
        // D s_366_4: cmp-eq s_366_1 s_366_3
        let s_366_4: bool = ((s_366_1) == (s_366_3));
        // D s_366_5: write-var gs#141952 <= s_366_4
        fn_state.gs_141952 = s_366_4;
        // N s_366_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_367_0: read-var CRn:u8
        let s_367_0: u8 = fn_state.CRn;
        // D s_367_1: cast zx s_367_0 -> bv
        let s_367_1: Bits = Bits::new(s_367_0 as u128, 4u16);
        // C s_367_2: const #9u : u8
        let s_367_2: u8 = 9;
        // C s_367_3: cast zx s_367_2 -> bv
        let s_367_3: Bits = Bits::new(s_367_2 as u128, 4u16);
        // D s_367_4: cmp-eq s_367_1 s_367_3
        let s_367_4: bool = ((s_367_1) == (s_367_3));
        // D s_367_5: write-var gs#141951 <= s_367_4
        fn_state.gs_141951 = s_367_4;
        // N s_367_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_368_0: read-var opc1:u8
        let s_368_0: u8 = fn_state.opc1;
        // D s_368_1: cast zx s_368_0 -> bv
        let s_368_1: Bits = Bits::new(s_368_0 as u128, 3u16);
        // C s_368_2: const #0u : u8
        let s_368_2: u8 = 0;
        // C s_368_3: cast zx s_368_2 -> bv
        let s_368_3: Bits = Bits::new(s_368_2 as u128, 3u16);
        // D s_368_4: cmp-eq s_368_1 s_368_3
        let s_368_4: bool = ((s_368_1) == (s_368_3));
        // D s_368_5: write-var gs#141950 <= s_368_4
        fn_state.gs_141950 = s_368_4;
        // N s_368_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_369_0: const #0s : i
        let s_369_0: i128 = 0;
        // D s_369_1: read-var CRm:u8
        let s_369_1: u8 = fn_state.CRm;
        // D s_369_2: cast zx s_369_1 -> bv
        let s_369_2: Bits = Bits::new(s_369_1 as u128, 4u16);
        // C s_369_3: const #1s : i64
        let s_369_3: i64 = 1;
        // C s_369_4: cast zx s_369_3 -> i
        let s_369_4: i128 = (i128::try_from(s_369_3).unwrap());
        // C s_369_5: const #1s : i
        let s_369_5: i128 = 1;
        // C s_369_6: add s_369_5 s_369_4
        let s_369_6: i128 = (s_369_5 + s_369_4);
        // D s_369_7: bit-extract s_369_2 s_369_0 s_369_6
        let s_369_7: Bits = (Bits::new(
            ((s_369_2) >> (s_369_0)).value(),
            u16::try_from(s_369_6).unwrap(),
        ));
        // D s_369_8: cast reint s_369_7 -> u8
        let s_369_8: u8 = (s_369_7.value() as u8);
        // C s_369_9: const #0s : i
        let s_369_9: i128 = 0;
        // D s_369_10: read-var opc2:u8
        let s_369_10: u8 = fn_state.opc2;
        // D s_369_11: cast zx s_369_10 -> bv
        let s_369_11: Bits = Bits::new(s_369_10 as u128, 3u16);
        // C s_369_12: const #1s : i64
        let s_369_12: i64 = 1;
        // C s_369_13: cast zx s_369_12 -> i
        let s_369_13: i128 = (i128::try_from(s_369_12).unwrap());
        // C s_369_14: const #2s : i
        let s_369_14: i128 = 2;
        // C s_369_15: add s_369_14 s_369_13
        let s_369_15: i128 = (s_369_14 + s_369_13);
        // D s_369_16: bit-extract s_369_11 s_369_9 s_369_15
        let s_369_16: Bits = (Bits::new(
            ((s_369_11) >> (s_369_9)).value(),
            u16::try_from(s_369_15).unwrap(),
        ));
        // D s_369_17: cast reint s_369_16 -> u8
        let s_369_17: u8 = (s_369_16.value() as u8);
        // D s_369_18: cast zx s_369_8 -> bv
        let s_369_18: Bits = Bits::new(s_369_8 as u128, 2u16);
        // D s_369_19: cast zx s_369_17 -> bv
        let s_369_19: Bits = Bits::new(s_369_17 as u128, 3u16);
        // D s_369_20: cast reint s_369_18 -> u128
        let s_369_20: u128 = (s_369_18.value() as u128);
        // D s_369_21: size-of s_369_18
        let s_369_21: u16 = s_369_18.length();
        // D s_369_22: cast reint s_369_19 -> u128
        let s_369_22: u128 = (s_369_19.value() as u128);
        // D s_369_23: size-of s_369_19
        let s_369_23: u16 = s_369_19.length();
        // D s_369_24: lsl s_369_20 s_369_23
        let s_369_24: u128 = s_369_20 << s_369_23;
        // D s_369_25: or s_369_24 s_369_22
        let s_369_25: u128 = ((s_369_24) | (s_369_22));
        // D s_369_26: add s_369_21 s_369_23
        let s_369_26: u16 = (s_369_21 + s_369_23);
        // D s_369_27: create-bits s_369_25 s_369_26
        let s_369_27: Bits = Bits::new(s_369_25, s_369_26);
        // D s_369_28: cast reint s_369_27 -> u8
        let s_369_28: u8 = (s_369_27.value() as u8);
        // D s_369_29: cast zx s_369_28 -> bv
        let s_369_29: Bits = Bits::new(s_369_28 as u128, 5u16);
        // D s_369_30: cast zx s_369_29 -> i
        let s_369_30: i128 = (s_369_29.value() as i128);
        // D s_369_31: cast reint s_369_30 -> i64
        let s_369_31: i64 = (s_369_30 as i64);
        // C s_369_32: const #() : ()
        let s_369_32: () = ();
        // S s_369_33: call GetNumEventCounters(s_369_32)
        let s_369_33: i128 = GetNumEventCounters(state, tracer, s_369_32);
        // C s_369_34: const #1s : i
        let s_369_34: i128 = 1;
        // S s_369_35: sub s_369_33 s_369_34
        let s_369_35: i128 = ((s_369_33) - (s_369_34));
        // D s_369_36: cast zx s_369_31 -> i
        let s_369_36: i128 = (i128::try_from(s_369_31).unwrap());
        // D s_369_37: cmp-gt s_369_36 s_369_35
        let s_369_37: bool = ((s_369_36) > (s_369_35));
        // N s_369_38: branch s_369_37 b400 b370
        if s_369_37 {
            return block_400(state, tracer, fn_state);
        } else {
            return block_370(state, tracer, fn_state);
        };
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_370_0: const #() : ()
        let s_370_0: () = ();
        // S s_370_1: call EL2Enabled(s_370_0)
        let s_370_1: bool = EL2Enabled(state, tracer, s_370_0);
        // N s_370_2: branch s_370_1 b384 b371
        if s_370_1 {
            return block_384(state, tracer, fn_state);
        } else {
            return block_371(state, tracer, fn_state);
        };
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_371_0: const #0u : u8
        let s_371_0: bool = false;
        // D s_371_1: write-var gs#141984 <= s_371_0
        fn_state.gs_141984 = s_371_0;
        // N s_371_2: jump b372
        return block_372(state, tracer, fn_state);
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_372_0: read-var gs#141984:u8
        let s_372_0: bool = fn_state.gs_141984;
        // N s_372_1: branch s_372_0 b383 b373
        if s_372_0 {
            return block_383(state, tracer, fn_state);
        } else {
            return block_373(state, tracer, fn_state);
        };
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_373_0: const #0u : u8
        let s_373_0: bool = false;
        // D s_373_1: write-var gs#141990 <= s_373_0
        fn_state.gs_141990 = s_373_0;
        // N s_373_2: jump b374
        return block_374(state, tracer, fn_state);
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_374_0: read-var gs#141990:u8
        let s_374_0: bool = fn_state.gs_141990;
        // D s_374_1: write-var gs#141991 <= s_374_0
        fn_state.gs_141991 = s_374_0;
        // N s_374_2: jump b375
        return block_375(state, tracer, fn_state);
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_375_0: read-var gs#141991:u8
        let s_375_0: bool = fn_state.gs_141991;
        // N s_375_1: branch s_375_0 b378 b376
        if s_375_0 {
            return block_378(state, tracer, fn_state);
        } else {
            return block_376(state, tracer, fn_state);
        };
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_376_0: jump b377
        return block_377(state, tracer, fn_state);
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_377_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_378_0: const #0s : i
        let s_378_0: i128 = 0;
        // D s_378_1: read-var CRm:u8
        let s_378_1: u8 = fn_state.CRm;
        // D s_378_2: cast zx s_378_1 -> bv
        let s_378_2: Bits = Bits::new(s_378_1 as u128, 4u16);
        // C s_378_3: const #1s : i64
        let s_378_3: i64 = 1;
        // C s_378_4: cast zx s_378_3 -> i
        let s_378_4: i128 = (i128::try_from(s_378_3).unwrap());
        // C s_378_5: const #1s : i
        let s_378_5: i128 = 1;
        // C s_378_6: add s_378_5 s_378_4
        let s_378_6: i128 = (s_378_5 + s_378_4);
        // D s_378_7: bit-extract s_378_2 s_378_0 s_378_6
        let s_378_7: Bits = (Bits::new(
            ((s_378_2) >> (s_378_0)).value(),
            u16::try_from(s_378_6).unwrap(),
        ));
        // D s_378_8: cast reint s_378_7 -> u8
        let s_378_8: u8 = (s_378_7.value() as u8);
        // C s_378_9: const #0s : i
        let s_378_9: i128 = 0;
        // D s_378_10: read-var opc2:u8
        let s_378_10: u8 = fn_state.opc2;
        // D s_378_11: cast zx s_378_10 -> bv
        let s_378_11: Bits = Bits::new(s_378_10 as u128, 3u16);
        // C s_378_12: const #1s : i64
        let s_378_12: i64 = 1;
        // C s_378_13: cast zx s_378_12 -> i
        let s_378_13: i128 = (i128::try_from(s_378_12).unwrap());
        // C s_378_14: const #2s : i
        let s_378_14: i128 = 2;
        // C s_378_15: add s_378_14 s_378_13
        let s_378_15: i128 = (s_378_14 + s_378_13);
        // D s_378_16: bit-extract s_378_11 s_378_9 s_378_15
        let s_378_16: Bits = (Bits::new(
            ((s_378_11) >> (s_378_9)).value(),
            u16::try_from(s_378_15).unwrap(),
        ));
        // D s_378_17: cast reint s_378_16 -> u8
        let s_378_17: u8 = (s_378_16.value() as u8);
        // D s_378_18: cast zx s_378_8 -> bv
        let s_378_18: Bits = Bits::new(s_378_8 as u128, 2u16);
        // D s_378_19: cast zx s_378_17 -> bv
        let s_378_19: Bits = Bits::new(s_378_17 as u128, 3u16);
        // D s_378_20: cast reint s_378_18 -> u128
        let s_378_20: u128 = (s_378_18.value() as u128);
        // D s_378_21: size-of s_378_18
        let s_378_21: u16 = s_378_18.length();
        // D s_378_22: cast reint s_378_19 -> u128
        let s_378_22: u128 = (s_378_19.value() as u128);
        // D s_378_23: size-of s_378_19
        let s_378_23: u16 = s_378_19.length();
        // D s_378_24: lsl s_378_20 s_378_23
        let s_378_24: u128 = s_378_20 << s_378_23;
        // D s_378_25: or s_378_24 s_378_22
        let s_378_25: u128 = ((s_378_24) | (s_378_22));
        // D s_378_26: add s_378_21 s_378_23
        let s_378_26: u16 = (s_378_21 + s_378_23);
        // D s_378_27: create-bits s_378_25 s_378_26
        let s_378_27: Bits = Bits::new(s_378_25, s_378_26);
        // D s_378_28: cast reint s_378_27 -> u8
        let s_378_28: u8 = (s_378_27.value() as u8);
        // D s_378_29: cast zx s_378_28 -> bv
        let s_378_29: Bits = Bits::new(s_378_28 as u128, 5u16);
        // D s_378_30: cast zx s_378_29 -> i
        let s_378_30: i128 = (s_378_29.value() as i128);
        // D s_378_31: cast reint s_378_30 -> i64
        let s_378_31: i64 = (s_378_30 as i64);
        // C s_378_32: const #() : ()
        let s_378_32: () = ();
        // S s_378_33: call GetNumEventCounters(s_378_32)
        let s_378_33: i128 = GetNumEventCounters(state, tracer, s_378_32);
        // C s_378_34: const #1s : i
        let s_378_34: i128 = 1;
        // S s_378_35: sub s_378_33 s_378_34
        let s_378_35: i128 = ((s_378_33) - (s_378_34));
        // D s_378_36: cast zx s_378_31 -> i
        let s_378_36: i128 = (i128::try_from(s_378_31).unwrap());
        // D s_378_37: cmp-gt s_378_36 s_378_35
        let s_378_37: bool = ((s_378_36) > (s_378_35));
        // N s_378_38: branch s_378_37 b382 b379
        if s_378_37 {
            return block_382(state, tracer, fn_state);
        } else {
            return block_379(state, tracer, fn_state);
        };
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_379_0: const #432u : u32
        let s_379_0: u32 = 432;
        // D s_379_1: read-reg s_379_0:u8
        let s_379_1: u8 = {
            let value = state.read_register::<u8>(s_379_0 as isize);
            tracer.read_register(s_379_0 as isize, value);
            value
        };
        // D s_379_2: call ELUsingAArch32(s_379_1)
        let s_379_2: bool = ELUsingAArch32(state, tracer, s_379_1);
        // N s_379_3: branch s_379_2 b381 b380
        if s_379_2 {
            return block_381(state, tracer, fn_state);
        } else {
            return block_380(state, tracer, fn_state);
        };
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_380_0: const #3u : u8
        let s_380_0: u8 = 3;
        // C s_380_1: cast zx s_380_0 -> bv
        let s_380_1: Bits = Bits::new(s_380_0 as u128, 8u16);
        // C s_380_2: cast zx s_380_1 -> i
        let s_380_2: i128 = (s_380_1.value() as i128);
        // C s_380_3: cast reint s_380_2 -> i64
        let s_380_3: i64 = (s_380_2 as i64);
        // C s_380_4: cast zx s_380_3 -> i
        let s_380_4: i128 = (i128::try_from(s_380_3).unwrap());
        // C s_380_5: const #432u : u32
        let s_380_5: u32 = 432;
        // D s_380_6: read-reg s_380_5:u8
        let s_380_6: u8 = {
            let value = state.read_register::<u8>(s_380_5 as isize);
            tracer.read_register(s_380_5 as isize, value);
            value
        };
        // D s_380_7: call AArch64_AArch32SystemAccessTrap(s_380_6, s_380_4)
        let s_380_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_380_6,
            s_380_4,
        );
        // N s_380_8: jump b377
        return block_377(state, tracer, fn_state);
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_381_0: const #3u : u8
        let s_381_0: u8 = 3;
        // C s_381_1: cast zx s_381_0 -> bv
        let s_381_1: Bits = Bits::new(s_381_0 as u128, 8u16);
        // C s_381_2: cast zx s_381_1 -> i
        let s_381_2: i128 = (s_381_1.value() as i128);
        // C s_381_3: cast reint s_381_2 -> i64
        let s_381_3: i64 = (s_381_2 as i64);
        // C s_381_4: cast zx s_381_3 -> i
        let s_381_4: i128 = (i128::try_from(s_381_3).unwrap());
        // S s_381_5: call AArch32_TakeHypTrapException(s_381_4)
        let s_381_5: () = AArch32_TakeHypTrapException(state, tracer, s_381_4);
        // N s_381_6: jump b377
        return block_377(state, tracer, fn_state);
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_382_0: panic
        panic!("{:?}", ());
        // N s_382_1: return
        return;
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_383_0: const #0s : i
        let s_383_0: i128 = 0;
        // D s_383_1: read-var CRm:u8
        let s_383_1: u8 = fn_state.CRm;
        // D s_383_2: cast zx s_383_1 -> bv
        let s_383_2: Bits = Bits::new(s_383_1 as u128, 4u16);
        // C s_383_3: const #1s : i64
        let s_383_3: i64 = 1;
        // C s_383_4: cast zx s_383_3 -> i
        let s_383_4: i128 = (i128::try_from(s_383_3).unwrap());
        // C s_383_5: const #1s : i
        let s_383_5: i128 = 1;
        // C s_383_6: add s_383_5 s_383_4
        let s_383_6: i128 = (s_383_5 + s_383_4);
        // D s_383_7: bit-extract s_383_2 s_383_0 s_383_6
        let s_383_7: Bits = (Bits::new(
            ((s_383_2) >> (s_383_0)).value(),
            u16::try_from(s_383_6).unwrap(),
        ));
        // D s_383_8: cast reint s_383_7 -> u8
        let s_383_8: u8 = (s_383_7.value() as u8);
        // C s_383_9: const #0s : i
        let s_383_9: i128 = 0;
        // D s_383_10: read-var opc2:u8
        let s_383_10: u8 = fn_state.opc2;
        // D s_383_11: cast zx s_383_10 -> bv
        let s_383_11: Bits = Bits::new(s_383_10 as u128, 3u16);
        // C s_383_12: const #1s : i64
        let s_383_12: i64 = 1;
        // C s_383_13: cast zx s_383_12 -> i
        let s_383_13: i128 = (i128::try_from(s_383_12).unwrap());
        // C s_383_14: const #2s : i
        let s_383_14: i128 = 2;
        // C s_383_15: add s_383_14 s_383_13
        let s_383_15: i128 = (s_383_14 + s_383_13);
        // D s_383_16: bit-extract s_383_11 s_383_9 s_383_15
        let s_383_16: Bits = (Bits::new(
            ((s_383_11) >> (s_383_9)).value(),
            u16::try_from(s_383_15).unwrap(),
        ));
        // D s_383_17: cast reint s_383_16 -> u8
        let s_383_17: u8 = (s_383_16.value() as u8);
        // D s_383_18: cast zx s_383_8 -> bv
        let s_383_18: Bits = Bits::new(s_383_8 as u128, 2u16);
        // D s_383_19: cast zx s_383_17 -> bv
        let s_383_19: Bits = Bits::new(s_383_17 as u128, 3u16);
        // D s_383_20: cast reint s_383_18 -> u128
        let s_383_20: u128 = (s_383_18.value() as u128);
        // D s_383_21: size-of s_383_18
        let s_383_21: u16 = s_383_18.length();
        // D s_383_22: cast reint s_383_19 -> u128
        let s_383_22: u128 = (s_383_19.value() as u128);
        // D s_383_23: size-of s_383_19
        let s_383_23: u16 = s_383_19.length();
        // D s_383_24: lsl s_383_20 s_383_23
        let s_383_24: u128 = s_383_20 << s_383_23;
        // D s_383_25: or s_383_24 s_383_22
        let s_383_25: u128 = ((s_383_24) | (s_383_22));
        // D s_383_26: add s_383_21 s_383_23
        let s_383_26: u16 = (s_383_21 + s_383_23);
        // D s_383_27: create-bits s_383_25 s_383_26
        let s_383_27: Bits = Bits::new(s_383_25, s_383_26);
        // D s_383_28: cast reint s_383_27 -> u8
        let s_383_28: u8 = (s_383_27.value() as u8);
        // D s_383_29: cast zx s_383_28 -> bv
        let s_383_29: Bits = Bits::new(s_383_28 as u128, 5u16);
        // D s_383_30: cast zx s_383_29 -> i
        let s_383_30: i128 = (s_383_29.value() as i128);
        // D s_383_31: cast reint s_383_30 -> i64
        let s_383_31: i64 = (s_383_30 as i64);
        // C s_383_32: const #() : ()
        let s_383_32: () = ();
        // S s_383_33: call AArch32_GetNumEventCountersAccessible(s_383_32)
        let s_383_33: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_383_32,
        );
        // C s_383_34: const #1s : i
        let s_383_34: i128 = 1;
        // S s_383_35: sub s_383_33 s_383_34
        let s_383_35: i128 = ((s_383_33) - (s_383_34));
        // D s_383_36: cast zx s_383_31 -> i
        let s_383_36: i128 = (i128::try_from(s_383_31).unwrap());
        // D s_383_37: cmp-gt s_383_36 s_383_35
        let s_383_37: bool = ((s_383_36) > (s_383_35));
        // D s_383_38: write-var gs#141990 <= s_383_37
        fn_state.gs_141990 = s_383_37;
        // N s_383_39: jump b374
        return block_374(state, tracer, fn_state);
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_384_0: const #16975u : u32
        let s_384_0: u32 = 16975;
        // D s_384_1: read-reg s_384_0:u8
        let s_384_1: u8 = {
            let value = state.read_register::<u8>(s_384_0 as isize);
            tracer.read_register(s_384_0 as isize, value);
            value
        };
        // D s_384_2: cast zx s_384_1 -> bv
        let s_384_2: Bits = Bits::new(s_384_1 as u128, 2u16);
        // C s_384_3: const #440u : u32
        let s_384_3: u32 = 440;
        // D s_384_4: read-reg s_384_3:u8
        let s_384_4: u8 = {
            let value = state.read_register::<u8>(s_384_3 as isize);
            tracer.read_register(s_384_3 as isize, value);
            value
        };
        // D s_384_5: cast zx s_384_4 -> bv
        let s_384_5: Bits = Bits::new(s_384_4 as u128, 2u16);
        // D s_384_6: cmp-eq s_384_2 s_384_5
        let s_384_6: bool = ((s_384_2) == (s_384_5));
        // N s_384_7: branch s_384_6 b399 b385
        if s_384_6 {
            return block_399(state, tracer, fn_state);
        } else {
            return block_385(state, tracer, fn_state);
        };
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_385_0: const #16975u : u32
        let s_385_0: u32 = 16975;
        // D s_385_1: read-reg s_385_0:u8
        let s_385_1: u8 = {
            let value = state.read_register::<u8>(s_385_0 as isize);
            tracer.read_register(s_385_0 as isize, value);
            value
        };
        // D s_385_2: cast zx s_385_1 -> bv
        let s_385_2: Bits = Bits::new(s_385_1 as u128, 2u16);
        // C s_385_3: const #448u : u32
        let s_385_3: u32 = 448;
        // D s_385_4: read-reg s_385_3:u8
        let s_385_4: u8 = {
            let value = state.read_register::<u8>(s_385_3 as isize);
            tracer.read_register(s_385_3 as isize, value);
            value
        };
        // D s_385_5: cast zx s_385_4 -> bv
        let s_385_5: Bits = Bits::new(s_385_4 as u128, 2u16);
        // D s_385_6: cmp-eq s_385_2 s_385_5
        let s_385_6: bool = ((s_385_2) == (s_385_5));
        // N s_385_7: branch s_385_6 b389 b386
        if s_385_6 {
            return block_389(state, tracer, fn_state);
        } else {
            return block_386(state, tracer, fn_state);
        };
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_386_0: const #0u : u8
        let s_386_0: bool = false;
        // D s_386_1: write-var gs#141982 <= s_386_0
        fn_state.gs_141982 = s_386_0;
        // N s_386_2: jump b387
        return block_387(state, tracer, fn_state);
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_387_0: read-var gs#141982:u8
        let s_387_0: bool = fn_state.gs_141982;
        // D s_387_1: write-var gs#141983 <= s_387_0
        fn_state.gs_141983 = s_387_0;
        // N s_387_2: jump b388
        return block_388(state, tracer, fn_state);
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_388_0: read-var gs#141983:u8
        let s_388_0: bool = fn_state.gs_141983;
        // D s_388_1: write-var gs#141984 <= s_388_0
        fn_state.gs_141984 = s_388_0;
        // N s_388_2: jump b372
        return block_372(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_389_0: const #2s : i
        let s_389_0: i128 = 2;
        // D s_389_1: read-var CRm:u8
        let s_389_1: u8 = fn_state.CRm;
        // D s_389_2: cast zx s_389_1 -> bv
        let s_389_2: Bits = Bits::new(s_389_1 as u128, 4u16);
        // C s_389_3: const #1s : i64
        let s_389_3: i64 = 1;
        // C s_389_4: cast zx s_389_3 -> i
        let s_389_4: i128 = (i128::try_from(s_389_3).unwrap());
        // C s_389_5: const #1s : i
        let s_389_5: i128 = 1;
        // C s_389_6: add s_389_5 s_389_4
        let s_389_6: i128 = (s_389_5 + s_389_4);
        // D s_389_7: bit-extract s_389_2 s_389_0 s_389_6
        let s_389_7: Bits = (Bits::new(
            ((s_389_2) >> (s_389_0)).value(),
            u16::try_from(s_389_6).unwrap(),
        ));
        // D s_389_8: cast reint s_389_7 -> u8
        let s_389_8: u8 = (s_389_7.value() as u8);
        // D s_389_9: cast zx s_389_8 -> bv
        let s_389_9: Bits = Bits::new(s_389_8 as u128, 2u16);
        // C s_389_10: const #2u : u8
        let s_389_10: u8 = 2;
        // C s_389_11: cast zx s_389_10 -> bv
        let s_389_11: Bits = Bits::new(s_389_10 as u128, 2u16);
        // D s_389_12: cmp-eq s_389_9 s_389_11
        let s_389_12: bool = ((s_389_9) == (s_389_11));
        // N s_389_13: branch s_389_12 b398 b390
        if s_389_12 {
            return block_398(state, tracer, fn_state);
        } else {
            return block_390(state, tracer, fn_state);
        };
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_390_0: const #0u : u8
        let s_390_0: bool = false;
        // D s_390_1: write-var gs#141977 <= s_390_0
        fn_state.gs_141977 = s_390_0;
        // N s_390_2: jump b391
        return block_391(state, tracer, fn_state);
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_391_0: read-var gs#141977:u8
        let s_391_0: bool = fn_state.gs_141977;
        // N s_391_1: branch s_391_0 b397 b392
        if s_391_0 {
            return block_397(state, tracer, fn_state);
        } else {
            return block_392(state, tracer, fn_state);
        };
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_392_0: const #2s : i
        let s_392_0: i128 = 2;
        // D s_392_1: read-var CRm:u8
        let s_392_1: u8 = fn_state.CRm;
        // D s_392_2: cast zx s_392_1 -> bv
        let s_392_2: Bits = Bits::new(s_392_1 as u128, 4u16);
        // C s_392_3: const #1s : i64
        let s_392_3: i64 = 1;
        // C s_392_4: cast zx s_392_3 -> i
        let s_392_4: i128 = (i128::try_from(s_392_3).unwrap());
        // C s_392_5: const #1s : i
        let s_392_5: i128 = 1;
        // C s_392_6: add s_392_5 s_392_4
        let s_392_6: i128 = (s_392_5 + s_392_4);
        // D s_392_7: bit-extract s_392_2 s_392_0 s_392_6
        let s_392_7: Bits = (Bits::new(
            ((s_392_2) >> (s_392_0)).value(),
            u16::try_from(s_392_6).unwrap(),
        ));
        // D s_392_8: cast reint s_392_7 -> u8
        let s_392_8: u8 = (s_392_7.value() as u8);
        // D s_392_9: cast zx s_392_8 -> bv
        let s_392_9: Bits = Bits::new(s_392_8 as u128, 2u16);
        // C s_392_10: const #3u : u8
        let s_392_10: u8 = 3;
        // C s_392_11: cast zx s_392_10 -> bv
        let s_392_11: Bits = Bits::new(s_392_10 as u128, 2u16);
        // D s_392_12: cmp-eq s_392_9 s_392_11
        let s_392_12: bool = ((s_392_9) == (s_392_11));
        // N s_392_13: branch s_392_12 b396 b393
        if s_392_12 {
            return block_396(state, tracer, fn_state);
        } else {
            return block_393(state, tracer, fn_state);
        };
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_393_0: const #0u : u8
        let s_393_0: bool = false;
        // D s_393_1: write-var gs#141980 <= s_393_0
        fn_state.gs_141980 = s_393_0;
        // N s_393_2: jump b394
        return block_394(state, tracer, fn_state);
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_394_0: read-var gs#141980:u8
        let s_394_0: bool = fn_state.gs_141980;
        // D s_394_1: write-var gs#141981 <= s_394_0
        fn_state.gs_141981 = s_394_0;
        // N s_394_2: jump b395
        return block_395(state, tracer, fn_state);
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_395_0: read-var gs#141981:u8
        let s_395_0: bool = fn_state.gs_141981;
        // D s_395_1: write-var gs#141982 <= s_395_0
        fn_state.gs_141982 = s_395_0;
        // N s_395_2: jump b387
        return block_387(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_396_0: const #102624u : u32
        let s_396_0: u32 = 102624;
        // D s_396_1: read-reg s_396_0:struct
        let s_396_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_396_0 as isize);
            tracer.read_register(s_396_0 as isize, value);
            value
        };
        // D s_396_2: call _get_PMUSERENR_EL0_Type_EN(s_396_1)
        let s_396_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_396_1);
        // D s_396_3: cast zx s_396_2 -> bv
        let s_396_3: Bits = Bits::new(s_396_2 as u128, 1u16);
        // C s_396_4: const #1u : u8
        let s_396_4: bool = true;
        // C s_396_5: cast zx s_396_4 -> bv
        let s_396_5: Bits = Bits::new(s_396_4 as u128, 1u16);
        // D s_396_6: cmp-eq s_396_3 s_396_5
        let s_396_6: bool = ((s_396_3) == (s_396_5));
        // D s_396_7: write-var gs#141980 <= s_396_6
        fn_state.gs_141980 = s_396_6;
        // N s_396_8: jump b394
        return block_394(state, tracer, fn_state);
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_397_0: const #1u : u8
        let s_397_0: bool = true;
        // D s_397_1: write-var gs#141981 <= s_397_0
        fn_state.gs_141981 = s_397_0;
        // N s_397_2: jump b395
        return block_395(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_398_0: const #102624u : u32
        let s_398_0: u32 = 102624;
        // D s_398_1: read-reg s_398_0:struct
        let s_398_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_398_0 as isize);
            tracer.read_register(s_398_0 as isize, value);
            value
        };
        // D s_398_2: call _get_PMUSERENR_EL0_Type_ER(s_398_1)
        let s_398_2: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_398_1);
        // C s_398_3: const #102624u : u32
        let s_398_3: u32 = 102624;
        // D s_398_4: read-reg s_398_3:struct
        let s_398_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_398_3 as isize);
            tracer.read_register(s_398_3 as isize, value);
            value
        };
        // D s_398_5: call _get_PMUSERENR_EL0_Type_EN(s_398_4)
        let s_398_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_398_4);
        // D s_398_6: cast zx s_398_2 -> bv
        let s_398_6: Bits = Bits::new(s_398_2 as u128, 1u16);
        // D s_398_7: cast zx s_398_5 -> bv
        let s_398_7: Bits = Bits::new(s_398_5 as u128, 1u16);
        // D s_398_8: cast reint s_398_6 -> u128
        let s_398_8: u128 = (s_398_6.value() as u128);
        // D s_398_9: size-of s_398_6
        let s_398_9: u16 = s_398_6.length();
        // D s_398_10: cast reint s_398_7 -> u128
        let s_398_10: u128 = (s_398_7.value() as u128);
        // D s_398_11: size-of s_398_7
        let s_398_11: u16 = s_398_7.length();
        // D s_398_12: lsl s_398_8 s_398_11
        let s_398_12: u128 = s_398_8 << s_398_11;
        // D s_398_13: or s_398_12 s_398_10
        let s_398_13: u128 = ((s_398_12) | (s_398_10));
        // D s_398_14: add s_398_9 s_398_11
        let s_398_14: u16 = (s_398_9 + s_398_11);
        // D s_398_15: create-bits s_398_13 s_398_14
        let s_398_15: Bits = Bits::new(s_398_13, s_398_14);
        // D s_398_16: cast reint s_398_15 -> u8
        let s_398_16: u8 = (s_398_15.value() as u8);
        // D s_398_17: cast zx s_398_16 -> bv
        let s_398_17: Bits = Bits::new(s_398_16 as u128, 2u16);
        // C s_398_18: const #0u : u8
        let s_398_18: u8 = 0;
        // C s_398_19: cast zx s_398_18 -> bv
        let s_398_19: Bits = Bits::new(s_398_18 as u128, 2u16);
        // D s_398_20: cmp-ne s_398_17 s_398_19
        let s_398_20: bool = ((s_398_17) != (s_398_19));
        // D s_398_21: write-var gs#141977 <= s_398_20
        fn_state.gs_141977 = s_398_20;
        // N s_398_22: jump b391
        return block_391(state, tracer, fn_state);
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_399_0: const #1u : u8
        let s_399_0: bool = true;
        // D s_399_1: write-var gs#141983 <= s_399_0
        fn_state.gs_141983 = s_399_0;
        // N s_399_2: jump b388
        return block_388(state, tracer, fn_state);
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_400_0: const #1u : u8
        let s_400_0: bool = true;
        // D s_400_1: write-var gs#141991 <= s_400_0
        fn_state.gs_141991 = s_400_0;
        // N s_400_2: jump b375
        return block_375(state, tracer, fn_state);
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_401_0: const #0s : i
        let s_401_0: i128 = 0;
        // D s_401_1: read-var CRm:u8
        let s_401_1: u8 = fn_state.CRm;
        // D s_401_2: cast zx s_401_1 -> bv
        let s_401_2: Bits = Bits::new(s_401_1 as u128, 4u16);
        // C s_401_3: const #1s : i64
        let s_401_3: i64 = 1;
        // C s_401_4: cast zx s_401_3 -> i
        let s_401_4: i128 = (i128::try_from(s_401_3).unwrap());
        // C s_401_5: const #1s : i
        let s_401_5: i128 = 1;
        // C s_401_6: add s_401_5 s_401_4
        let s_401_6: i128 = (s_401_5 + s_401_4);
        // D s_401_7: bit-extract s_401_2 s_401_0 s_401_6
        let s_401_7: Bits = (Bits::new(
            ((s_401_2) >> (s_401_0)).value(),
            u16::try_from(s_401_6).unwrap(),
        ));
        // D s_401_8: cast reint s_401_7 -> u8
        let s_401_8: u8 = (s_401_7.value() as u8);
        // C s_401_9: const #0s : i
        let s_401_9: i128 = 0;
        // D s_401_10: read-var opc2:u8
        let s_401_10: u8 = fn_state.opc2;
        // D s_401_11: cast zx s_401_10 -> bv
        let s_401_11: Bits = Bits::new(s_401_10 as u128, 3u16);
        // C s_401_12: const #1s : i64
        let s_401_12: i64 = 1;
        // C s_401_13: cast zx s_401_12 -> i
        let s_401_13: i128 = (i128::try_from(s_401_12).unwrap());
        // C s_401_14: const #2s : i
        let s_401_14: i128 = 2;
        // C s_401_15: add s_401_14 s_401_13
        let s_401_15: i128 = (s_401_14 + s_401_13);
        // D s_401_16: bit-extract s_401_11 s_401_9 s_401_15
        let s_401_16: Bits = (Bits::new(
            ((s_401_11) >> (s_401_9)).value(),
            u16::try_from(s_401_15).unwrap(),
        ));
        // D s_401_17: cast reint s_401_16 -> u8
        let s_401_17: u8 = (s_401_16.value() as u8);
        // D s_401_18: cast zx s_401_8 -> bv
        let s_401_18: Bits = Bits::new(s_401_8 as u128, 2u16);
        // D s_401_19: cast zx s_401_17 -> bv
        let s_401_19: Bits = Bits::new(s_401_17 as u128, 3u16);
        // D s_401_20: cast reint s_401_18 -> u128
        let s_401_20: u128 = (s_401_18.value() as u128);
        // D s_401_21: size-of s_401_18
        let s_401_21: u16 = s_401_18.length();
        // D s_401_22: cast reint s_401_19 -> u128
        let s_401_22: u128 = (s_401_19.value() as u128);
        // D s_401_23: size-of s_401_19
        let s_401_23: u16 = s_401_19.length();
        // D s_401_24: lsl s_401_20 s_401_23
        let s_401_24: u128 = s_401_20 << s_401_23;
        // D s_401_25: or s_401_24 s_401_22
        let s_401_25: u128 = ((s_401_24) | (s_401_22));
        // D s_401_26: add s_401_21 s_401_23
        let s_401_26: u16 = (s_401_21 + s_401_23);
        // D s_401_27: create-bits s_401_25 s_401_26
        let s_401_27: Bits = Bits::new(s_401_25, s_401_26);
        // D s_401_28: cast reint s_401_27 -> u8
        let s_401_28: u8 = (s_401_27.value() as u8);
        // D s_401_29: cast zx s_401_28 -> bv
        let s_401_29: Bits = Bits::new(s_401_28 as u128, 5u16);
        // C s_401_30: const #31u : u8
        let s_401_30: u8 = 31;
        // C s_401_31: cast zx s_401_30 -> bv
        let s_401_31: Bits = Bits::new(s_401_30 as u128, 5u16);
        // D s_401_32: cmp-ne s_401_29 s_401_31
        let s_401_32: bool = ((s_401_29) != (s_401_31));
        // D s_401_33: write-var gs#141948 <= s_401_32
        fn_state.gs_141948 = s_401_32;
        // N s_401_34: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_402_0: const #2s : i
        let s_402_0: i128 = 2;
        // D s_402_1: read-var CRm:u8
        let s_402_1: u8 = fn_state.CRm;
        // D s_402_2: cast zx s_402_1 -> bv
        let s_402_2: Bits = Bits::new(s_402_1 as u128, 4u16);
        // C s_402_3: const #1s : i64
        let s_402_3: i64 = 1;
        // C s_402_4: cast zx s_402_3 -> i
        let s_402_4: i128 = (i128::try_from(s_402_3).unwrap());
        // C s_402_5: const #1s : i
        let s_402_5: i128 = 1;
        // C s_402_6: add s_402_5 s_402_4
        let s_402_6: i128 = (s_402_5 + s_402_4);
        // D s_402_7: bit-extract s_402_2 s_402_0 s_402_6
        let s_402_7: Bits = (Bits::new(
            ((s_402_2) >> (s_402_0)).value(),
            u16::try_from(s_402_6).unwrap(),
        ));
        // D s_402_8: cast reint s_402_7 -> u8
        let s_402_8: u8 = (s_402_7.value() as u8);
        // D s_402_9: cast zx s_402_8 -> bv
        let s_402_9: Bits = Bits::new(s_402_8 as u128, 2u16);
        // C s_402_10: const #2u : u8
        let s_402_10: u8 = 2;
        // C s_402_11: cast zx s_402_10 -> bv
        let s_402_11: Bits = Bits::new(s_402_10 as u128, 2u16);
        // D s_402_12: cmp-eq s_402_9 s_402_11
        let s_402_12: bool = ((s_402_9) == (s_402_11));
        // N s_402_13: branch s_402_12 b405 b403
        if s_402_12 {
            return block_405(state, tracer, fn_state);
        } else {
            return block_403(state, tracer, fn_state);
        };
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_403_0: const #2s : i
        let s_403_0: i128 = 2;
        // D s_403_1: read-var CRm:u8
        let s_403_1: u8 = fn_state.CRm;
        // D s_403_2: cast zx s_403_1 -> bv
        let s_403_2: Bits = Bits::new(s_403_1 as u128, 4u16);
        // C s_403_3: const #1s : i64
        let s_403_3: i64 = 1;
        // C s_403_4: cast zx s_403_3 -> i
        let s_403_4: i128 = (i128::try_from(s_403_3).unwrap());
        // C s_403_5: const #1s : i
        let s_403_5: i128 = 1;
        // C s_403_6: add s_403_5 s_403_4
        let s_403_6: i128 = (s_403_5 + s_403_4);
        // D s_403_7: bit-extract s_403_2 s_403_0 s_403_6
        let s_403_7: Bits = (Bits::new(
            ((s_403_2) >> (s_403_0)).value(),
            u16::try_from(s_403_6).unwrap(),
        ));
        // D s_403_8: cast reint s_403_7 -> u8
        let s_403_8: u8 = (s_403_7.value() as u8);
        // D s_403_9: cast zx s_403_8 -> bv
        let s_403_9: Bits = Bits::new(s_403_8 as u128, 2u16);
        // C s_403_10: const #3u : u8
        let s_403_10: u8 = 3;
        // C s_403_11: cast zx s_403_10 -> bv
        let s_403_11: Bits = Bits::new(s_403_10 as u128, 2u16);
        // D s_403_12: cmp-eq s_403_9 s_403_11
        let s_403_12: bool = ((s_403_9) == (s_403_11));
        // D s_403_13: write-var gs#141942 <= s_403_12
        fn_state.gs_141942 = s_403_12;
        // N s_403_14: jump b404
        return block_404(state, tracer, fn_state);
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_404_0: read-var gs#141942:u8
        let s_404_0: bool = fn_state.gs_141942;
        // D s_404_1: write-var gs#141943 <= s_404_0
        fn_state.gs_141943 = s_404_0;
        // N s_404_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_405_0: const #1u : u8
        let s_405_0: bool = true;
        // D s_405_1: write-var gs#141942 <= s_405_0
        fn_state.gs_141942 = s_405_0;
        // N s_405_2: jump b404
        return block_404(state, tracer, fn_state);
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_406_0: read-var CRn:u8
        let s_406_0: u8 = fn_state.CRn;
        // D s_406_1: cast zx s_406_0 -> bv
        let s_406_1: Bits = Bits::new(s_406_0 as u128, 4u16);
        // C s_406_2: const #14u : u8
        let s_406_2: u8 = 14;
        // C s_406_3: cast zx s_406_2 -> bv
        let s_406_3: Bits = Bits::new(s_406_2 as u128, 4u16);
        // D s_406_4: cmp-eq s_406_1 s_406_3
        let s_406_4: bool = ((s_406_1) == (s_406_3));
        // D s_406_5: write-var gs#141937 <= s_406_4
        fn_state.gs_141937 = s_406_4;
        // N s_406_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_407_0: read-var opc1:u8
        let s_407_0: u8 = fn_state.opc1;
        // D s_407_1: cast zx s_407_0 -> bv
        let s_407_1: Bits = Bits::new(s_407_0 as u128, 3u16);
        // C s_407_2: const #0u : u8
        let s_407_2: u8 = 0;
        // C s_407_3: cast zx s_407_2 -> bv
        let s_407_3: Bits = Bits::new(s_407_2 as u128, 3u16);
        // D s_407_4: cmp-eq s_407_1 s_407_3
        let s_407_4: bool = ((s_407_1) == (s_407_3));
        // D s_407_5: write-var gs#141936 <= s_407_4
        fn_state.gs_141936 = s_407_4;
        // N s_407_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_408_0: const #0s : i
        let s_408_0: i128 = 0;
        // D s_408_1: read-var CRm:u8
        let s_408_1: u8 = fn_state.CRm;
        // D s_408_2: cast zx s_408_1 -> bv
        let s_408_2: Bits = Bits::new(s_408_1 as u128, 4u16);
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
        // D s_408_19: read-var opc2:u8
        let s_408_19: u8 = fn_state.opc2;
        // D s_408_20: cast zx s_408_19 -> bv
        let s_408_20: Bits = Bits::new(s_408_19 as u128, 3u16);
        // D s_408_21: cast reint s_408_18 -> u128
        let s_408_21: u128 = (s_408_18.value() as u128);
        // D s_408_22: size-of s_408_18
        let s_408_22: u16 = s_408_18.length();
        // D s_408_23: cast reint s_408_20 -> u128
        let s_408_23: u128 = (s_408_20.value() as u128);
        // D s_408_24: size-of s_408_20
        let s_408_24: u16 = s_408_20.length();
        // D s_408_25: lsl s_408_21 s_408_24
        let s_408_25: u128 = s_408_21 << s_408_24;
        // D s_408_26: or s_408_25 s_408_23
        let s_408_26: u128 = ((s_408_25) | (s_408_23));
        // D s_408_27: add s_408_22 s_408_24
        let s_408_27: u16 = (s_408_22 + s_408_24);
        // D s_408_28: create-bits s_408_26 s_408_27
        let s_408_28: Bits = Bits::new(s_408_26, s_408_27);
        // D s_408_29: cast reint s_408_28 -> u8
        let s_408_29: u8 = (s_408_28.value() as u8);
        // D s_408_30: cast zx s_408_29 -> bv
        let s_408_30: Bits = Bits::new(s_408_29 as u128, 4u16);
        // D s_408_31: cast zx s_408_30 -> i
        let s_408_31: i128 = (s_408_30.value() as i128);
        // D s_408_32: cast reint s_408_31 -> i64
        let s_408_32: i64 = (s_408_31 as i64);
        // D s_408_33: write-var nshadow#1013 <= s_408_32
        fn_state.nshadow_1013 = s_408_32;
        // C s_408_34: const #1s : i
        let s_408_34: i128 = 1;
        // D s_408_35: read-var CRm:u8
        let s_408_35: u8 = fn_state.CRm;
        // D s_408_36: cast zx s_408_35 -> bv
        let s_408_36: Bits = Bits::new(s_408_35 as u128, 4u16);
        // C s_408_37: const #1s : i64
        let s_408_37: i64 = 1;
        // C s_408_38: cast zx s_408_37 -> i
        let s_408_38: i128 = (i128::try_from(s_408_37).unwrap());
        // C s_408_39: const #2s : i
        let s_408_39: i128 = 2;
        // C s_408_40: add s_408_39 s_408_38
        let s_408_40: i128 = (s_408_39 + s_408_38);
        // D s_408_41: bit-extract s_408_36 s_408_34 s_408_40
        let s_408_41: Bits = (Bits::new(
            ((s_408_36) >> (s_408_34)).value(),
            u16::try_from(s_408_40).unwrap(),
        ));
        // D s_408_42: cast reint s_408_41 -> u8
        let s_408_42: u8 = (s_408_41.value() as u8);
        // D s_408_43: cast zx s_408_42 -> bv
        let s_408_43: Bits = Bits::new(s_408_42 as u128, 3u16);
        // C s_408_44: const #3u : u8
        let s_408_44: u8 = 3;
        // C s_408_45: cast zx s_408_44 -> bv
        let s_408_45: Bits = Bits::new(s_408_44 as u128, 3u16);
        // D s_408_46: cmp-eq s_408_43 s_408_45
        let s_408_46: bool = ((s_408_43) == (s_408_45));
        // N s_408_47: branch s_408_46 b416 b409
        if s_408_46 {
            return block_416(state, tracer, fn_state);
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
        // C s_410_0: const #1s : i
        let s_410_0: i128 = 1;
        // D s_410_1: read-var CRm:u8
        let s_410_1: u8 = fn_state.CRm;
        // D s_410_2: cast zx s_410_1 -> bv
        let s_410_2: Bits = Bits::new(s_410_1 as u128, 4u16);
        // C s_410_3: const #1s : i64
        let s_410_3: i64 = 1;
        // C s_410_4: cast zx s_410_3 -> i
        let s_410_4: i128 = (i128::try_from(s_410_3).unwrap());
        // C s_410_5: const #2s : i
        let s_410_5: i128 = 2;
        // C s_410_6: add s_410_5 s_410_4
        let s_410_6: i128 = (s_410_5 + s_410_4);
        // D s_410_7: bit-extract s_410_2 s_410_0 s_410_6
        let s_410_7: Bits = (Bits::new(
            ((s_410_2) >> (s_410_0)).value(),
            u16::try_from(s_410_6).unwrap(),
        ));
        // D s_410_8: cast reint s_410_7 -> u8
        let s_410_8: u8 = (s_410_7.value() as u8);
        // D s_410_9: cast zx s_410_8 -> bv
        let s_410_9: Bits = Bits::new(s_410_8 as u128, 3u16);
        // C s_410_10: const #7u : u8
        let s_410_10: u8 = 7;
        // C s_410_11: cast zx s_410_10 -> bv
        let s_410_11: Bits = Bits::new(s_410_10 as u128, 3u16);
        // D s_410_12: cmp-eq s_410_9 s_410_11
        let s_410_12: bool = ((s_410_9) == (s_410_11));
        // N s_410_13: branch s_410_12 b413 b411
        if s_410_12 {
            return block_413(state, tracer, fn_state);
        } else {
            return block_411(state, tracer, fn_state);
        };
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_411_0: jump b412
        return block_412(state, tracer, fn_state);
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_412_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_413_0: const #() : ()
        let s_413_0: () = ();
        // S s_413_1: call AMCGCR_read(s_413_0)
        let s_413_1: ProductType700c18a878c5601b = AMCGCR_read(state, tracer, s_413_0);
        // S s_413_2: call _get_AMCGCR_Type_CG1NC(s_413_1)
        let s_413_2: u8 = u_get_AMCGCR_Type_CG1NC(state, tracer, s_413_1);
        // S s_413_3: cast zx s_413_2 -> bv
        let s_413_3: Bits = Bits::new(s_413_2 as u128, 8u16);
        // S s_413_4: cast zx s_413_3 -> i
        let s_413_4: i128 = (s_413_3.value() as i128);
        // S s_413_5: cast reint s_413_4 -> i64
        let s_413_5: i64 = (s_413_4 as i64);
        // D s_413_6: read-var nshadow#1013:i64
        let s_413_6: i64 = fn_state.nshadow_1013;
        // D s_413_7: cast zx s_413_6 -> i
        let s_413_7: i128 = (i128::try_from(s_413_6).unwrap());
        // S s_413_8: cast zx s_413_5 -> i
        let s_413_8: i128 = (i128::try_from(s_413_5).unwrap());
        // D s_413_9: cmp-ge s_413_7 s_413_8
        let s_413_9: bool = ((s_413_7) >= (s_413_8));
        // N s_413_10: branch s_413_9 b415 b414
        if s_413_9 {
            return block_415(state, tracer, fn_state);
        } else {
            return block_414(state, tracer, fn_state);
        };
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_414_0: jump b412
        return block_412(state, tracer, fn_state);
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_415_0: panic
        panic!("{:?}", ());
        // N s_415_1: return
        return;
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_416_0: const #() : ()
        let s_416_0: () = ();
        // S s_416_1: call AMCGCR_read(s_416_0)
        let s_416_1: ProductType700c18a878c5601b = AMCGCR_read(state, tracer, s_416_0);
        // S s_416_2: call _get_AMCGCR_Type_CG0NC(s_416_1)
        let s_416_2: u8 = u_get_AMCGCR_Type_CG0NC(state, tracer, s_416_1);
        // S s_416_3: cast zx s_416_2 -> bv
        let s_416_3: Bits = Bits::new(s_416_2 as u128, 8u16);
        // S s_416_4: cast zx s_416_3 -> i
        let s_416_4: i128 = (s_416_3.value() as i128);
        // S s_416_5: cast reint s_416_4 -> i64
        let s_416_5: i64 = (s_416_4 as i64);
        // D s_416_6: read-var nshadow#1013:i64
        let s_416_6: i64 = fn_state.nshadow_1013;
        // D s_416_7: cast zx s_416_6 -> i
        let s_416_7: i128 = (i128::try_from(s_416_6).unwrap());
        // S s_416_8: cast zx s_416_5 -> i
        let s_416_8: i128 = (i128::try_from(s_416_5).unwrap());
        // D s_416_9: cmp-ge s_416_7 s_416_8
        let s_416_9: bool = ((s_416_7) >= (s_416_8));
        // N s_416_10: branch s_416_9 b418 b417
        if s_416_9 {
            return block_418(state, tracer, fn_state);
        } else {
            return block_417(state, tracer, fn_state);
        };
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_417_0: jump b410
        return block_410(state, tracer, fn_state);
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_418_0: panic
        panic!("{:?}", ());
        // N s_418_1: return
        return;
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_419_0: read-var CRn:u8
        let s_419_0: u8 = fn_state.CRn;
        // D s_419_1: cast zx s_419_0 -> bv
        let s_419_1: Bits = Bits::new(s_419_0 as u128, 4u16);
        // C s_419_2: const #13u : u8
        let s_419_2: u8 = 13;
        // C s_419_3: cast zx s_419_2 -> bv
        let s_419_3: Bits = Bits::new(s_419_2 as u128, 4u16);
        // D s_419_4: cmp-eq s_419_1 s_419_3
        let s_419_4: bool = ((s_419_1) == (s_419_3));
        // D s_419_5: write-var gs#141934 <= s_419_4
        fn_state.gs_141934 = s_419_4;
        // N s_419_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_420_0: read-var opc1:u8
        let s_420_0: u8 = fn_state.opc1;
        // D s_420_1: cast zx s_420_0 -> bv
        let s_420_1: Bits = Bits::new(s_420_0 as u128, 3u16);
        // C s_420_2: const #0u : u8
        let s_420_2: u8 = 0;
        // C s_420_3: cast zx s_420_2 -> bv
        let s_420_3: Bits = Bits::new(s_420_2 as u128, 3u16);
        // D s_420_4: cmp-eq s_420_1 s_420_3
        let s_420_4: bool = ((s_420_1) == (s_420_3));
        // D s_420_5: write-var gs#141933 <= s_420_4
        fn_state.gs_141933 = s_420_4;
        // N s_420_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_421_0: const #0u : u8
        let s_421_0: u8 = 0;
        // D s_421_1: write-var opc2 <= s_421_0
        fn_state.opc2 = s_421_0;
        // N s_421_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_422_0: read-var CRm:u8
        let s_422_0: u8 = fn_state.CRm;
        // D s_422_1: cast zx s_422_0 -> bv
        let s_422_1: Bits = Bits::new(s_422_0 as u128, 4u16);
        // C s_422_2: const #0u : u8
        let s_422_2: u8 = 0;
        // C s_422_3: cast zx s_422_2 -> bv
        let s_422_3: Bits = Bits::new(s_422_2 as u128, 4u16);
        // D s_422_4: cmp-eq s_422_1 s_422_3
        let s_422_4: bool = ((s_422_1) == (s_422_3));
        // D s_422_5: write-var gs#141931 <= s_422_4
        fn_state.gs_141931 = s_422_4;
        // N s_422_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_423_0: read-var CRn:u8
        let s_423_0: u8 = fn_state.CRn;
        // D s_423_1: cast zx s_423_0 -> bv
        let s_423_1: Bits = Bits::new(s_423_0 as u128, 4u16);
        // C s_423_2: const #0u : u8
        let s_423_2: u8 = 0;
        // C s_423_3: cast zx s_423_2 -> bv
        let s_423_3: Bits = Bits::new(s_423_2 as u128, 4u16);
        // D s_423_4: cmp-eq s_423_1 s_423_3
        let s_423_4: bool = ((s_423_1) == (s_423_3));
        // D s_423_5: write-var gs#141930 <= s_423_4
        fn_state.gs_141930 = s_423_4;
        // N s_423_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_424_0: read-var opc2:u8
        let s_424_0: u8 = fn_state.opc2;
        // D s_424_1: cast zx s_424_0 -> bv
        let s_424_1: Bits = Bits::new(s_424_0 as u128, 3u16);
        // C s_424_2: const #4u : u8
        let s_424_2: u8 = 4;
        // C s_424_3: cast zx s_424_2 -> bv
        let s_424_3: Bits = Bits::new(s_424_2 as u128, 3u16);
        // D s_424_4: cmp-eq s_424_1 s_424_3
        let s_424_4: bool = ((s_424_1) == (s_424_3));
        // N s_424_5: branch s_424_4 b427 b425
        if s_424_4 {
            return block_427(state, tracer, fn_state);
        } else {
            return block_425(state, tracer, fn_state);
        };
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_425_0: read-var opc2:u8
        let s_425_0: u8 = fn_state.opc2;
        // D s_425_1: cast zx s_425_0 -> bv
        let s_425_1: Bits = Bits::new(s_425_0 as u128, 3u16);
        // C s_425_2: const #7u : u8
        let s_425_2: u8 = 7;
        // C s_425_3: cast zx s_425_2 -> bv
        let s_425_3: Bits = Bits::new(s_425_2 as u128, 3u16);
        // D s_425_4: cmp-eq s_425_1 s_425_3
        let s_425_4: bool = ((s_425_1) == (s_425_3));
        // D s_425_5: write-var gs#141928 <= s_425_4
        fn_state.gs_141928 = s_425_4;
        // N s_425_6: jump b426
        return block_426(state, tracer, fn_state);
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_426_0: read-var gs#141928:u8
        let s_426_0: bool = fn_state.gs_141928;
        // D s_426_1: write-var gs#141929 <= s_426_0
        fn_state.gs_141929 = s_426_0;
        // N s_426_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_427_0: const #1u : u8
        let s_427_0: bool = true;
        // D s_427_1: write-var gs#141928 <= s_427_0
        fn_state.gs_141928 = s_427_0;
        // N s_427_2: jump b426
        return block_426(state, tracer, fn_state);
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_428_0: read-var opc1:u8
        let s_428_0: u8 = fn_state.opc1;
        // D s_428_1: cast zx s_428_0 -> bv
        let s_428_1: Bits = Bits::new(s_428_0 as u128, 3u16);
        // C s_428_2: const #0u : u8
        let s_428_2: u8 = 0;
        // C s_428_3: cast zx s_428_2 -> bv
        let s_428_3: Bits = Bits::new(s_428_2 as u128, 3u16);
        // D s_428_4: cmp-eq s_428_1 s_428_3
        let s_428_4: bool = ((s_428_1) == (s_428_3));
        // D s_428_5: write-var gs#141927 <= s_428_4
        fn_state.gs_141927 = s_428_4;
        // N s_428_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
