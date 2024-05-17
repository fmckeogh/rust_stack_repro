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
use execute_aarch64_instrs_integer_tags_mcaddtag::*;
use common::*;
pub fn decode_addg_aarch64_instrs_integer_tags_mcaddtag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Xd: u8,
    Xn: u8,
    uimm4: u8,
    op3: u8,
    uimm6: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Xd: u8,
        Xn: u8,
        uimm4: u8,
        op3: u8,
        uimm6: u8,
    }
    let fn_state = FunctionState {
        Xd,
        Xn,
        uimm4,
        op3,
        uimm6,
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
        // D s_1_0: read-var Xd:u8
        let s_1_0: u8 = fn_state.Xd;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: read-var Xn:u8
        let s_1_4: u8 = fn_state.Xn;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 5u16);
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (s_1_5.value() as i128);
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var uimm4:u8
        let s_1_8: u8 = fn_state.uimm4;
        // C s_1_9: const #64s : i
        let s_1_9: i128 = 64;
        // D s_1_10: read-var uimm6:u8
        let s_1_10: u8 = fn_state.uimm6;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 6u16);
        // D s_1_12: bits-cast zx s_1_11 -> bv length s_1_9
        let s_1_12: Bits = s_1_11.zero_extend(s_1_9);
        // D s_1_13: cast reint s_1_12 -> u64
        let s_1_13: u64 = (s_1_12.value() as u64);
        // D s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 64u16);
        // C s_1_15: const #456u : u32
        let s_1_15: u32 = 456;
        // D s_1_16: read-reg s_1_15:i64
        let s_1_16: i64 = {
            let value = state.read_register::<i64>(s_1_15 as isize);
            tracer.read_register(s_1_15 as isize, value);
            value
        };
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: lsl s_1_14 s_1_17
        let s_1_18: Bits = s_1_14 << s_1_17;
        // D s_1_19: cast reint s_1_18 -> u64
        let s_1_19: u64 = (s_1_18.value() as u64);
        // C s_1_20: const #1u : u8
        let s_1_20: bool = true;
        // D s_1_21: call execute_aarch64_instrs_integer_tags_mcaddtag(s_1_20, s_1_3, s_1_7, s_1_19, s_1_8)
        let s_1_21: () = execute_aarch64_instrs_integer_tags_mcaddtag(
            state,
            tracer,
            s_1_20,
            s_1_3,
            s_1_7,
            s_1_19,
            s_1_8,
        );
        // N s_1_22: return
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