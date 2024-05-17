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
use R_set::*;
use ALUWritePC::*;
use PC_read__1::*;
use Align_bits::*;
use common::*;
pub fn execute_aarch32_instrs_ADR_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d: i64,
    imm32: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u32,
        add: bool,
        d: i64,
        imm32: u32,
    }
    let fn_state = FunctionState {
        add,
        d,
        imm32,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var add:u8
        let s_0_0: bool = fn_state.add;
        // N s_0_1: branch s_0_0 b5 b1
        if s_0_0 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call PC_read__1(s_1_0)
        let s_1_1: u32 = PC_read__1(state, tracer, s_1_0);
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // S s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 32u16);
        // S s_1_4: call Align_bits(s_1_3, s_1_2)
        let s_1_4: Bits = Align_bits(state, tracer, s_1_3, s_1_2);
        // S s_1_5: cast reint s_1_4 -> u32
        let s_1_5: u32 = (s_1_4.value() as u32);
        // S s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_7: read-var imm32:u32
        let s_1_7: u32 = fn_state.imm32;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 32u16);
        // D s_1_9: sub s_1_6 s_1_8
        let s_1_9: Bits = ((s_1_6) - (s_1_8));
        // D s_1_10: cast reint s_1_9 -> u32
        let s_1_10: u32 = (s_1_9.value() as u32);
        // D s_1_11: write-var result <= s_1_10
        fn_state.result = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #15s : i
        let s_2_0: i128 = 15;
        // D s_2_1: read-var d:i64
        let s_2_1: i64 = fn_state.d;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b4 b3
        if s_2_3 {
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
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var result:u32
        let s_3_2: u32 = fn_state.result;
        // D s_3_3: call R_set(s_3_1, s_3_2)
        let s_3_3: () = R_set(state, tracer, s_3_1, s_3_2);
        // N s_3_4: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var result:u32
        let s_4_0: u32 = fn_state.result;
        // D s_4_1: call ALUWritePC(s_4_0)
        let s_4_1: () = ALUWritePC(state, tracer, s_4_0);
        // N s_4_2: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PC_read__1(s_5_0)
        let s_5_1: u32 = PC_read__1(state, tracer, s_5_0);
        // C s_5_2: const #4s : i
        let s_5_2: i128 = 4;
        // S s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 32u16);
        // S s_5_4: call Align_bits(s_5_3, s_5_2)
        let s_5_4: Bits = Align_bits(state, tracer, s_5_3, s_5_2);
        // S s_5_5: cast reint s_5_4 -> u32
        let s_5_5: u32 = (s_5_4.value() as u32);
        // S s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 32u16);
        // D s_5_7: read-var imm32:u32
        let s_5_7: u32 = fn_state.imm32;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 32u16);
        // D s_5_9: add s_5_6 s_5_8
        let s_5_9: Bits = (s_5_6 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> u32
        let s_5_10: u32 = (s_5_9.value() as u32);
        // D s_5_11: write-var result <= s_5_10
        fn_state.result = s_5_10;
        // N s_5_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
