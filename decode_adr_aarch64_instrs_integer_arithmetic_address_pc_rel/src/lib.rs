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
use execute_aarch64_instrs_integer_arithmetic_address_pc_rel::*;
use Zeros::*;
use common::*;
pub fn decode_adr_aarch64_instrs_integer_arithmetic_address_pc_rel<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    immhi: u32,
    immlo: u8,
    op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        page: bool,
        imm: u64,
        d: i64,
        Rd: u8,
        immhi: u32,
        immlo: u8,
        op: bool,
    }
    let fn_state = FunctionState {
        Rd,
        immhi,
        immlo,
        op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var op:u8
        let s_0_5: bool = fn_state.op;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // D s_0_9: cmp-eq s_0_6 s_0_8
        let s_0_9: bool = ((s_0_6) == (s_0_8));
        // D s_0_10: write-var page <= s_0_9
        fn_state.page = s_0_9;
        // D s_0_11: read-var page:u8
        let s_0_11: bool = fn_state.page;
        // N s_0_12: branch s_0_11 b3 b1
        if s_0_11 {
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
        // D s_1_0: read-var immhi:u19
        let s_1_0: u32 = fn_state.immhi;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 19u16);
        // D s_1_2: read-var immlo:u8
        let s_1_2: u8 = fn_state.immlo;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u21
        let s_1_12: u32 = (s_1_11.value() as u32);
        // C s_1_13: const #64s : i
        let s_1_13: i128 = 64;
        // D s_1_14: cast zx s_1_12 -> bv
        let s_1_14: Bits = Bits::new(s_1_12 as u128, 21u16);
        // D s_1_15: bits-cast sx s_1_14 -> bv length s_1_13
        let s_1_15: Bits = s_1_14.sign_extend(s_1_13);
        // D s_1_16: cast reint s_1_15 -> u64
        let s_1_16: u64 = (s_1_15.value() as u64);
        // D s_1_17: write-var imm <= s_1_16
        fn_state.imm = s_1_16;
        // N s_1_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var d:i64
        let s_2_0: i64 = fn_state.d;
        // D s_2_1: read-var imm:u64
        let s_2_1: u64 = fn_state.imm;
        // D s_2_2: read-var page:u8
        let s_2_2: bool = fn_state.page;
        // D s_2_3: call execute_aarch64_instrs_integer_arithmetic_address_pc_rel(s_2_0, s_2_1, s_2_2)
        let s_2_3: () = execute_aarch64_instrs_integer_arithmetic_address_pc_rel(
            state,
            tracer,
            s_2_0,
            s_2_1,
            s_2_2,
        );
        // N s_2_4: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var immhi:u19
        let s_3_0: u32 = fn_state.immhi;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 19u16);
        // D s_3_2: read-var immlo:u8
        let s_3_2: u8 = fn_state.immlo;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u21
        let s_3_12: u32 = (s_3_11.value() as u32);
        // C s_3_13: const #12s : i
        let s_3_13: i128 = 12;
        // S s_3_14: call Zeros(s_3_13)
        let s_3_14: Bits = Zeros(state, tracer, s_3_13);
        // S s_3_15: cast reint s_3_14 -> u12
        let s_3_15: u16 = (s_3_14.value() as u16);
        // D s_3_16: cast zx s_3_12 -> bv
        let s_3_16: Bits = Bits::new(s_3_12 as u128, 21u16);
        // S s_3_17: cast zx s_3_15 -> bv
        let s_3_17: Bits = Bits::new(s_3_15 as u128, 12u16);
        // D s_3_18: cast reint s_3_16 -> u128
        let s_3_18: u128 = (s_3_16.value() as u128);
        // D s_3_19: size-of s_3_16
        let s_3_19: u16 = s_3_16.length();
        // S s_3_20: cast reint s_3_17 -> u128
        let s_3_20: u128 = (s_3_17.value() as u128);
        // D s_3_21: size-of s_3_17
        let s_3_21: u16 = s_3_17.length();
        // D s_3_22: lsl s_3_18 s_3_21
        let s_3_22: u128 = s_3_18 << s_3_21;
        // D s_3_23: or s_3_22 s_3_20
        let s_3_23: u128 = ((s_3_22) | (s_3_20));
        // D s_3_24: add s_3_19 s_3_21
        let s_3_24: u16 = (s_3_19 + s_3_21);
        // D s_3_25: create-bits s_3_23 s_3_24
        let s_3_25: Bits = Bits::new(s_3_23, s_3_24);
        // D s_3_26: cast reint s_3_25 -> u33
        let s_3_26: u64 = (s_3_25.value() as u64);
        // C s_3_27: const #64s : i
        let s_3_27: i128 = 64;
        // D s_3_28: cast zx s_3_26 -> bv
        let s_3_28: Bits = Bits::new(s_3_26 as u128, 33u16);
        // D s_3_29: bits-cast sx s_3_28 -> bv length s_3_27
        let s_3_29: Bits = s_3_28.sign_extend(s_3_27);
        // D s_3_30: cast reint s_3_29 -> u64
        let s_3_30: u64 = (s_3_29.value() as u64);
        // D s_3_31: write-var imm <= s_3_30
        fn_state.imm = s_3_30;
        // N s_3_32: jump b2
        return block_2(state, tracer, fn_state);
    }
}
