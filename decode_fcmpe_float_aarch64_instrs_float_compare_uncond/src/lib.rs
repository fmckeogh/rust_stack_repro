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
use execute_aarch64_instrs_float_compare_uncond::*;
use common::*;
pub fn decode_fcmpe_float_aarch64_instrs_float_compare_uncond<T: Tracer>(
    state: &mut State,
    tracer: &T,
    opc: u8,
    Rn: u8,
    Rm: u8,
    ftype: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        datasize: i64,
        n: i64,
        opc: u8,
        Rn: u8,
        Rm: u8,
        ftype: u8,
    }
    let fn_state = FunctionState {
        opc,
        Rn,
        Rm,
        ftype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rn:u8
        let s_0_0: u8 = fn_state.Rn;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var Rm:u8
        let s_0_5: u8 = fn_state.Rm;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var m <= s_0_8
        fn_state.m = s_0_8;
        // C s_0_10: const #16s : i64
        let s_0_10: i64 = 16;
        // D s_0_11: write-var datasize <= s_0_10
        fn_state.datasize = s_0_10;
        // D s_0_12: read-var ftype:u8
        let s_0_12: u8 = fn_state.ftype;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // C s_0_14: const #0u : u8
        let s_0_14: u8 = 0;
        // C s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_13 s_0_15
        let s_0_16: bool = ((s_0_13) == (s_0_15));
        // D s_0_17: not s_0_16
        let s_0_17: bool = !s_0_16;
        // N s_0_18: branch s_0_17 b3 b1
        if s_0_17 {
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
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var datasize <= s_1_0
        fn_state.datasize = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasize:i64
        let s_2_0: i64 = fn_state.datasize;
        // C s_2_1: const #1s : i
        let s_2_1: i128 = 1;
        // D s_2_2: read-var opc:u8
        let s_2_2: u8 = fn_state.opc;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // C s_2_4: const #1u : u64
        let s_2_4: u64 = 1;
        // D s_2_5: bit-extract s_2_3 s_2_1 s_2_4
        let s_2_5: Bits = (Bits::new(
            ((s_2_3) >> (s_2_1)).value(),
            u16::try_from(s_2_4).unwrap(),
        ));
        // D s_2_6: cast reint s_2_5 -> u8
        let s_2_6: bool = ((s_2_5.value()) != 0);
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // C s_2_8: const #0u : u64
        let s_2_8: u64 = 0;
        // D s_2_9: cast zx s_2_6 -> u64
        let s_2_9: u64 = (s_2_6 as u64);
        // C s_2_10: const #1u : u64
        let s_2_10: u64 = 1;
        // D s_2_11: and s_2_9 s_2_10
        let s_2_11: u64 = ((s_2_9) & (s_2_10));
        // D s_2_12: cmp-eq s_2_11 s_2_10
        let s_2_12: bool = ((s_2_11) == (s_2_10));
        // D s_2_13: lsl s_2_9 s_2_7
        let s_2_13: u64 = s_2_9 << s_2_7;
        // D s_2_14: or s_2_8 s_2_13
        let s_2_14: u64 = ((s_2_8) | (s_2_13));
        // D s_2_15: cmpl s_2_13
        let s_2_15: u64 = !s_2_13;
        // D s_2_16: and s_2_8 s_2_15
        let s_2_16: u64 = ((s_2_8) & (s_2_15));
        // D s_2_17: select s_2_12 s_2_14 s_2_16
        let s_2_17: u64 = if s_2_12 { s_2_14 } else { s_2_16 };
        // D s_2_18: cast trunc s_2_17 -> u8
        let s_2_18: bool = ((s_2_17) != 0);
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // C s_2_20: const #1u : u8
        let s_2_20: bool = true;
        // C s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 1u16);
        // D s_2_22: cmp-eq s_2_19 s_2_21
        let s_2_22: bool = ((s_2_19) == (s_2_21));
        // C s_2_23: const #0s : i
        let s_2_23: i128 = 0;
        // D s_2_24: read-var opc:u8
        let s_2_24: u8 = fn_state.opc;
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 2u16);
        // C s_2_26: const #1u : u64
        let s_2_26: u64 = 1;
        // D s_2_27: bit-extract s_2_25 s_2_23 s_2_26
        let s_2_27: Bits = (Bits::new(
            ((s_2_25) >> (s_2_23)).value(),
            u16::try_from(s_2_26).unwrap(),
        ));
        // D s_2_28: cast reint s_2_27 -> u8
        let s_2_28: bool = ((s_2_27.value()) != 0);
        // C s_2_29: const #0s : i
        let s_2_29: i128 = 0;
        // C s_2_30: const #0u : u64
        let s_2_30: u64 = 0;
        // D s_2_31: cast zx s_2_28 -> u64
        let s_2_31: u64 = (s_2_28 as u64);
        // C s_2_32: const #1u : u64
        let s_2_32: u64 = 1;
        // D s_2_33: and s_2_31 s_2_32
        let s_2_33: u64 = ((s_2_31) & (s_2_32));
        // D s_2_34: cmp-eq s_2_33 s_2_32
        let s_2_34: bool = ((s_2_33) == (s_2_32));
        // D s_2_35: lsl s_2_31 s_2_29
        let s_2_35: u64 = s_2_31 << s_2_29;
        // D s_2_36: or s_2_30 s_2_35
        let s_2_36: u64 = ((s_2_30) | (s_2_35));
        // D s_2_37: cmpl s_2_35
        let s_2_37: u64 = !s_2_35;
        // D s_2_38: and s_2_30 s_2_37
        let s_2_38: u64 = ((s_2_30) & (s_2_37));
        // D s_2_39: select s_2_34 s_2_36 s_2_38
        let s_2_39: u64 = if s_2_34 { s_2_36 } else { s_2_38 };
        // D s_2_40: cast trunc s_2_39 -> u8
        let s_2_40: bool = ((s_2_39) != 0);
        // D s_2_41: cast zx s_2_40 -> bv
        let s_2_41: Bits = Bits::new(s_2_40 as u128, 1u16);
        // C s_2_42: const #1u : u8
        let s_2_42: bool = true;
        // C s_2_43: cast zx s_2_42 -> bv
        let s_2_43: Bits = Bits::new(s_2_42 as u128, 1u16);
        // D s_2_44: cmp-eq s_2_41 s_2_43
        let s_2_44: bool = ((s_2_41) == (s_2_43));
        // D s_2_45: cast zx s_2_0 -> i
        let s_2_45: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_46: cast reint s_2_45 -> i64
        let s_2_46: i64 = (s_2_45 as i64);
        // D s_2_47: read-var m:i64
        let s_2_47: i64 = fn_state.m;
        // D s_2_48: read-var n:i64
        let s_2_48: i64 = fn_state.n;
        // D s_2_49: call execute_aarch64_instrs_float_compare_uncond(s_2_44, s_2_46, s_2_47, s_2_48, s_2_22)
        let s_2_49: () = execute_aarch64_instrs_float_compare_uncond(
            state,
            tracer,
            s_2_44,
            s_2_46,
            s_2_47,
            s_2_48,
            s_2_22,
        );
        // N s_2_50: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ftype:u8
        let s_3_0: u8 = fn_state.ftype;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: write-var datasize <= s_4_0
        fn_state.datasize = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ftype:u8
        let s_5_0: u8 = fn_state.ftype;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
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
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16s : i64
        let s_7_0: i64 = 16;
        // D s_7_1: write-var datasize <= s_7_0
        fn_state.datasize = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
