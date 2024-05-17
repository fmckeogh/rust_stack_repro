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
use asl_Int::*;
use Elem_set::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_ADR_Z_AZ_D_s32_scaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    mbytes: i64,
    n: i64,
    osize: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_185235: i64,
        base: Bits,
        offs: Bits,
        result: Bits,
        VLshadow_2464: i64,
        VLshadow_2463: i64,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        mbytes: i64,
        n: i64,
        osize: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        mbytes,
        n,
        osize,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2463 <= s_0_2
        fn_state.VLshadow_2463 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2463:i64
        let s_1_0: i64 = fn_state.VLshadow_2463;
        // D s_1_1: write-var VLshadow#2464 <= s_1_0
        fn_state.VLshadow_2464 = s_1_0;
        // D s_1_2: read-var VLshadow#2464:i64
        let s_1_2: i64 = fn_state.VLshadow_2464;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#2464:i64
        let s_1_8: i64 = fn_state.VLshadow_2464;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var base <= s_1_14
        fn_state.base = s_1_14;
        // D s_1_16: read-var VLshadow#2464:i64
        let s_1_16: i64 = fn_state.VLshadow_2464;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var m:i64
        let s_1_19: i64 = fn_state.m;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var offs <= s_1_22
        fn_state.offs = s_1_22;
        // C s_1_24: const #0s : i64
        let s_1_24: i64 = 0;
        // C s_1_25: const #1s : i
        let s_1_25: i128 = 1;
        // D s_1_26: cast zx s_1_7 -> i
        let s_1_26: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_27: sub s_1_26 s_1_25
        let s_1_27: i128 = ((s_1_26) - (s_1_25));
        // D s_1_28: cast reint s_1_27 -> i64
        let s_1_28: i64 = (s_1_27 as i64);
        // D s_1_29: write-var gs#185235 <= s_1_28
        fn_state.gs_185235 = s_1_28;
        // D s_1_30: write-var e <= s_1_24
        fn_state.e = s_1_24;
        // N s_1_31: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#185235:i64
        let s_2_1: i64 = fn_state.gs_185235;
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
        // D s_3_0: read-var esize:i64
        let s_3_0: i64 = fn_state.esize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var base:bv
        let s_3_6: Bits = fn_state.base;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: read-var esize:i64
        let s_3_8: i64 = fn_state.esize;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: read-var e:i64
        let s_3_11: i64 = fn_state.e;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast zx s_3_10 -> i
        let s_3_13: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_14: read-var offs:bv
        let s_3_14: Bits = fn_state.offs;
        // D s_3_15: call Elem_read(s_3_14, s_3_12, s_3_13)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_14, s_3_12, s_3_13);
        // D s_3_16: cast reint s_3_15 -> u64
        let s_3_16: u64 = (s_3_15.value() as u64);
        // C s_3_17: const #0s : i
        let s_3_17: i128 = 0;
        // C s_3_18: const #32s : i
        let s_3_18: i128 = 32;
        // D s_3_19: cast zx s_3_16 -> bv
        let s_3_19: Bits = Bits::new(s_3_16 as u128, 64u16);
        // D s_3_20: bit-extract s_3_19 s_3_17 s_3_18
        let s_3_20: Bits = (Bits::new(
            ((s_3_19) >> (s_3_17)).value(),
            u16::try_from(s_3_18).unwrap(),
        ));
        // D s_3_21: cast reint s_3_20 -> u32
        let s_3_21: u32 = (s_3_20.value() as u32);
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 32u16);
        // D s_3_23: read-var is_unsigned:u8
        let s_3_23: bool = fn_state.is_unsigned;
        // D s_3_24: call asl_Int(s_3_22, s_3_23)
        let s_3_24: i128 = asl_Int(state, tracer, s_3_22, s_3_23);
        // D s_3_25: read-var esize:i64
        let s_3_25: i64 = fn_state.esize;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: read-var mbytes:i64
        let s_3_28: i64 = fn_state.mbytes;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: mul s_3_24 s_3_29
        let s_3_30: i128 = ((s_3_24) * (s_3_29));
        // D s_3_31: cast cvt s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 128);
        // D s_3_32: add s_3_7 s_3_31
        let s_3_32: Bits = (s_3_7 + s_3_31);
        // D s_3_33: cast reint s_3_32 -> u64
        let s_3_33: u64 = (s_3_32.value() as u64);
        // D s_3_34: read-var e:i64
        let s_3_34: i64 = fn_state.e;
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_36: cast zx s_3_27 -> i
        let s_3_36: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_37: cast zx s_3_33 -> bv
        let s_3_37: Bits = Bits::new(s_3_33 as u128, 64u16);
        // D s_3_38: read-var result:bv
        let s_3_38: Bits = fn_state.result;
        // D s_3_39: call Elem_set(s_3_38, s_3_35, s_3_36, s_3_37)
        let s_3_39: Bits = Elem_set(state, tracer, s_3_38, s_3_35, s_3_36, s_3_37);
        // D s_3_40: write-var result <= s_3_39
        fn_state.result = s_3_39;
        // D s_3_41: read-var e:i64
        let s_3_41: i64 = fn_state.e;
        // C s_3_42: const #1s : i64
        let s_3_42: i64 = 1;
        // D s_3_43: add s_3_41 s_3_42
        let s_3_43: i64 = (s_3_41 + s_3_42);
        // D s_3_44: write-var e <= s_3_43
        fn_state.e = s_3_43;
        // N s_3_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2464:i64
        let s_4_0: i64 = fn_state.VLshadow_2464;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
