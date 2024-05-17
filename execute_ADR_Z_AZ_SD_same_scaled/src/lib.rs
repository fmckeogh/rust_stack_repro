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
use u__id::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_ADR_Z_AZ_SD_same_scaled<T: Tracer>(
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
        VLshadow_2467: i64,
        base: Bits,
        osizeshadow_2465: i64,
        VLshadow_2468: i64,
        gs_185287: i64,
        esizeshadow_2466: i64,
        offs: Bits,
        result: Bits,
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
        // D s_0_0: read-var osize:i64
        let s_0_0: i64 = fn_state.osize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var osizeshadow#2465 <= s_0_2
        fn_state.osizeshadow_2465 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#2466 <= s_0_6
        fn_state.esizeshadow_2466 = s_0_6;
        // D s_0_8: read-var VL:i64
        let s_0_8: i64 = fn_state.VL;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var VLshadow#2467 <= s_0_10
        fn_state.VLshadow_2467 = s_0_10;
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call CheckNonStreamingSVEEnabled(s_0_12)
        let s_0_13: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_12);
        // N s_0_14: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2467:i64
        let s_1_0: i64 = fn_state.VLshadow_2467;
        // D s_1_1: write-var VLshadow#2468 <= s_1_0
        fn_state.VLshadow_2468 = s_1_0;
        // D s_1_2: read-var VLshadow#2468:i64
        let s_1_2: i64 = fn_state.VLshadow_2468;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2466:i64
        let s_1_4: i64 = fn_state.esizeshadow_2466;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#2468:i64
        let s_1_8: i64 = fn_state.VLshadow_2468;
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
        // D s_1_16: read-var VLshadow#2468:i64
        let s_1_16: i64 = fn_state.VLshadow_2468;
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
        // D s_1_29: write-var gs#185287 <= s_1_28
        fn_state.gs_185287 = s_1_28;
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
        // D s_2_1: read-var gs#185287:i64
        let s_2_1: i64 = fn_state.gs_185287;
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
        // D s_3_0: read-var esizeshadow#2466:i64
        let s_3_0: i64 = fn_state.esizeshadow_2466;
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
        // D s_3_8: read-var osizeshadow#2465:i64
        let s_3_8: i64 = fn_state.osizeshadow_2465;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: call __id(s_3_9)
        let s_3_10: i128 = u__id(state, tracer, s_3_9);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // C s_3_12: const #1s : i
        let s_3_12: i128 = 1;
        // D s_3_13: cast zx s_3_11 -> i
        let s_3_13: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_14: sub s_3_13 s_3_12
        let s_3_14: i128 = ((s_3_13) - (s_3_12));
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var esizeshadow#2466:i64
        let s_3_16: i64 = fn_state.esizeshadow_2466;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: call __id(s_3_17)
        let s_3_18: i128 = u__id(state, tracer, s_3_17);
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: cast zx s_3_15 -> i
        let s_3_20: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_21: cast zx s_3_19 -> i
        let s_3_21: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_22: cmp-lt s_3_20 s_3_21
        let s_3_22: bool = ((s_3_20) < (s_3_21));
        // N s_3_23: assert s_3_22
        let s_3_23: () = assert!(s_3_22);
        // D s_3_24: read-var esizeshadow#2466:i64
        let s_3_24: i64 = fn_state.esizeshadow_2466;
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // D s_3_27: read-var e:i64
        let s_3_27: i64 = fn_state.e;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: cast zx s_3_26 -> i
        let s_3_29: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_30: read-var offs:bv
        let s_3_30: Bits = fn_state.offs;
        // D s_3_31: call Elem_read(s_3_30, s_3_28, s_3_29)
        let s_3_31: Bits = Elem_read(state, tracer, s_3_30, s_3_28, s_3_29);
        // C s_3_32: const #1s : i
        let s_3_32: i128 = 1;
        // D s_3_33: read-var osizeshadow#2465:i64
        let s_3_33: i64 = fn_state.osizeshadow_2465;
        // D s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_35: sub s_3_34 s_3_32
        let s_3_35: i128 = ((s_3_34) - (s_3_32));
        // D s_3_36: cast reint s_3_35 -> i64
        let s_3_36: i64 = (s_3_35 as i64);
        // C s_3_37: const #0s : i
        let s_3_37: i128 = 0;
        // D s_3_38: cast zx s_3_36 -> i
        let s_3_38: i128 = (i128::try_from(s_3_36).unwrap());
        // C s_3_39: const #1s : i64
        let s_3_39: i64 = 1;
        // C s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: sub s_3_38 s_3_37
        let s_3_41: i128 = ((s_3_38) - (s_3_37));
        // D s_3_42: add s_3_41 s_3_40
        let s_3_42: i128 = (s_3_41 + s_3_40);
        // D s_3_43: bit-extract s_3_31 s_3_37 s_3_42
        let s_3_43: Bits = (Bits::new(
            ((s_3_31) >> (s_3_37)).value(),
            u16::try_from(s_3_42).unwrap(),
        ));
        // D s_3_44: read-var is_unsigned:u8
        let s_3_44: bool = fn_state.is_unsigned;
        // D s_3_45: call asl_Int(s_3_43, s_3_44)
        let s_3_45: i128 = asl_Int(state, tracer, s_3_43, s_3_44);
        // D s_3_46: read-var esizeshadow#2466:i64
        let s_3_46: i64 = fn_state.esizeshadow_2466;
        // D s_3_47: cast zx s_3_46 -> i
        let s_3_47: i128 = (i128::try_from(s_3_46).unwrap());
        // D s_3_48: cast reint s_3_47 -> i64
        let s_3_48: i64 = (s_3_47 as i64);
        // D s_3_49: read-var mbytes:i64
        let s_3_49: i64 = fn_state.mbytes;
        // D s_3_50: cast zx s_3_49 -> i
        let s_3_50: i128 = (i128::try_from(s_3_49).unwrap());
        // D s_3_51: mul s_3_45 s_3_50
        let s_3_51: i128 = ((s_3_45) * (s_3_50));
        // D s_3_52: cast cvt s_3_51 -> bv
        let s_3_52: Bits = Bits::new(s_3_51 as u128, 128);
        // D s_3_53: add s_3_7 s_3_52
        let s_3_53: Bits = (s_3_7 + s_3_52);
        // D s_3_54: read-var e:i64
        let s_3_54: i64 = fn_state.e;
        // D s_3_55: cast zx s_3_54 -> i
        let s_3_55: i128 = (i128::try_from(s_3_54).unwrap());
        // D s_3_56: cast zx s_3_48 -> i
        let s_3_56: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_57: read-var result:bv
        let s_3_57: Bits = fn_state.result;
        // D s_3_58: call Elem_set(s_3_57, s_3_55, s_3_56, s_3_53)
        let s_3_58: Bits = Elem_set(state, tracer, s_3_57, s_3_55, s_3_56, s_3_53);
        // D s_3_59: write-var result <= s_3_58
        fn_state.result = s_3_58;
        // D s_3_60: read-var e:i64
        let s_3_60: i64 = fn_state.e;
        // C s_3_61: const #1s : i64
        let s_3_61: i64 = 1;
        // D s_3_62: add s_3_60 s_3_61
        let s_3_62: i64 = (s_3_60 + s_3_61);
        // D s_3_63: write-var e <= s_3_62
        fn_state.e = s_3_62;
        // N s_3_64: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2468:i64
        let s_4_0: i64 = fn_state.VLshadow_2468;
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
