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
use Align_bits::*;
use PC_read__1::*;
use Hint_PreloadData::*;
use common::*;
pub fn execute_aarch32_instrs_PLD_l_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    imm32: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        add: bool,
        imm32: u32,
    }
    let fn_state = FunctionState {
        add,
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
        // N s_0_1: branch s_0_0 b3 b1
        if s_0_0 {
            return block_3(state, tracer, fn_state);
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
        // D s_1_11: write-var address <= s_1_10
        fn_state.address = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var address:u32
        let s_2_0: u32 = fn_state.address;
        // D s_2_1: call Hint_PreloadData(s_2_0)
        let s_2_1: () = Hint_PreloadData(state, tracer, s_2_0);
        // N s_2_2: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call PC_read__1(s_3_0)
        let s_3_1: u32 = PC_read__1(state, tracer, s_3_0);
        // C s_3_2: const #4s : i
        let s_3_2: i128 = 4;
        // S s_3_3: cast zx s_3_1 -> bv
        let s_3_3: Bits = Bits::new(s_3_1 as u128, 32u16);
        // S s_3_4: call Align_bits(s_3_3, s_3_2)
        let s_3_4: Bits = Align_bits(state, tracer, s_3_3, s_3_2);
        // S s_3_5: cast reint s_3_4 -> u32
        let s_3_5: u32 = (s_3_4.value() as u32);
        // S s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 32u16);
        // D s_3_7: read-var imm32:u32
        let s_3_7: u32 = fn_state.imm32;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 32u16);
        // D s_3_9: add s_3_6 s_3_8
        let s_3_9: Bits = (s_3_6 + s_3_8);
        // D s_3_10: cast reint s_3_9 -> u32
        let s_3_10: u32 = (s_3_9.value() as u32);
        // D s_3_11: write-var address <= s_3_10
        fn_state.address = s_3_10;
        // N s_3_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
