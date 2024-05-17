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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_shift_sisd::*;
use common::*;
pub fn decode_sshl_advsimd_aarch64_instrs_vector_arithmetic_binary_uniform_shift_sisd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    S: bool,
    R: bool,
    Rm: u8,
    size: u8,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        is_unsigned: bool,
        rounding: bool,
        gs_171454: bool,
        saturating: bool,
        datasize: i64,
        esize: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        S: bool,
        R: bool,
        Rm: u8,
        size: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        S,
        R,
        Rm,
        size,
        U,
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
        // D s_0_10: read-var Rm:u8
        let s_0_10: u8 = fn_state.Rm;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var m <= s_0_13
        fn_state.m = s_0_13;
        // D s_0_15: read-var size:u8
        let s_0_15: u8 = fn_state.size;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (s_0_16.value() as i128);
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // C s_0_19: const #8s : i64
        let s_0_19: i64 = 8;
        // D s_0_20: lsl s_0_19 s_0_18
        let s_0_20: i64 = s_0_19 << s_0_18;
        // D s_0_21: write-var esize <= s_0_20
        fn_state.esize = s_0_20;
        // D s_0_22: read-var esize:i64
        let s_0_22: i64 = fn_state.esize;
        // D s_0_23: write-var datasize <= s_0_22
        fn_state.datasize = s_0_22;
        // D s_0_24: read-var U:u8
        let s_0_24: bool = fn_state.U;
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 1u16);
        // C s_0_26: const #1u : u8
        let s_0_26: bool = true;
        // C s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 1u16);
        // D s_0_28: cmp-eq s_0_25 s_0_27
        let s_0_28: bool = ((s_0_25) == (s_0_27));
        // D s_0_29: write-var is_unsigned <= s_0_28
        fn_state.is_unsigned = s_0_28;
        // D s_0_30: read-var R:u8
        let s_0_30: bool = fn_state.R;
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 1u16);
        // C s_0_32: const #1u : u8
        let s_0_32: bool = true;
        // C s_0_33: cast zx s_0_32 -> bv
        let s_0_33: Bits = Bits::new(s_0_32 as u128, 1u16);
        // D s_0_34: cmp-eq s_0_31 s_0_33
        let s_0_34: bool = ((s_0_31) == (s_0_33));
        // D s_0_35: write-var rounding <= s_0_34
        fn_state.rounding = s_0_34;
        // D s_0_36: read-var S:u8
        let s_0_36: bool = fn_state.S;
        // D s_0_37: cast zx s_0_36 -> bv
        let s_0_37: Bits = Bits::new(s_0_36 as u128, 1u16);
        // C s_0_38: const #1u : u8
        let s_0_38: bool = true;
        // C s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 1u16);
        // D s_0_40: cmp-eq s_0_37 s_0_39
        let s_0_40: bool = ((s_0_37) == (s_0_39));
        // D s_0_41: write-var saturating <= s_0_40
        fn_state.saturating = s_0_40;
        // D s_0_42: read-var S:u8
        let s_0_42: bool = fn_state.S;
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 1u16);
        // C s_0_44: const #0u : u8
        let s_0_44: bool = false;
        // C s_0_45: cast zx s_0_44 -> bv
        let s_0_45: Bits = Bits::new(s_0_44 as u128, 1u16);
        // D s_0_46: cmp-eq s_0_43 s_0_45
        let s_0_46: bool = ((s_0_43) == (s_0_45));
        // N s_0_47: branch s_0_46 b5 b1
        if s_0_46 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#171454 <= s_1_0
        fn_state.gs_171454 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#171454:u8
        let s_2_0: bool = fn_state.gs_171454;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // D s_3_0: read-var datasize:i64
        let s_3_0: i64 = fn_state.datasize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var esize:i64
        let s_3_3: i64 = fn_state.esize;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // C s_3_6: const #1s : i
        let s_3_6: i128 = 1;
        // D s_3_7: read-var d:i64
        let s_3_7: i64 = fn_state.d;
        // D s_3_8: read-var m:i64
        let s_3_8: i64 = fn_state.m;
        // D s_3_9: read-var n:i64
        let s_3_9: i64 = fn_state.n;
        // D s_3_10: read-var rounding:u8
        let s_3_10: bool = fn_state.rounding;
        // D s_3_11: read-var saturating:u8
        let s_3_11: bool = fn_state.saturating;
        // D s_3_12: read-var is_unsigned:u8
        let s_3_12: bool = fn_state.is_unsigned;
        // D s_3_13: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_shift_sisd(s_3_7, s_3_2, s_3_6, s_3_5, s_3_8, s_3_9, s_3_10, s_3_11, s_3_12)
        let s_3_13: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_shift_sisd(
            state,
            tracer,
            s_3_7,
            s_3_2,
            s_3_6,
            s_3_5,
            s_3_8,
            s_3_9,
            s_3_10,
            s_3_11,
            s_3_12,
        );
        // N s_3_14: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #3u : u8
        let s_5_2: u8 = 3;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-ne s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) != (s_5_3));
        // D s_5_5: write-var gs#171454 <= s_5_4
        fn_state.gs_171454 = s_5_4;
        // N s_5_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
