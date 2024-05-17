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
use IsZeroBit::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_ORN_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    carry: bool,
    d: i64,
    imm32: u32,
    n: i64,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u32,
        carry: bool,
        d: i64,
        imm32: u32,
        n: i64,
        setflags: bool,
    }
    let fn_state = FunctionState {
        carry,
        d,
        imm32,
        n,
        setflags,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: read-var imm32:u32
        let s_0_3: u32 = fn_state.imm32;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 32u16);
        // D s_0_5: not s_0_4
        let s_0_5: Bits = !s_0_4;
        // D s_0_6: cast reint s_0_5 -> u32
        let s_0_6: u32 = (s_0_5.value() as u32);
        // D s_0_7: cast zx s_0_2 -> bv
        let s_0_7: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_8: cast zx s_0_6 -> bv
        let s_0_8: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_9: or s_0_7 s_0_8
        let s_0_9: Bits = ((s_0_7) | (s_0_8));
        // D s_0_10: cast reint s_0_9 -> u32
        let s_0_10: u32 = (s_0_9.value() as u32);
        // D s_0_11: write-var result <= s_0_10
        fn_state.result = s_0_10;
        // D s_0_12: read-var d:i64
        let s_0_12: i64 = fn_state.d;
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // D s_0_14: read-var result:u32
        let s_0_14: u32 = fn_state.result;
        // D s_0_15: call R_set(s_0_13, s_0_14)
        let s_0_15: () = R_set(state, tracer, s_0_13, s_0_14);
        // D s_0_16: read-var setflags:u8
        let s_0_16: bool = fn_state.setflags;
        // N s_0_17: branch s_0_16 b2 b1
        if s_0_16 {
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
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var result:u32
        let s_2_1: u32 = fn_state.result;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // C s_2_18: const #16984u : u32
        let s_2_18: u32 = 16984;
        // N s_2_19: write-reg s_2_18 <= s_2_17
        let s_2_19: () = {
            state.write_register::<bool>(s_2_18 as isize, s_2_17);
            tracer.write_register(s_2_18 as isize, s_2_17);
        };
        // D s_2_20: read-var result:u32
        let s_2_20: u32 = fn_state.result;
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 32u16);
        // D s_2_22: call IsZeroBit(s_2_21)
        let s_2_22: bool = IsZeroBit(state, tracer, s_2_21);
        // C s_2_23: const #16997u : u32
        let s_2_23: u32 = 16997;
        // N s_2_24: write-reg s_2_23 <= s_2_22
        let s_2_24: () = {
            state.write_register::<bool>(s_2_23 as isize, s_2_22);
            tracer.write_register(s_2_23 as isize, s_2_22);
        };
        // D s_2_25: read-var carry:u8
        let s_2_25: bool = fn_state.carry;
        // C s_2_26: const #16971u : u32
        let s_2_26: u32 = 16971;
        // N s_2_27: write-reg s_2_26 <= s_2_25
        let s_2_27: () = {
            state.write_register::<bool>(s_2_26 as isize, s_2_25);
            tracer.write_register(s_2_26 as isize, s_2_25);
        };
        // N s_2_28: return
        return;
    }
}
