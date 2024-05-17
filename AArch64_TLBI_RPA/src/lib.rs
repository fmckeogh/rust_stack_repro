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
use zext_ones::*;
use HaveRME::*;
use Broadcast::*;
use DecodePGS::*;
use TLBI::*;
use Zeros::*;
use u_get_GPCCR_EL3_Type_PGS::*;
use common::*;
pub fn AArch64_TLBI_RPA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    level: u32,
    Xt: u64,
    shareability: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        p: i64,
        range_bitsshadow_504: i128,
        ga_20932: u8,
        ga_20936: u8,
        pshadow_503: i64,
        ga_20934: u32,
        BaseADDR: u64,
        range_bits: i128,
        gs_27173: bool,
        level: u32,
        Xt: u64,
        shareability: u32,
    }
    let fn_state = FunctionState {
        level,
        Xt,
        shareability,
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
        // S s_0_1: call HaveRME(s_0_0)
        let s_0_1: bool = HaveRME(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b43 b1
        if s_0_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#27173 <= s_1_0
        fn_state.gs_27173 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#27173:u8
        let s_2_0: bool = fn_state.gs_27173;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #12s : i64
        let s_2_2: i64 = 12;
        // D s_2_3: write-var p <= s_2_2
        fn_state.p = s_2_2;
        // C s_2_4: const #22u : u32
        let s_2_4: u32 = 22;
        // D s_2_5: write-var r.9 <= s_2_4
        fn_state.r._9 = s_2_4;
        // D s_2_6: read-var level:u32
        let s_2_6: u32 = fn_state.level;
        // D s_2_7: write-var r.8 <= s_2_6
        fn_state.r._8 = s_2_6;
        // C s_2_8: const #0u : u32
        let s_2_8: u32 = 0;
        // D s_2_9: write-var r.2 <= s_2_8
        fn_state.r._2 = s_2_8;
        // C s_2_10: const #44s : i
        let s_2_10: i128 = 44;
        // D s_2_11: read-var Xt:u64
        let s_2_11: u64 = fn_state.Xt;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 64u16);
        // C s_2_13: const #1s : i64
        let s_2_13: i64 = 1;
        // C s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // C s_2_15: const #3s : i
        let s_2_15: i128 = 3;
        // C s_2_16: add s_2_15 s_2_14
        let s_2_16: i128 = (s_2_15 + s_2_14);
        // D s_2_17: bit-extract s_2_12 s_2_10 s_2_16
        let s_2_17: Bits = (Bits::new(
            ((s_2_12) >> (s_2_10)).value(),
            u16::try_from(s_2_16).unwrap(),
        ));
        // D s_2_18: cast reint s_2_17 -> u8
        let s_2_18: u8 = (s_2_17.value() as u8);
        // D s_2_19: write-var ga#20932 <= s_2_18
        fn_state.ga_20932 = s_2_18;
        // D s_2_20: read-var ga#20932:u8
        let s_2_20: u8 = fn_state.ga_20932;
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 4u16);
        // C s_2_22: const #0u : u8
        let s_2_22: u8 = 0;
        // C s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 4u16);
        // D s_2_24: cmp-eq s_2_21 s_2_23
        let s_2_24: bool = ((s_2_21) == (s_2_23));
        // D s_2_25: not s_2_24
        let s_2_25: bool = !s_2_24;
        // N s_2_26: branch s_2_25 b24 b3
        if s_2_25 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #12s : i
        let s_3_0: i128 = 12;
        // D s_3_1: write-var range_bits <= s_3_0
        fn_state.range_bits = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16648u : u32
        let s_4_0: u32 = 16648;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_GPCCR_EL3_Type_PGS(s_4_1)
        let s_4_2: u8 = u_get_GPCCR_EL3_Type_PGS(state, tracer, s_4_1);
        // D s_4_3: call DecodePGS(s_4_2)
        let s_4_3: u32 = DecodePGS(state, tracer, s_4_2);
        // D s_4_4: write-var ga#20934 <= s_4_3
        fn_state.ga_20934 = s_4_3;
        // C s_4_5: const #0u : u32
        let s_4_5: u32 = 0;
        // D s_4_6: read-var ga#20934:u32
        let s_4_6: u32 = fn_state.ga_20934;
        // D s_4_7: cmp-eq s_4_5 s_4_6
        let s_4_7: bool = ((s_4_5) == (s_4_6));
        // D s_4_8: not s_4_7
        let s_4_8: bool = !s_4_7;
        // N s_4_9: branch s_4_8 b19 b5
        if s_4_8 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #12s : i64
        let s_5_0: i64 = 12;
        // D s_5_1: write-var p <= s_5_0
        fn_state.p = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var p:i64
        let s_6_0: i64 = fn_state.p;
        // D s_6_1: write-var pshadow#503 <= s_6_0
        fn_state.pshadow_503 = s_6_0;
        // D s_6_2: read-var pshadow#503:i64
        let s_6_2: i64 = fn_state.pshadow_503;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var range_bits:i
        let s_6_4: i128 = fn_state.range_bits;
        // D s_6_5: cmp-lt s_6_4 s_6_3
        let s_6_5: bool = ((s_6_4) < (s_6_3));
        // N s_6_6: branch s_6_5 b18 b7
        if s_6_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var range_bits:i
        let s_8_0: i128 = fn_state.range_bits;
        // D s_8_1: write-var range_bitsshadow#504 <= s_8_0
        fn_state.range_bitsshadow_504 = s_8_0;
        // C s_8_2: const #52s : i
        let s_8_2: i128 = 52;
        // S s_8_3: call Zeros(s_8_2)
        let s_8_3: Bits = Zeros(state, tracer, s_8_2);
        // S s_8_4: cast reint s_8_3 -> u52
        let s_8_4: u64 = (s_8_3.value() as u64);
        // D s_8_5: write-var BaseADDR <= s_8_4
        fn_state.BaseADDR = s_8_4;
        // C s_8_6: const #16648u : u32
        let s_8_6: u32 = 16648;
        // D s_8_7: read-reg s_8_6:struct
        let s_8_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_6 as isize);
            tracer.read_register(s_8_6 as isize, value);
            value
        };
        // D s_8_8: call _get_GPCCR_EL3_Type_PGS(s_8_7)
        let s_8_8: u8 = u_get_GPCCR_EL3_Type_PGS(state, tracer, s_8_7);
        // D s_8_9: write-var ga#20936 <= s_8_8
        fn_state.ga_20936 = s_8_8;
        // D s_8_10: read-var ga#20936:u8
        let s_8_10: u8 = fn_state.ga_20936;
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 2u16);
        // C s_8_12: const #0u : u8
        let s_8_12: u8 = 0;
        // C s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 2u16);
        // D s_8_14: cmp-eq s_8_11 s_8_13
        let s_8_14: bool = ((s_8_11) == (s_8_13));
        // D s_8_15: not s_8_14
        let s_8_15: bool = !s_8_14;
        // N s_8_16: branch s_8_15 b13 b9
        if s_8_15 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var Xt:u64
        let s_9_1: u64 = fn_state.Xt;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 64u16);
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #39s : i
        let s_9_5: i128 = 39;
        // C s_9_6: add s_9_5 s_9_4
        let s_9_6: i128 = (s_9_5 + s_9_4);
        // D s_9_7: bit-extract s_9_2 s_9_0 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u40
        let s_9_8: u64 = (s_9_7.value() as u64);
        // C s_9_9: const #12s : i
        let s_9_9: i128 = 12;
        // D s_9_10: read-var BaseADDR:u52
        let s_9_10: u64 = fn_state.BaseADDR;
        // D s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 52u16);
        // D s_9_12: cast zx s_9_8 -> bv
        let s_9_12: Bits = Bits::new(s_9_8 as u128, 40u16);
        // C s_9_13: const #39s : i
        let s_9_13: i128 = 39;
        // C s_9_14: const #1u : u64
        let s_9_14: u64 = 1;
        // C s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 64u16);
        // C s_9_16: lsl s_9_15 s_9_13
        let s_9_16: Bits = s_9_15 << s_9_13;
        // C s_9_17: sub s_9_16 s_9_15
        let s_9_17: Bits = ((s_9_16) - (s_9_15));
        // D s_9_18: and s_9_12 s_9_17
        let s_9_18: Bits = ((s_9_12) & (s_9_17));
        // D s_9_19: lsl s_9_18 s_9_9
        let s_9_19: Bits = s_9_18 << s_9_9;
        // C s_9_20: lsl s_9_17 s_9_9
        let s_9_20: Bits = s_9_17 << s_9_9;
        // C s_9_21: cmpl s_9_20
        let s_9_21: Bits = !s_9_20;
        // D s_9_22: and s_9_11 s_9_21
        let s_9_22: Bits = ((s_9_11) & (s_9_21));
        // D s_9_23: or s_9_22 s_9_19
        let s_9_23: Bits = ((s_9_22) | (s_9_19));
        // D s_9_24: cast reint s_9_23 -> u52
        let s_9_24: u64 = (s_9_23.value() as u64);
        // D s_9_25: write-var BaseADDR <= s_9_24
        fn_state.BaseADDR = s_9_24;
        // N s_9_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var range_bitsshadow#504:i
        let s_10_0: i128 = fn_state.range_bitsshadow_504;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #0s : i
        let s_10_2: i128 = 0;
        // D s_10_3: cmp-ge s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) >= (s_10_2));
        // N s_10_4: assert s_10_3
        let s_10_4: () = assert!(s_10_3);
        // D s_10_5: read-var range_bitsshadow#504:i
        let s_10_5: i128 = fn_state.range_bitsshadow_504;
        // D s_10_6: call __id(s_10_5)
        let s_10_6: i128 = u__id(state, tracer, s_10_5);
        // C s_10_7: const #52s : i
        let s_10_7: i128 = 52;
        // D s_10_8: cmp-ge s_10_7 s_10_6
        let s_10_8: bool = ((s_10_7) >= (s_10_6));
        // N s_10_9: assert s_10_8
        let s_10_9: () = assert!(s_10_8);
        // C s_10_10: const #52s : i
        let s_10_10: i128 = 52;
        // D s_10_11: read-var range_bitsshadow#504:i
        let s_10_11: i128 = fn_state.range_bitsshadow_504;
        // D s_10_12: call zext_ones(s_10_10, s_10_11)
        let s_10_12: Bits = zext_ones(state, tracer, s_10_10, s_10_11);
        // D s_10_13: cast reint s_10_12 -> u52
        let s_10_13: u64 = (s_10_12.value() as u64);
        // D s_10_14: cast zx s_10_13 -> bv
        let s_10_14: Bits = Bits::new(s_10_13 as u128, 52u16);
        // D s_10_15: not s_10_14
        let s_10_15: Bits = !s_10_14;
        // D s_10_16: cast reint s_10_15 -> u52
        let s_10_16: u64 = (s_10_15.value() as u64);
        // D s_10_17: read-var BaseADDR:u52
        let s_10_17: u64 = fn_state.BaseADDR;
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 52u16);
        // D s_10_19: cast zx s_10_16 -> bv
        let s_10_19: Bits = Bits::new(s_10_16 as u128, 52u16);
        // D s_10_20: and s_10_18 s_10_19
        let s_10_20: Bits = ((s_10_18) & (s_10_19));
        // D s_10_21: cast reint s_10_20 -> u52
        let s_10_21: u64 = (s_10_20.value() as u64);
        // C s_10_22: const #52s : i
        let s_10_22: i128 = 52;
        // D s_10_23: read-var range_bitsshadow#504:i
        let s_10_23: i128 = fn_state.range_bitsshadow_504;
        // D s_10_24: call zext_ones(s_10_22, s_10_23)
        let s_10_24: Bits = zext_ones(state, tracer, s_10_22, s_10_23);
        // D s_10_25: cast reint s_10_24 -> u52
        let s_10_25: u64 = (s_10_24.value() as u64);
        // D s_10_26: cast zx s_10_21 -> bv
        let s_10_26: Bits = Bits::new(s_10_21 as u128, 52u16);
        // D s_10_27: cast zx s_10_25 -> bv
        let s_10_27: Bits = Bits::new(s_10_25 as u128, 52u16);
        // D s_10_28: add s_10_26 s_10_27
        let s_10_28: Bits = (s_10_26 + s_10_27);
        // D s_10_29: cast reint s_10_28 -> u52
        let s_10_29: u64 = (s_10_28.value() as u64);
        // C s_10_30: const #64s : i
        let s_10_30: i128 = 64;
        // D s_10_31: cast zx s_10_21 -> bv
        let s_10_31: Bits = Bits::new(s_10_21 as u128, 52u16);
        // D s_10_32: bits-cast zx s_10_31 -> bv length s_10_30
        let s_10_32: Bits = s_10_31.zero_extend(s_10_30);
        // D s_10_33: cast reint s_10_32 -> u64
        let s_10_33: u64 = (s_10_32.value() as u64);
        // D s_10_34: write-var r.0 <= s_10_33
        fn_state.r._0 = s_10_33;
        // C s_10_35: const #64s : i
        let s_10_35: i128 = 64;
        // D s_10_36: cast zx s_10_29 -> bv
        let s_10_36: Bits = Bits::new(s_10_29 as u128, 52u16);
        // D s_10_37: bits-cast zx s_10_36 -> bv length s_10_35
        let s_10_37: Bits = s_10_36.zero_extend(s_10_35);
        // D s_10_38: cast reint s_10_37 -> u64
        let s_10_38: u64 = (s_10_37.value() as u64);
        // D s_10_39: write-var r.5 <= s_10_38
        fn_state.r._5 = s_10_38;
        // D s_10_40: read-var r:struct
        let s_10_40: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_10_41: read-var shareability:u32
        let s_10_41: u32 = fn_state.shareability;
        // D s_10_42: call TLBI(s_10_40, s_10_41)
        let s_10_42: () = TLBI(state, tracer, s_10_40, s_10_41);
        // D s_10_43: read-var shareability:u32
        let s_10_43: u32 = fn_state.shareability;
        // C s_10_44: const #0u : u32
        let s_10_44: u32 = 0;
        // D s_10_45: cmp-eq s_10_43 s_10_44
        let s_10_45: bool = ((s_10_43) == (s_10_44));
        // N s_10_46: branch s_10_45 b12 b11
        if s_10_45 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var shareability:u32
        let s_12_0: u32 = fn_state.shareability;
        // D s_12_1: read-var r:struct
        let s_12_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_12_2: call Broadcast(s_12_0, s_12_1)
        let s_12_2: () = Broadcast(state, tracer, s_12_0, s_12_1);
        // N s_12_3: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#20936:u8
        let s_13_0: u8 = fn_state.ga_20936;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #2s : i
        let s_14_0: i128 = 2;
        // D s_14_1: read-var Xt:u64
        let s_14_1: u64 = fn_state.Xt;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 64u16);
        // C s_14_3: const #1s : i64
        let s_14_3: i64 = 1;
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #37s : i
        let s_14_5: i128 = 37;
        // C s_14_6: add s_14_5 s_14_4
        let s_14_6: i128 = (s_14_5 + s_14_4);
        // D s_14_7: bit-extract s_14_2 s_14_0 s_14_6
        let s_14_7: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_6).unwrap(),
        ));
        // D s_14_8: cast reint s_14_7 -> u38
        let s_14_8: u64 = (s_14_7.value() as u64);
        // C s_14_9: const #14s : i
        let s_14_9: i128 = 14;
        // D s_14_10: read-var BaseADDR:u52
        let s_14_10: u64 = fn_state.BaseADDR;
        // D s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 52u16);
        // D s_14_12: cast zx s_14_8 -> bv
        let s_14_12: Bits = Bits::new(s_14_8 as u128, 38u16);
        // C s_14_13: const #37s : i
        let s_14_13: i128 = 37;
        // C s_14_14: const #1u : u64
        let s_14_14: u64 = 1;
        // C s_14_15: cast zx s_14_14 -> bv
        let s_14_15: Bits = Bits::new(s_14_14 as u128, 64u16);
        // C s_14_16: lsl s_14_15 s_14_13
        let s_14_16: Bits = s_14_15 << s_14_13;
        // C s_14_17: sub s_14_16 s_14_15
        let s_14_17: Bits = ((s_14_16) - (s_14_15));
        // D s_14_18: and s_14_12 s_14_17
        let s_14_18: Bits = ((s_14_12) & (s_14_17));
        // D s_14_19: lsl s_14_18 s_14_9
        let s_14_19: Bits = s_14_18 << s_14_9;
        // C s_14_20: lsl s_14_17 s_14_9
        let s_14_20: Bits = s_14_17 << s_14_9;
        // C s_14_21: cmpl s_14_20
        let s_14_21: Bits = !s_14_20;
        // D s_14_22: and s_14_11 s_14_21
        let s_14_22: Bits = ((s_14_11) & (s_14_21));
        // D s_14_23: or s_14_22 s_14_19
        let s_14_23: Bits = ((s_14_22) | (s_14_19));
        // D s_14_24: cast reint s_14_23 -> u52
        let s_14_24: u64 = (s_14_23.value() as u64);
        // D s_14_25: write-var BaseADDR <= s_14_24
        fn_state.BaseADDR = s_14_24;
        // N s_14_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#20936:u8
        let s_15_0: u8 = fn_state.ga_20936;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #1u : u8
        let s_15_2: u8 = 1;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #4s : i
        let s_16_0: i128 = 4;
        // D s_16_1: read-var Xt:u64
        let s_16_1: u64 = fn_state.Xt;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 64u16);
        // C s_16_3: const #1s : i64
        let s_16_3: i64 = 1;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #35s : i
        let s_16_5: i128 = 35;
        // C s_16_6: add s_16_5 s_16_4
        let s_16_6: i128 = (s_16_5 + s_16_4);
        // D s_16_7: bit-extract s_16_2 s_16_0 s_16_6
        let s_16_7: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D s_16_8: cast reint s_16_7 -> u36
        let s_16_8: u64 = (s_16_7.value() as u64);
        // C s_16_9: const #16s : i
        let s_16_9: i128 = 16;
        // D s_16_10: read-var BaseADDR:u52
        let s_16_10: u64 = fn_state.BaseADDR;
        // D s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 52u16);
        // D s_16_12: cast zx s_16_8 -> bv
        let s_16_12: Bits = Bits::new(s_16_8 as u128, 36u16);
        // C s_16_13: const #35s : i
        let s_16_13: i128 = 35;
        // C s_16_14: const #1u : u64
        let s_16_14: u64 = 1;
        // C s_16_15: cast zx s_16_14 -> bv
        let s_16_15: Bits = Bits::new(s_16_14 as u128, 64u16);
        // C s_16_16: lsl s_16_15 s_16_13
        let s_16_16: Bits = s_16_15 << s_16_13;
        // C s_16_17: sub s_16_16 s_16_15
        let s_16_17: Bits = ((s_16_16) - (s_16_15));
        // D s_16_18: and s_16_12 s_16_17
        let s_16_18: Bits = ((s_16_12) & (s_16_17));
        // D s_16_19: lsl s_16_18 s_16_9
        let s_16_19: Bits = s_16_18 << s_16_9;
        // C s_16_20: lsl s_16_17 s_16_9
        let s_16_20: Bits = s_16_17 << s_16_9;
        // C s_16_21: cmpl s_16_20
        let s_16_21: Bits = !s_16_20;
        // D s_16_22: and s_16_11 s_16_21
        let s_16_22: Bits = ((s_16_11) & (s_16_21));
        // D s_16_23: or s_16_22 s_16_19
        let s_16_23: Bits = ((s_16_22) | (s_16_19));
        // D s_16_24: cast reint s_16_23 -> u52
        let s_16_24: u64 = (s_16_23.value() as u64);
        // D s_16_25: write-var BaseADDR <= s_16_24
        fn_state.BaseADDR = s_16_24;
        // N s_16_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var pshadow#503:i64
        let s_18_0: i64 = fn_state.pshadow_503;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: write-var range_bits <= s_18_1
        fn_state.range_bits = s_18_1;
        // N s_18_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u32
        let s_19_0: u32 = 1;
        // D s_19_1: read-var ga#20934:u32
        let s_19_1: u32 = fn_state.ga_20934;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b21 b20
        if s_19_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #14s : i64
        let s_20_0: i64 = 14;
        // D s_20_1: write-var p <= s_20_0
        fn_state.p = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #2u : u32
        let s_21_0: u32 = 2;
        // D s_21_1: read-var ga#20934:u32
        let s_21_1: u32 = fn_state.ga_20934;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: not s_21_2
        let s_21_3: bool = !s_21_2;
        // N s_21_4: branch s_21_3 b23 b22
        if s_21_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #16s : i64
        let s_22_0: i64 = 16;
        // D s_22_1: write-var p <= s_22_0
        fn_state.p = s_22_0;
        // N s_22_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#20932:u8
        let s_24_0: u8 = fn_state.ga_20932;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 4u16);
        // C s_24_2: const #1u : u8
        let s_24_2: u8 = 1;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 4u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: not s_24_4
        let s_24_5: bool = !s_24_4;
        // N s_24_6: branch s_24_5 b26 b25
        if s_24_5 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #14s : i
        let s_25_0: i128 = 14;
        // D s_25_1: write-var range_bits <= s_25_0
        fn_state.range_bits = s_25_0;
        // N s_25_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var ga#20932:u8
        let s_26_0: u8 = fn_state.ga_20932;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 4u16);
        // C s_26_2: const #2u : u8
        let s_26_2: u8 = 2;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 4u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: not s_26_4
        let s_26_5: bool = !s_26_4;
        // N s_26_6: branch s_26_5 b28 b27
        if s_26_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #16s : i
        let s_27_0: i128 = 16;
        // D s_27_1: write-var range_bits <= s_27_0
        fn_state.range_bits = s_27_0;
        // N s_27_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var ga#20932:u8
        let s_28_0: u8 = fn_state.ga_20932;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 4u16);
        // C s_28_2: const #3u : u8
        let s_28_2: u8 = 3;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 4u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: not s_28_4
        let s_28_5: bool = !s_28_4;
        // N s_28_6: branch s_28_5 b30 b29
        if s_28_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #21s : i
        let s_29_0: i128 = 21;
        // D s_29_1: write-var range_bits <= s_29_0
        fn_state.range_bits = s_29_0;
        // N s_29_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ga#20932:u8
        let s_30_0: u8 = fn_state.ga_20932;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 4u16);
        // C s_30_2: const #4u : u8
        let s_30_2: u8 = 4;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 4u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: not s_30_4
        let s_30_5: bool = !s_30_4;
        // N s_30_6: branch s_30_5 b32 b31
        if s_30_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #25s : i
        let s_31_0: i128 = 25;
        // D s_31_1: write-var range_bits <= s_31_0
        fn_state.range_bits = s_31_0;
        // N s_31_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var ga#20932:u8
        let s_32_0: u8 = fn_state.ga_20932;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 4u16);
        // C s_32_2: const #5u : u8
        let s_32_2: u8 = 5;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 4u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: not s_32_4
        let s_32_5: bool = !s_32_4;
        // N s_32_6: branch s_32_5 b34 b33
        if s_32_5 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #29s : i
        let s_33_0: i128 = 29;
        // D s_33_1: write-var range_bits <= s_33_0
        fn_state.range_bits = s_33_0;
        // N s_33_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var ga#20932:u8
        let s_34_0: u8 = fn_state.ga_20932;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 4u16);
        // C s_34_2: const #6u : u8
        let s_34_2: u8 = 6;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 4u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: not s_34_4
        let s_34_5: bool = !s_34_4;
        // N s_34_6: branch s_34_5 b36 b35
        if s_34_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #30s : i
        let s_35_0: i128 = 30;
        // D s_35_1: write-var range_bits <= s_35_0
        fn_state.range_bits = s_35_0;
        // N s_35_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var ga#20932:u8
        let s_36_0: u8 = fn_state.ga_20932;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 4u16);
        // C s_36_2: const #7u : u8
        let s_36_2: u8 = 7;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 4u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b38 b37
        if s_36_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #34s : i
        let s_37_0: i128 = 34;
        // D s_37_1: write-var range_bits <= s_37_0
        fn_state.range_bits = s_37_0;
        // N s_37_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var ga#20932:u8
        let s_38_0: u8 = fn_state.ga_20932;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 4u16);
        // C s_38_2: const #8u : u8
        let s_38_2: u8 = 8;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 4u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: not s_38_4
        let s_38_5: bool = !s_38_4;
        // N s_38_6: branch s_38_5 b40 b39
        if s_38_5 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #36s : i
        let s_39_0: i128 = 36;
        // D s_39_1: write-var range_bits <= s_39_0
        fn_state.range_bits = s_39_0;
        // N s_39_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var ga#20932:u8
        let s_40_0: u8 = fn_state.ga_20932;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 4u16);
        // C s_40_2: const #9u : u8
        let s_40_2: u8 = 9;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 4u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: not s_40_4
        let s_40_5: bool = !s_40_4;
        // N s_40_6: branch s_40_5 b42 b41
        if s_40_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #39s : i
        let s_41_0: i128 = 39;
        // D s_41_1: write-var range_bits <= s_41_0
        fn_state.range_bits = s_41_0;
        // N s_41_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0s : i
        let s_42_0: i128 = 0;
        // D s_42_1: write-var range_bits <= s_42_0
        fn_state.range_bits = s_42_0;
        // N s_42_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #16975u : u32
        let s_43_0: u32 = 16975;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 2u16);
        // C s_43_3: const #424u : u32
        let s_43_3: u32 = 424;
        // D s_43_4: read-reg s_43_3:u8
        let s_43_4: u8 = {
            let value = state.read_register::<u8>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 2u16);
        // D s_43_6: cmp-eq s_43_2 s_43_5
        let s_43_6: bool = ((s_43_2) == (s_43_5));
        // D s_43_7: write-var gs#27173 <= s_43_6
        fn_state.gs_27173 = s_43_6;
        // N s_43_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
