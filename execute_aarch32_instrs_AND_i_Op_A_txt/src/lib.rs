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
use ALUExceptionReturn::*;
use IsZeroBit::*;
use R_set::*;
use R_read::*;
use ALUWritePC::*;
use common::*;
pub fn execute_aarch32_instrs_AND_i_Op_A_txt<T: Tracer>(
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
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var imm32:u32
        let s_0_4: u32 = fn_state.imm32;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 32u16);
        // D s_0_6: and s_0_3 s_0_5
        let s_0_6: Bits = ((s_0_3) & (s_0_5));
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // D s_0_8: write-var result <= s_0_7
        fn_state.result = s_0_7;
        // C s_0_9: const #15s : i
        let s_0_9: i128 = 15;
        // D s_0_10: read-var d:i64
        let s_0_10: i64 = fn_state.d;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: cmp-eq s_0_11 s_0_9
        let s_0_12: bool = ((s_0_11) == (s_0_9));
        // N s_0_13: branch s_0_12 b4 b1
        if s_0_12 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var d:i64
        let s_1_0: i64 = fn_state.d;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var result:u32
        let s_1_2: u32 = fn_state.result;
        // D s_1_3: call R_set(s_1_1, s_1_2)
        let s_1_3: () = R_set(state, tracer, s_1_1, s_1_2);
        // D s_1_4: read-var setflags:u8
        let s_1_4: bool = fn_state.setflags;
        // N s_1_5: branch s_1_4 b3 b2
        if s_1_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #31s : i
        let s_3_0: i128 = 31;
        // D s_3_1: read-var result:u32
        let s_3_1: u32 = fn_state.result;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 32u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // C s_3_18: const #16984u : u32
        let s_3_18: u32 = 16984;
        // N s_3_19: write-reg s_3_18 <= s_3_17
        let s_3_19: () = {
            state.write_register::<bool>(s_3_18 as isize, s_3_17);
            tracer.write_register(s_3_18 as isize, s_3_17);
        };
        // D s_3_20: read-var result:u32
        let s_3_20: u32 = fn_state.result;
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 32u16);
        // D s_3_22: call IsZeroBit(s_3_21)
        let s_3_22: bool = IsZeroBit(state, tracer, s_3_21);
        // C s_3_23: const #16997u : u32
        let s_3_23: u32 = 16997;
        // N s_3_24: write-reg s_3_23 <= s_3_22
        let s_3_24: () = {
            state.write_register::<bool>(s_3_23 as isize, s_3_22);
            tracer.write_register(s_3_23 as isize, s_3_22);
        };
        // D s_3_25: read-var carry:u8
        let s_3_25: bool = fn_state.carry;
        // C s_3_26: const #16971u : u32
        let s_3_26: u32 = 16971;
        // N s_3_27: write-reg s_3_26 <= s_3_25
        let s_3_27: () = {
            state.write_register::<bool>(s_3_26 as isize, s_3_25);
            tracer.write_register(s_3_26 as isize, s_3_25);
        };
        // N s_3_28: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var setflags:u8
        let s_4_0: bool = fn_state.setflags;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var result:u32
        let s_5_0: u32 = fn_state.result;
        // D s_5_1: call ALUWritePC(s_5_0)
        let s_5_1: () = ALUWritePC(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var result:u32
        let s_6_0: u32 = fn_state.result;
        // D s_6_1: call ALUExceptionReturn(s_6_0)
        let s_6_1: () = ALUExceptionReturn(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
}
