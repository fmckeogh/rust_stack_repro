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
use ConditionSyndrome::*;
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_AdvSIMDFPAccessTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        vect_offset: i64,
        gs_21108: bool,
        preferred_exception_return: u64,
        gs_21109: bool,
        target_el: u8,
    }
    let fn_state = FunctionState {
        target_el,
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
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call ThisInstrAddr(s_0_1)
        let s_0_2: Bits = ThisInstrAddr(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // D s_0_4: write-var preferred_exception_return <= s_0_3
        fn_state.preferred_exception_return = s_0_3;
        // C s_0_5: const #0u : u8
        let s_0_5: u8 = 0;
        // C s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 4u16);
        // C s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // C s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var vect_offset <= s_0_8
        fn_state.vect_offset = s_0_8;
        // D s_0_10: read-var target_el:u8
        let s_0_10: u8 = fn_state.target_el;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // C s_0_12: const #440u : u32
        let s_0_12: u32 = 440;
        // D s_0_13: read-reg s_0_12:u8
        let s_0_13: u8 = {
            let value = state.read_register::<u8>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // D s_0_14: cast zx s_0_13 -> bv
        let s_0_14: Bits = Bits::new(s_0_13 as u128, 2u16);
        // D s_0_15: cmp-eq s_0_11 s_0_14
        let s_0_15: bool = ((s_0_11) == (s_0_14));
        // N s_0_16: branch s_0_15 b8 b1
        if s_0_15 {
            return block_8(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#21108 <= s_1_0
        fn_state.gs_21108 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#21108:u8
        let s_2_0: bool = fn_state.gs_21108;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#21109 <= s_3_0
        fn_state.gs_21109 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#21109:u8
        let s_4_0: bool = fn_state.gs_21109;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #7u : u32
        let s_5_0: u32 = 7;
        // S s_5_1: call ExceptionSyndrome(s_5_0)
        let s_5_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_5_0);
        // C s_5_2: const #() : ()
        let s_5_2: () = ();
        // S s_5_3: call ConditionSyndrome(s_5_2)
        let s_5_3: u8 = ConditionSyndrome(state, tracer, s_5_2);
        // D s_5_4: read-var vect_offset:i64
        let s_5_4: i64 = fn_state.vect_offset;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var target_el:u8
        let s_5_6: u8 = fn_state.target_el;
        // D s_5_7: read-var preferred_exception_return:u64
        let s_5_7: u64 = fn_state.preferred_exception_return;
        // D s_5_8: call AArch64_TakeException(s_5_6, s_5_1, s_5_7, s_5_5)
        let s_5_8: () = AArch64_TakeException(state, tracer, s_5_6, s_5_1, s_5_7, s_5_5);
        // N s_5_9: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u32
        let s_6_0: u32 = 0;
        // S s_6_1: call ExceptionSyndrome(s_6_0)
        let s_6_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_6_0);
        // D s_6_2: read-var vect_offset:i64
        let s_6_2: i64 = fn_state.vect_offset;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // C s_6_4: const #432u : u32
        let s_6_4: u32 = 432;
        // D s_6_5: read-reg s_6_4:u8
        let s_6_5: u8 = {
            let value = state.read_register::<u8>(s_6_4 as isize);
            tracer.read_register(s_6_4 as isize, value);
            value
        };
        // D s_6_6: read-var preferred_exception_return:u64
        let s_6_6: u64 = fn_state.preferred_exception_return;
        // D s_6_7: call AArch64_TakeException(s_6_5, s_6_1, s_6_6, s_6_3)
        let s_6_7: () = AArch64_TakeException(state, tracer, s_6_5, s_6_1, s_6_6, s_6_3);
        // N s_6_8: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #102552u : u32
        let s_7_0: u32 = 102552;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_HCR_EL2_Type_TGE(s_7_1)
        let s_7_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // D s_7_7: write-var gs#21109 <= s_7_6
        fn_state.gs_21109 = s_7_6;
        // N s_7_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call EL2Enabled(s_8_0)
        let s_8_1: bool = EL2Enabled(state, tracer, s_8_0);
        // D s_8_2: write-var gs#21108 <= s_8_1
        fn_state.gs_21108 = s_8_1;
        // N s_8_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
