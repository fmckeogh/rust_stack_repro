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
use R_read::*;
use Hint_PreloadInstr::*;
use PC_read__1::*;
use Align_bits::*;
use common::*;
pub fn execute_aarch32_instrs_PLI_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    imm32: u32,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        base: u32,
        address: u32,
        add: bool,
        imm32: u32,
        n: i64,
    }
    let fn_state = FunctionState {
        add,
        imm32,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #15s : i
        let s_0_0: i128 = 15;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b6 b1
        if s_0_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: write-var base <= s_1_2
        fn_state.base = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var add:u8
        let s_2_0: bool = fn_state.add;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var base:u32
        let s_3_0: u32 = fn_state.base;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 32u16);
        // D s_3_2: read-var imm32:u32
        let s_3_2: u32 = fn_state.imm32;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 32u16);
        // D s_3_4: sub s_3_1 s_3_3
        let s_3_4: Bits = ((s_3_1) - (s_3_3));
        // D s_3_5: cast reint s_3_4 -> u32
        let s_3_5: u32 = (s_3_4.value() as u32);
        // D s_3_6: write-var address <= s_3_5
        fn_state.address = s_3_5;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var address:u32
        let s_4_0: u32 = fn_state.address;
        // D s_4_1: call Hint_PreloadInstr(s_4_0)
        let s_4_1: () = Hint_PreloadInstr(state, tracer, s_4_0);
        // N s_4_2: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var base:u32
        let s_5_0: u32 = fn_state.base;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 32u16);
        // D s_5_2: read-var imm32:u32
        let s_5_2: u32 = fn_state.imm32;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: Bits = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> u32
        let s_5_5: u32 = (s_5_4.value() as u32);
        // D s_5_6: write-var address <= s_5_5
        fn_state.address = s_5_5;
        // N s_5_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call PC_read__1(s_6_0)
        let s_6_1: u32 = PC_read__1(state, tracer, s_6_0);
        // C s_6_2: const #4s : i
        let s_6_2: i128 = 4;
        // S s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 32u16);
        // S s_6_4: call Align_bits(s_6_3, s_6_2)
        let s_6_4: Bits = Align_bits(state, tracer, s_6_3, s_6_2);
        // S s_6_5: cast reint s_6_4 -> u32
        let s_6_5: u32 = (s_6_4.value() as u32);
        // D s_6_6: write-var base <= s_6_5
        fn_state.base = s_6_5;
        // N s_6_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
