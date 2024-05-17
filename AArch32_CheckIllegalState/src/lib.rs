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
use ThisInstrAddr::*;
use HCR_read::*;
use ExceptionSyndrome::*;
use AArch32_TakeUndefInstrException::*;
use AArch32_GeneralExceptionsToAArch64::*;
use EL2Enabled::*;
use AArch64_CheckIllegalState::*;
use u_get_HCR_Type_TGE::*;
use AArch32_EnterHypMode::*;
use common::*;
pub fn AArch32_CheckIllegalState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31844: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31849: bool,
        vect_offsetshadow_595: i64,
        preferred_exception_return: u32,
        gs_31845: bool,
        exceptshadow_596: ProductTypeb7f99f96751e17c4,
        gs_31846: bool,
        route_to_hypshadow_594: bool,
        gs_31844: (),
    }
    let fn_state = FunctionState {
        gs_31844,
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
        // S s_0_1: call AArch32_GeneralExceptionsToAArch64(s_0_0)
        let s_0_1: bool = AArch32_GeneralExceptionsToAArch64(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b17 b1
        if s_0_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16980u : u32
        let s_1_0: u32 = 16980;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: bool = {
            let value = state.read_register::<bool>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 1u16);
        // C s_1_3: const #1u : u8
        let s_1_3: bool = true;
        // C s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 1u16);
        // D s_1_5: cmp-eq s_1_2 s_1_4
        let s_1_5: bool = ((s_1_2) == (s_1_4));
        // N s_1_6: branch s_1_5 b3 b2
        if s_1_5 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #448u : u32
        let s_3_3: u32 = 448;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b16 b4
        if s_3_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#31845 <= s_4_0
        fn_state.gs_31845 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#31845:u8
        let s_5_0: bool = fn_state.gs_31845;
        // N s_5_1: branch s_5_0 b15 b6
        if s_5_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#31846 <= s_6_0
        fn_state.gs_31846 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#31846:u8
        let s_7_0: bool = fn_state.gs_31846;
        // D s_7_1: write-var route_to_hypshadow#594 <= s_7_0
        fn_state.route_to_hypshadow_594 = s_7_0;
        // C s_7_2: const #32s : i64
        let s_7_2: i64 = 32;
        // C s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // S s_7_4: call ThisInstrAddr(s_7_3)
        let s_7_4: Bits = ThisInstrAddr(state, tracer, s_7_3);
        // S s_7_5: cast reint s_7_4 -> u32
        let s_7_5: u32 = (s_7_4.value() as u32);
        // D s_7_6: write-var preferred_exception_return <= s_7_5
        fn_state.preferred_exception_return = s_7_5;
        // C s_7_7: const #4u : u8
        let s_7_7: u8 = 4;
        // C s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 8u16);
        // C s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (s_7_8.value() as i128);
        // C s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: write-var vect_offsetshadow#595 <= s_7_10
        fn_state.vect_offsetshadow_595 = s_7_10;
        // C s_7_12: const #16975u : u32
        let s_7_12: u32 = 16975;
        // D s_7_13: read-reg s_7_12:u8
        let s_7_13: u8 = {
            let value = state.read_register::<u8>(s_7_12 as isize);
            tracer.read_register(s_7_12 as isize, value);
            value
        };
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 2u16);
        // C s_7_15: const #432u : u32
        let s_7_15: u32 = 432;
        // D s_7_16: read-reg s_7_15:u8
        let s_7_16: u8 = {
            let value = state.read_register::<u8>(s_7_15 as isize);
            tracer.read_register(s_7_15 as isize, value);
            value
        };
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 2u16);
        // D s_7_18: cmp-eq s_7_14 s_7_17
        let s_7_18: bool = ((s_7_14) == (s_7_17));
        // N s_7_19: branch s_7_18 b14 b8
        if s_7_18 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var route_to_hypshadow#594:u8
        let s_8_0: bool = fn_state.route_to_hypshadow_594;
        // D s_8_1: write-var gs#31849 <= s_8_0
        fn_state.gs_31849 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#31849:u8
        let s_9_0: bool = fn_state.gs_31849;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call AArch32_TakeUndefInstrException(s_10_0)
        let s_10_1: () = AArch32_TakeUndefInstrException(state, tracer, s_10_0);
        // N s_10_2: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #11u : u32
        let s_11_0: u32 = 11;
        // S s_11_1: call ExceptionSyndrome(s_11_0)
        let s_11_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_11_0,
        );
        // D s_11_2: write-var exceptshadow#596 <= s_11_1
        fn_state.exceptshadow_596 = s_11_1;
        // C s_11_3: const #16975u : u32
        let s_11_3: u32 = 16975;
        // D s_11_4: read-reg s_11_3:u8
        let s_11_4: u8 = {
            let value = state.read_register::<u8>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 2u16);
        // C s_11_6: const #432u : u32
        let s_11_6: u32 = 432;
        // D s_11_7: read-reg s_11_6:u8
        let s_11_7: u8 = {
            let value = state.read_register::<u8>(s_11_6 as isize);
            tracer.read_register(s_11_6 as isize, value);
            value
        };
        // D s_11_8: cast zx s_11_7 -> bv
        let s_11_8: Bits = Bits::new(s_11_7 as u128, 2u16);
        // D s_11_9: cmp-eq s_11_5 s_11_8
        let s_11_9: bool = ((s_11_5) == (s_11_8));
        // N s_11_10: branch s_11_9 b13 b12
        if s_11_9 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #20u : u8
        let s_12_0: u8 = 20;
        // C s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 8u16);
        // C s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (s_12_1.value() as i128);
        // C s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: read-var exceptshadow#596:struct
        let s_12_5: ProductTypeb7f99f96751e17c4 = fn_state.exceptshadow_596;
        // D s_12_6: read-var preferred_exception_return:u32
        let s_12_6: u32 = fn_state.preferred_exception_return;
        // D s_12_7: call AArch32_EnterHypMode(s_12_5, s_12_6, s_12_4)
        let s_12_7: () = AArch32_EnterHypMode(state, tracer, s_12_5, s_12_6, s_12_4);
        // N s_12_8: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var vect_offsetshadow#595:i64
        let s_13_0: i64 = fn_state.vect_offsetshadow_595;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var exceptshadow#596:struct
        let s_13_2: ProductTypeb7f99f96751e17c4 = fn_state.exceptshadow_596;
        // D s_13_3: read-var preferred_exception_return:u32
        let s_13_3: u32 = fn_state.preferred_exception_return;
        // D s_13_4: call AArch32_EnterHypMode(s_13_2, s_13_3, s_13_1)
        let s_13_4: () = AArch32_EnterHypMode(state, tracer, s_13_2, s_13_3, s_13_1);
        // N s_13_5: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#31849 <= s_14_0
        fn_state.gs_31849 = s_14_0;
        // N s_14_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HCR_read(s_15_0)
        let s_15_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_15_0);
        // S s_15_2: call _get_HCR_Type_TGE(s_15_1)
        let s_15_2: bool = u_get_HCR_Type_TGE(state, tracer, s_15_1);
        // S s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #1u : u8
        let s_15_4: bool = true;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // S s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: write-var gs#31846 <= s_15_6
        fn_state.gs_31846 = s_15_6;
        // N s_15_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call EL2Enabled(s_16_0)
        let s_16_1: bool = EL2Enabled(state, tracer, s_16_0);
        // D s_16_2: write-var gs#31845 <= s_16_1
        fn_state.gs_31845 = s_16_1;
        // N s_16_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call AArch64_CheckIllegalState(s_17_0)
        let s_17_1: () = AArch64_CheckIllegalState(state, tracer, s_17_0);
        // N s_17_2: return
        return;
    }
}
