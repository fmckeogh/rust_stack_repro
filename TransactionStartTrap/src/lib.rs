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
use EL2Enabled::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use integer_subrange::*;
use common::*;
pub fn TransactionStartTrap<T: Tracer>(state: &mut State, tracer: &T, dreg: i128) -> () {
    #[derive(Default)]
    struct FunctionState {
        vect_offset: i64,
        gs_24570: bool,
        targetEL: u8,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        dreg: i128,
    }
    let fn_state = FunctionState {
        dreg,
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
        // C s_0_10: const #34u : u32
        let s_0_10: u32 = 34;
        // S s_0_11: call ExceptionSyndrome(s_0_10)
        let s_0_11: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_0_10,
        );
        // D s_0_12: write-var except <= s_0_11
        fn_state.except = s_0_11;
        // C s_0_13: const #4s : i
        let s_0_13: i128 = 4;
        // C s_0_14: const #0s : i
        let s_0_14: i128 = 0;
        // D s_0_15: read-var dreg:i
        let s_0_15: i128 = fn_state.dreg;
        // D s_0_16: call integer_subrange(s_0_15, s_0_13, s_0_14)
        let s_0_16: Bits = integer_subrange(state, tracer, s_0_15, s_0_13, s_0_14);
        // D s_0_17: read-var except:struct
        let s_0_17: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_18: write-var except <= s_0_17
        fn_state.except = s_0_17;
        // C s_0_19: const #16975u : u32
        let s_0_19: u32 = 16975;
        // D s_0_20: read-reg s_0_19:u8
        let s_0_20: u8 = {
            let value = state.read_register::<u8>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 2u16);
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (s_0_21.value() as i128);
        // D s_0_23: cast reint s_0_22 -> i64
        let s_0_23: i64 = (s_0_22 as i64);
        // C s_0_24: const #440u : u32
        let s_0_24: u32 = 440;
        // D s_0_25: read-reg s_0_24:u8
        let s_0_25: u8 = {
            let value = state.read_register::<u8>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 2u16);
        // D s_0_27: cast zx s_0_26 -> i
        let s_0_27: i128 = (s_0_26.value() as i128);
        // D s_0_28: cast reint s_0_27 -> i64
        let s_0_28: i64 = (s_0_27 as i64);
        // D s_0_29: cast zx s_0_23 -> i
        let s_0_29: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_30: cast zx s_0_28 -> i
        let s_0_30: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_31: cmp-gt s_0_29 s_0_30
        let s_0_31: bool = ((s_0_29) > (s_0_30));
        // N s_0_32: branch s_0_31 b8 b1
        if s_0_31 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call EL2Enabled(s_1_0)
        let s_1_1: bool = EL2Enabled(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b7 b2
        if s_1_1 {
            return block_7(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#24570 <= s_2_0
        fn_state.gs_24570 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#24570:u8
        let s_3_0: bool = fn_state.gs_24570;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #440u : u32
        let s_4_0: u32 = 440;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: write-var targetEL <= s_4_1
        fn_state.targetEL = s_4_1;
        // N s_4_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var vect_offset:i64
        let s_5_0: i64 = fn_state.vect_offset;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var targetEL:u8
        let s_5_2: u8 = fn_state.targetEL;
        // D s_5_3: read-var except:struct
        let s_5_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_5_4: read-var preferred_exception_return:u64
        let s_5_4: u64 = fn_state.preferred_exception_return;
        // D s_5_5: call AArch64_TakeException(s_5_2, s_5_3, s_5_4, s_5_1)
        let s_5_5: () = AArch64_TakeException(state, tracer, s_5_2, s_5_3, s_5_4, s_5_1);
        // N s_5_6: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #432u : u32
        let s_6_0: u32 = 432;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: write-var targetEL <= s_6_1
        fn_state.targetEL = s_6_1;
        // N s_6_3: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_7_7: write-var gs#24570 <= s_7_6
        fn_state.gs_24570 = s_7_6;
        // N s_7_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: write-var targetEL <= s_8_1
        fn_state.targetEL = s_8_1;
        // N s_8_3: jump b5
        return block_5(state, tracer, fn_state);
    }
}
