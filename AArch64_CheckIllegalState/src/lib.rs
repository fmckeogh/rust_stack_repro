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
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_CheckIllegalState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24658: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2shadow_469: bool,
        gs_24660: bool,
        vect_offsetshadow_470: i64,
        preferred_exception_return: u64,
        gs_24661: bool,
        exceptshadow_471: ProductTypeb7f99f96751e17c4,
        gs_24658: (),
    }
    let fn_state = FunctionState {
        gs_24658,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16980u : u32
        let s_0_0: u32 = 16980;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 1u16);
        // C s_0_3: const #1u : u8
        let s_0_3: bool = true;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 1u16);
        // D s_0_5: cmp-eq s_0_2 s_0_4
        let s_0_5: bool = ((s_0_2) == (s_0_4));
        // N s_0_6: branch s_0_5 b2 b1
        if s_0_5 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16975u : u32
        let s_2_0: u32 = 16975;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #448u : u32
        let s_2_3: u32 = 448;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // N s_2_7: branch s_2_6 b12 b3
        if s_2_6 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#24660 <= s_3_0
        fn_state.gs_24660 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#24660:u8
        let s_4_0: bool = fn_state.gs_24660;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#24661 <= s_5_0
        fn_state.gs_24661 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#24661:u8
        let s_6_0: bool = fn_state.gs_24661;
        // D s_6_1: write-var route_to_el2shadow#469 <= s_6_0
        fn_state.route_to_el2shadow_469 = s_6_0;
        // C s_6_2: const #64s : i64
        let s_6_2: i64 = 64;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // S s_6_4: call ThisInstrAddr(s_6_3)
        let s_6_4: Bits = ThisInstrAddr(state, tracer, s_6_3);
        // S s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // D s_6_6: write-var preferred_exception_return <= s_6_5
        fn_state.preferred_exception_return = s_6_5;
        // C s_6_7: const #0u : u8
        let s_6_7: u8 = 0;
        // C s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 4u16);
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (s_6_8.value() as i128);
        // C s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: write-var vect_offsetshadow#470 <= s_6_10
        fn_state.vect_offsetshadow_470 = s_6_10;
        // C s_6_12: const #11u : u32
        let s_6_12: u32 = 11;
        // S s_6_13: call ExceptionSyndrome(s_6_12)
        let s_6_13: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_6_12,
        );
        // D s_6_14: write-var exceptshadow#471 <= s_6_13
        fn_state.exceptshadow_471 = s_6_13;
        // C s_6_15: const #16975u : u32
        let s_6_15: u32 = 16975;
        // D s_6_16: read-reg s_6_15:u8
        let s_6_16: u8 = {
            let value = state.read_register::<u8>(s_6_15 as isize);
            tracer.read_register(s_6_15 as isize, value);
            value
        };
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 2u16);
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (s_6_17.value() as i128);
        // D s_6_19: cast reint s_6_18 -> i64
        let s_6_19: i64 = (s_6_18 as i64);
        // C s_6_20: const #440u : u32
        let s_6_20: u32 = 440;
        // D s_6_21: read-reg s_6_20:u8
        let s_6_21: u8 = {
            let value = state.read_register::<u8>(s_6_20 as isize);
            tracer.read_register(s_6_20 as isize, value);
            value
        };
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 2u16);
        // D s_6_23: cast zx s_6_22 -> i
        let s_6_23: i128 = (s_6_22.value() as i128);
        // D s_6_24: cast reint s_6_23 -> i64
        let s_6_24: i64 = (s_6_23 as i64);
        // D s_6_25: cast zx s_6_19 -> i
        let s_6_25: i128 = (i128::try_from(s_6_19).unwrap());
        // D s_6_26: cast zx s_6_24 -> i
        let s_6_26: i128 = (i128::try_from(s_6_24).unwrap());
        // D s_6_27: cmp-gt s_6_25 s_6_26
        let s_6_27: bool = ((s_6_25) > (s_6_26));
        // N s_6_28: branch s_6_27 b10 b7
        if s_6_27 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var route_to_el2shadow#469:u8
        let s_7_0: bool = fn_state.route_to_el2shadow_469;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var vect_offsetshadow#470:i64
        let s_8_0: i64 = fn_state.vect_offsetshadow_470;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // C s_8_2: const #440u : u32
        let s_8_2: u32 = 440;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: read-var exceptshadow#471:struct
        let s_8_4: ProductTypeb7f99f96751e17c4 = fn_state.exceptshadow_471;
        // D s_8_5: read-var preferred_exception_return:u64
        let s_8_5: u64 = fn_state.preferred_exception_return;
        // D s_8_6: call AArch64_TakeException(s_8_3, s_8_4, s_8_5, s_8_1)
        let s_8_6: () = AArch64_TakeException(state, tracer, s_8_3, s_8_4, s_8_5, s_8_1);
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var vect_offsetshadow#470:i64
        let s_9_0: i64 = fn_state.vect_offsetshadow_470;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // C s_9_2: const #432u : u32
        let s_9_2: u32 = 432;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: read-var exceptshadow#471:struct
        let s_9_4: ProductTypeb7f99f96751e17c4 = fn_state.exceptshadow_471;
        // D s_9_5: read-var preferred_exception_return:u64
        let s_9_5: u64 = fn_state.preferred_exception_return;
        // D s_9_6: call AArch64_TakeException(s_9_3, s_9_4, s_9_5, s_9_1)
        let s_9_6: () = AArch64_TakeException(state, tracer, s_9_3, s_9_4, s_9_5, s_9_1);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16975u : u32
        let s_10_0: u32 = 16975;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: read-var vect_offsetshadow#470:i64
        let s_10_2: i64 = fn_state.vect_offsetshadow_470;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var exceptshadow#471:struct
        let s_10_4: ProductTypeb7f99f96751e17c4 = fn_state.exceptshadow_471;
        // D s_10_5: read-var preferred_exception_return:u64
        let s_10_5: u64 = fn_state.preferred_exception_return;
        // D s_10_6: call AArch64_TakeException(s_10_1, s_10_4, s_10_5, s_10_3)
        let s_10_6: () = AArch64_TakeException(
            state,
            tracer,
            s_10_1,
            s_10_4,
            s_10_5,
            s_10_3,
        );
        // N s_10_7: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #102552u : u32
        let s_11_0: u32 = 102552;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_HCR_EL2_Type_TGE(s_11_1)
        let s_11_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // D s_11_7: write-var gs#24661 <= s_11_6
        fn_state.gs_24661 = s_11_6;
        // N s_11_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // D s_12_2: write-var gs#24660 <= s_12_1
        fn_state.gs_24660 = s_12_1;
        // N s_12_3: jump b4
        return block_4(state, tracer, fn_state);
    }
}
