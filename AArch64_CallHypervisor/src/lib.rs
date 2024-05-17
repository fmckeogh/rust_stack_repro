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
use UsingAArch32::*;
use AArch32_ITAdvance::*;
use SSAdvance::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use NextInstrAddr::*;
use common::*;
pub fn AArch64_CallHypervisor<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        vect_offset: i64,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
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
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call UsingAArch32(s_0_5)
        let s_0_6: bool = UsingAArch32(state, tracer, s_0_5);
        // N s_0_7: branch s_0_6 b5 b1
        if s_0_6 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call SSAdvance(s_2_0)
        let s_2_1: () = SSAdvance(state, tracer, s_2_0);
        // C s_2_2: const #64s : i64
        let s_2_2: i64 = 64;
        // C s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // S s_2_4: call NextInstrAddr(s_2_3)
        let s_2_4: Bits = NextInstrAddr(state, tracer, s_2_3);
        // S s_2_5: cast reint s_2_4 -> u64
        let s_2_5: u64 = (s_2_4.value() as u64);
        // D s_2_6: write-var preferred_exception_return <= s_2_5
        fn_state.preferred_exception_return = s_2_5;
        // C s_2_7: const #0u : u8
        let s_2_7: u8 = 0;
        // C s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 4u16);
        // C s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (s_2_8.value() as i128);
        // C s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // D s_2_11: write-var vect_offset <= s_2_10
        fn_state.vect_offset = s_2_10;
        // C s_2_12: const #13u : u32
        let s_2_12: u32 = 13;
        // S s_2_13: call ExceptionSyndrome(s_2_12)
        let s_2_13: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_2_12,
        );
        // D s_2_14: write-var except <= s_2_13
        fn_state.except = s_2_13;
        // D s_2_15: read-var except:struct
        let s_2_15: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_2_16: write-var except <= s_2_15
        fn_state.except = s_2_15;
        // C s_2_17: const #16975u : u32
        let s_2_17: u32 = 16975;
        // D s_2_18: read-reg s_2_17:u8
        let s_2_18: u8 = {
            let value = state.read_register::<u8>(s_2_17 as isize);
            tracer.read_register(s_2_17 as isize, value);
            value
        };
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 2u16);
        // C s_2_20: const #424u : u32
        let s_2_20: u32 = 424;
        // D s_2_21: read-reg s_2_20:u8
        let s_2_21: u8 = {
            let value = state.read_register::<u8>(s_2_20 as isize);
            tracer.read_register(s_2_20 as isize, value);
            value
        };
        // D s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 2u16);
        // D s_2_23: cmp-eq s_2_19 s_2_22
        let s_2_23: bool = ((s_2_19) == (s_2_22));
        // N s_2_24: branch s_2_23 b4 b3
        if s_2_23 {
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
        // D s_3_0: read-var vect_offset:i64
        let s_3_0: i64 = fn_state.vect_offset;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: read-var except:struct
        let s_3_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_3_5: read-var preferred_exception_return:u64
        let s_3_5: u64 = fn_state.preferred_exception_return;
        // D s_3_6: call AArch64_TakeException(s_3_3, s_3_4, s_3_5, s_3_1)
        let s_3_6: () = AArch64_TakeException(state, tracer, s_3_3, s_3_4, s_3_5, s_3_1);
        // N s_3_7: return
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
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: read-var except:struct
        let s_4_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_5: read-var preferred_exception_return:u64
        let s_4_5: u64 = fn_state.preferred_exception_return;
        // D s_4_6: call AArch64_TakeException(s_4_3, s_4_4, s_4_5, s_4_1)
        let s_4_6: () = AArch64_TakeException(state, tracer, s_4_3, s_4_4, s_4_5, s_4_1);
        // N s_4_7: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call AArch32_ITAdvance(s_5_0)
        let s_5_1: () = AArch32_ITAdvance(state, tracer, s_5_0);
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
