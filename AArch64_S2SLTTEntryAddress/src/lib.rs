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
use AArch64_S2StartLevel::*;
use u__id::*;
use TGxGranuleBits::*;
use place_subrange::*;
use AArch64_IASize::*;
use common::*;
pub fn AArch64_S2SLTTEntryAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
    ipa: u64,
    tablebase: ProductTypeda0231e9dc169f81,
) -> ProductTypeda0231e9dc169f81 {
    #[derive(Default)]
    struct FunctionState {
        gs_19279: bool,
        granulebits: i128,
        startlevel: i128,
        descaddress: ProductTypeda0231e9dc169f81,
        ga_14460: i64,
        gs_19278: bool,
        lsb: i128,
        iasize: i128,
        msb: i128,
        descsizelog2: i64,
        walkparams: ProductTypeb05ce25a107f0c5e,
        ipa: u64,
        tablebase: ProductTypeda0231e9dc169f81,
    }
    let fn_state = FunctionState {
        walkparams,
        ipa,
        tablebase,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_0_0: read-var walkparams:struct
        let s_0_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_0_1: call AArch64_S2StartLevel(s_0_0)
        let s_0_1: i128 = AArch64_S2StartLevel(state, tracer, s_0_0);
        // D s_0_2: write-var startlevel <= s_0_1
        fn_state.startlevel = s_0_1;
        // D s_0_3: read-var walkparams.29:struct
        let s_0_3: u8 = fn_state.walkparams._29;
        // D s_0_4: call AArch64_IASize(s_0_3)
        let s_0_4: i128 = AArch64_IASize(state, tracer, s_0_3);
        // D s_0_5: write-var iasize <= s_0_4
        fn_state.iasize = s_0_4;
        // D s_0_6: read-var walkparams.26:struct
        let s_0_6: u32 = fn_state.walkparams._26;
        // D s_0_7: call TGxGranuleBits(s_0_6)
        let s_0_7: i128 = TGxGranuleBits(state, tracer, s_0_6);
        // D s_0_8: write-var granulebits <= s_0_7
        fn_state.granulebits = s_0_7;
        // D s_0_9: read-var walkparams.2:struct
        let s_0_9: bool = fn_state.walkparams._2;
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 1u16);
        // C s_0_11: const #1u : u8
        let s_0_11: bool = true;
        // C s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 1u16);
        // D s_0_13: cmp-eq s_0_10 s_0_12
        let s_0_13: bool = ((s_0_10) == (s_0_12));
        // N s_0_14: branch s_0_13 b9 b1
        if s_0_13 {
            return block_9(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#14460 <= s_1_0
        fn_state.ga_14460 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_2_0: read-var ga#14460:i64
        let s_2_0: i64 = fn_state.ga_14460;
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
        // C s_2_6: const #800u : u32
        let s_2_6: u32 = 800;
        // D s_2_7: read-reg s_2_6:i64
        let s_2_7: i64 = {
            let value = state.read_register::<i64>(s_2_6 as isize);
            tracer.read_register(s_2_6 as isize, value);
            value
        };
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: read-var startlevel:i
        let s_2_9: i128 = fn_state.startlevel;
        // D s_2_10: sub s_2_8 s_2_9
        let s_2_10: i128 = ((s_2_8) - (s_2_9));
        // D s_2_11: mul s_2_10 s_2_5
        let s_2_11: i128 = ((s_2_10) * (s_2_5));
        // D s_2_12: read-var granulebits:i
        let s_2_12: i128 = fn_state.granulebits;
        // D s_2_13: add s_2_11 s_2_12
        let s_2_13: i128 = (s_2_11 + s_2_12);
        // D s_2_14: write-var lsb <= s_2_13
        fn_state.lsb = s_2_13;
        // C s_2_15: const #1s : i
        let s_2_15: i128 = 1;
        // D s_2_16: read-var iasize:i
        let s_2_16: i128 = fn_state.iasize;
        // D s_2_17: sub s_2_16 s_2_15
        let s_2_17: i128 = ((s_2_16) - (s_2_15));
        // D s_2_18: write-var msb <= s_2_17
        fn_state.msb = s_2_17;
        // D s_2_19: read-var lsb:i
        let s_2_19: i128 = fn_state.lsb;
        // D s_2_20: call __id(s_2_19)
        let s_2_20: i128 = u__id(state, tracer, s_2_19);
        // C s_2_21: const #0s : i
        let s_2_21: i128 = 0;
        // D s_2_22: cmp-le s_2_21 s_2_20
        let s_2_22: bool = ((s_2_21) <= (s_2_20));
        // N s_2_23: branch s_2_22 b5 b3
        if s_2_22 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#19279 <= s_3_0
        fn_state.gs_19279 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_4_0: read-var gs#19279:u8
        let s_4_0: bool = fn_state.gs_19279;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var msb:i
        let s_4_2: i128 = fn_state.msb;
        // D s_4_3: call __id(s_4_2)
        let s_4_3: i128 = u__id(state, tracer, s_4_2);
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: read-var lsb:i
        let s_4_5: i128 = fn_state.lsb;
        // D s_4_6: call __id(s_4_5)
        let s_4_6: i128 = u__id(state, tracer, s_4_5);
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // D s_4_8: cast zx s_4_4 -> i
        let s_4_8: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_9: cast zx s_4_7 -> i
        let s_4_9: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_10: sub s_4_8 s_4_9
        let s_4_10: i128 = ((s_4_8) - (s_4_9));
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // C s_4_12: const #1s : i
        let s_4_12: i128 = 1;
        // D s_4_13: cast zx s_4_11 -> i
        let s_4_13: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_14: add s_4_13 s_4_12
        let s_4_14: i128 = (s_4_13 + s_4_12);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: read-var descsizelog2:i64
        let s_4_16: i64 = fn_state.descsizelog2;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: call __id(s_4_17)
        let s_4_18: i128 = u__id(state, tracer, s_4_17);
        // D s_4_19: cast reint s_4_18 -> i64
        let s_4_19: i64 = (s_4_18 as i64);
        // D s_4_20: cast zx s_4_15 -> i
        let s_4_20: i128 = (i128::try_from(s_4_15).unwrap());
        // D s_4_21: cast zx s_4_19 -> i
        let s_4_21: i128 = (i128::try_from(s_4_19).unwrap());
        // D s_4_22: add s_4_20 s_4_21
        let s_4_22: i128 = (s_4_20 + s_4_21);
        // D s_4_23: cast reint s_4_22 -> i64
        let s_4_23: i64 = (s_4_22 as i64);
        // C s_4_24: const #56s : i
        let s_4_24: i128 = 56;
        // D s_4_25: cast zx s_4_23 -> i
        let s_4_25: i128 = (i128::try_from(s_4_23).unwrap());
        // D s_4_26: cmp-ge s_4_24 s_4_25
        let s_4_26: bool = ((s_4_24) >= (s_4_25));
        // N s_4_27: assert s_4_26
        let s_4_27: () = assert!(s_4_26);
        // C s_4_28: const #56s : i
        let s_4_28: i128 = 56;
        // D s_4_29: read-var ipa:u56
        let s_4_29: u64 = fn_state.ipa;
        // D s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 56u16);
        // D s_4_31: read-var descsizelog2:i64
        let s_4_31: i64 = fn_state.descsizelog2;
        // D s_4_32: cast zx s_4_31 -> i
        let s_4_32: i128 = (i128::try_from(s_4_31).unwrap());
        // D s_4_33: read-var msb:i
        let s_4_33: i128 = fn_state.msb;
        // D s_4_34: read-var lsb:i
        let s_4_34: i128 = fn_state.lsb;
        // D s_4_35: call place_subrange(s_4_28, s_4_30, s_4_33, s_4_34, s_4_32)
        let s_4_35: Bits = place_subrange(
            state,
            tracer,
            s_4_28,
            s_4_30,
            s_4_33,
            s_4_34,
            s_4_32,
        );
        // D s_4_36: cast reint s_4_35 -> u56
        let s_4_36: u64 = (s_4_35.value() as u64);
        // D s_4_37: read-var tablebase.0:struct
        let s_4_37: u64 = fn_state.tablebase._0;
        // D s_4_38: cast zx s_4_37 -> bv
        let s_4_38: Bits = Bits::new(s_4_37 as u128, 56u16);
        // D s_4_39: cast zx s_4_36 -> bv
        let s_4_39: Bits = Bits::new(s_4_36 as u128, 56u16);
        // D s_4_40: or s_4_38 s_4_39
        let s_4_40: Bits = ((s_4_38) | (s_4_39));
        // D s_4_41: cast reint s_4_40 -> u56
        let s_4_41: u64 = (s_4_40.value() as u64);
        // D s_4_42: write-var descaddress.0 <= s_4_41
        fn_state.descaddress._0 = s_4_41;
        // D s_4_43: read-var tablebase.1:struct
        let s_4_43: u32 = fn_state.tablebase._1;
        // D s_4_44: write-var descaddress.1 <= s_4_43
        fn_state.descaddress._1 = s_4_43;
        // D s_4_45: read-var descaddress:struct
        let s_4_45: ProductTypeda0231e9dc169f81 = fn_state.descaddress;
        // N s_4_46: return s_4_45
        return s_4_45;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_5_0: read-var lsb:i
        let s_5_0: i128 = fn_state.lsb;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // D s_5_2: read-var msb:i
        let s_5_2: i128 = fn_state.msb;
        // D s_5_3: call __id(s_5_2)
        let s_5_3: i128 = u__id(state, tracer, s_5_2);
        // D s_5_4: cmp-le s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) <= (s_5_3));
        // N s_5_5: branch s_5_4 b8 b6
        if s_5_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#19278 <= s_6_0
        fn_state.gs_19278 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_7_0: read-var gs#19278:u8
        let s_7_0: bool = fn_state.gs_19278;
        // D s_7_1: write-var gs#19279 <= s_7_0
        fn_state.gs_19279 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_8_0: read-var msb:i
        let s_8_0: i128 = fn_state.msb;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #56s : i
        let s_8_2: i128 = 56;
        // D s_8_3: cmp-lt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) < (s_8_2));
        // D s_8_4: write-var gs#19278 <= s_8_3
        fn_state.gs_19278 = s_8_3;
        // N s_8_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_9_0: const #4s : i64
        let s_9_0: i64 = 4;
        // D s_9_1: write-var ga#14460 <= s_9_0
        fn_state.ga_14460 = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
