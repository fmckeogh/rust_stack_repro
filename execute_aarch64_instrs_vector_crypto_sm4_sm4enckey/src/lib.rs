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
use ROL::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use Sbox::*;
use V_set::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sm4_sm4enckey<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        index: i64,
        intval: u32,
        Vm: u128,
        roundresult: u128,
        i: i64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
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
        // D s_1_1: read-var m:i64
        let s_1_1: i64 = fn_state.m;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // D s_1_5: write-var Vm <= s_1_4
        fn_state.Vm = s_1_4;
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
        // D s_1_11: write-var roundresult <= s_1_10
        fn_state.roundresult = s_1_10;
        // C s_1_12: const #0s : i64
        let s_1_12: i64 = 0;
        // D s_1_13: write-var index <= s_1_12
        fn_state.index = s_1_12;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var index:i64
        let s_2_0: i64 = fn_state.index;
        // C s_2_1: const #3s : i64
        let s_2_1: i64 = 3;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_1: read-var Vm:u128
        let s_3_1: u128 = fn_state.Vm;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 128u16);
        // D s_3_3: read-var index:i64
        let s_3_3: i64 = fn_state.index;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: cast zx s_3_0 -> i
        let s_3_5: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_6: call Elem_read(s_3_2, s_3_4, s_3_5)
        let s_3_6: Bits = Elem_read(state, tracer, s_3_2, s_3_4, s_3_5);
        // D s_3_7: cast reint s_3_6 -> u32
        let s_3_7: u32 = (s_3_6.value() as u32);
        // C s_3_8: const #96s : i
        let s_3_8: i128 = 96;
        // D s_3_9: read-var roundresult:u128
        let s_3_9: u128 = fn_state.roundresult;
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 128u16);
        // C s_3_11: const #1s : i64
        let s_3_11: i64 = 1;
        // C s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // C s_3_13: const #31s : i
        let s_3_13: i128 = 31;
        // C s_3_14: add s_3_13 s_3_12
        let s_3_14: i128 = (s_3_13 + s_3_12);
        // D s_3_15: bit-extract s_3_10 s_3_8 s_3_14
        let s_3_15: Bits = (Bits::new(
            ((s_3_10) >> (s_3_8)).value(),
            u16::try_from(s_3_14).unwrap(),
        ));
        // D s_3_16: cast reint s_3_15 -> u32
        let s_3_16: u32 = (s_3_15.value() as u32);
        // C s_3_17: const #64s : i
        let s_3_17: i128 = 64;
        // D s_3_18: read-var roundresult:u128
        let s_3_18: u128 = fn_state.roundresult;
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 128u16);
        // C s_3_20: const #1s : i64
        let s_3_20: i64 = 1;
        // C s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // C s_3_22: const #31s : i
        let s_3_22: i128 = 31;
        // C s_3_23: add s_3_22 s_3_21
        let s_3_23: i128 = (s_3_22 + s_3_21);
        // D s_3_24: bit-extract s_3_19 s_3_17 s_3_23
        let s_3_24: Bits = (Bits::new(
            ((s_3_19) >> (s_3_17)).value(),
            u16::try_from(s_3_23).unwrap(),
        ));
        // D s_3_25: cast reint s_3_24 -> u32
        let s_3_25: u32 = (s_3_24.value() as u32);
        // D s_3_26: cast zx s_3_16 -> bv
        let s_3_26: Bits = Bits::new(s_3_16 as u128, 32u16);
        // D s_3_27: cast zx s_3_25 -> bv
        let s_3_27: Bits = Bits::new(s_3_25 as u128, 32u16);
        // D s_3_28: xor s_3_26 s_3_27
        let s_3_28: Bits = ((s_3_26) ^ (s_3_27));
        // D s_3_29: cast reint s_3_28 -> u32
        let s_3_29: u32 = (s_3_28.value() as u32);
        // C s_3_30: const #32s : i
        let s_3_30: i128 = 32;
        // D s_3_31: read-var roundresult:u128
        let s_3_31: u128 = fn_state.roundresult;
        // D s_3_32: cast zx s_3_31 -> bv
        let s_3_32: Bits = Bits::new(s_3_31 as u128, 128u16);
        // C s_3_33: const #1s : i64
        let s_3_33: i64 = 1;
        // C s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (i128::try_from(s_3_33).unwrap());
        // C s_3_35: const #31s : i
        let s_3_35: i128 = 31;
        // C s_3_36: add s_3_35 s_3_34
        let s_3_36: i128 = (s_3_35 + s_3_34);
        // D s_3_37: bit-extract s_3_32 s_3_30 s_3_36
        let s_3_37: Bits = (Bits::new(
            ((s_3_32) >> (s_3_30)).value(),
            u16::try_from(s_3_36).unwrap(),
        ));
        // D s_3_38: cast reint s_3_37 -> u32
        let s_3_38: u32 = (s_3_37.value() as u32);
        // D s_3_39: cast zx s_3_29 -> bv
        let s_3_39: Bits = Bits::new(s_3_29 as u128, 32u16);
        // D s_3_40: cast zx s_3_38 -> bv
        let s_3_40: Bits = Bits::new(s_3_38 as u128, 32u16);
        // D s_3_41: xor s_3_39 s_3_40
        let s_3_41: Bits = ((s_3_39) ^ (s_3_40));
        // D s_3_42: cast reint s_3_41 -> u32
        let s_3_42: u32 = (s_3_41.value() as u32);
        // D s_3_43: cast zx s_3_42 -> bv
        let s_3_43: Bits = Bits::new(s_3_42 as u128, 32u16);
        // D s_3_44: cast zx s_3_7 -> bv
        let s_3_44: Bits = Bits::new(s_3_7 as u128, 32u16);
        // D s_3_45: xor s_3_43 s_3_44
        let s_3_45: Bits = ((s_3_43) ^ (s_3_44));
        // D s_3_46: cast reint s_3_45 -> u32
        let s_3_46: u32 = (s_3_45.value() as u32);
        // D s_3_47: write-var intval <= s_3_46
        fn_state.intval = s_3_46;
        // C s_3_48: const #0s : i64
        let s_3_48: i64 = 0;
        // D s_3_49: write-var i <= s_3_48
        fn_state.i = s_3_48;
        // N s_3_50: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
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
        // C s_5_0: const #8s : i64
        let s_5_0: i64 = 8;
        // C s_5_1: const #8s : i64
        let s_5_1: i64 = 8;
        // D s_5_2: read-var intval:u32
        let s_5_2: u32 = fn_state.intval;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_4: read-var i:i64
        let s_5_4: i64 = fn_state.i;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: cast zx s_5_1 -> i
        let s_5_6: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_7: call Elem_read(s_5_3, s_5_5, s_5_6)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_3, s_5_5, s_5_6);
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // D s_5_9: call Sbox(s_5_8)
        let s_5_9: u8 = Sbox(state, tracer, s_5_8);
        // D s_5_10: read-var intval:u32
        let s_5_10: u32 = fn_state.intval;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 32u16);
        // D s_5_12: read-var i:i64
        let s_5_12: i64 = fn_state.i;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // C s_5_14: cast zx s_5_0 -> i
        let s_5_14: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_15: cast zx s_5_9 -> bv
        let s_5_15: Bits = Bits::new(s_5_9 as u128, 8u16);
        // D s_5_16: call Elem_set(s_5_11, s_5_13, s_5_14, s_5_15)
        let s_5_16: Bits = Elem_set(state, tracer, s_5_11, s_5_13, s_5_14, s_5_15);
        // D s_5_17: cast reint s_5_16 -> u32
        let s_5_17: u32 = (s_5_16.value() as u32);
        // D s_5_18: write-var intval <= s_5_17
        fn_state.intval = s_5_17;
        // D s_5_19: read-var i:i64
        let s_5_19: i64 = fn_state.i;
        // C s_5_20: const #1s : i64
        let s_5_20: i64 = 1;
        // D s_5_21: add s_5_19 s_5_20
        let s_5_21: i64 = (s_5_19 + s_5_20);
        // D s_5_22: write-var i <= s_5_21
        fn_state.i = s_5_21;
        // N s_5_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #13s : i
        let s_6_0: i128 = 13;
        // D s_6_1: read-var intval:u32
        let s_6_1: u32 = fn_state.intval;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_3: call ROL(s_6_2, s_6_0)
        let s_6_3: Bits = ROL(state, tracer, s_6_2, s_6_0);
        // D s_6_4: cast reint s_6_3 -> u32
        let s_6_4: u32 = (s_6_3.value() as u32);
        // D s_6_5: read-var intval:u32
        let s_6_5: u32 = fn_state.intval;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 32u16);
        // D s_6_7: cast zx s_6_4 -> bv
        let s_6_7: Bits = Bits::new(s_6_4 as u128, 32u16);
        // D s_6_8: xor s_6_6 s_6_7
        let s_6_8: Bits = ((s_6_6) ^ (s_6_7));
        // D s_6_9: cast reint s_6_8 -> u32
        let s_6_9: u32 = (s_6_8.value() as u32);
        // C s_6_10: const #23s : i
        let s_6_10: i128 = 23;
        // D s_6_11: read-var intval:u32
        let s_6_11: u32 = fn_state.intval;
        // D s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 32u16);
        // D s_6_13: call ROL(s_6_12, s_6_10)
        let s_6_13: Bits = ROL(state, tracer, s_6_12, s_6_10);
        // D s_6_14: cast reint s_6_13 -> u32
        let s_6_14: u32 = (s_6_13.value() as u32);
        // D s_6_15: cast zx s_6_9 -> bv
        let s_6_15: Bits = Bits::new(s_6_9 as u128, 32u16);
        // D s_6_16: cast zx s_6_14 -> bv
        let s_6_16: Bits = Bits::new(s_6_14 as u128, 32u16);
        // D s_6_17: xor s_6_15 s_6_16
        let s_6_17: Bits = ((s_6_15) ^ (s_6_16));
        // D s_6_18: cast reint s_6_17 -> u32
        let s_6_18: u32 = (s_6_17.value() as u32);
        // D s_6_19: write-var intval <= s_6_18
        fn_state.intval = s_6_18;
        // C s_6_20: const #0s : i
        let s_6_20: i128 = 0;
        // D s_6_21: read-var roundresult:u128
        let s_6_21: u128 = fn_state.roundresult;
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 128u16);
        // C s_6_23: const #1s : i64
        let s_6_23: i64 = 1;
        // C s_6_24: cast zx s_6_23 -> i
        let s_6_24: i128 = (i128::try_from(s_6_23).unwrap());
        // C s_6_25: const #31s : i
        let s_6_25: i128 = 31;
        // C s_6_26: add s_6_25 s_6_24
        let s_6_26: i128 = (s_6_25 + s_6_24);
        // D s_6_27: bit-extract s_6_22 s_6_20 s_6_26
        let s_6_27: Bits = (Bits::new(
            ((s_6_22) >> (s_6_20)).value(),
            u16::try_from(s_6_26).unwrap(),
        ));
        // D s_6_28: cast reint s_6_27 -> u32
        let s_6_28: u32 = (s_6_27.value() as u32);
        // D s_6_29: read-var intval:u32
        let s_6_29: u32 = fn_state.intval;
        // D s_6_30: cast zx s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 32u16);
        // D s_6_31: cast zx s_6_28 -> bv
        let s_6_31: Bits = Bits::new(s_6_28 as u128, 32u16);
        // D s_6_32: xor s_6_30 s_6_31
        let s_6_32: Bits = ((s_6_30) ^ (s_6_31));
        // D s_6_33: cast reint s_6_32 -> u32
        let s_6_33: u32 = (s_6_32.value() as u32);
        // D s_6_34: write-var intval <= s_6_33
        fn_state.intval = s_6_33;
        // C s_6_35: const #32s : i
        let s_6_35: i128 = 32;
        // D s_6_36: read-var roundresult:u128
        let s_6_36: u128 = fn_state.roundresult;
        // D s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 128u16);
        // C s_6_38: const #1s : i64
        let s_6_38: i64 = 1;
        // C s_6_39: cast zx s_6_38 -> i
        let s_6_39: i128 = (i128::try_from(s_6_38).unwrap());
        // C s_6_40: const #31s : i
        let s_6_40: i128 = 31;
        // C s_6_41: add s_6_40 s_6_39
        let s_6_41: i128 = (s_6_40 + s_6_39);
        // D s_6_42: bit-extract s_6_37 s_6_35 s_6_41
        let s_6_42: Bits = (Bits::new(
            ((s_6_37) >> (s_6_35)).value(),
            u16::try_from(s_6_41).unwrap(),
        ));
        // D s_6_43: cast reint s_6_42 -> u32
        let s_6_43: u32 = (s_6_42.value() as u32);
        // C s_6_44: const #0s : i
        let s_6_44: i128 = 0;
        // D s_6_45: read-var roundresult:u128
        let s_6_45: u128 = fn_state.roundresult;
        // D s_6_46: cast zx s_6_45 -> bv
        let s_6_46: Bits = Bits::new(s_6_45 as u128, 128u16);
        // D s_6_47: cast zx s_6_43 -> bv
        let s_6_47: Bits = Bits::new(s_6_43 as u128, 32u16);
        // C s_6_48: const #31s : i
        let s_6_48: i128 = 31;
        // C s_6_49: const #1u : u64
        let s_6_49: u64 = 1;
        // C s_6_50: cast zx s_6_49 -> bv
        let s_6_50: Bits = Bits::new(s_6_49 as u128, 64u16);
        // C s_6_51: lsl s_6_50 s_6_48
        let s_6_51: Bits = s_6_50 << s_6_48;
        // C s_6_52: sub s_6_51 s_6_50
        let s_6_52: Bits = ((s_6_51) - (s_6_50));
        // D s_6_53: and s_6_47 s_6_52
        let s_6_53: Bits = ((s_6_47) & (s_6_52));
        // D s_6_54: lsl s_6_53 s_6_44
        let s_6_54: Bits = s_6_53 << s_6_44;
        // C s_6_55: lsl s_6_52 s_6_44
        let s_6_55: Bits = s_6_52 << s_6_44;
        // C s_6_56: cmpl s_6_55
        let s_6_56: Bits = !s_6_55;
        // D s_6_57: and s_6_46 s_6_56
        let s_6_57: Bits = ((s_6_46) & (s_6_56));
        // D s_6_58: or s_6_57 s_6_54
        let s_6_58: Bits = ((s_6_57) | (s_6_54));
        // D s_6_59: cast reint s_6_58 -> u128
        let s_6_59: u128 = (s_6_58.value() as u128);
        // D s_6_60: write-var roundresult <= s_6_59
        fn_state.roundresult = s_6_59;
        // C s_6_61: const #64s : i
        let s_6_61: i128 = 64;
        // D s_6_62: read-var roundresult:u128
        let s_6_62: u128 = fn_state.roundresult;
        // D s_6_63: cast zx s_6_62 -> bv
        let s_6_63: Bits = Bits::new(s_6_62 as u128, 128u16);
        // C s_6_64: const #1s : i64
        let s_6_64: i64 = 1;
        // C s_6_65: cast zx s_6_64 -> i
        let s_6_65: i128 = (i128::try_from(s_6_64).unwrap());
        // C s_6_66: const #31s : i
        let s_6_66: i128 = 31;
        // C s_6_67: add s_6_66 s_6_65
        let s_6_67: i128 = (s_6_66 + s_6_65);
        // D s_6_68: bit-extract s_6_63 s_6_61 s_6_67
        let s_6_68: Bits = (Bits::new(
            ((s_6_63) >> (s_6_61)).value(),
            u16::try_from(s_6_67).unwrap(),
        ));
        // D s_6_69: cast reint s_6_68 -> u32
        let s_6_69: u32 = (s_6_68.value() as u32);
        // C s_6_70: const #32s : i
        let s_6_70: i128 = 32;
        // D s_6_71: read-var roundresult:u128
        let s_6_71: u128 = fn_state.roundresult;
        // D s_6_72: cast zx s_6_71 -> bv
        let s_6_72: Bits = Bits::new(s_6_71 as u128, 128u16);
        // D s_6_73: cast zx s_6_69 -> bv
        let s_6_73: Bits = Bits::new(s_6_69 as u128, 32u16);
        // C s_6_74: const #31s : i
        let s_6_74: i128 = 31;
        // C s_6_75: const #1u : u64
        let s_6_75: u64 = 1;
        // C s_6_76: cast zx s_6_75 -> bv
        let s_6_76: Bits = Bits::new(s_6_75 as u128, 64u16);
        // C s_6_77: lsl s_6_76 s_6_74
        let s_6_77: Bits = s_6_76 << s_6_74;
        // C s_6_78: sub s_6_77 s_6_76
        let s_6_78: Bits = ((s_6_77) - (s_6_76));
        // D s_6_79: and s_6_73 s_6_78
        let s_6_79: Bits = ((s_6_73) & (s_6_78));
        // D s_6_80: lsl s_6_79 s_6_70
        let s_6_80: Bits = s_6_79 << s_6_70;
        // C s_6_81: lsl s_6_78 s_6_70
        let s_6_81: Bits = s_6_78 << s_6_70;
        // C s_6_82: cmpl s_6_81
        let s_6_82: Bits = !s_6_81;
        // D s_6_83: and s_6_72 s_6_82
        let s_6_83: Bits = ((s_6_72) & (s_6_82));
        // D s_6_84: or s_6_83 s_6_80
        let s_6_84: Bits = ((s_6_83) | (s_6_80));
        // D s_6_85: cast reint s_6_84 -> u128
        let s_6_85: u128 = (s_6_84.value() as u128);
        // D s_6_86: write-var roundresult <= s_6_85
        fn_state.roundresult = s_6_85;
        // C s_6_87: const #96s : i
        let s_6_87: i128 = 96;
        // D s_6_88: read-var roundresult:u128
        let s_6_88: u128 = fn_state.roundresult;
        // D s_6_89: cast zx s_6_88 -> bv
        let s_6_89: Bits = Bits::new(s_6_88 as u128, 128u16);
        // C s_6_90: const #1s : i64
        let s_6_90: i64 = 1;
        // C s_6_91: cast zx s_6_90 -> i
        let s_6_91: i128 = (i128::try_from(s_6_90).unwrap());
        // C s_6_92: const #31s : i
        let s_6_92: i128 = 31;
        // C s_6_93: add s_6_92 s_6_91
        let s_6_93: i128 = (s_6_92 + s_6_91);
        // D s_6_94: bit-extract s_6_89 s_6_87 s_6_93
        let s_6_94: Bits = (Bits::new(
            ((s_6_89) >> (s_6_87)).value(),
            u16::try_from(s_6_93).unwrap(),
        ));
        // D s_6_95: cast reint s_6_94 -> u32
        let s_6_95: u32 = (s_6_94.value() as u32);
        // C s_6_96: const #64s : i
        let s_6_96: i128 = 64;
        // D s_6_97: read-var roundresult:u128
        let s_6_97: u128 = fn_state.roundresult;
        // D s_6_98: cast zx s_6_97 -> bv
        let s_6_98: Bits = Bits::new(s_6_97 as u128, 128u16);
        // D s_6_99: cast zx s_6_95 -> bv
        let s_6_99: Bits = Bits::new(s_6_95 as u128, 32u16);
        // C s_6_100: const #31s : i
        let s_6_100: i128 = 31;
        // C s_6_101: const #1u : u64
        let s_6_101: u64 = 1;
        // C s_6_102: cast zx s_6_101 -> bv
        let s_6_102: Bits = Bits::new(s_6_101 as u128, 64u16);
        // C s_6_103: lsl s_6_102 s_6_100
        let s_6_103: Bits = s_6_102 << s_6_100;
        // C s_6_104: sub s_6_103 s_6_102
        let s_6_104: Bits = ((s_6_103) - (s_6_102));
        // D s_6_105: and s_6_99 s_6_104
        let s_6_105: Bits = ((s_6_99) & (s_6_104));
        // D s_6_106: lsl s_6_105 s_6_96
        let s_6_106: Bits = s_6_105 << s_6_96;
        // C s_6_107: lsl s_6_104 s_6_96
        let s_6_107: Bits = s_6_104 << s_6_96;
        // C s_6_108: cmpl s_6_107
        let s_6_108: Bits = !s_6_107;
        // D s_6_109: and s_6_98 s_6_108
        let s_6_109: Bits = ((s_6_98) & (s_6_108));
        // D s_6_110: or s_6_109 s_6_106
        let s_6_110: Bits = ((s_6_109) | (s_6_106));
        // D s_6_111: cast reint s_6_110 -> u128
        let s_6_111: u128 = (s_6_110.value() as u128);
        // D s_6_112: write-var roundresult <= s_6_111
        fn_state.roundresult = s_6_111;
        // C s_6_113: const #96s : i
        let s_6_113: i128 = 96;
        // D s_6_114: read-var roundresult:u128
        let s_6_114: u128 = fn_state.roundresult;
        // D s_6_115: cast zx s_6_114 -> bv
        let s_6_115: Bits = Bits::new(s_6_114 as u128, 128u16);
        // D s_6_116: read-var intval:u32
        let s_6_116: u32 = fn_state.intval;
        // D s_6_117: cast zx s_6_116 -> bv
        let s_6_117: Bits = Bits::new(s_6_116 as u128, 32u16);
        // C s_6_118: const #31s : i
        let s_6_118: i128 = 31;
        // C s_6_119: const #1u : u64
        let s_6_119: u64 = 1;
        // C s_6_120: cast zx s_6_119 -> bv
        let s_6_120: Bits = Bits::new(s_6_119 as u128, 64u16);
        // C s_6_121: lsl s_6_120 s_6_118
        let s_6_121: Bits = s_6_120 << s_6_118;
        // C s_6_122: sub s_6_121 s_6_120
        let s_6_122: Bits = ((s_6_121) - (s_6_120));
        // D s_6_123: and s_6_117 s_6_122
        let s_6_123: Bits = ((s_6_117) & (s_6_122));
        // D s_6_124: lsl s_6_123 s_6_113
        let s_6_124: Bits = s_6_123 << s_6_113;
        // C s_6_125: lsl s_6_122 s_6_113
        let s_6_125: Bits = s_6_122 << s_6_113;
        // C s_6_126: cmpl s_6_125
        let s_6_126: Bits = !s_6_125;
        // D s_6_127: and s_6_115 s_6_126
        let s_6_127: Bits = ((s_6_115) & (s_6_126));
        // D s_6_128: or s_6_127 s_6_124
        let s_6_128: Bits = ((s_6_127) | (s_6_124));
        // D s_6_129: cast reint s_6_128 -> u128
        let s_6_129: u128 = (s_6_128.value() as u128);
        // D s_6_130: write-var roundresult <= s_6_129
        fn_state.roundresult = s_6_129;
        // D s_6_131: read-var index:i64
        let s_6_131: i64 = fn_state.index;
        // C s_6_132: const #1s : i64
        let s_6_132: i64 = 1;
        // D s_6_133: add s_6_131 s_6_132
        let s_6_133: i64 = (s_6_131 + s_6_132);
        // D s_6_134: write-var index <= s_6_133
        fn_state.index = s_6_133;
        // N s_6_135: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #128s : i64
        let s_7_0: i64 = 128;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var roundresult:u128
        let s_7_3: u128 = fn_state.roundresult;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 128u16);
        // D s_7_5: call V_set(s_7_2, s_7_0, s_7_4)
        let s_7_5: () = V_set(state, tracer, s_7_2, s_7_0, s_7_4);
        // N s_7_6: return
        return;
    }
}
