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
use D_set::*;
use Din_read::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch32_instrs_VUSDOT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        res: u32,
        gs_327266: i64,
        b: i64,
        result: u64,
        operand1: u64,
        operand2: u64,
        d: i64,
        m: i64,
        n: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        regs,
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
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#327266 <= s_1_5
        fn_state.gs_327266 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#327266:i64
        let s_2_1: i64 = fn_state.gs_327266;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: call Din_read(s_3_6)
        let s_3_7: u64 = Din_read(state, tracer, s_3_6);
        // D s_3_8: write-var operand1 <= s_3_7
        fn_state.operand1 = s_3_7;
        // D s_3_9: read-var m:i64
        let s_3_9: i64 = fn_state.m;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var r:i64
        let s_3_11: i64 = fn_state.r;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: add s_3_10 s_3_12
        let s_3_13: i128 = (s_3_10 + s_3_12);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: call Din_read(s_3_15)
        let s_3_16: u64 = Din_read(state, tracer, s_3_15);
        // D s_3_17: write-var operand2 <= s_3_16
        fn_state.operand2 = s_3_16;
        // D s_3_18: read-var d:i64
        let s_3_18: i64 = fn_state.d;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: read-var r:i64
        let s_3_20: i64 = fn_state.r;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: add s_3_19 s_3_21
        let s_3_22: i128 = (s_3_19 + s_3_21);
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: call Din_read(s_3_24)
        let s_3_25: u64 = Din_read(state, tracer, s_3_24);
        // D s_3_26: write-var result <= s_3_25
        fn_state.result = s_3_25;
        // C s_3_27: const #0s : i64
        let s_3_27: i64 = 0;
        // D s_3_28: write-var e <= s_3_27
        fn_state.e = s_3_27;
        // N s_3_29: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // D s_5_1: read-var result:u64
        let s_5_1: u64 = fn_state.result;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 64u16);
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: cast zx s_5_0 -> i
        let s_5_5: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_6: call Elem_read(s_5_2, s_5_4, s_5_5)
        let s_5_6: Bits = Elem_read(state, tracer, s_5_2, s_5_4, s_5_5);
        // D s_5_7: cast reint s_5_6 -> u32
        let s_5_7: u32 = (s_5_6.value() as u32);
        // D s_5_8: write-var res <= s_5_7
        fn_state.res = s_5_7;
        // C s_5_9: const #0s : i64
        let s_5_9: i64 = 0;
        // D s_5_10: write-var b <= s_5_9
        fn_state.b = s_5_9;
        // N s_5_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var b:i64
        let s_6_0: i64 = fn_state.b;
        // C s_6_1: const #3s : i64
        let s_6_1: i64 = 3;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #4s : i
        let s_7_0: i128 = 4;
        // D s_7_1: read-var e:i64
        let s_7_1: i64 = fn_state.e;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var b:i64
        let s_7_6: i64 = fn_state.b;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: add s_7_5 s_7_7
        let s_7_8: i128 = (s_7_5 + s_7_7);
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // C s_7_10: const #8s : i64
        let s_7_10: i64 = 8;
        // D s_7_11: read-var operand1:u64
        let s_7_11: u64 = fn_state.operand1;
        // D s_7_12: cast zx s_7_11 -> bv
        let s_7_12: Bits = Bits::new(s_7_11 as u128, 64u16);
        // D s_7_13: cast zx s_7_9 -> i
        let s_7_13: i128 = (i128::try_from(s_7_9).unwrap());
        // C s_7_14: cast zx s_7_10 -> i
        let s_7_14: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_15: call Elem_read(s_7_12, s_7_13, s_7_14)
        let s_7_15: Bits = Elem_read(state, tracer, s_7_12, s_7_13, s_7_14);
        // D s_7_16: cast reint s_7_15 -> u8
        let s_7_16: u8 = (s_7_15.value() as u8);
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 8u16);
        // D s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (s_7_17.value() as i128);
        // C s_7_19: const #4s : i
        let s_7_19: i128 = 4;
        // D s_7_20: read-var e:i64
        let s_7_20: i64 = fn_state.e;
        // D s_7_21: cast zx s_7_20 -> i
        let s_7_21: i128 = (i128::try_from(s_7_20).unwrap());
        // D s_7_22: mul s_7_19 s_7_21
        let s_7_22: i128 = ((s_7_19) * (s_7_21));
        // D s_7_23: cast reint s_7_22 -> i64
        let s_7_23: i64 = (s_7_22 as i64);
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: read-var b:i64
        let s_7_25: i64 = fn_state.b;
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_27: add s_7_24 s_7_26
        let s_7_27: i128 = (s_7_24 + s_7_26);
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // C s_7_29: const #8s : i64
        let s_7_29: i64 = 8;
        // D s_7_30: read-var operand2:u64
        let s_7_30: u64 = fn_state.operand2;
        // D s_7_31: cast zx s_7_30 -> bv
        let s_7_31: Bits = Bits::new(s_7_30 as u128, 64u16);
        // D s_7_32: cast zx s_7_28 -> i
        let s_7_32: i128 = (i128::try_from(s_7_28).unwrap());
        // C s_7_33: cast zx s_7_29 -> i
        let s_7_33: i128 = (i128::try_from(s_7_29).unwrap());
        // D s_7_34: call Elem_read(s_7_31, s_7_32, s_7_33)
        let s_7_34: Bits = Elem_read(state, tracer, s_7_31, s_7_32, s_7_33);
        // D s_7_35: cast reint s_7_34 -> u8
        let s_7_35: u8 = (s_7_34.value() as u8);
        // D s_7_36: cast zx s_7_35 -> bv
        let s_7_36: Bits = Bits::new(s_7_35 as u128, 8u16);
        // D s_7_37: cast sx s_7_36 -> i
        let s_7_37: i128 = {
            let sign_bit = s_7_36.length() - 1;
            let mut result = s_7_36.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_38: mul s_7_18 s_7_37
        let s_7_38: i128 = ((s_7_18) * (s_7_37));
        // D s_7_39: read-var res:u32
        let s_7_39: u32 = fn_state.res;
        // D s_7_40: cast zx s_7_39 -> bv
        let s_7_40: Bits = Bits::new(s_7_39 as u128, 32u16);
        // D s_7_41: cast cvt s_7_38 -> bv
        let s_7_41: Bits = Bits::new(s_7_38 as u128, 128);
        // D s_7_42: add s_7_40 s_7_41
        let s_7_42: Bits = (s_7_40 + s_7_41);
        // D s_7_43: cast reint s_7_42 -> u32
        let s_7_43: u32 = (s_7_42.value() as u32);
        // D s_7_44: write-var res <= s_7_43
        fn_state.res = s_7_43;
        // D s_7_45: read-var b:i64
        let s_7_45: i64 = fn_state.b;
        // C s_7_46: const #1s : i64
        let s_7_46: i64 = 1;
        // D s_7_47: add s_7_45 s_7_46
        let s_7_47: i64 = (s_7_45 + s_7_46);
        // D s_7_48: write-var b <= s_7_47
        fn_state.b = s_7_47;
        // N s_7_49: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i64
        let s_8_0: i64 = 32;
        // D s_8_1: read-var result:u64
        let s_8_1: u64 = fn_state.result;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 64u16);
        // D s_8_3: read-var e:i64
        let s_8_3: i64 = fn_state.e;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: cast zx s_8_0 -> i
        let s_8_5: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_6: read-var res:u32
        let s_8_6: u32 = fn_state.res;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 32u16);
        // D s_8_8: call Elem_set(s_8_2, s_8_4, s_8_5, s_8_7)
        let s_8_8: Bits = Elem_set(state, tracer, s_8_2, s_8_4, s_8_5, s_8_7);
        // D s_8_9: cast reint s_8_8 -> u64
        let s_8_9: u64 = (s_8_8.value() as u64);
        // D s_8_10: write-var result <= s_8_9
        fn_state.result = s_8_9;
        // D s_8_11: read-var e:i64
        let s_8_11: i64 = fn_state.e;
        // C s_8_12: const #1s : i64
        let s_8_12: i64 = 1;
        // D s_8_13: add s_8_11 s_8_12
        let s_8_13: i64 = (s_8_11 + s_8_12);
        // D s_8_14: write-var e <= s_8_13
        fn_state.e = s_8_13;
        // N s_8_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var r:i64
        let s_9_2: i64 = fn_state.r;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: add s_9_1 s_9_3
        let s_9_4: i128 = (s_9_1 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: read-var result:u64
        let s_9_7: u64 = fn_state.result;
        // D s_9_8: call D_set(s_9_6, s_9_7)
        let s_9_8: () = D_set(state, tracer, s_9_6, s_9_7);
        // D s_9_9: read-var r:i64
        let s_9_9: i64 = fn_state.r;
        // C s_9_10: const #1s : i64
        let s_9_10: i64 = 1;
        // D s_9_11: add s_9_9 s_9_10
        let s_9_11: i64 = (s_9_9 + s_9_10);
        // D s_9_12: write-var r <= s_9_11
        fn_state.r = s_9_11;
        // N s_9_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
