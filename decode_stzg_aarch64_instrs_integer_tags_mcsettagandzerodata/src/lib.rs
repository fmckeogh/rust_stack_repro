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
use HaveMTEExt::*;
use execute_aarch64_instrs_integer_tags_mcsettagandzerodatapost::*;
use common::*;
pub fn decode_stzg_aarch64_instrs_integer_tags_mcsettagandzerodata<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Xt: u8,
    Xn: u8,
    imm9: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Xt: u8,
        Xn: u8,
        imm9: u16,
    }
    let fn_state = FunctionState {
        Xt,
        Xn,
        imm9,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveMTEExt(s_0_0)
        let s_0_1: bool = HaveMTEExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
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
        // D s_1_0: read-var Xn:u8
        let s_1_0: u8 = fn_state.Xn;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: read-var Xt:u8
        let s_1_4: u8 = fn_state.Xt;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 5u16);
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (s_1_5.value() as i128);
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // C s_1_8: const #64s : i
        let s_1_8: i128 = 64;
        // D s_1_9: read-var imm9:u9
        let s_1_9: u16 = fn_state.imm9;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 9u16);
        // D s_1_11: bits-cast sx s_1_10 -> bv length s_1_8
        let s_1_11: Bits = s_1_10.sign_extend(s_1_8);
        // D s_1_12: cast reint s_1_11 -> u64
        let s_1_12: u64 = (s_1_11.value() as u64);
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 64u16);
        // C s_1_14: const #456u : u32
        let s_1_14: u32 = 456;
        // D s_1_15: read-reg s_1_14:i64
        let s_1_15: i64 = {
            let value = state.read_register::<i64>(s_1_14 as isize);
            tracer.read_register(s_1_14 as isize, value);
            value
        };
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: lsl s_1_13 s_1_16
        let s_1_17: Bits = s_1_13 << s_1_16;
        // D s_1_18: cast reint s_1_17 -> u64
        let s_1_18: u64 = (s_1_17.value() as u64);
        // C s_1_19: const #0u : u8
        let s_1_19: bool = false;
        // C s_1_20: const #0u : u8
        let s_1_20: bool = false;
        // C s_1_21: const #1u : u8
        let s_1_21: bool = true;
        // D s_1_22: call execute_aarch64_instrs_integer_tags_mcsettagandzerodatapost(s_1_3, s_1_18, s_1_20, s_1_7, s_1_19, s_1_21)
        let s_1_22: () = execute_aarch64_instrs_integer_tags_mcsettagandzerodatapost(
            state,
            tracer,
            s_1_3,
            s_1_18,
            s_1_20,
            s_1_7,
            s_1_19,
            s_1_21,
        );
        // N s_1_23: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
