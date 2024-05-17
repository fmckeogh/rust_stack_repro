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
use execute_aarch64_instrs_integer_tags_mcsettaganddatapairpost::*;
use HaveMTEExt::*;
use common::*;
pub fn decode_stgp_aarch64_instrs_integer_tags_mcsettaganddatapair<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Xt: u8,
    Xn: u8,
    Xt2: u8,
    simm7: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Xt: u8,
        Xn: u8,
        Xt2: u8,
        simm7: u8,
    }
    let fn_state = FunctionState {
        Xt,
        Xn,
        Xt2,
        simm7,
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
        // D s_1_8: read-var Xt2:u8
        let s_1_8: u8 = fn_state.Xt2;
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 5u16);
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (s_1_9.value() as i128);
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // C s_1_12: const #64s : i
        let s_1_12: i128 = 64;
        // D s_1_13: read-var simm7:u8
        let s_1_13: u8 = fn_state.simm7;
        // D s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 7u16);
        // D s_1_15: bits-cast sx s_1_14 -> bv length s_1_12
        let s_1_15: Bits = s_1_14.sign_extend(s_1_12);
        // D s_1_16: cast reint s_1_15 -> u64
        let s_1_16: u64 = (s_1_15.value() as u64);
        // D s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 64u16);
        // C s_1_18: const #456u : u32
        let s_1_18: u32 = 456;
        // D s_1_19: read-reg s_1_18:i64
        let s_1_19: i64 = {
            let value = state.read_register::<i64>(s_1_18 as isize);
            tracer.read_register(s_1_18 as isize, value);
            value
        };
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: lsl s_1_17 s_1_20
        let s_1_21: Bits = s_1_17 << s_1_20;
        // D s_1_22: cast reint s_1_21 -> u64
        let s_1_22: u64 = (s_1_21.value() as u64);
        // C s_1_23: const #0u : u8
        let s_1_23: bool = false;
        // C s_1_24: const #0u : u8
        let s_1_24: bool = false;
        // D s_1_25: call execute_aarch64_instrs_integer_tags_mcsettaganddatapairpost(s_1_3, s_1_22, s_1_24, s_1_7, s_1_11, s_1_23)
        let s_1_25: () = execute_aarch64_instrs_integer_tags_mcsettaganddatapairpost(
            state,
            tracer,
            s_1_3,
            s_1_22,
            s_1_24,
            s_1_7,
            s_1_11,
            s_1_23,
        );
        // N s_1_26: return
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
