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
use SP_read::*;
use X_read::*;
use Hint_RangePrefetch::*;
use common::*;
pub fn execute_aarch64_instrs_memory_single_general_range<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    n: i64,
    operation: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        stride: i64,
        reuse: i128,
        address: u64,
        metadata: u64,
        count: i64,
        length: i64,
        m: i64,
        n: i64,
        operation: u8,
    }
    let fn_state = FunctionState {
        m,
        n,
        operation,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #31s : i
        let s_0_0: i128 = 31;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b6 b1
        if s_0_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var address <= s_1_4
        fn_state.address = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var m:i64
        let s_2_1: i64 = fn_state.m;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var metadata <= s_2_4
        fn_state.metadata = s_2_4;
        // C s_2_6: const #38s : i
        let s_2_6: i128 = 38;
        // D s_2_7: read-var metadata:u64
        let s_2_7: u64 = fn_state.metadata;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 64u16);
        // C s_2_9: const #1s : i64
        let s_2_9: i64 = 1;
        // C s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // C s_2_11: const #21s : i
        let s_2_11: i128 = 21;
        // C s_2_12: add s_2_11 s_2_10
        let s_2_12: i128 = (s_2_11 + s_2_10);
        // D s_2_13: bit-extract s_2_8 s_2_6 s_2_12
        let s_2_13: Bits = (Bits::new(
            ((s_2_8) >> (s_2_6)).value(),
            u16::try_from(s_2_12).unwrap(),
        ));
        // D s_2_14: cast reint s_2_13 -> u22
        let s_2_14: u32 = (s_2_13.value() as u32);
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 22u16);
        // D s_2_16: cast sx s_2_15 -> i
        let s_2_16: i128 = {
            let sign_bit = s_2_15.length() - 1;
            let mut result = s_2_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_17: cast reint s_2_16 -> i64
        let s_2_17: i64 = (s_2_16 as i64);
        // D s_2_18: write-var stride <= s_2_17
        fn_state.stride = s_2_17;
        // C s_2_19: const #22s : i
        let s_2_19: i128 = 22;
        // D s_2_20: read-var metadata:u64
        let s_2_20: u64 = fn_state.metadata;
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 64u16);
        // C s_2_22: const #1s : i64
        let s_2_22: i64 = 1;
        // C s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // C s_2_24: const #15s : i
        let s_2_24: i128 = 15;
        // C s_2_25: add s_2_24 s_2_23
        let s_2_25: i128 = (s_2_24 + s_2_23);
        // D s_2_26: bit-extract s_2_21 s_2_19 s_2_25
        let s_2_26: Bits = (Bits::new(
            ((s_2_21) >> (s_2_19)).value(),
            u16::try_from(s_2_25).unwrap(),
        ));
        // D s_2_27: cast reint s_2_26 -> u16
        let s_2_27: u16 = (s_2_26.value() as u16);
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 16u16);
        // D s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (s_2_28.value() as i128);
        // D s_2_30: cast reint s_2_29 -> i64
        let s_2_30: i64 = (s_2_29 as i64);
        // C s_2_31: const #1s : i
        let s_2_31: i128 = 1;
        // D s_2_32: cast zx s_2_30 -> i
        let s_2_32: i128 = (i128::try_from(s_2_30).unwrap());
        // D s_2_33: add s_2_32 s_2_31
        let s_2_33: i128 = (s_2_32 + s_2_31);
        // D s_2_34: cast reint s_2_33 -> i64
        let s_2_34: i64 = (s_2_33 as i64);
        // D s_2_35: write-var count <= s_2_34
        fn_state.count = s_2_34;
        // C s_2_36: const #0s : i
        let s_2_36: i128 = 0;
        // D s_2_37: read-var metadata:u64
        let s_2_37: u64 = fn_state.metadata;
        // D s_2_38: cast zx s_2_37 -> bv
        let s_2_38: Bits = Bits::new(s_2_37 as u128, 64u16);
        // C s_2_39: const #1s : i64
        let s_2_39: i64 = 1;
        // C s_2_40: cast zx s_2_39 -> i
        let s_2_40: i128 = (i128::try_from(s_2_39).unwrap());
        // C s_2_41: const #21s : i
        let s_2_41: i128 = 21;
        // C s_2_42: add s_2_41 s_2_40
        let s_2_42: i128 = (s_2_41 + s_2_40);
        // D s_2_43: bit-extract s_2_38 s_2_36 s_2_42
        let s_2_43: Bits = (Bits::new(
            ((s_2_38) >> (s_2_36)).value(),
            u16::try_from(s_2_42).unwrap(),
        ));
        // D s_2_44: cast reint s_2_43 -> u22
        let s_2_44: u32 = (s_2_43.value() as u32);
        // D s_2_45: cast zx s_2_44 -> bv
        let s_2_45: Bits = Bits::new(s_2_44 as u128, 22u16);
        // D s_2_46: cast sx s_2_45 -> i
        let s_2_46: i128 = {
            let sign_bit = s_2_45.length() - 1;
            let mut result = s_2_45.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_47: cast reint s_2_46 -> i64
        let s_2_47: i64 = (s_2_46 as i64);
        // D s_2_48: write-var length <= s_2_47
        fn_state.length = s_2_47;
        // C s_2_49: const #60s : i
        let s_2_49: i128 = 60;
        // D s_2_50: read-var metadata:u64
        let s_2_50: u64 = fn_state.metadata;
        // D s_2_51: cast zx s_2_50 -> bv
        let s_2_51: Bits = Bits::new(s_2_50 as u128, 64u16);
        // C s_2_52: const #1s : i64
        let s_2_52: i64 = 1;
        // C s_2_53: cast zx s_2_52 -> i
        let s_2_53: i128 = (i128::try_from(s_2_52).unwrap());
        // C s_2_54: const #3s : i
        let s_2_54: i128 = 3;
        // C s_2_55: add s_2_54 s_2_53
        let s_2_55: i128 = (s_2_54 + s_2_53);
        // D s_2_56: bit-extract s_2_51 s_2_49 s_2_55
        let s_2_56: Bits = (Bits::new(
            ((s_2_51) >> (s_2_49)).value(),
            u16::try_from(s_2_55).unwrap(),
        ));
        // D s_2_57: cast reint s_2_56 -> u8
        let s_2_57: u8 = (s_2_56.value() as u8);
        // D s_2_58: cast zx s_2_57 -> bv
        let s_2_58: Bits = Bits::new(s_2_57 as u128, 4u16);
        // C s_2_59: const #0u : u8
        let s_2_59: u8 = 0;
        // C s_2_60: cast zx s_2_59 -> bv
        let s_2_60: Bits = Bits::new(s_2_59 as u128, 4u16);
        // D s_2_61: cmp-eq s_2_58 s_2_60
        let s_2_61: bool = ((s_2_58) == (s_2_60));
        // N s_2_62: branch s_2_61 b5 b3
        if s_2_61 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #60s : i
        let s_3_0: i128 = 60;
        // D s_3_1: read-var metadata:u64
        let s_3_1: u64 = fn_state.metadata;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #3s : i
        let s_3_5: i128 = 3;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 4u16);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (s_3_9.value() as i128);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // C s_3_12: const #15s : i
        let s_3_12: i128 = 15;
        // D s_3_13: cast zx s_3_11 -> i
        let s_3_13: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_14: sub s_3_12 s_3_13
        let s_3_14: i128 = ((s_3_12) - (s_3_13));
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // C s_3_16: const #32768s : i
        let s_3_16: i128 = 32768;
        // D s_3_17: cast zx s_3_15 -> i
        let s_3_17: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_18: lsl s_3_16 s_3_17
        let s_3_18: i128 = s_3_16 << s_3_17;
        // D s_3_19: write-var reuse <= s_3_18
        fn_state.reuse = s_3_18;
        // N s_3_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var reuse:i
        let s_4_0: i128 = fn_state.reuse;
        // D s_4_1: read-var length:i64
        let s_4_1: i64 = fn_state.length;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: read-var stride:i64
        let s_4_3: i64 = fn_state.stride;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var count:i64
        let s_4_5: i64 = fn_state.count;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: read-var address:u64
        let s_4_7: u64 = fn_state.address;
        // D s_4_8: read-var operation:u8
        let s_4_8: u8 = fn_state.operation;
        // D s_4_9: call Hint_RangePrefetch(s_4_7, s_4_2, s_4_4, s_4_6, s_4_0, s_4_8)
        let s_4_9: () = Hint_RangePrefetch(
            state,
            tracer,
            s_4_7,
            s_4_2,
            s_4_4,
            s_4_6,
            s_4_0,
            s_4_8,
        );
        // N s_4_10: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // C s_5_1: neg s_5_0
        let s_5_1: i128 = -s_5_0;
        // D s_5_2: write-var reuse <= s_5_1
        fn_state.reuse = s_5_1;
        // N s_5_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call SP_read(s_6_0)
        let s_6_1: u64 = SP_read(state, tracer, s_6_0);
        // D s_6_2: write-var address <= s_6_1
        fn_state.address = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
