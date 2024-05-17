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
use neq_int::*;
use AArch32_SetLSInstructionSyndrome::*;
use LoadWritePC::*;
use R_read::*;
use MemU_read::*;
use Shift::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_LDR_r_OpT_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    shift_n: i128,
    shift_t: u32,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_880507: Bits,
        data: u32,
        address: u32,
        m: i64,
        n: i64,
        shift_n: i128,
        shift_t: u32,
        t: i64,
    }
    let fn_state = FunctionState {
        m,
        n,
        shift_n,
        shift_t,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #16971u : u32
        let s_0_3: u32 = 16971;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_6: read-var shift_t:u32
        let s_0_6: u32 = fn_state.shift_t;
        // D s_0_7: read-var shift_n:i
        let s_0_7: i128 = fn_state.shift_n;
        // D s_0_8: call Shift(s_0_5, s_0_6, s_0_7, s_0_4)
        let s_0_8: Bits = Shift(state, tracer, s_0_5, s_0_6, s_0_7, s_0_4);
        // D s_0_9: cast reint s_0_8 -> u32
        let s_0_9: u32 = (s_0_8.value() as u32);
        // D s_0_10: read-var n:i64
        let s_0_10: i64 = fn_state.n;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: call R_read(s_0_11)
        let s_0_12: u32 = R_read(state, tracer, s_0_11);
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 32u16);
        // D s_0_14: cast zx s_0_9 -> bv
        let s_0_14: Bits = Bits::new(s_0_9 as u128, 32u16);
        // D s_0_15: add s_0_13 s_0_14
        let s_0_15: Bits = (s_0_13 + s_0_14);
        // D s_0_16: cast reint s_0_15 -> u32
        let s_0_16: u32 = (s_0_15.value() as u32);
        // D s_0_17: write-var address <= s_0_16
        fn_state.address = s_0_16;
        // C s_0_18: const #15s : i
        let s_0_18: i128 = 15;
        // D s_0_19: read-var t:i64
        let s_0_19: i64 = fn_state.t;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: call neq_int(s_0_20, s_0_18)
        let s_0_21: bool = neq_int(state, tracer, s_0_20, s_0_18);
        // N s_0_22: branch s_0_21 b8 b1
        if s_0_21 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #4s : i64
        let s_2_0: i64 = 4;
        // D s_2_1: read-var address:u32
        let s_2_1: u32 = fn_state.address;
        // D s_2_2: call MemU_read(s_2_1, s_2_0)
        let s_2_2: Bits = MemU_read(state, tracer, s_2_1, s_2_0);
        // D s_2_3: write-var gs#880507 <= s_2_2
        fn_state.gs_880507 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#880507:bv
        let s_3_0: Bits = fn_state.gs_880507;
        // D s_3_1: cast reint s_3_0 -> u32
        let s_3_1: u32 = (s_3_0.value() as u32);
        // D s_3_2: write-var data <= s_3_1
        fn_state.data = s_3_1;
        // C s_3_3: const #15s : i
        let s_3_3: i128 = 15;
        // D s_3_4: read-var t:i64
        let s_3_4: i64 = fn_state.t;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cmp-eq s_3_5 s_3_3
        let s_3_6: bool = ((s_3_5) == (s_3_3));
        // N s_3_7: branch s_3_6 b5 b4
        if s_3_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var t:i64
        let s_4_0: i64 = fn_state.t;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var data:u32
        let s_4_2: u32 = fn_state.data;
        // D s_4_3: call R_set(s_4_1, s_4_2)
        let s_4_3: () = R_set(state, tracer, s_4_1, s_4_2);
        // N s_4_4: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var address:u32
        let s_5_1: u32 = fn_state.address;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 32u16);
        // C s_5_3: const #1s : i64
        let s_5_3: i64 = 1;
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: const #1s : i
        let s_5_5: i128 = 1;
        // C s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: bit-extract s_5_2 s_5_0 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 2u16);
        // C s_5_10: const #0u : u8
        let s_5_10: u8 = 0;
        // C s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 2u16);
        // D s_5_12: cmp-eq s_5_9 s_5_11
        let s_5_12: bool = ((s_5_9) == (s_5_11));
        // N s_5_13: branch s_5_12 b7 b6
        if s_5_12 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var data:u32
        let s_7_0: u32 = fn_state.data;
        // D s_7_1: call LoadWritePC(s_7_0)
        let s_7_1: () = LoadWritePC(state, tracer, s_7_0);
        // N s_7_2: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #4s : i
        let s_8_0: i128 = 4;
        // D s_8_1: read-var t:i64
        let s_8_1: i64 = fn_state.t;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_3: const #0u : u8
        let s_8_3: bool = false;
        // C s_8_4: const #0u : u8
        let s_8_4: bool = false;
        // D s_8_5: call AArch32_SetLSInstructionSyndrome(s_8_0, s_8_3, s_8_2, s_8_4)
        let s_8_5: () = AArch32_SetLSInstructionSyndrome(
            state,
            tracer,
            s_8_0,
            s_8_3,
            s_8_2,
            s_8_4,
        );
        // N s_8_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
