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
use Q_set::*;
use Q_read::*;
use Elem_set::*;
use Elem_read::*;
use CheckCryptoEnabled32::*;
use ROR::*;
use common::*;
pub fn execute_aarch32_instrs_SHA256SU1_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        u_9571: i64,
        elt: u32,
        result: u128,
        T0: u128,
        T1shadow_7912: u64,
        x: u128,
        T1: u64,
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
        // S s_0_1: call CheckCryptoEnabled32(s_0_0)
        let s_0_1: () = CheckCryptoEnabled32(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i64
        let s_1_0: i64 = 1;
        // D s_1_1: read-var d:i64
        let s_1_1: i64 = fn_state.d;
        // D s_1_2: lsr s_1_1 s_1_0
        let s_1_2: i64 = s_1_1 >> s_1_0;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Q_read(s_1_3)
        let s_1_4: u128 = Q_read(state, tracer, s_1_3);
        // D s_1_5: write-var x <= s_1_4
        fn_state.x = s_1_4;
        // C s_1_6: const #1s : i64
        let s_1_6: i64 = 1;
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: lsr s_1_7 s_1_6
        let s_1_8: i64 = s_1_7 >> s_1_6;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: call Q_read(s_1_9)
        let s_1_10: u128 = Q_read(state, tracer, s_1_9);
        // C s_1_11: const #1s : i64
        let s_1_11: i64 = 1;
        // D s_1_12: read-var m:i64
        let s_1_12: i64 = fn_state.m;
        // D s_1_13: lsr s_1_12 s_1_11
        let s_1_13: i64 = s_1_12 >> s_1_11;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: call Q_read(s_1_14)
        let s_1_15: u128 = Q_read(state, tracer, s_1_14);
        // C s_1_16: const #0s : i
        let s_1_16: i128 = 0;
        // D s_1_17: cast zx s_1_15 -> bv
        let s_1_17: Bits = Bits::new(s_1_15 as u128, 128u16);
        // C s_1_18: const #1s : i64
        let s_1_18: i64 = 1;
        // C s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // C s_1_20: const #31s : i
        let s_1_20: i128 = 31;
        // C s_1_21: add s_1_20 s_1_19
        let s_1_21: i128 = (s_1_20 + s_1_19);
        // D s_1_22: bit-extract s_1_17 s_1_16 s_1_21
        let s_1_22: Bits = (Bits::new(
            ((s_1_17) >> (s_1_16)).value(),
            u16::try_from(s_1_21).unwrap(),
        ));
        // D s_1_23: cast reint s_1_22 -> u32
        let s_1_23: u32 = (s_1_22.value() as u32);
        // C s_1_24: const #32s : i
        let s_1_24: i128 = 32;
        // D s_1_25: cast zx s_1_10 -> bv
        let s_1_25: Bits = Bits::new(s_1_10 as u128, 128u16);
        // C s_1_26: const #1s : i64
        let s_1_26: i64 = 1;
        // C s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // C s_1_28: const #95s : i
        let s_1_28: i128 = 95;
        // C s_1_29: add s_1_28 s_1_27
        let s_1_29: i128 = (s_1_28 + s_1_27);
        // D s_1_30: bit-extract s_1_25 s_1_24 s_1_29
        let s_1_30: Bits = (Bits::new(
            ((s_1_25) >> (s_1_24)).value(),
            u16::try_from(s_1_29).unwrap(),
        ));
        // D s_1_31: cast reint s_1_30 -> u96
        let s_1_31: u128 = (s_1_30.value() as u128);
        // D s_1_32: cast zx s_1_23 -> bv
        let s_1_32: Bits = Bits::new(s_1_23 as u128, 32u16);
        // D s_1_33: cast zx s_1_31 -> bv
        let s_1_33: Bits = Bits::new(s_1_31 as u128, 96u16);
        // D s_1_34: cast reint s_1_32 -> u128
        let s_1_34: u128 = (s_1_32.value() as u128);
        // D s_1_35: size-of s_1_32
        let s_1_35: u16 = s_1_32.length();
        // D s_1_36: cast reint s_1_33 -> u128
        let s_1_36: u128 = (s_1_33.value() as u128);
        // D s_1_37: size-of s_1_33
        let s_1_37: u16 = s_1_33.length();
        // D s_1_38: lsl s_1_34 s_1_37
        let s_1_38: u128 = s_1_34 << s_1_37;
        // D s_1_39: or s_1_38 s_1_36
        let s_1_39: u128 = ((s_1_38) | (s_1_36));
        // D s_1_40: add s_1_35 s_1_37
        let s_1_40: u16 = (s_1_35 + s_1_37);
        // D s_1_41: create-bits s_1_39 s_1_40
        let s_1_41: Bits = Bits::new(s_1_39, s_1_40);
        // D s_1_42: cast reint s_1_41 -> u128
        let s_1_42: u128 = (s_1_41.value() as u128);
        // D s_1_43: write-var T0 <= s_1_42
        fn_state.T0 = s_1_42;
        // C s_1_44: const #64s : i
        let s_1_44: i128 = 64;
        // D s_1_45: cast zx s_1_15 -> bv
        let s_1_45: Bits = Bits::new(s_1_15 as u128, 128u16);
        // C s_1_46: const #1s : i64
        let s_1_46: i64 = 1;
        // C s_1_47: cast zx s_1_46 -> i
        let s_1_47: i128 = (i128::try_from(s_1_46).unwrap());
        // C s_1_48: const #63s : i
        let s_1_48: i128 = 63;
        // C s_1_49: add s_1_48 s_1_47
        let s_1_49: i128 = (s_1_48 + s_1_47);
        // D s_1_50: bit-extract s_1_45 s_1_44 s_1_49
        let s_1_50: Bits = (Bits::new(
            ((s_1_45) >> (s_1_44)).value(),
            u16::try_from(s_1_49).unwrap(),
        ));
        // D s_1_51: cast reint s_1_50 -> u64
        let s_1_51: u64 = (s_1_50.value() as u64);
        // D s_1_52: write-var T1 <= s_1_51
        fn_state.T1 = s_1_51;
        // C s_1_53: const #0s : i64
        let s_1_53: i64 = 0;
        // D s_1_54: write-var e <= s_1_53
        fn_state.e = s_1_53;
        // N s_1_55: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // C s_2_1: const #1s : i64
        let s_2_1: i64 = 1;
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
        // D s_3_1: read-var T1:u64
        let s_3_1: u64 = fn_state.T1;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
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
        // D s_3_8: write-var elt <= s_3_7
        fn_state.elt = s_3_7;
        // C s_3_9: const #17s : i
        let s_3_9: i128 = 17;
        // D s_3_10: read-var elt:u32
        let s_3_10: u32 = fn_state.elt;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 32u16);
        // D s_3_12: call ROR(s_3_11, s_3_9)
        let s_3_12: Bits = ROR(state, tracer, s_3_11, s_3_9);
        // D s_3_13: cast reint s_3_12 -> u32
        let s_3_13: u32 = (s_3_12.value() as u32);
        // C s_3_14: const #19s : i
        let s_3_14: i128 = 19;
        // D s_3_15: read-var elt:u32
        let s_3_15: u32 = fn_state.elt;
        // D s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 32u16);
        // D s_3_17: call ROR(s_3_16, s_3_14)
        let s_3_17: Bits = ROR(state, tracer, s_3_16, s_3_14);
        // D s_3_18: cast reint s_3_17 -> u32
        let s_3_18: u32 = (s_3_17.value() as u32);
        // D s_3_19: cast zx s_3_13 -> bv
        let s_3_19: Bits = Bits::new(s_3_13 as u128, 32u16);
        // D s_3_20: cast zx s_3_18 -> bv
        let s_3_20: Bits = Bits::new(s_3_18 as u128, 32u16);
        // D s_3_21: xor s_3_19 s_3_20
        let s_3_21: Bits = ((s_3_19) ^ (s_3_20));
        // D s_3_22: cast reint s_3_21 -> u32
        let s_3_22: u32 = (s_3_21.value() as u32);
        // C s_3_23: const #10s : i
        let s_3_23: i128 = 10;
        // D s_3_24: read-var elt:u32
        let s_3_24: u32 = fn_state.elt;
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 32u16);
        // D s_3_26: lsr s_3_25 s_3_23
        let s_3_26: Bits = s_3_25 >> s_3_23;
        // D s_3_27: cast reint s_3_26 -> u32
        let s_3_27: u32 = (s_3_26.value() as u32);
        // D s_3_28: cast zx s_3_22 -> bv
        let s_3_28: Bits = Bits::new(s_3_22 as u128, 32u16);
        // D s_3_29: cast zx s_3_27 -> bv
        let s_3_29: Bits = Bits::new(s_3_27 as u128, 32u16);
        // D s_3_30: xor s_3_28 s_3_29
        let s_3_30: Bits = ((s_3_28) ^ (s_3_29));
        // D s_3_31: cast reint s_3_30 -> u32
        let s_3_31: u32 = (s_3_30.value() as u32);
        // D s_3_32: write-var elt <= s_3_31
        fn_state.elt = s_3_31;
        // C s_3_33: const #32s : i64
        let s_3_33: i64 = 32;
        // D s_3_34: read-var x:u128
        let s_3_34: u128 = fn_state.x;
        // D s_3_35: cast zx s_3_34 -> bv
        let s_3_35: Bits = Bits::new(s_3_34 as u128, 128u16);
        // D s_3_36: read-var e:i64
        let s_3_36: i64 = fn_state.e;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // C s_3_38: cast zx s_3_33 -> i
        let s_3_38: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_39: call Elem_read(s_3_35, s_3_37, s_3_38)
        let s_3_39: Bits = Elem_read(state, tracer, s_3_35, s_3_37, s_3_38);
        // D s_3_40: cast reint s_3_39 -> u32
        let s_3_40: u32 = (s_3_39.value() as u32);
        // D s_3_41: read-var elt:u32
        let s_3_41: u32 = fn_state.elt;
        // D s_3_42: cast zx s_3_41 -> bv
        let s_3_42: Bits = Bits::new(s_3_41 as u128, 32u16);
        // D s_3_43: cast zx s_3_40 -> bv
        let s_3_43: Bits = Bits::new(s_3_40 as u128, 32u16);
        // D s_3_44: add s_3_42 s_3_43
        let s_3_44: Bits = (s_3_42 + s_3_43);
        // D s_3_45: cast reint s_3_44 -> u32
        let s_3_45: u32 = (s_3_44.value() as u32);
        // C s_3_46: const #32s : i64
        let s_3_46: i64 = 32;
        // D s_3_47: read-var T0:u128
        let s_3_47: u128 = fn_state.T0;
        // D s_3_48: cast zx s_3_47 -> bv
        let s_3_48: Bits = Bits::new(s_3_47 as u128, 128u16);
        // D s_3_49: read-var e:i64
        let s_3_49: i64 = fn_state.e;
        // D s_3_50: cast zx s_3_49 -> i
        let s_3_50: i128 = (i128::try_from(s_3_49).unwrap());
        // C s_3_51: cast zx s_3_46 -> i
        let s_3_51: i128 = (i128::try_from(s_3_46).unwrap());
        // D s_3_52: call Elem_read(s_3_48, s_3_50, s_3_51)
        let s_3_52: Bits = Elem_read(state, tracer, s_3_48, s_3_50, s_3_51);
        // D s_3_53: cast reint s_3_52 -> u32
        let s_3_53: u32 = (s_3_52.value() as u32);
        // D s_3_54: cast zx s_3_45 -> bv
        let s_3_54: Bits = Bits::new(s_3_45 as u128, 32u16);
        // D s_3_55: cast zx s_3_53 -> bv
        let s_3_55: Bits = Bits::new(s_3_53 as u128, 32u16);
        // D s_3_56: add s_3_54 s_3_55
        let s_3_56: Bits = (s_3_54 + s_3_55);
        // D s_3_57: cast reint s_3_56 -> u32
        let s_3_57: u32 = (s_3_56.value() as u32);
        // D s_3_58: write-var elt <= s_3_57
        fn_state.elt = s_3_57;
        // C s_3_59: const #32s : i64
        let s_3_59: i64 = 32;
        // D s_3_60: read-var result:u128
        let s_3_60: u128 = fn_state.result;
        // D s_3_61: cast zx s_3_60 -> bv
        let s_3_61: Bits = Bits::new(s_3_60 as u128, 128u16);
        // D s_3_62: read-var e:i64
        let s_3_62: i64 = fn_state.e;
        // D s_3_63: cast zx s_3_62 -> i
        let s_3_63: i128 = (i128::try_from(s_3_62).unwrap());
        // C s_3_64: cast zx s_3_59 -> i
        let s_3_64: i128 = (i128::try_from(s_3_59).unwrap());
        // D s_3_65: read-var elt:u32
        let s_3_65: u32 = fn_state.elt;
        // D s_3_66: cast zx s_3_65 -> bv
        let s_3_66: Bits = Bits::new(s_3_65 as u128, 32u16);
        // D s_3_67: call Elem_set(s_3_61, s_3_63, s_3_64, s_3_66)
        let s_3_67: Bits = Elem_set(state, tracer, s_3_61, s_3_63, s_3_64, s_3_66);
        // D s_3_68: cast reint s_3_67 -> u128
        let s_3_68: u128 = (s_3_67.value() as u128);
        // D s_3_69: write-var result <= s_3_68
        fn_state.result = s_3_68;
        // D s_3_70: read-var e:i64
        let s_3_70: i64 = fn_state.e;
        // C s_3_71: const #1s : i64
        let s_3_71: i64 = 1;
        // D s_3_72: add s_3_70 s_3_71
        let s_3_72: i64 = (s_3_70 + s_3_71);
        // D s_3_73: write-var e <= s_3_72
        fn_state.e = s_3_72;
        // N s_3_74: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var result:u128
        let s_4_1: u128 = fn_state.result;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 128u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #63s : i
        let s_4_5: i128 = 63;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u64
        let s_4_8: u64 = (s_4_7.value() as u64);
        // D s_4_9: write-var T1shadow#7912 <= s_4_8
        fn_state.T1shadow_7912 = s_4_8;
        // C s_4_10: const #2s : i64
        let s_4_10: i64 = 2;
        // D s_4_11: write-var u#9571 <= s_4_10
        fn_state.u_9571 = s_4_10;
        // N s_4_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var u#9571:i64
        let s_5_0: i64 = fn_state.u_9571;
        // C s_5_1: const #3s : i64
        let s_5_1: i64 = 3;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var u#9571:i64
        let s_6_1: i64 = fn_state.u_9571;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: sub s_6_2 s_6_0
        let s_6_3: i128 = ((s_6_2) - (s_6_0));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // C s_6_5: const #32s : i64
        let s_6_5: i64 = 32;
        // D s_6_6: read-var T1shadow#7912:u64
        let s_6_6: u64 = fn_state.T1shadow_7912;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 64u16);
        // D s_6_8: cast zx s_6_4 -> i
        let s_6_8: i128 = (i128::try_from(s_6_4).unwrap());
        // C s_6_9: cast zx s_6_5 -> i
        let s_6_9: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_10: call Elem_read(s_6_7, s_6_8, s_6_9)
        let s_6_10: Bits = Elem_read(state, tracer, s_6_7, s_6_8, s_6_9);
        // D s_6_11: cast reint s_6_10 -> u32
        let s_6_11: u32 = (s_6_10.value() as u32);
        // D s_6_12: write-var elt <= s_6_11
        fn_state.elt = s_6_11;
        // C s_6_13: const #17s : i
        let s_6_13: i128 = 17;
        // D s_6_14: read-var elt:u32
        let s_6_14: u32 = fn_state.elt;
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 32u16);
        // D s_6_16: call ROR(s_6_15, s_6_13)
        let s_6_16: Bits = ROR(state, tracer, s_6_15, s_6_13);
        // D s_6_17: cast reint s_6_16 -> u32
        let s_6_17: u32 = (s_6_16.value() as u32);
        // C s_6_18: const #19s : i
        let s_6_18: i128 = 19;
        // D s_6_19: read-var elt:u32
        let s_6_19: u32 = fn_state.elt;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 32u16);
        // D s_6_21: call ROR(s_6_20, s_6_18)
        let s_6_21: Bits = ROR(state, tracer, s_6_20, s_6_18);
        // D s_6_22: cast reint s_6_21 -> u32
        let s_6_22: u32 = (s_6_21.value() as u32);
        // D s_6_23: cast zx s_6_17 -> bv
        let s_6_23: Bits = Bits::new(s_6_17 as u128, 32u16);
        // D s_6_24: cast zx s_6_22 -> bv
        let s_6_24: Bits = Bits::new(s_6_22 as u128, 32u16);
        // D s_6_25: xor s_6_23 s_6_24
        let s_6_25: Bits = ((s_6_23) ^ (s_6_24));
        // D s_6_26: cast reint s_6_25 -> u32
        let s_6_26: u32 = (s_6_25.value() as u32);
        // C s_6_27: const #10s : i
        let s_6_27: i128 = 10;
        // D s_6_28: read-var elt:u32
        let s_6_28: u32 = fn_state.elt;
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 32u16);
        // D s_6_30: lsr s_6_29 s_6_27
        let s_6_30: Bits = s_6_29 >> s_6_27;
        // D s_6_31: cast reint s_6_30 -> u32
        let s_6_31: u32 = (s_6_30.value() as u32);
        // D s_6_32: cast zx s_6_26 -> bv
        let s_6_32: Bits = Bits::new(s_6_26 as u128, 32u16);
        // D s_6_33: cast zx s_6_31 -> bv
        let s_6_33: Bits = Bits::new(s_6_31 as u128, 32u16);
        // D s_6_34: xor s_6_32 s_6_33
        let s_6_34: Bits = ((s_6_32) ^ (s_6_33));
        // D s_6_35: cast reint s_6_34 -> u32
        let s_6_35: u32 = (s_6_34.value() as u32);
        // D s_6_36: write-var elt <= s_6_35
        fn_state.elt = s_6_35;
        // C s_6_37: const #32s : i64
        let s_6_37: i64 = 32;
        // D s_6_38: read-var x:u128
        let s_6_38: u128 = fn_state.x;
        // D s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 128u16);
        // D s_6_40: read-var u#9571:i64
        let s_6_40: i64 = fn_state.u_9571;
        // D s_6_41: cast zx s_6_40 -> i
        let s_6_41: i128 = (i128::try_from(s_6_40).unwrap());
        // C s_6_42: cast zx s_6_37 -> i
        let s_6_42: i128 = (i128::try_from(s_6_37).unwrap());
        // D s_6_43: call Elem_read(s_6_39, s_6_41, s_6_42)
        let s_6_43: Bits = Elem_read(state, tracer, s_6_39, s_6_41, s_6_42);
        // D s_6_44: cast reint s_6_43 -> u32
        let s_6_44: u32 = (s_6_43.value() as u32);
        // D s_6_45: read-var elt:u32
        let s_6_45: u32 = fn_state.elt;
        // D s_6_46: cast zx s_6_45 -> bv
        let s_6_46: Bits = Bits::new(s_6_45 as u128, 32u16);
        // D s_6_47: cast zx s_6_44 -> bv
        let s_6_47: Bits = Bits::new(s_6_44 as u128, 32u16);
        // D s_6_48: add s_6_46 s_6_47
        let s_6_48: Bits = (s_6_46 + s_6_47);
        // D s_6_49: cast reint s_6_48 -> u32
        let s_6_49: u32 = (s_6_48.value() as u32);
        // C s_6_50: const #32s : i64
        let s_6_50: i64 = 32;
        // D s_6_51: read-var T0:u128
        let s_6_51: u128 = fn_state.T0;
        // D s_6_52: cast zx s_6_51 -> bv
        let s_6_52: Bits = Bits::new(s_6_51 as u128, 128u16);
        // D s_6_53: read-var u#9571:i64
        let s_6_53: i64 = fn_state.u_9571;
        // D s_6_54: cast zx s_6_53 -> i
        let s_6_54: i128 = (i128::try_from(s_6_53).unwrap());
        // C s_6_55: cast zx s_6_50 -> i
        let s_6_55: i128 = (i128::try_from(s_6_50).unwrap());
        // D s_6_56: call Elem_read(s_6_52, s_6_54, s_6_55)
        let s_6_56: Bits = Elem_read(state, tracer, s_6_52, s_6_54, s_6_55);
        // D s_6_57: cast reint s_6_56 -> u32
        let s_6_57: u32 = (s_6_56.value() as u32);
        // D s_6_58: cast zx s_6_49 -> bv
        let s_6_58: Bits = Bits::new(s_6_49 as u128, 32u16);
        // D s_6_59: cast zx s_6_57 -> bv
        let s_6_59: Bits = Bits::new(s_6_57 as u128, 32u16);
        // D s_6_60: add s_6_58 s_6_59
        let s_6_60: Bits = (s_6_58 + s_6_59);
        // D s_6_61: cast reint s_6_60 -> u32
        let s_6_61: u32 = (s_6_60.value() as u32);
        // D s_6_62: write-var elt <= s_6_61
        fn_state.elt = s_6_61;
        // C s_6_63: const #32s : i64
        let s_6_63: i64 = 32;
        // D s_6_64: read-var result:u128
        let s_6_64: u128 = fn_state.result;
        // D s_6_65: cast zx s_6_64 -> bv
        let s_6_65: Bits = Bits::new(s_6_64 as u128, 128u16);
        // D s_6_66: read-var u#9571:i64
        let s_6_66: i64 = fn_state.u_9571;
        // D s_6_67: cast zx s_6_66 -> i
        let s_6_67: i128 = (i128::try_from(s_6_66).unwrap());
        // C s_6_68: cast zx s_6_63 -> i
        let s_6_68: i128 = (i128::try_from(s_6_63).unwrap());
        // D s_6_69: read-var elt:u32
        let s_6_69: u32 = fn_state.elt;
        // D s_6_70: cast zx s_6_69 -> bv
        let s_6_70: Bits = Bits::new(s_6_69 as u128, 32u16);
        // D s_6_71: call Elem_set(s_6_65, s_6_67, s_6_68, s_6_70)
        let s_6_71: Bits = Elem_set(state, tracer, s_6_65, s_6_67, s_6_68, s_6_70);
        // D s_6_72: cast reint s_6_71 -> u128
        let s_6_72: u128 = (s_6_71.value() as u128);
        // D s_6_73: write-var result <= s_6_72
        fn_state.result = s_6_72;
        // D s_6_74: read-var u#9571:i64
        let s_6_74: i64 = fn_state.u_9571;
        // C s_6_75: const #1s : i64
        let s_6_75: i64 = 1;
        // D s_6_76: add s_6_74 s_6_75
        let s_6_76: i64 = (s_6_74 + s_6_75);
        // D s_6_77: write-var u#9571 <= s_6_76
        fn_state.u_9571 = s_6_76;
        // N s_6_78: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i64
        let s_7_0: i64 = 1;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // D s_7_2: lsr s_7_1 s_7_0
        let s_7_2: i64 = s_7_1 >> s_7_0;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var result:u128
        let s_7_4: u128 = fn_state.result;
        // D s_7_5: call Q_set(s_7_3, s_7_4)
        let s_7_5: () = Q_set(state, tracer, s_7_3, s_7_4);
        // N s_7_6: return
        return;
    }
}
