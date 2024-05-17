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
use u__id::*;
use HighestSetBit::*;
use u_shl_int_general::*;
use execute_aarch64_instrs_vector_shift_right_narrow_logical::*;
use fdiv_int::*;
use common::*;
pub fn decode_rshrn_advsimd_aarch64_instrs_vector_shift_right_narrow_logical<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op: bool,
    immb: u8,
    immh: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i128,
        shift: i128,
        part: i64,
        n: i64,
        gs_166862: bool,
        round: bool,
        d: i64,
        gs_166864: bool,
        gs_166866: bool,
        elements: i128,
        gs_166868: bool,
        Rd: u8,
        Rn: u8,
        op: bool,
        immb: u8,
        immh: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        immb,
        immh,
        Q,
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
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var immh:u8
        let s_0_10: u8 = fn_state.immh;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 4u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b16 b1
        if s_0_14 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #3s : i
        let s_1_0: i128 = 3;
        // D s_1_1: read-var immh:u8
        let s_1_1: u8 = fn_state.immh;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 4u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #1u : u8
        let s_1_19: bool = true;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // N s_1_22: branch s_1_21 b15 b2
        if s_1_21 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var immh:u8
        let s_2_0: u8 = fn_state.immh;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: call HighestSetBit(s_2_1)
        let s_2_2: i128 = HighestSetBit(state, tracer, s_2_1);
        // C s_2_3: const #8s : i
        let s_2_3: i128 = 8;
        // D s_2_4: call _shl_int_general(s_2_3, s_2_2)
        let s_2_4: i128 = u_shl_int_general(state, tracer, s_2_3, s_2_2);
        // D s_2_5: write-var esize <= s_2_4
        fn_state.esize = s_2_4;
        // D s_2_6: read-var Q:u8
        let s_2_6: bool = fn_state.Q;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var part <= s_2_9
        fn_state.part = s_2_9;
        // C s_2_11: const #64s : i
        let s_2_11: i128 = 64;
        // D s_2_12: read-var esize:i
        let s_2_12: i128 = fn_state.esize;
        // D s_2_13: call fdiv_int(s_2_11, s_2_12)
        let s_2_13: i128 = fdiv_int(state, tracer, s_2_11, s_2_12);
        // D s_2_14: write-var elements <= s_2_13
        fn_state.elements = s_2_13;
        // C s_2_15: const #2s : i
        let s_2_15: i128 = 2;
        // D s_2_16: read-var esize:i
        let s_2_16: i128 = fn_state.esize;
        // D s_2_17: mul s_2_15 s_2_16
        let s_2_17: i128 = ((s_2_15) * (s_2_16));
        // D s_2_18: read-var immh:u8
        let s_2_18: u8 = fn_state.immh;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 4u16);
        // D s_2_20: read-var immb:u8
        let s_2_20: u8 = fn_state.immb;
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 3u16);
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // D s_2_24: cast reint s_2_21 -> u128
        let s_2_24: u128 = (s_2_21.value() as u128);
        // D s_2_25: size-of s_2_21
        let s_2_25: u16 = s_2_21.length();
        // D s_2_26: lsl s_2_22 s_2_25
        let s_2_26: u128 = s_2_22 << s_2_25;
        // D s_2_27: or s_2_26 s_2_24
        let s_2_27: u128 = ((s_2_26) | (s_2_24));
        // D s_2_28: add s_2_23 s_2_25
        let s_2_28: u16 = (s_2_23 + s_2_25);
        // D s_2_29: create-bits s_2_27 s_2_28
        let s_2_29: Bits = Bits::new(s_2_27, s_2_28);
        // D s_2_30: cast reint s_2_29 -> u8
        let s_2_30: u8 = (s_2_29.value() as u8);
        // D s_2_31: cast zx s_2_30 -> bv
        let s_2_31: Bits = Bits::new(s_2_30 as u128, 7u16);
        // D s_2_32: cast zx s_2_31 -> i
        let s_2_32: i128 = (s_2_31.value() as i128);
        // D s_2_33: cast reint s_2_32 -> i64
        let s_2_33: i64 = (s_2_32 as i64);
        // D s_2_34: cast zx s_2_33 -> i
        let s_2_34: i128 = (i128::try_from(s_2_33).unwrap());
        // D s_2_35: sub s_2_17 s_2_34
        let s_2_35: i128 = ((s_2_17) - (s_2_34));
        // D s_2_36: write-var shift <= s_2_35
        fn_state.shift = s_2_35;
        // D s_2_37: read-var op:u8
        let s_2_37: bool = fn_state.op;
        // D s_2_38: cast zx s_2_37 -> bv
        let s_2_38: Bits = Bits::new(s_2_37 as u128, 1u16);
        // C s_2_39: const #1u : u8
        let s_2_39: bool = true;
        // C s_2_40: cast zx s_2_39 -> bv
        let s_2_40: Bits = Bits::new(s_2_39 as u128, 1u16);
        // D s_2_41: cmp-eq s_2_38 s_2_40
        let s_2_41: bool = ((s_2_38) == (s_2_40));
        // D s_2_42: write-var round <= s_2_41
        fn_state.round = s_2_41;
        // D s_2_43: read-var esize:i
        let s_2_43: i128 = fn_state.esize;
        // D s_2_44: call __id(s_2_43)
        let s_2_44: i128 = u__id(state, tracer, s_2_43);
        // C s_2_45: const #4s : i
        let s_2_45: i128 = 4;
        // D s_2_46: cmp-eq s_2_44 s_2_45
        let s_2_46: bool = ((s_2_44) == (s_2_45));
        // N s_2_47: branch s_2_46 b14 b3
        if s_2_46 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esize:i
        let s_3_0: i128 = fn_state.esize;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #8s : i
        let s_3_2: i128 = 8;
        // D s_3_3: cmp-eq s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) == (s_3_2));
        // D s_3_4: write-var gs#166862 <= s_3_3
        fn_state.gs_166862 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#166862:u8
        let s_4_0: bool = fn_state.gs_166862;
        // N s_4_1: branch s_4_0 b13 b5
        if s_4_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i
        let s_5_0: i128 = fn_state.esize;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // C s_5_2: const #16s : i
        let s_5_2: i128 = 16;
        // D s_5_3: cmp-eq s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) == (s_5_2));
        // D s_5_4: write-var gs#166864 <= s_5_3
        fn_state.gs_166864 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#166864:u8
        let s_6_0: bool = fn_state.gs_166864;
        // N s_6_1: branch s_6_0 b12 b7
        if s_6_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i
        let s_7_0: i128 = fn_state.esize;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #32s : i
        let s_7_2: i128 = 32;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: write-var gs#166866 <= s_7_3
        fn_state.gs_166866 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#166866:u8
        let s_8_0: bool = fn_state.gs_166866;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i
        let s_9_0: i128 = fn_state.esize;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // C s_9_2: const #64s : i
        let s_9_2: i128 = 64;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: write-var gs#166868 <= s_9_3
        fn_state.gs_166868 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#166868:u8
        let s_10_0: bool = fn_state.gs_166868;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // D s_10_2: read-var esize:i
        let s_10_2: i128 = fn_state.esize;
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #64s : i64
        let s_10_4: i64 = 64;
        // D s_10_5: read-var d:i64
        let s_10_5: i64 = fn_state.d;
        // D s_10_6: read-var elements:i
        let s_10_6: i128 = fn_state.elements;
        // D s_10_7: read-var n:i64
        let s_10_7: i64 = fn_state.n;
        // D s_10_8: read-var part:i64
        let s_10_8: i64 = fn_state.part;
        // D s_10_9: read-var round:u8
        let s_10_9: bool = fn_state.round;
        // D s_10_10: read-var shift:i
        let s_10_10: i128 = fn_state.shift;
        // D s_10_11: call execute_aarch64_instrs_vector_shift_right_narrow_logical(s_10_5, s_10_4, s_10_6, s_10_3, s_10_7, s_10_8, s_10_9, s_10_10)
        let s_10_11: () = execute_aarch64_instrs_vector_shift_right_narrow_logical(
            state,
            tracer,
            s_10_5,
            s_10_4,
            s_10_6,
            s_10_3,
            s_10_7,
            s_10_8,
            s_10_9,
            s_10_10,
        );
        // N s_10_12: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#166868 <= s_11_0
        fn_state.gs_166868 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#166866 <= s_12_0
        fn_state.gs_166866 = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#166864 <= s_13_0
        fn_state.gs_166864 = s_13_0;
        // N s_13_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#166862 <= s_14_0
        fn_state.gs_166862 = s_14_0;
        // N s_14_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
}
