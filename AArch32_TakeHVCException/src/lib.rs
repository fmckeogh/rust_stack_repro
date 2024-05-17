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
use AArch32_EnterHypMode::*;
use AArch32_ITAdvance::*;
use SSAdvance::*;
use ELUsingAArch32::*;
use ExceptionSyndrome::*;
use NextInstrAddr::*;
use common::*;
pub fn AArch32_TakeHVCException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        vect_offset: i64,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u32,
        gs_31802: bool,
        immediate: u16,
    }
    let fn_state = FunctionState {
        immediate,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b5 b1
        if s_0_3 {
            return block_5(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#31802 <= s_1_0
        fn_state.gs_31802 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31802:u8
        let s_2_0: bool = fn_state.gs_31802;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call AArch32_ITAdvance(s_2_2)
        let s_2_3: () = AArch32_ITAdvance(state, tracer, s_2_2);
        // C s_2_4: const #() : ()
        let s_2_4: () = ();
        // S s_2_5: call SSAdvance(s_2_4)
        let s_2_5: () = SSAdvance(state, tracer, s_2_4);
        // C s_2_6: const #32s : i64
        let s_2_6: i64 = 32;
        // C s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // S s_2_8: call NextInstrAddr(s_2_7)
        let s_2_8: Bits = NextInstrAddr(state, tracer, s_2_7);
        // S s_2_9: cast reint s_2_8 -> u32
        let s_2_9: u32 = (s_2_8.value() as u32);
        // D s_2_10: write-var preferred_exception_return <= s_2_9
        fn_state.preferred_exception_return = s_2_9;
        // C s_2_11: const #8u : u8
        let s_2_11: u8 = 8;
        // C s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 8u16);
        // C s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // C s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var vect_offset <= s_2_14
        fn_state.vect_offset = s_2_14;
        // C s_2_16: const #13u : u32
        let s_2_16: u32 = 13;
        // S s_2_17: call ExceptionSyndrome(s_2_16)
        let s_2_17: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_2_16,
        );
        // D s_2_18: write-var except <= s_2_17
        fn_state.except = s_2_17;
        // D s_2_19: read-var except:struct
        let s_2_19: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_2_20: write-var except <= s_2_19
        fn_state.except = s_2_19;
        // C s_2_21: const #16975u : u32
        let s_2_21: u32 = 16975;
        // D s_2_22: read-reg s_2_21:u8
        let s_2_22: u8 = {
            let value = state.read_register::<u8>(s_2_21 as isize);
            tracer.read_register(s_2_21 as isize, value);
            value
        };
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 2u16);
        // C s_2_24: const #432u : u32
        let s_2_24: u32 = 432;
        // D s_2_25: read-reg s_2_24:u8
        let s_2_25: u8 = {
            let value = state.read_register::<u8>(s_2_24 as isize);
            tracer.read_register(s_2_24 as isize, value);
            value
        };
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 2u16);
        // D s_2_27: cmp-eq s_2_23 s_2_26
        let s_2_27: bool = ((s_2_23) == (s_2_26));
        // N s_2_28: branch s_2_27 b4 b3
        if s_2_27 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #20u : u8
        let s_3_0: u8 = 20;
        // C s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 8u16);
        // C s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // C s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var except:struct
        let s_3_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_3_6: read-var preferred_exception_return:u32
        let s_3_6: u32 = fn_state.preferred_exception_return;
        // D s_3_7: call AArch32_EnterHypMode(s_3_5, s_3_6, s_3_4)
        let s_3_7: () = AArch32_EnterHypMode(state, tracer, s_3_5, s_3_6, s_3_4);
        // N s_3_8: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var vect_offset:i64
        let s_4_0: i64 = fn_state.vect_offset;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var except:struct
        let s_4_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_3: read-var preferred_exception_return:u32
        let s_4_3: u32 = fn_state.preferred_exception_return;
        // D s_4_4: call AArch32_EnterHypMode(s_4_2, s_4_3, s_4_1)
        let s_4_4: () = AArch32_EnterHypMode(state, tracer, s_4_2, s_4_3, s_4_1);
        // N s_4_5: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call ELUsingAArch32(s_5_1)
        let s_5_2: bool = ELUsingAArch32(state, tracer, s_5_1);
        // D s_5_3: write-var gs#31802 <= s_5_2
        fn_state.gs_31802 = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
