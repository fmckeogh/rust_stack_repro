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
use TGxGranuleBits::*;
use place_subrange::*;
use AArch64_IASize::*;
use common::*;
pub fn AArch64_TTEntryAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    level: i128,
    d128: bool,
    skl: u8,
    tgx: u32,
    txsz: u8,
    ia: u64,
    tablebase: ProductTypeda0231e9dc169f81,
) -> ProductTypeda0231e9dc169f81 {
    #[derive(Default)]
    struct FunctionState {
        granulebits: i128,
        ga_14497: i64,
        gs_19320: bool,
        descaddress: ProductTypeda0231e9dc169f81,
        msbshadow_320: i128,
        stride: i128,
        lsb: i128,
        gs_19319: bool,
        descsizelog2: i64,
        msb: i128,
        level: i128,
        d128: bool,
        skl: u8,
        tgx: u32,
        txsz: u8,
        ia: u64,
        tablebase: ProductTypeda0231e9dc169f81,
    }
    let fn_state = FunctionState {
        level,
        d128,
        skl,
        tgx,
        txsz,
        ia,
        tablebase,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_0_0: read-var txsz:u8
        let s_0_0: u8 = fn_state.txsz;
        // D s_0_1: call AArch64_IASize(s_0_0)
        let s_0_1: i128 = AArch64_IASize(state, tracer, s_0_0);
        // D s_0_2: read-var tgx:u32
        let s_0_2: u32 = fn_state.tgx;
        // D s_0_3: call TGxGranuleBits(s_0_2)
        let s_0_3: i128 = TGxGranuleBits(state, tracer, s_0_2);
        // D s_0_4: write-var granulebits <= s_0_3
        fn_state.granulebits = s_0_3;
        // D s_0_5: read-var d128:u8
        let s_0_5: bool = fn_state.d128;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // D s_0_9: cmp-eq s_0_6 s_0_8
        let s_0_9: bool = ((s_0_6) == (s_0_8));
        // N s_0_10: branch s_0_9 b12 b1
        if s_0_9 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_1_0: const #3s : i64
        let s_1_0: i64 = 3;
        // D s_1_1: write-var ga#14497 <= s_1_0
        fn_state.ga_14497 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_2_0: read-var ga#14497:i64
        let s_2_0: i64 = fn_state.ga_14497;
        // D s_2_1: write-var descsizelog2 <= s_2_0
        fn_state.descsizelog2 = s_2_0;
        // D s_2_2: read-var descsizelog2:i64
        let s_2_2: i64 = fn_state.descsizelog2;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: read-var granulebits:i
        let s_2_4: i128 = fn_state.granulebits;
        // D s_2_5: sub s_2_4 s_2_3
        let s_2_5: i128 = ((s_2_4) - (s_2_3));
        // D s_2_6: write-var stride <= s_2_5
        fn_state.stride = s_2_5;
        // C s_2_7: const #800u : u32
        let s_2_7: u32 = 800;
        // D s_2_8: read-reg s_2_7:i64
        let s_2_8: i64 = {
            let value = state.read_register::<i64>(s_2_7 as isize);
            tracer.read_register(s_2_7 as isize, value);
            value
        };
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_10: read-var level:i
        let s_2_10: i128 = fn_state.level;
        // D s_2_11: sub s_2_9 s_2_10
        let s_2_11: i128 = ((s_2_9) - (s_2_10));
        // D s_2_12: read-var stride:i
        let s_2_12: i128 = fn_state.stride;
        // D s_2_13: mul s_2_11 s_2_12
        let s_2_13: i128 = ((s_2_11) * (s_2_12));
        // D s_2_14: read-var granulebits:i
        let s_2_14: i128 = fn_state.granulebits;
        // D s_2_15: add s_2_13 s_2_14
        let s_2_15: i128 = (s_2_13 + s_2_14);
        // D s_2_16: write-var lsb <= s_2_15
        fn_state.lsb = s_2_15;
        // D s_2_17: read-var d128:u8
        let s_2_17: bool = fn_state.d128;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // N s_2_22: branch s_2_21 b11 b3
        if s_2_21 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_3_0: read-var lsb:i
        let s_3_0: i128 = fn_state.lsb;
        // D s_3_1: read-var stride:i
        let s_3_1: i128 = fn_state.stride;
        // D s_3_2: add s_3_0 s_3_1
        let s_3_2: i128 = (s_3_0 + s_3_1);
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: sub s_3_2 s_3_3
        let s_3_4: i128 = ((s_3_2) - (s_3_3));
        // D s_3_5: write-var msb <= s_3_4
        fn_state.msb = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_4_0: read-var msb:i
        let s_4_0: i128 = fn_state.msb;
        // D s_4_1: write-var msbshadow#320 <= s_4_0
        fn_state.msbshadow_320 = s_4_0;
        // D s_4_2: read-var lsb:i
        let s_4_2: i128 = fn_state.lsb;
        // D s_4_3: call __id(s_4_2)
        let s_4_3: i128 = u__id(state, tracer, s_4_2);
        // C s_4_4: const #0s : i
        let s_4_4: i128 = 0;
        // D s_4_5: cmp-le s_4_4 s_4_3
        let s_4_5: bool = ((s_4_4) <= (s_4_3));
        // N s_4_6: branch s_4_5 b7 b5
        if s_4_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#19320 <= s_5_0
        fn_state.gs_19320 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_6_0: read-var gs#19320:u8
        let s_6_0: bool = fn_state.gs_19320;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var msbshadow#320:i
        let s_6_2: i128 = fn_state.msbshadow_320;
        // D s_6_3: call __id(s_6_2)
        let s_6_3: i128 = u__id(state, tracer, s_6_2);
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: read-var lsb:i
        let s_6_5: i128 = fn_state.lsb;
        // D s_6_6: call __id(s_6_5)
        let s_6_6: i128 = u__id(state, tracer, s_6_5);
        // D s_6_7: cast reint s_6_6 -> i64
        let s_6_7: i64 = (s_6_6 as i64);
        // D s_6_8: cast zx s_6_4 -> i
        let s_6_8: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_9: cast zx s_6_7 -> i
        let s_6_9: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_10: sub s_6_8 s_6_9
        let s_6_10: i128 = ((s_6_8) - (s_6_9));
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // C s_6_12: const #1s : i
        let s_6_12: i128 = 1;
        // D s_6_13: cast zx s_6_11 -> i
        let s_6_13: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_14: add s_6_13 s_6_12
        let s_6_14: i128 = (s_6_13 + s_6_12);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: read-var descsizelog2:i64
        let s_6_16: i64 = fn_state.descsizelog2;
        // D s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_18: call __id(s_6_17)
        let s_6_18: i128 = u__id(state, tracer, s_6_17);
        // D s_6_19: cast reint s_6_18 -> i64
        let s_6_19: i64 = (s_6_18 as i64);
        // D s_6_20: cast zx s_6_15 -> i
        let s_6_20: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_21: cast zx s_6_19 -> i
        let s_6_21: i128 = (i128::try_from(s_6_19).unwrap());
        // D s_6_22: add s_6_20 s_6_21
        let s_6_22: i128 = (s_6_20 + s_6_21);
        // D s_6_23: cast reint s_6_22 -> i64
        let s_6_23: i64 = (s_6_22 as i64);
        // C s_6_24: const #56s : i
        let s_6_24: i128 = 56;
        // D s_6_25: cast zx s_6_23 -> i
        let s_6_25: i128 = (i128::try_from(s_6_23).unwrap());
        // D s_6_26: cmp-ge s_6_24 s_6_25
        let s_6_26: bool = ((s_6_24) >= (s_6_25));
        // N s_6_27: assert s_6_26
        let s_6_27: () = assert!(s_6_26);
        // C s_6_28: const #56s : i
        let s_6_28: i128 = 56;
        // D s_6_29: read-var ia:u64
        let s_6_29: u64 = fn_state.ia;
        // D s_6_30: cast zx s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 64u16);
        // D s_6_31: read-var descsizelog2:i64
        let s_6_31: i64 = fn_state.descsizelog2;
        // D s_6_32: cast zx s_6_31 -> i
        let s_6_32: i128 = (i128::try_from(s_6_31).unwrap());
        // D s_6_33: read-var msbshadow#320:i
        let s_6_33: i128 = fn_state.msbshadow_320;
        // D s_6_34: read-var lsb:i
        let s_6_34: i128 = fn_state.lsb;
        // D s_6_35: call place_subrange(s_6_28, s_6_30, s_6_33, s_6_34, s_6_32)
        let s_6_35: Bits = place_subrange(
            state,
            tracer,
            s_6_28,
            s_6_30,
            s_6_33,
            s_6_34,
            s_6_32,
        );
        // D s_6_36: cast reint s_6_35 -> u56
        let s_6_36: u64 = (s_6_35.value() as u64);
        // D s_6_37: read-var tablebase.0:struct
        let s_6_37: u64 = fn_state.tablebase._0;
        // D s_6_38: cast zx s_6_37 -> bv
        let s_6_38: Bits = Bits::new(s_6_37 as u128, 56u16);
        // D s_6_39: cast zx s_6_36 -> bv
        let s_6_39: Bits = Bits::new(s_6_36 as u128, 56u16);
        // D s_6_40: or s_6_38 s_6_39
        let s_6_40: Bits = ((s_6_38) | (s_6_39));
        // D s_6_41: cast reint s_6_40 -> u56
        let s_6_41: u64 = (s_6_40.value() as u64);
        // D s_6_42: write-var descaddress.0 <= s_6_41
        fn_state.descaddress._0 = s_6_41;
        // D s_6_43: read-var tablebase.1:struct
        let s_6_43: u32 = fn_state.tablebase._1;
        // D s_6_44: write-var descaddress.1 <= s_6_43
        fn_state.descaddress._1 = s_6_43;
        // D s_6_45: read-var descaddress:struct
        let s_6_45: ProductTypeda0231e9dc169f81 = fn_state.descaddress;
        // N s_6_46: return s_6_45
        return s_6_45;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_7_0: read-var lsb:i
        let s_7_0: i128 = fn_state.lsb;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // D s_7_2: read-var msbshadow#320:i
        let s_7_2: i128 = fn_state.msbshadow_320;
        // D s_7_3: call __id(s_7_2)
        let s_7_3: i128 = u__id(state, tracer, s_7_2);
        // D s_7_4: cmp-le s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) <= (s_7_3));
        // N s_7_5: branch s_7_4 b10 b8
        if s_7_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#19319 <= s_8_0
        fn_state.gs_19319 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_9_0: read-var gs#19319:u8
        let s_9_0: bool = fn_state.gs_19319;
        // D s_9_1: write-var gs#19320 <= s_9_0
        fn_state.gs_19320 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_10_0: read-var msbshadow#320:i
        let s_10_0: i128 = fn_state.msbshadow_320;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #64s : i
        let s_10_2: i128 = 64;
        // D s_10_3: cmp-lt s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) < (s_10_2));
        // D s_10_4: write-var gs#19319 <= s_10_3
        fn_state.gs_19319 = s_10_3;
        // N s_10_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_11_0: read-var skl:u8
        let s_11_0: u8 = fn_state.skl;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (s_11_1.value() as i128);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #1s : i
        let s_11_4: i128 = 1;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: add s_11_4 s_11_5
        let s_11_6: i128 = (s_11_4 + s_11_5);
        // D s_11_7: cast reint s_11_6 -> i64
        let s_11_7: i64 = (s_11_6 as i64);
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_9: read-var stride:i
        let s_11_9: i128 = fn_state.stride;
        // D s_11_10: mul s_11_9 s_11_8
        let s_11_10: i128 = ((s_11_9) * (s_11_8));
        // D s_11_11: read-var lsb:i
        let s_11_11: i128 = fn_state.lsb;
        // D s_11_12: add s_11_11 s_11_10
        let s_11_12: i128 = (s_11_11 + s_11_10);
        // C s_11_13: const #1s : i
        let s_11_13: i128 = 1;
        // D s_11_14: sub s_11_12 s_11_13
        let s_11_14: i128 = ((s_11_12) - (s_11_13));
        // D s_11_15: write-var msb <= s_11_14
        fn_state.msb = s_11_14;
        // N s_11_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_12_0: const #4s : i64
        let s_12_0: i64 = 4;
        // D s_12_1: write-var ga#14497 <= s_12_0
        fn_state.ga_14497 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
