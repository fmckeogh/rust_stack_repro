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
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use ROR::*;
use V_set::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sha2op_sha256_sched0<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        T: u128,
        result: u128,
        operand1: u128,
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        n,
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
        // S s_0_1: call AArch64_CheckFPAdvSIMDEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var d:i64
        let s_1_1: i64 = fn_state.d;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // D s_1_5: write-var operand1 <= s_1_4
        fn_state.operand1 = s_1_4;
        // C s_1_6: const #128s : i64
        let s_1_6: i64 = 128;
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: cast reint s_1_9 -> u128
        let s_1_10: u128 = (s_1_9.value() as u128);
        // C s_1_11: const #0s : i
        let s_1_11: i128 = 0;
        // D s_1_12: cast zx s_1_10 -> bv
        let s_1_12: Bits = Bits::new(s_1_10 as u128, 128u16);
        // C s_1_13: const #1s : i64
        let s_1_13: i64 = 1;
        // C s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // C s_1_15: const #31s : i
        let s_1_15: i128 = 31;
        // C s_1_16: add s_1_15 s_1_14
        let s_1_16: i128 = (s_1_15 + s_1_14);
        // D s_1_17: bit-extract s_1_12 s_1_11 s_1_16
        let s_1_17: Bits = (Bits::new(
            ((s_1_12) >> (s_1_11)).value(),
            u16::try_from(s_1_16).unwrap(),
        ));
        // D s_1_18: cast reint s_1_17 -> u32
        let s_1_18: u32 = (s_1_17.value() as u32);
        // C s_1_19: const #32s : i
        let s_1_19: i128 = 32;
        // D s_1_20: read-var operand1:u128
        let s_1_20: u128 = fn_state.operand1;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 128u16);
        // C s_1_22: const #1s : i64
        let s_1_22: i64 = 1;
        // C s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // C s_1_24: const #95s : i
        let s_1_24: i128 = 95;
        // C s_1_25: add s_1_24 s_1_23
        let s_1_25: i128 = (s_1_24 + s_1_23);
        // D s_1_26: bit-extract s_1_21 s_1_19 s_1_25
        let s_1_26: Bits = (Bits::new(
            ((s_1_21) >> (s_1_19)).value(),
            u16::try_from(s_1_25).unwrap(),
        ));
        // D s_1_27: cast reint s_1_26 -> u96
        let s_1_27: u128 = (s_1_26.value() as u128);
        // D s_1_28: cast zx s_1_18 -> bv
        let s_1_28: Bits = Bits::new(s_1_18 as u128, 32u16);
        // D s_1_29: cast zx s_1_27 -> bv
        let s_1_29: Bits = Bits::new(s_1_27 as u128, 96u16);
        // D s_1_30: cast reint s_1_28 -> u128
        let s_1_30: u128 = (s_1_28.value() as u128);
        // D s_1_31: size-of s_1_28
        let s_1_31: u16 = s_1_28.length();
        // D s_1_32: cast reint s_1_29 -> u128
        let s_1_32: u128 = (s_1_29.value() as u128);
        // D s_1_33: size-of s_1_29
        let s_1_33: u16 = s_1_29.length();
        // D s_1_34: lsl s_1_30 s_1_33
        let s_1_34: u128 = s_1_30 << s_1_33;
        // D s_1_35: or s_1_34 s_1_32
        let s_1_35: u128 = ((s_1_34) | (s_1_32));
        // D s_1_36: add s_1_31 s_1_33
        let s_1_36: u16 = (s_1_31 + s_1_33);
        // D s_1_37: create-bits s_1_35 s_1_36
        let s_1_37: Bits = Bits::new(s_1_35, s_1_36);
        // D s_1_38: cast reint s_1_37 -> u128
        let s_1_38: u128 = (s_1_37.value() as u128);
        // D s_1_39: write-var T <= s_1_38
        fn_state.T = s_1_38;
        // C s_1_40: const #0s : i64
        let s_1_40: i64 = 0;
        // D s_1_41: write-var e <= s_1_40
        fn_state.e = s_1_40;
        // N s_1_42: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // C s_2_1: const #3s : i64
        let s_2_1: i64 = 3;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
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
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: read-var T:u128
        let s_3_1: u128 = fn_state.T;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 128u16);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: cast zx s_3_0 -> i
        let s_3_5: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_6: call Elem_read(s_3_2, s_3_4, s_3_5)
        let s_3_6: Bits = Elem_read(state, tracer, s_3_2, s_3_4, s_3_5);
        // D s_3_7: cast reint s_3_6 -> u32
        let s_3_7: u32 = (s_3_6.value() as u32);
        // C s_3_8: const #7s : i
        let s_3_8: i128 = 7;
        // D s_3_9: cast zx s_3_7 -> bv
        let s_3_9: Bits = Bits::new(s_3_7 as u128, 32u16);
        // D s_3_10: call ROR(s_3_9, s_3_8)
        let s_3_10: Bits = ROR(state, tracer, s_3_9, s_3_8);
        // D s_3_11: cast reint s_3_10 -> u32
        let s_3_11: u32 = (s_3_10.value() as u32);
        // C s_3_12: const #18s : i
        let s_3_12: i128 = 18;
        // D s_3_13: cast zx s_3_7 -> bv
        let s_3_13: Bits = Bits::new(s_3_7 as u128, 32u16);
        // D s_3_14: call ROR(s_3_13, s_3_12)
        let s_3_14: Bits = ROR(state, tracer, s_3_13, s_3_12);
        // D s_3_15: cast reint s_3_14 -> u32
        let s_3_15: u32 = (s_3_14.value() as u32);
        // D s_3_16: cast zx s_3_11 -> bv
        let s_3_16: Bits = Bits::new(s_3_11 as u128, 32u16);
        // D s_3_17: cast zx s_3_15 -> bv
        let s_3_17: Bits = Bits::new(s_3_15 as u128, 32u16);
        // D s_3_18: xor s_3_16 s_3_17
        let s_3_18: Bits = ((s_3_16) ^ (s_3_17));
        // D s_3_19: cast reint s_3_18 -> u32
        let s_3_19: u32 = (s_3_18.value() as u32);
        // C s_3_20: const #3s : i
        let s_3_20: i128 = 3;
        // D s_3_21: cast zx s_3_7 -> bv
        let s_3_21: Bits = Bits::new(s_3_7 as u128, 32u16);
        // D s_3_22: lsr s_3_21 s_3_20
        let s_3_22: Bits = s_3_21 >> s_3_20;
        // D s_3_23: cast reint s_3_22 -> u32
        let s_3_23: u32 = (s_3_22.value() as u32);
        // D s_3_24: cast zx s_3_19 -> bv
        let s_3_24: Bits = Bits::new(s_3_19 as u128, 32u16);
        // D s_3_25: cast zx s_3_23 -> bv
        let s_3_25: Bits = Bits::new(s_3_23 as u128, 32u16);
        // D s_3_26: xor s_3_24 s_3_25
        let s_3_26: Bits = ((s_3_24) ^ (s_3_25));
        // D s_3_27: cast reint s_3_26 -> u32
        let s_3_27: u32 = (s_3_26.value() as u32);
        // C s_3_28: const #32s : i64
        let s_3_28: i64 = 32;
        // C s_3_29: const #32s : i64
        let s_3_29: i64 = 32;
        // D s_3_30: read-var operand1:u128
        let s_3_30: u128 = fn_state.operand1;
        // D s_3_31: cast zx s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 128u16);
        // D s_3_32: read-var e:i64
        let s_3_32: i64 = fn_state.e;
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (i128::try_from(s_3_32).unwrap());
        // C s_3_34: cast zx s_3_29 -> i
        let s_3_34: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_35: call Elem_read(s_3_31, s_3_33, s_3_34)
        let s_3_35: Bits = Elem_read(state, tracer, s_3_31, s_3_33, s_3_34);
        // D s_3_36: cast reint s_3_35 -> u32
        let s_3_36: u32 = (s_3_35.value() as u32);
        // D s_3_37: cast zx s_3_27 -> bv
        let s_3_37: Bits = Bits::new(s_3_27 as u128, 32u16);
        // D s_3_38: cast zx s_3_36 -> bv
        let s_3_38: Bits = Bits::new(s_3_36 as u128, 32u16);
        // D s_3_39: add s_3_37 s_3_38
        let s_3_39: Bits = (s_3_37 + s_3_38);
        // D s_3_40: cast reint s_3_39 -> u32
        let s_3_40: u32 = (s_3_39.value() as u32);
        // D s_3_41: read-var result:u128
        let s_3_41: u128 = fn_state.result;
        // D s_3_42: cast zx s_3_41 -> bv
        let s_3_42: Bits = Bits::new(s_3_41 as u128, 128u16);
        // D s_3_43: read-var e:i64
        let s_3_43: i64 = fn_state.e;
        // D s_3_44: cast zx s_3_43 -> i
        let s_3_44: i128 = (i128::try_from(s_3_43).unwrap());
        // C s_3_45: cast zx s_3_28 -> i
        let s_3_45: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_46: cast zx s_3_40 -> bv
        let s_3_46: Bits = Bits::new(s_3_40 as u128, 32u16);
        // D s_3_47: call Elem_set(s_3_42, s_3_44, s_3_45, s_3_46)
        let s_3_47: Bits = Elem_set(state, tracer, s_3_42, s_3_44, s_3_45, s_3_46);
        // D s_3_48: cast reint s_3_47 -> u128
        let s_3_48: u128 = (s_3_47.value() as u128);
        // D s_3_49: write-var result <= s_3_48
        fn_state.result = s_3_48;
        // D s_3_50: read-var e:i64
        let s_3_50: i64 = fn_state.e;
        // C s_3_51: const #1s : i64
        let s_3_51: i64 = 1;
        // D s_3_52: add s_3_50 s_3_51
        let s_3_52: i64 = (s_3_50 + s_3_51);
        // D s_3_53: write-var e <= s_3_52
        fn_state.e = s_3_52;
        // N s_3_54: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: read-var d:i64
        let s_4_1: i64 = fn_state.d;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: read-var result:u128
        let s_4_3: u128 = fn_state.result;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 128u16);
        // D s_4_5: call V_set(s_4_2, s_4_0, s_4_4)
        let s_4_5: () = V_set(state, tracer, s_4_2, s_4_0, s_4_4);
        // N s_4_6: return
        return;
    }
}
