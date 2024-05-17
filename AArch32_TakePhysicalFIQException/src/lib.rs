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
use IsInHost::*;
use u_get_SCR_Type_FIQ::*;
use HCR_read::*;
use AArch64_TakePhysicalFIQException::*;
use AArch32_EnterMonitorMode::*;
use u_get_SCR_EL3_Type_FIQ::*;
use u_get_HCR_Type_TGE::*;
use AArch32_EnterHypMode::*;
use ThisInstrAddr::*;
use u_get_HCR_EL2_Type_FMO::*;
use ELUsingAArch32::*;
use ExceptionSyndrome::*;
use u_get_HCR_Type_FMO::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use AArch32_EnterMode::*;
use common::*;
pub fn AArch32_TakePhysicalFIQException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_32062: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_monitor: bool,
        route_to_hyp: bool,
        gs_32075: bool,
        gs_32065: bool,
        gs_32076: bool,
        preferred_exception_return: u32,
        gs_32064: bool,
        route_to_aarch64: bool,
        gs_32072: bool,
        gs_32083: bool,
        gs_32066: bool,
        gs_32074: bool,
        vect_offset: i64,
        gs_32067: bool,
        gs_32071: bool,
        gs_32063: bool,
        gs_32073: bool,
        gs_32077: bool,
        gs_32062: (),
    }
    let fn_state = FunctionState {
        gs_32062,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b52 b1
        if s_0_6 {
            return block_52(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#32063 <= s_1_0
        fn_state.gs_32063 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#32063:u8
        let s_2_0: bool = fn_state.gs_32063;
        // D s_2_1: write-var route_to_aarch64 <= s_2_0
        fn_state.route_to_aarch64 = s_2_0;
        // D s_2_2: read-var route_to_aarch64:u8
        let s_2_2: bool = fn_state.route_to_aarch64;
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b51 b3
        if s_2_3 {
            return block_51(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#32064 <= s_3_0
        fn_state.gs_32064 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#32064:u8
        let s_4_0: bool = fn_state.gs_32064;
        // N s_4_1: branch s_4_0 b50 b5
        if s_4_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#32065 <= s_5_0
        fn_state.gs_32065 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#32065:u8
        let s_6_0: bool = fn_state.gs_32065;
        // N s_6_1: branch s_6_0 b43 b7
        if s_6_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_8_0: read-var route_to_aarch64:u8
        let s_8_0: bool = fn_state.route_to_aarch64;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b42 b9
        if s_8_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#32066 <= s_9_0
        fn_state.gs_32066 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#32066:u8
        let s_10_0: bool = fn_state.gs_32066;
        // N s_10_1: branch s_10_0 b41 b11
        if s_10_0 {
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
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#32067 <= s_11_0
        fn_state.gs_32067 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#32067:u8
        let s_12_0: bool = fn_state.gs_32067;
        // N s_12_1: branch s_12_0 b40 b13
        if s_12_0 {
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
        // D s_14_0: read-var route_to_aarch64:u8
        let s_14_0: bool = fn_state.route_to_aarch64;
        // N s_14_1: branch s_14_0 b39 b15
        if s_14_0 {
            return block_39(state, tracer, fn_state);
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
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // N s_16_4: branch s_16_3 b38 b17
        if s_16_3 {
            return block_38(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#32073 <= s_17_0
        fn_state.gs_32073 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#32073:u8
        let s_18_0: bool = fn_state.gs_32073;
        // D s_18_1: write-var route_to_monitor <= s_18_0
        fn_state.route_to_monitor = s_18_0;
        // C s_18_2: const #16975u : u32
        let s_18_2: u32 = 16975;
        // D s_18_3: read-reg s_18_2:u8
        let s_18_3: u8 = {
            let value = state.read_register::<u8>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 2u16);
        // C s_18_5: const #448u : u32
        let s_18_5: u32 = 448;
        // D s_18_6: read-reg s_18_5:u8
        let s_18_6: u8 = {
            let value = state.read_register::<u8>(s_18_5 as isize);
            tracer.read_register(s_18_5 as isize, value);
            value
        };
        // D s_18_7: cast zx s_18_6 -> bv
        let s_18_7: Bits = Bits::new(s_18_6 as u128, 2u16);
        // D s_18_8: cmp-eq s_18_4 s_18_7
        let s_18_8: bool = ((s_18_4) == (s_18_7));
        // N s_18_9: branch s_18_8 b37 b19
        if s_18_8 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16975u : u32
        let s_19_0: u32 = 16975;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 2u16);
        // C s_19_3: const #440u : u32
        let s_19_3: u32 = 440;
        // D s_19_4: read-reg s_19_3:u8
        let s_19_4: u8 = {
            let value = state.read_register::<u8>(s_19_3 as isize);
            tracer.read_register(s_19_3 as isize, value);
            value
        };
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 2u16);
        // D s_19_6: cmp-eq s_19_2 s_19_5
        let s_19_6: bool = ((s_19_2) == (s_19_5));
        // D s_19_7: write-var gs#32074 <= s_19_6
        fn_state.gs_32074 = s_19_6;
        // N s_19_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#32074:u8
        let s_20_0: bool = fn_state.gs_32074;
        // N s_20_1: branch s_20_0 b36 b21
        if s_20_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#32075 <= s_21_0
        fn_state.gs_32075 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#32075:u8
        let s_22_0: bool = fn_state.gs_32075;
        // N s_22_1: branch s_22_0 b32 b23
        if s_22_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#32077 <= s_23_0
        fn_state.gs_32077 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#32077:u8
        let s_24_0: bool = fn_state.gs_32077;
        // D s_24_1: write-var route_to_hyp <= s_24_0
        fn_state.route_to_hyp = s_24_0;
        // C s_24_2: const #32s : i64
        let s_24_2: i64 = 32;
        // C s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // S s_24_4: call ThisInstrAddr(s_24_3)
        let s_24_4: Bits = ThisInstrAddr(state, tracer, s_24_3);
        // S s_24_5: cast reint s_24_4 -> u32
        let s_24_5: u32 = (s_24_4.value() as u32);
        // D s_24_6: write-var preferred_exception_return <= s_24_5
        fn_state.preferred_exception_return = s_24_5;
        // C s_24_7: const #28u : u8
        let s_24_7: u8 = 28;
        // C s_24_8: cast zx s_24_7 -> bv
        let s_24_8: Bits = Bits::new(s_24_7 as u128, 8u16);
        // C s_24_9: cast zx s_24_8 -> i
        let s_24_9: i128 = (s_24_8.value() as i128);
        // C s_24_10: cast reint s_24_9 -> i64
        let s_24_10: i64 = (s_24_9 as i64);
        // D s_24_11: write-var vect_offset <= s_24_10
        fn_state.vect_offset = s_24_10;
        // D s_24_12: read-var route_to_monitor:u8
        let s_24_12: bool = fn_state.route_to_monitor;
        // N s_24_13: branch s_24_12 b31 b25
        if s_24_12 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #16975u : u32
        let s_25_0: u32 = 16975;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 2u16);
        // C s_25_3: const #432u : u32
        let s_25_3: u32 = 432;
        // D s_25_4: read-reg s_25_3:u8
        let s_25_4: u8 = {
            let value = state.read_register::<u8>(s_25_3 as isize);
            tracer.read_register(s_25_3 as isize, value);
            value
        };
        // D s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 2u16);
        // D s_25_6: cmp-eq s_25_2 s_25_5
        let s_25_6: bool = ((s_25_2) == (s_25_5));
        // N s_25_7: branch s_25_6 b30 b26
        if s_25_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var route_to_hyp:u8
        let s_26_0: bool = fn_state.route_to_hyp;
        // D s_26_1: write-var gs#32083 <= s_26_0
        fn_state.gs_32083 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#32083:u8
        let s_27_0: bool = fn_state.gs_32083;
        // N s_27_1: branch s_27_0 b29 b28
        if s_27_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #4s : i64
        let s_28_0: i64 = 4;
        // C s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: read-var vect_offset:i64
        let s_28_2: i64 = fn_state.vect_offset;
        // D s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // C s_28_4: const #360u : u32
        let s_28_4: u32 = 360;
        // D s_28_5: read-reg s_28_4:u8
        let s_28_5: u8 = {
            let value = state.read_register::<u8>(s_28_4 as isize);
            tracer.read_register(s_28_4 as isize, value);
            value
        };
        // D s_28_6: read-var preferred_exception_return:u32
        let s_28_6: u32 = fn_state.preferred_exception_return;
        // D s_28_7: call AArch32_EnterMode(s_28_5, s_28_6, s_28_1, s_28_3)
        let s_28_7: () = AArch32_EnterMode(
            state,
            tracer,
            s_28_5,
            s_28_6,
            s_28_1,
            s_28_3,
        );
        // N s_28_8: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #40u : u32
        let s_29_0: u32 = 40;
        // S s_29_1: call ExceptionSyndrome(s_29_0)
        let s_29_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_29_0,
        );
        // D s_29_2: read-var vect_offset:i64
        let s_29_2: i64 = fn_state.vect_offset;
        // D s_29_3: cast zx s_29_2 -> i
        let s_29_3: i128 = (i128::try_from(s_29_2).unwrap());
        // D s_29_4: read-var preferred_exception_return:u32
        let s_29_4: u32 = fn_state.preferred_exception_return;
        // D s_29_5: call AArch32_EnterHypMode(s_29_1, s_29_4, s_29_3)
        let s_29_5: () = AArch32_EnterHypMode(state, tracer, s_29_1, s_29_4, s_29_3);
        // N s_29_6: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#32083 <= s_30_0
        fn_state.gs_32083 = s_30_0;
        // N s_30_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #4s : i64
        let s_31_0: i64 = 4;
        // C s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: read-var vect_offset:i64
        let s_31_2: i64 = fn_state.vect_offset;
        // D s_31_3: cast zx s_31_2 -> i
        let s_31_3: i128 = (i128::try_from(s_31_2).unwrap());
        // D s_31_4: read-var preferred_exception_return:u32
        let s_31_4: u32 = fn_state.preferred_exception_return;
        // D s_31_5: call AArch32_EnterMonitorMode(s_31_4, s_31_1, s_31_3)
        let s_31_5: () = AArch32_EnterMonitorMode(state, tracer, s_31_4, s_31_1, s_31_3);
        // N s_31_6: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HCR_read(s_32_0)
        let s_32_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_32_0);
        // S s_32_2: call _get_HCR_Type_TGE(s_32_1)
        let s_32_2: bool = u_get_HCR_Type_TGE(state, tracer, s_32_1);
        // S s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // C s_32_4: const #1u : u8
        let s_32_4: bool = true;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 1u16);
        // S s_32_6: cmp-eq s_32_3 s_32_5
        let s_32_6: bool = ((s_32_3) == (s_32_5));
        // N s_32_7: branch s_32_6 b35 b33
        if s_32_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call HCR_read(s_33_0)
        let s_33_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_33_0);
        // S s_33_2: call _get_HCR_Type_FMO(s_33_1)
        let s_33_2: bool = u_get_HCR_Type_FMO(state, tracer, s_33_1);
        // S s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #1u : u8
        let s_33_4: bool = true;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // S s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // D s_33_7: write-var gs#32076 <= s_33_6
        fn_state.gs_32076 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#32076:u8
        let s_34_0: bool = fn_state.gs_32076;
        // D s_34_1: write-var gs#32077 <= s_34_0
        fn_state.gs_32077 = s_34_0;
        // N s_34_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#32076 <= s_35_0
        fn_state.gs_32076 = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EL2Enabled(s_36_0)
        let s_36_1: bool = EL2Enabled(state, tracer, s_36_0);
        // D s_36_2: write-var gs#32075 <= s_36_1
        fn_state.gs_32075 = s_36_1;
        // N s_36_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#32074 <= s_37_0
        fn_state.gs_32074 = s_37_0;
        // N s_37_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #20920u : u32
        let s_38_0: u32 = 20920;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_SCR_Type_FIQ(s_38_1)
        let s_38_2: bool = u_get_SCR_Type_FIQ(state, tracer, s_38_1);
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#32073 <= s_38_6
        fn_state.gs_32073 = s_38_6;
        // N s_38_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call AArch64_TakePhysicalFIQException(s_39_0)
        let s_39_1: () = AArch64_TakePhysicalFIQException(state, tracer, s_39_0);
        // N s_39_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #90704u : u32
        let s_40_0: u32 = 90704;
        // D s_40_1: read-reg s_40_0:struct
        let s_40_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call _get_SCR_EL3_Type_FIQ(s_40_1)
        let s_40_2: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_40_1);
        // D s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // C s_40_4: const #1u : u8
        let s_40_4: bool = true;
        // C s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 1u16);
        // D s_40_6: cmp-eq s_40_3 s_40_5
        let s_40_6: bool = ((s_40_3) == (s_40_5));
        // D s_40_7: write-var route_to_aarch64 <= s_40_6
        fn_state.route_to_aarch64 = s_40_6;
        // N s_40_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #424u : u32
        let s_41_0: u32 = 424;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call ELUsingAArch32(s_41_1)
        let s_41_2: bool = ELUsingAArch32(state, tracer, s_41_1);
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // D s_41_4: write-var gs#32067 <= s_41_3
        fn_state.gs_32067 = s_41_3;
        // N s_41_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #424u : u32
        let s_42_0: u32 = 424;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // C s_42_2: const #2u : u8
        let s_42_2: u8 = 2;
        // D s_42_3: cmp-lt s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) < (s_42_2));
        // D s_42_4: write-var gs#32066 <= s_42_3
        fn_state.gs_32066 = s_42_3;
        // N s_42_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #102552u : u32
        let s_43_0: u32 = 102552;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_HCR_EL2_Type_TGE(s_43_1)
        let s_43_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_43_1);
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // D s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // N s_43_7: branch s_43_6 b49 b44
        if s_43_6 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #102552u : u32
        let s_44_0: u32 = 102552;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_HCR_EL2_Type_FMO(s_44_1)
        let s_44_2: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_44_1);
        // D s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // C s_44_4: const #1u : u8
        let s_44_4: bool = true;
        // C s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 1u16);
        // D s_44_6: cmp-eq s_44_3 s_44_5
        let s_44_6: bool = ((s_44_3) == (s_44_5));
        // N s_44_7: branch s_44_6 b48 b45
        if s_44_6 {
            return block_48(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#32071 <= s_45_0
        fn_state.gs_32071 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#32071:u8
        let s_46_0: bool = fn_state.gs_32071;
        // D s_46_1: write-var gs#32072 <= s_46_0
        fn_state.gs_32072 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#32072:u8
        let s_47_0: bool = fn_state.gs_32072;
        // D s_47_1: write-var route_to_aarch64 <= s_47_0
        fn_state.route_to_aarch64 = s_47_0;
        // N s_47_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call IsInHost(s_48_0)
        let s_48_1: bool = IsInHost(state, tracer, s_48_0);
        // S s_48_2: not s_48_1
        let s_48_2: bool = !s_48_1;
        // D s_48_3: write-var gs#32071 <= s_48_2
        fn_state.gs_32071 = s_48_2;
        // N s_48_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#32072 <= s_49_0
        fn_state.gs_32072 = s_49_0;
        // N s_49_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #432u : u32
        let s_50_0: u32 = 432;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call ELUsingAArch32(s_50_1)
        let s_50_2: bool = ELUsingAArch32(state, tracer, s_50_1);
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // D s_50_4: write-var gs#32065 <= s_50_3
        fn_state.gs_32065 = s_50_3;
        // N s_50_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EL2Enabled(s_51_0)
        let s_51_1: bool = EL2Enabled(state, tracer, s_51_0);
        // D s_51_2: write-var gs#32064 <= s_51_1
        fn_state.gs_32064 = s_51_1;
        // N s_51_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #440u : u32
        let s_52_0: u32 = 440;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call ELUsingAArch32(s_52_1)
        let s_52_2: bool = ELUsingAArch32(state, tracer, s_52_1);
        // D s_52_3: not s_52_2
        let s_52_3: bool = !s_52_2;
        // D s_52_4: write-var gs#32063 <= s_52_3
        fn_state.gs_32063 = s_52_3;
        // N s_52_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
