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
use DBGBCR_read::*;
use AArch32_StateMatch::*;
use ConstrainUnpredictableBool::*;
use u_get_DBGBCR_Type_E::*;
use u_get_DBGBCR_Type_PMC::*;
use S1TranslationRegime__1::*;
use u_get_DBGBCR_Type_HMC::*;
use NumBreakpointsImplemented::*;
use u_get_DBGBCR_Type_SSC::*;
use u_get_DBGBCR_Type_BAS::*;
use ELUsingAArch32::*;
use AArch32_BreakpointValueMatch::*;
use u_get_DBGBCR_Type_BT::*;
use u_get_DBGBCR_Type_LBN::*;
use common::*;
pub fn AArch32_BreakpointMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    vaddress: u32,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        gs_29993: bool,
        state_match: bool,
        gs_30003: bool,
        val_match: bool,
        gs_30009: bool,
        enabled: bool,
        match_i: bool,
        ga_23186: ProductType8b847afc727d5818,
        gs_29978: bool,
        gs_30002: bool,
        value_mismatch_name: bool,
        b__0: u8,
        mismatch_i: bool,
        value_match_name: bool,
        ga_23189: ProductType8b847afc727d5818,
        gs_30010: bool,
        gs_30008: bool,
        gs_30011: bool,
        gs_29983: bool,
        n: i64,
        vaddress: u32,
        accdesc: ProductType9878976b5bcce9c9,
        size: i128,
    }
    let fn_state = FunctionState {
        n,
        vaddress,
        accdesc,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call S1TranslationRegime__1(s_0_0)
        let s_0_1: u8 = S1TranslationRegime__1(state, tracer, s_0_0);
        // S s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call NumBreakpointsImplemented(s_0_4)
        let s_0_5: i128 = NumBreakpointsImplemented(state, tracer, s_0_4);
        // D s_0_6: read-var n:i64
        let s_0_6: i64 = fn_state.n;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: cmp-lt s_0_7 s_0_5
        let s_0_8: bool = ((s_0_7) < (s_0_5));
        // N s_0_9: assert s_0_8
        let s_0_9: () = assert!(s_0_8);
        // D s_0_10: read-var n:i64
        let s_0_10: i64 = fn_state.n;
        // D s_0_11: call DBGBCR_read(s_0_10)
        let s_0_11: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_0_10);
        // D s_0_12: call _get_DBGBCR_Type_E(s_0_11)
        let s_0_12: bool = u_get_DBGBCR_Type_E(state, tracer, s_0_11);
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // C s_0_14: const #1u : u8
        let s_0_14: bool = true;
        // C s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 1u16);
        // D s_0_16: cmp-eq s_0_13 s_0_15
        let s_0_16: bool = ((s_0_13) == (s_0_15));
        // D s_0_17: write-var enabled <= s_0_16
        fn_state.enabled = s_0_16;
        // D s_0_18: read-var n:i64
        let s_0_18: i64 = fn_state.n;
        // D s_0_19: call DBGBCR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_0_18);
        // D s_0_20: call _get_DBGBCR_Type_BT(s_0_19)
        let s_0_20: u8 = u_get_DBGBCR_Type_BT(state, tracer, s_0_19);
        // D s_0_21: write-var b__0 <= s_0_20
        fn_state.b__0 = s_0_20;
        // C s_0_22: const #3s : i
        let s_0_22: i128 = 3;
        // D s_0_23: read-var b__0:u8
        let s_0_23: u8 = fn_state.b__0;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 4u16);
        // C s_0_25: const #1s : i64
        let s_0_25: i64 = 1;
        // C s_0_26: cast zx s_0_25 -> i
        let s_0_26: i128 = (i128::try_from(s_0_25).unwrap());
        // C s_0_27: const #0s : i
        let s_0_27: i128 = 0;
        // C s_0_28: add s_0_27 s_0_26
        let s_0_28: i128 = (s_0_27 + s_0_26);
        // D s_0_29: bit-extract s_0_24 s_0_22 s_0_28
        let s_0_29: Bits = (Bits::new(
            ((s_0_24) >> (s_0_22)).value(),
            u16::try_from(s_0_28).unwrap(),
        ));
        // D s_0_30: cast reint s_0_29 -> u8
        let s_0_30: bool = ((s_0_29.value()) != 0);
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 1u16);
        // C s_0_32: const #0u : u8
        let s_0_32: bool = false;
        // C s_0_33: cast zx s_0_32 -> bv
        let s_0_33: Bits = Bits::new(s_0_32 as u128, 1u16);
        // D s_0_34: cmp-eq s_0_31 s_0_33
        let s_0_34: bool = ((s_0_31) == (s_0_33));
        // N s_0_35: branch s_0_34 b45 b1
        if s_0_34 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#29983 <= s_1_0
        fn_state.gs_29983 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_2_0: read-var gs#29983:u8
        let s_2_0: bool = fn_state.gs_29983;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b44 b3
        if s_2_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#29978 <= s_3_0
        fn_state.gs_29978 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_4_0: read-var gs#29978:u8
        let s_4_0: bool = fn_state.gs_29978;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: call DBGBCR_read(s_4_1)
        let s_4_2: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_4_1);
        // D s_4_3: call _get_DBGBCR_Type_LBN(s_4_2)
        let s_4_3: u8 = u_get_DBGBCR_Type_LBN(state, tracer, s_4_2);
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 4u16);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (s_4_4.value() as i128);
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: read-var n:i64
        let s_4_7: i64 = fn_state.n;
        // D s_4_8: call DBGBCR_read(s_4_7)
        let s_4_8: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_4_7);
        // D s_4_9: call _get_DBGBCR_Type_SSC(s_4_8)
        let s_4_9: u8 = u_get_DBGBCR_Type_SSC(state, tracer, s_4_8);
        // D s_4_10: read-var n:i64
        let s_4_10: i64 = fn_state.n;
        // D s_4_11: call DBGBCR_read(s_4_10)
        let s_4_11: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_4_10);
        // D s_4_12: call _get_DBGBCR_Type_HMC(s_4_11)
        let s_4_12: bool = u_get_DBGBCR_Type_HMC(state, tracer, s_4_11);
        // D s_4_13: read-var n:i64
        let s_4_13: i64 = fn_state.n;
        // D s_4_14: call DBGBCR_read(s_4_13)
        let s_4_14: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_4_13);
        // D s_4_15: call _get_DBGBCR_Type_PMC(s_4_14)
        let s_4_15: u8 = u_get_DBGBCR_Type_PMC(state, tracer, s_4_14);
        // D s_4_16: cast zx s_4_6 -> i
        let s_4_16: i128 = (i128::try_from(s_4_6).unwrap());
        // C s_4_17: const #1u : u8
        let s_4_17: bool = true;
        // D s_4_18: read-var accdesc:struct
        let s_4_18: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_19: call AArch32_StateMatch(s_4_9, s_4_12, s_4_15, s_4_0, s_4_16, s_4_17, s_4_18)
        let s_4_19: bool = AArch32_StateMatch(
            state,
            tracer,
            s_4_9,
            s_4_12,
            s_4_15,
            s_4_0,
            s_4_16,
            s_4_17,
            s_4_18,
        );
        // D s_4_20: write-var state_match <= s_4_19
        fn_state.state_match = s_4_19;
        // D s_4_21: read-var n:i64
        let s_4_21: i64 = fn_state.n;
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_23: read-var vaddress:u32
        let s_4_23: u32 = fn_state.vaddress;
        // C s_4_24: const #0u : u8
        let s_4_24: bool = false;
        // D s_4_25: call AArch32_BreakpointValueMatch(s_4_22, s_4_23, s_4_24)
        let s_4_25: ProductType8b847afc727d5818 = AArch32_BreakpointValueMatch(
            state,
            tracer,
            s_4_22,
            s_4_23,
            s_4_24,
        );
        // D s_4_26: write-var ga#23186 <= s_4_25
        fn_state.ga_23186 = s_4_25;
        // D s_4_27: read-var ga#23186.0:struct
        let s_4_27: bool = fn_state.ga_23186._0;
        // D s_4_28: read-var ga#23186.1:struct
        let s_4_28: bool = fn_state.ga_23186._1;
        // D s_4_29: write-var value_match_name <= s_4_27
        fn_state.value_match_name = s_4_27;
        // D s_4_30: write-var value_mismatch_name <= s_4_28
        fn_state.value_mismatch_name = s_4_28;
        // C s_4_31: const #4s : i
        let s_4_31: i128 = 4;
        // D s_4_32: read-var size:i
        let s_4_32: i128 = fn_state.size;
        // D s_4_33: cmp-eq s_4_32 s_4_31
        let s_4_33: bool = ((s_4_32) == (s_4_31));
        // N s_4_34: branch s_4_33 b31 b5
        if s_4_33 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var vaddress:u32
        let s_6_1: u32 = fn_state.vaddress;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // C s_6_3: const #1u : u64
        let s_6_3: u64 = 1;
        // D s_6_4: bit-extract s_6_2 s_6_0 s_6_3
        let s_6_4: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: bool = ((s_6_4.value()) != 0);
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // C s_6_7: const #0u : u64
        let s_6_7: u64 = 0;
        // D s_6_8: cast zx s_6_5 -> u64
        let s_6_8: u64 = (s_6_5 as u64);
        // C s_6_9: const #1u : u64
        let s_6_9: u64 = 1;
        // D s_6_10: and s_6_8 s_6_9
        let s_6_10: u64 = ((s_6_8) & (s_6_9));
        // D s_6_11: cmp-eq s_6_10 s_6_9
        let s_6_11: bool = ((s_6_10) == (s_6_9));
        // D s_6_12: lsl s_6_8 s_6_6
        let s_6_12: u64 = s_6_8 << s_6_6;
        // D s_6_13: or s_6_7 s_6_12
        let s_6_13: u64 = ((s_6_7) | (s_6_12));
        // D s_6_14: cmpl s_6_12
        let s_6_14: u64 = !s_6_12;
        // D s_6_15: and s_6_7 s_6_14
        let s_6_15: u64 = ((s_6_7) & (s_6_14));
        // D s_6_16: select s_6_11 s_6_13 s_6_15
        let s_6_16: u64 = if s_6_11 { s_6_13 } else { s_6_15 };
        // D s_6_17: cast trunc s_6_16 -> u8
        let s_6_17: bool = ((s_6_16) != 0);
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // C s_6_19: const #1u : u8
        let s_6_19: bool = true;
        // C s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cmp-eq s_6_18 s_6_20
        let s_6_21: bool = ((s_6_18) == (s_6_20));
        // N s_6_22: branch s_6_21 b30 b7
        if s_6_21 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#29993 <= s_7_0
        fn_state.gs_29993 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_8_0: read-var gs#29993:u8
        let s_8_0: bool = fn_state.gs_29993;
        // N s_8_1: branch s_8_0 b23 b9
        if s_8_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_10_0: read-var value_match_name:u8
        let s_10_0: bool = fn_state.value_match_name;
        // N s_10_1: branch s_10_0 b22 b11
        if s_10_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#30008 <= s_11_0
        fn_state.gs_30008 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_12_0: read-var gs#30008:u8
        let s_12_0: bool = fn_state.gs_30008;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#30009 <= s_13_0
        fn_state.gs_30009 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_14_0: read-var gs#30009:u8
        let s_14_0: bool = fn_state.gs_30009;
        // D s_14_1: write-var val_match <= s_14_0
        fn_state.val_match = s_14_0;
        // D s_14_2: read-var value_mismatch_name:u8
        let s_14_2: bool = fn_state.value_mismatch_name;
        // N s_14_3: branch s_14_2 b20 b15
        if s_14_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#30010 <= s_15_0
        fn_state.gs_30010 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_16_0: read-var gs#30010:u8
        let s_16_0: bool = fn_state.gs_30010;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#30011 <= s_17_0
        fn_state.gs_30011 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_18_0: read-var gs#30011:u8
        let s_18_0: bool = fn_state.gs_30011;
        // D s_18_1: read-var val_match:u8
        let s_18_1: bool = fn_state.val_match;
        // D s_18_2: create-product struct = ["s_18_1", "s_18_0"]
        let s_18_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_18_1,
            _1: s_18_0,
        };
        // N s_18_3: return s_18_2
        return s_18_2;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_19_0: read-var enabled:u8
        let s_19_0: bool = fn_state.enabled;
        // D s_19_1: write-var gs#30011 <= s_19_0
        fn_state.gs_30011 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_20_0: read-var state_match:u8
        let s_20_0: bool = fn_state.state_match;
        // D s_20_1: write-var gs#30010 <= s_20_0
        fn_state.gs_30010 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_21_0: read-var enabled:u8
        let s_21_0: bool = fn_state.enabled;
        // D s_21_1: write-var gs#30009 <= s_21_0
        fn_state.gs_30009 = s_21_0;
        // N s_21_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_22_0: read-var state_match:u8
        let s_22_0: bool = fn_state.state_match;
        // D s_22_1: write-var gs#30008 <= s_22_0
        fn_state.gs_30008 = s_22_0;
        // N s_22_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_23_0: read-var value_match_name:u8
        let s_23_0: bool = fn_state.value_match_name;
        // N s_23_1: branch s_23_0 b29 b24
        if s_23_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_25_0: read-var value_mismatch_name:u8
        let s_25_0: bool = fn_state.value_mismatch_name;
        // D s_25_1: not s_25_0
        let s_25_1: bool = !s_25_0;
        // N s_25_2: branch s_25_1 b28 b26
        if s_25_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_27_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_28_0: const #37u : u32
        let s_28_0: u32 = 37;
        // S s_28_1: call ConstrainUnpredictableBool(s_28_0)
        let s_28_1: bool = ConstrainUnpredictableBool(state, tracer, s_28_0);
        // D s_28_2: write-var value_mismatch_name <= s_28_1
        fn_state.value_mismatch_name = s_28_1;
        // N s_28_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_29_0: const #36u : u32
        let s_29_0: u32 = 36;
        // S s_29_1: call ConstrainUnpredictableBool(s_29_0)
        let s_29_1: bool = ConstrainUnpredictableBool(state, tracer, s_29_0);
        // D s_29_2: write-var value_match_name <= s_29_1
        fn_state.value_match_name = s_29_1;
        // N s_29_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_30_0: read-var n:i64
        let s_30_0: i64 = fn_state.n;
        // D s_30_1: call DBGBCR_read(s_30_0)
        let s_30_1: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_30_0);
        // D s_30_2: call _get_DBGBCR_Type_BAS(s_30_1)
        let s_30_2: u8 = u_get_DBGBCR_Type_BAS(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 4u16);
        // C s_30_4: const #15u : u8
        let s_30_4: u8 = 15;
        // C s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 4u16);
        // D s_30_6: cmp-eq s_30_3 s_30_5
        let s_30_6: bool = ((s_30_3) == (s_30_5));
        // D s_30_7: write-var gs#29993 <= s_30_6
        fn_state.gs_29993 = s_30_6;
        // N s_30_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_31_0: const #2s : i
        let s_31_0: i128 = 2;
        // D s_31_1: read-var vaddress:u32
        let s_31_1: u32 = fn_state.vaddress;
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 32u16);
        // C s_31_3: cast cvt s_31_0 -> bv
        let s_31_3: Bits = Bits::new(s_31_0 as u128, 128);
        // D s_31_4: add s_31_2 s_31_3
        let s_31_4: Bits = (s_31_2 + s_31_3);
        // D s_31_5: cast reint s_31_4 -> u32
        let s_31_5: u32 = (s_31_4.value() as u32);
        // D s_31_6: read-var n:i64
        let s_31_6: i64 = fn_state.n;
        // D s_31_7: cast zx s_31_6 -> i
        let s_31_7: i128 = (i128::try_from(s_31_6).unwrap());
        // C s_31_8: const #0u : u8
        let s_31_8: bool = false;
        // D s_31_9: call AArch32_BreakpointValueMatch(s_31_7, s_31_5, s_31_8)
        let s_31_9: ProductType8b847afc727d5818 = AArch32_BreakpointValueMatch(
            state,
            tracer,
            s_31_7,
            s_31_5,
            s_31_8,
        );
        // D s_31_10: write-var ga#23189 <= s_31_9
        fn_state.ga_23189 = s_31_9;
        // D s_31_11: read-var ga#23189.0:struct
        let s_31_11: bool = fn_state.ga_23189._0;
        // D s_31_12: read-var ga#23189.1:struct
        let s_31_12: bool = fn_state.ga_23189._1;
        // D s_31_13: write-var match_i <= s_31_11
        fn_state.match_i = s_31_11;
        // D s_31_14: write-var mismatch_i <= s_31_12
        fn_state.mismatch_i = s_31_12;
        // D s_31_15: read-var value_match_name:u8
        let s_31_15: bool = fn_state.value_match_name;
        // D s_31_16: not s_31_15
        let s_31_16: bool = !s_31_15;
        // N s_31_17: branch s_31_16 b43 b32
        if s_31_16 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#30002 <= s_32_0
        fn_state.gs_30002 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_33_0: read-var gs#30002:u8
        let s_33_0: bool = fn_state.gs_30002;
        // N s_33_1: branch s_33_0 b42 b34
        if s_33_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_35_0: read-var value_mismatch_name:u8
        let s_35_0: bool = fn_state.value_mismatch_name;
        // N s_35_1: branch s_35_0 b41 b36
        if s_35_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#30003 <= s_36_0
        fn_state.gs_30003 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_37_0: read-var gs#30003:u8
        let s_37_0: bool = fn_state.gs_30003;
        // N s_37_1: branch s_37_0 b40 b38
        if s_37_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_39_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_40_0: const #37u : u32
        let s_40_0: u32 = 37;
        // S s_40_1: call ConstrainUnpredictableBool(s_40_0)
        let s_40_1: bool = ConstrainUnpredictableBool(state, tracer, s_40_0);
        // D s_40_2: write-var value_mismatch_name <= s_40_1
        fn_state.value_mismatch_name = s_40_1;
        // N s_40_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_41_0: read-var mismatch_i:u8
        let s_41_0: bool = fn_state.mismatch_i;
        // D s_41_1: not s_41_0
        let s_41_1: bool = !s_41_0;
        // D s_41_2: write-var gs#30003 <= s_41_1
        fn_state.gs_30003 = s_41_1;
        // N s_41_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_42_0: const #36u : u32
        let s_42_0: u32 = 36;
        // S s_42_1: call ConstrainUnpredictableBool(s_42_0)
        let s_42_1: bool = ConstrainUnpredictableBool(state, tracer, s_42_0);
        // D s_42_2: write-var value_match_name <= s_42_1
        fn_state.value_match_name = s_42_1;
        // N s_42_3: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_43_0: read-var match_i:u8
        let s_43_0: bool = fn_state.match_i;
        // D s_43_1: write-var gs#30002 <= s_43_0
        fn_state.gs_30002 = s_43_0;
        // N s_43_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#29978 <= s_44_0
        fn_state.gs_29978 = s_44_0;
        // N s_44_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_45_0: const #0s : i
        let s_45_0: i128 = 0;
        // D s_45_1: read-var b__0:u8
        let s_45_1: u8 = fn_state.b__0;
        // D s_45_2: cast zx s_45_1 -> bv
        let s_45_2: Bits = Bits::new(s_45_1 as u128, 4u16);
        // C s_45_3: const #1s : i64
        let s_45_3: i64 = 1;
        // C s_45_4: cast zx s_45_3 -> i
        let s_45_4: i128 = (i128::try_from(s_45_3).unwrap());
        // C s_45_5: const #1s : i
        let s_45_5: i128 = 1;
        // C s_45_6: add s_45_5 s_45_4
        let s_45_6: i128 = (s_45_5 + s_45_4);
        // D s_45_7: bit-extract s_45_2 s_45_0 s_45_6
        let s_45_7: Bits = (Bits::new(
            ((s_45_2) >> (s_45_0)).value(),
            u16::try_from(s_45_6).unwrap(),
        ));
        // D s_45_8: cast reint s_45_7 -> u8
        let s_45_8: u8 = (s_45_7.value() as u8);
        // D s_45_9: cast zx s_45_8 -> bv
        let s_45_9: Bits = Bits::new(s_45_8 as u128, 2u16);
        // C s_45_10: const #1u : u8
        let s_45_10: u8 = 1;
        // C s_45_11: cast zx s_45_10 -> bv
        let s_45_11: Bits = Bits::new(s_45_10 as u128, 2u16);
        // D s_45_12: cmp-eq s_45_9 s_45_11
        let s_45_12: bool = ((s_45_9) == (s_45_11));
        // D s_45_13: write-var gs#29983 <= s_45_12
        fn_state.gs_29983 = s_45_12;
        // N s_45_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
