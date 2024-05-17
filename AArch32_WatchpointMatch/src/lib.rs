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
use u_get_DBGWCR_Type_HMC::*;
use u_get_DBGWCR_Type_WT::*;
use NumWatchpointsImplemented::*;
use u_get_DBGWCR_Type_SSC::*;
use u_get_DBGWCR_Type_PAC::*;
use ELUsingAArch32::*;
use DBGWCR_read::*;
use AArch32_StateMatch::*;
use AArch32_WatchpointByteMatch::*;
use S1TranslationRegime__1::*;
use u_get_DBGWCR_Type_LSC::*;
use u_get_DBGWCR_Type_E::*;
use u_get_DBGWCR_EL1_Type_LBN::*;
use common::*;
pub fn AArch32_WatchpointMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    vaddress: u32,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_30205: bool,
        gs_30206: bool,
        gs_30204: bool,
        gs_30190: bool,
        state_match: bool,
        ga_23378: u8,
        enabled: bool,
        value_match_name: bool,
        gs_30201: bool,
        gs_30198: i64,
        ls_match: bool,
        byte: i64,
        n: i64,
        vaddress: u32,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        n,
        vaddress,
        size,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
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
        // S s_0_5: call NumWatchpointsImplemented(s_0_4)
        let s_0_5: i128 = NumWatchpointsImplemented(state, tracer, s_0_4);
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
        // D s_0_11: call DBGWCR_read(s_0_10)
        let s_0_11: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_0_10);
        // D s_0_12: call _get_DBGWCR_Type_E(s_0_11)
        let s_0_12: bool = u_get_DBGWCR_Type_E(state, tracer, s_0_11);
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
        // D s_0_19: call DBGWCR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_0_18);
        // D s_0_20: call _get_DBGWCR_Type_WT(s_0_19)
        let s_0_20: bool = u_get_DBGWCR_Type_WT(state, tracer, s_0_19);
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 1u16);
        // C s_0_22: const #1u : u8
        let s_0_22: bool = true;
        // C s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 1u16);
        // D s_0_24: cmp-eq s_0_21 s_0_23
        let s_0_24: bool = ((s_0_21) == (s_0_23));
        // C s_0_25: const #0u : u8
        let s_0_25: bool = false;
        // C s_0_26: const #103984u : u32
        let s_0_26: u32 = 103984;
        // D s_0_27: read-reg s_0_26:[struct; 64]
        let s_0_27: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // D s_0_28: read-var n:i64
        let s_0_28: i64 = fn_state.n;
        // D s_0_29: cast zx s_0_28 -> i
        let s_0_29: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_30: read-element s_0_27[s_0_29]
        let s_0_30: ProductType5c790c8ef59cc8b2 = s_0_27[(s_0_29) as usize];
        // D s_0_31: call _get_DBGWCR_EL1_Type_LBN(s_0_30)
        let s_0_31: u8 = u_get_DBGWCR_EL1_Type_LBN(state, tracer, s_0_30);
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 4u16);
        // D s_0_33: cast zx s_0_32 -> i
        let s_0_33: i128 = (s_0_32.value() as i128);
        // D s_0_34: cast reint s_0_33 -> i64
        let s_0_34: i64 = (s_0_33 as i64);
        // D s_0_35: read-var n:i64
        let s_0_35: i64 = fn_state.n;
        // D s_0_36: call DBGWCR_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_0_35);
        // D s_0_37: call _get_DBGWCR_Type_SSC(s_0_36)
        let s_0_37: u8 = u_get_DBGWCR_Type_SSC(state, tracer, s_0_36);
        // D s_0_38: read-var n:i64
        let s_0_38: i64 = fn_state.n;
        // D s_0_39: call DBGWCR_read(s_0_38)
        let s_0_39: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_0_38);
        // D s_0_40: call _get_DBGWCR_Type_HMC(s_0_39)
        let s_0_40: bool = u_get_DBGWCR_Type_HMC(state, tracer, s_0_39);
        // D s_0_41: read-var n:i64
        let s_0_41: i64 = fn_state.n;
        // D s_0_42: call DBGWCR_read(s_0_41)
        let s_0_42: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_0_41);
        // D s_0_43: call _get_DBGWCR_Type_PAC(s_0_42)
        let s_0_43: u8 = u_get_DBGWCR_Type_PAC(state, tracer, s_0_42);
        // D s_0_44: cast zx s_0_34 -> i
        let s_0_44: i128 = (i128::try_from(s_0_34).unwrap());
        // D s_0_45: read-var accdesc:struct
        let s_0_45: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_46: call AArch32_StateMatch(s_0_37, s_0_40, s_0_43, s_0_24, s_0_44, s_0_25, s_0_45)
        let s_0_46: bool = AArch32_StateMatch(
            state,
            tracer,
            s_0_37,
            s_0_40,
            s_0_43,
            s_0_24,
            s_0_44,
            s_0_25,
            s_0_45,
        );
        // D s_0_47: write-var state_match <= s_0_46
        fn_state.state_match = s_0_46;
        // D s_0_48: read-var n:i64
        let s_0_48: i64 = fn_state.n;
        // D s_0_49: call DBGWCR_read(s_0_48)
        let s_0_49: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_0_48);
        // D s_0_50: call _get_DBGWCR_Type_LSC(s_0_49)
        let s_0_50: u8 = u_get_DBGWCR_Type_LSC(state, tracer, s_0_49);
        // C s_0_51: const #0s : i
        let s_0_51: i128 = 0;
        // D s_0_52: cast zx s_0_50 -> bv
        let s_0_52: Bits = Bits::new(s_0_50 as u128, 2u16);
        // C s_0_53: const #1s : i64
        let s_0_53: i64 = 1;
        // C s_0_54: cast zx s_0_53 -> i
        let s_0_54: i128 = (i128::try_from(s_0_53).unwrap());
        // C s_0_55: const #1s : i
        let s_0_55: i128 = 1;
        // C s_0_56: add s_0_55 s_0_54
        let s_0_56: i128 = (s_0_55 + s_0_54);
        // D s_0_57: bit-extract s_0_52 s_0_51 s_0_56
        let s_0_57: Bits = (Bits::new(
            ((s_0_52) >> (s_0_51)).value(),
            u16::try_from(s_0_56).unwrap(),
        ));
        // D s_0_58: cast reint s_0_57 -> u8
        let s_0_58: u8 = (s_0_57.value() as u8);
        // D s_0_59: write-var ga#23378 <= s_0_58
        fn_state.ga_23378 = s_0_58;
        // D s_0_60: read-var ga#23378:u8
        let s_0_60: u8 = fn_state.ga_23378;
        // D s_0_61: cast zx s_0_60 -> bv
        let s_0_61: Bits = Bits::new(s_0_60 as u128, 2u16);
        // C s_0_62: const #0u : u8
        let s_0_62: u8 = 0;
        // C s_0_63: cast zx s_0_62 -> bv
        let s_0_63: Bits = Bits::new(s_0_62 as u128, 2u16);
        // D s_0_64: cmp-eq s_0_61 s_0_63
        let s_0_64: bool = ((s_0_61) == (s_0_63));
        // D s_0_65: not s_0_64
        let s_0_65: bool = !s_0_64;
        // N s_0_66: branch s_0_65 b18 b1
        if s_0_65 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var ls_match <= s_1_0
        fn_state.ls_match = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var value_match_name <= s_2_0
        fn_state.value_match_name = s_2_0;
        // C s_2_2: const #0s : i64
        let s_2_2: i64 = 0;
        // C s_2_3: const #1s : i
        let s_2_3: i128 = 1;
        // D s_2_4: read-var size:i
        let s_2_4: i128 = fn_state.size;
        // D s_2_5: sub s_2_4 s_2_3
        let s_2_5: i128 = ((s_2_4) - (s_2_3));
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: write-var gs#30198 <= s_2_6
        fn_state.gs_30198 = s_2_6;
        // D s_2_8: write-var byte <= s_2_2
        fn_state.byte = s_2_2;
        // N s_2_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var byte:i64
        let s_3_0: i64 = fn_state.byte;
        // D s_3_1: read-var gs#30198:i64
        let s_3_1: i64 = fn_state.gs_30198;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b8 b4
        if s_3_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var value_match_name:u8
        let s_4_0: bool = fn_state.value_match_name;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var vaddress:u32
        let s_5_0: u32 = fn_state.vaddress;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 32u16);
        // D s_5_2: read-var byte:i64
        let s_5_2: i64 = fn_state.byte;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cast cvt s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 128);
        // D s_5_5: add s_5_1 s_5_4
        let s_5_5: Bits = (s_5_1 + s_5_4);
        // D s_5_6: cast reint s_5_5 -> u32
        let s_5_6: u32 = (s_5_5.value() as u32);
        // D s_5_7: read-var n:i64
        let s_5_7: i64 = fn_state.n;
        // D s_5_8: call AArch32_WatchpointByteMatch(s_5_7, s_5_6)
        let s_5_8: bool = AArch32_WatchpointByteMatch(state, tracer, s_5_7, s_5_6);
        // D s_5_9: write-var gs#30201 <= s_5_8
        fn_state.gs_30201 = s_5_8;
        // N s_5_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#30201:u8
        let s_6_0: bool = fn_state.gs_30201;
        // D s_6_1: write-var value_match_name <= s_6_0
        fn_state.value_match_name = s_6_0;
        // D s_6_2: read-var byte:i64
        let s_6_2: i64 = fn_state.byte;
        // C s_6_3: const #1s : i64
        let s_6_3: i64 = 1;
        // D s_6_4: add s_6_2 s_6_3
        let s_6_4: i64 = (s_6_2 + s_6_3);
        // D s_6_5: write-var byte <= s_6_4
        fn_state.byte = s_6_4;
        // N s_6_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#30201 <= s_7_0
        fn_state.gs_30201 = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var value_match_name:u8
        let s_8_0: bool = fn_state.value_match_name;
        // N s_8_1: branch s_8_0 b17 b9
        if s_8_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#30204 <= s_9_0
        fn_state.gs_30204 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#30204:u8
        let s_10_0: bool = fn_state.gs_30204;
        // N s_10_1: branch s_10_0 b16 b11
        if s_10_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#30205 <= s_11_0
        fn_state.gs_30205 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#30205:u8
        let s_12_0: bool = fn_state.gs_30205;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#30206 <= s_13_0
        fn_state.gs_30206 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#30206:u8
        let s_14_0: bool = fn_state.gs_30206;
        // N s_14_1: return s_14_0
        return s_14_0;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var enabled:u8
        let s_15_0: bool = fn_state.enabled;
        // D s_15_1: write-var gs#30206 <= s_15_0
        fn_state.gs_30206 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var ls_match:u8
        let s_16_0: bool = fn_state.ls_match;
        // D s_16_1: write-var gs#30205 <= s_16_0
        fn_state.gs_30205 = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var state_match:u8
        let s_17_0: bool = fn_state.state_match;
        // D s_17_1: write-var gs#30204 <= s_17_0
        fn_state.gs_30204 = s_17_0;
        // N s_17_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var ga#23378:u8
        let s_18_0: u8 = fn_state.ga_23378;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #1u : u8
        let s_18_2: u8 = 1;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var accdesc.23:struct
        let s_19_0: bool = fn_state.accdesc._23;
        // D s_19_1: write-var ls_match <= s_19_0
        fn_state.ls_match = s_19_0;
        // N s_19_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var ga#23378:u8
        let s_20_0: u8 = fn_state.ga_23378;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 2u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b25 b21
        if s_20_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var accdesc.32:struct
        let s_21_0: bool = fn_state.accdesc._32;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var accdesc.1:struct
        let s_22_0: u32 = fn_state.accdesc._1;
        // C s_22_1: const #6u : u32
        let s_22_1: u32 = 6;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: write-var gs#30190 <= s_22_2
        fn_state.gs_30190 = s_22_2;
        // N s_22_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#30190:u8
        let s_23_0: bool = fn_state.gs_30190;
        // D s_23_1: write-var ls_match <= s_23_0
        fn_state.ls_match = s_23_0;
        // N s_23_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#30190 <= s_24_0
        fn_state.gs_30190 = s_24_0;
        // N s_24_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var ga#23378:u8
        let s_25_0: u8 = fn_state.ga_23378;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 2u16);
        // C s_25_2: const #3u : u8
        let s_25_2: u8 = 3;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 2u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b27 b26
        if s_25_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var ls_match <= s_26_0
        fn_state.ls_match = s_26_0;
        // N s_26_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_27_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
