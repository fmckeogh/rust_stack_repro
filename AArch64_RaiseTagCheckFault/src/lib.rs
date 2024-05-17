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
use AArch64_AbortSyndrome::*;
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_RaiseTagCheckFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u64,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target_el: u8,
        vect_offset: i64,
        gs_15878: bool,
        gs_15879: bool,
        preferred_exception_return: u64,
        va: u64,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        va,
        fault,
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
        // C s_0_10: const #440u : u32
        let s_0_10: u32 = 440;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: write-var target_el <= s_0_11
        fn_state.target_el = s_0_11;
        // C s_0_13: const #16975u : u32
        let s_0_13: u32 = 16975;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (s_0_15.value() as i128);
        // D s_0_17: cast reint s_0_16 -> i64
        let s_0_17: i64 = (s_0_16 as i64);
        // C s_0_18: const #440u : u32
        let s_0_18: u32 = 440;
        // D s_0_19: read-reg s_0_18:u8
        let s_0_19: u8 = {
            let value = state.read_register::<u8>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // D s_0_21: cast zx s_0_20 -> i
        let s_0_21: i128 = (s_0_20.value() as i128);
        // D s_0_22: cast reint s_0_21 -> i64
        let s_0_22: i64 = (s_0_21 as i64);
        // D s_0_23: cast zx s_0_17 -> i
        let s_0_23: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_24: cast zx s_0_22 -> i
        let s_0_24: i128 = (i128::try_from(s_0_22).unwrap());
        // D s_0_25: cmp-gt s_0_23 s_0_24
        let s_0_25: bool = ((s_0_23) > (s_0_24));
        // N s_0_26: branch s_0_25 b11 b1
        if s_0_25 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b10 b2
        if s_1_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#15878 <= s_2_0
        fn_state.gs_15878 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#15878:u8
        let s_3_0: bool = fn_state.gs_15878;
        // N s_3_1: branch s_3_0 b9 b4
        if s_3_0 {
            return block_9(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#15879 <= s_4_0
        fn_state.gs_15879 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#15879:u8
        let s_5_0: bool = fn_state.gs_15879;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #19u : u32
        let s_7_0: u32 = 19;
        // D s_7_1: read-var fault:struct
        let s_7_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_7_2: read-var va:u64
        let s_7_2: u64 = fn_state.va;
        // D s_7_3: read-var target_el:u8
        let s_7_3: u8 = fn_state.target_el;
        // D s_7_4: call AArch64_AbortSyndrome(s_7_0, s_7_1, s_7_2, s_7_3)
        let s_7_4: ProductTypeb7f99f96751e17c4 = AArch64_AbortSyndrome(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
            s_7_3,
        );
        // D s_7_5: read-var vect_offset:i64
        let s_7_5: i64 = fn_state.vect_offset;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var target_el:u8
        let s_7_7: u8 = fn_state.target_el;
        // D s_7_8: read-var preferred_exception_return:u64
        let s_7_8: u64 = fn_state.preferred_exception_return;
        // D s_7_9: call AArch64_TakeException(s_7_7, s_7_4, s_7_8, s_7_6)
        let s_7_9: () = AArch64_TakeException(state, tracer, s_7_7, s_7_4, s_7_8, s_7_6);
        // N s_7_10: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #432u : u32
        let s_8_0: u32 = 432;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: write-var target_el <= s_8_1
        fn_state.target_el = s_8_1;
        // N s_8_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #102552u : u32
        let s_9_0: u32 = 102552;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_HCR_EL2_Type_TGE(s_9_1)
        let s_9_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var gs#15879 <= s_9_6
        fn_state.gs_15879 = s_9_6;
        // N s_9_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EL2Enabled(s_10_0)
        let s_10_1: bool = EL2Enabled(state, tracer, s_10_0);
        // D s_10_2: write-var gs#15878 <= s_10_1
        fn_state.gs_15878 = s_10_1;
        // N s_10_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16975u : u32
        let s_11_0: u32 = 16975;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: write-var target_el <= s_11_1
        fn_state.target_el = s_11_1;
        // N s_11_3: jump b7
        return block_7(state, tracer, fn_state);
    }
}
