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
use AArch64_StateMatch::*;
use u_get_DBGBCR_EL1_Type_BT2::*;
use u_get_DBGBCR_EL1_Type_SSC::*;
use HaveRME::*;
use ConstrainUnpredictableBool::*;
use u_get_DBGBCR_EL1_Type_HMC::*;
use S1TranslationRegime__1::*;
use HaveFeatABLE::*;
use Havev8p9Debug::*;
use IsBreakpointEnabled::*;
use u_get_DBGBCR_EL1_Type_BAS::*;
use NumBreakpointsImplemented::*;
use u_get_DBGBCR_EL1_Type_SSCE::*;
use ELUsingAArch32::*;
use HaveAArch32::*;
use u_get_DBGBCR_EL1_Type_LBNX::*;
use u_get_DBGBCR_EL1_Type_BT::*;
use AArch64_BreakpointValueMatch::*;
use u_get_DBGBCR_EL1_Type_PMC::*;
use u_get_DBGBCR_EL1_Type_LBN::*;
use common::*;
pub fn AArch64_BreakpointMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    vaddress: u64,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        gs_16523: bool,
        ga_12328: u8,
        gs_16506: bool,
        state_match: bool,
        match_i: bool,
        valid_mismatch_name: bool,
        b__0: u8,
        value_match_name: bool,
        gs_16507: bool,
        b__1: u8,
        gs_16533: bool,
        gs_16515: bool,
        gs_16503: bool,
        gs_16526: bool,
        gs_16537: bool,
        gs_16497: bool,
        linked_n: i64,
        val_match: bool,
        return_value: ProductType8b847afc727d5818,
        b__2: u8,
        enabled: bool,
        ga_12297: ProductType8b847afc727d5818,
        gs_16540: bool,
        lbnx: u8,
        gs_16538: bool,
        gs_16510: bool,
        gs_16492: bool,
        ga_12301: ProductType8b847afc727d5818,
        linked: bool,
        ssce: bool,
        gs_16539: bool,
        n: i64,
        vaddress: u64,
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
        // S s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call NumBreakpointsImplemented(s_0_5)
        let s_0_6: i128 = NumBreakpointsImplemented(state, tracer, s_0_5);
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: cmp-lt s_0_8 s_0_6
        let s_0_9: bool = ((s_0_8) < (s_0_6));
        // N s_0_10: assert s_0_9
        let s_0_10: () = assert!(s_0_9);
        // C s_0_11: const #12184u : u32
        let s_0_11: u32 = 12184;
        // D s_0_12: read-reg s_0_11:[struct; 64]
        let s_0_12: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: read-var n:i64
        let s_0_13: i64 = fn_state.n;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: read-element s_0_12[s_0_14]
        let s_0_15: ProductType5c790c8ef59cc8b2 = s_0_12[(s_0_14) as usize];
        // D s_0_16: call _get_DBGBCR_EL1_Type_BT(s_0_15)
        let s_0_16: u8 = u_get_DBGBCR_EL1_Type_BT(state, tracer, s_0_15);
        // D s_0_17: write-var ga#12328 <= s_0_16
        fn_state.ga_12328 = s_0_16;
        // D s_0_18: read-var ga#12328:u8
        let s_0_18: u8 = fn_state.ga_12328;
        // D s_0_19: write-var b__1 <= s_0_18
        fn_state.b__1 = s_0_18;
        // C s_0_20: const #3s : i
        let s_0_20: i128 = 3;
        // D s_0_21: read-var b__1:u8
        let s_0_21: u8 = fn_state.b__1;
        // D s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 4u16);
        // C s_0_23: const #1s : i64
        let s_0_23: i64 = 1;
        // C s_0_24: cast zx s_0_23 -> i
        let s_0_24: i128 = (i128::try_from(s_0_23).unwrap());
        // C s_0_25: const #0s : i
        let s_0_25: i128 = 0;
        // C s_0_26: add s_0_25 s_0_24
        let s_0_26: i128 = (s_0_25 + s_0_24);
        // D s_0_27: bit-extract s_0_22 s_0_20 s_0_26
        let s_0_27: Bits = (Bits::new(
            ((s_0_22) >> (s_0_20)).value(),
            u16::try_from(s_0_26).unwrap(),
        ));
        // D s_0_28: cast reint s_0_27 -> u8
        let s_0_28: bool = ((s_0_27.value()) != 0);
        // D s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 1u16);
        // C s_0_30: const #0u : u8
        let s_0_30: bool = false;
        // C s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 1u16);
        // D s_0_32: cmp-eq s_0_29 s_0_31
        let s_0_32: bool = ((s_0_29) == (s_0_31));
        // N s_0_33: branch s_0_32 b65 b1
        if s_0_32 {
            return block_65(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#16497 <= s_1_0
        fn_state.gs_16497 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_2_0: read-var gs#16497:u8
        let s_2_0: bool = fn_state.gs_16497;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b59 b3
        if s_2_1 {
            return block_59(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#16492 <= s_3_0
        fn_state.gs_16492 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_4_0: read-var gs#16492:u8
        let s_4_0: bool = fn_state.gs_16492;
        // N s_4_1: branch s_4_0 b58 b5
        if s_4_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveFeatABLE(s_5_0)
        let s_5_1: bool = HaveFeatABLE(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b57 b6
        if s_5_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#16506 <= s_6_0
        fn_state.gs_16506 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_7_0: read-var gs#16506:u8
        let s_7_0: bool = fn_state.gs_16506;
        // D s_7_1: write-var gs#16507 <= s_7_0
        fn_state.gs_16507 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_8_0: read-var gs#16507:u8
        let s_8_0: bool = fn_state.gs_16507;
        // N s_8_1: branch s_8_0 b56 b9
        if s_8_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // D s_9_1: call IsBreakpointEnabled(s_9_0)
        let s_9_1: bool = IsBreakpointEnabled(state, tracer, s_9_0);
        // D s_9_2: write-var enabled <= s_9_1
        fn_state.enabled = s_9_1;
        // C s_9_3: const #12184u : u32
        let s_9_3: u32 = 12184;
        // D s_9_4: read-reg s_9_3:[struct; 64]
        let s_9_4: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: read-var n:i64
        let s_9_5: i64 = fn_state.n;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: read-element s_9_4[s_9_6]
        let s_9_7: ProductType5c790c8ef59cc8b2 = s_9_4[(s_9_6) as usize];
        // D s_9_8: call _get_DBGBCR_EL1_Type_BT(s_9_7)
        let s_9_8: u8 = u_get_DBGBCR_EL1_Type_BT(state, tracer, s_9_7);
        // D s_9_9: write-var b__0 <= s_9_8
        fn_state.b__0 = s_9_8;
        // C s_9_10: const #3s : i
        let s_9_10: i128 = 3;
        // D s_9_11: read-var b__0:u8
        let s_9_11: u8 = fn_state.b__0;
        // D s_9_12: cast zx s_9_11 -> bv
        let s_9_12: Bits = Bits::new(s_9_11 as u128, 4u16);
        // C s_9_13: const #1s : i64
        let s_9_13: i64 = 1;
        // C s_9_14: cast zx s_9_13 -> i
        let s_9_14: i128 = (i128::try_from(s_9_13).unwrap());
        // C s_9_15: const #0s : i
        let s_9_15: i128 = 0;
        // C s_9_16: add s_9_15 s_9_14
        let s_9_16: i128 = (s_9_15 + s_9_14);
        // D s_9_17: bit-extract s_9_12 s_9_10 s_9_16
        let s_9_17: Bits = (Bits::new(
            ((s_9_12) >> (s_9_10)).value(),
            u16::try_from(s_9_16).unwrap(),
        ));
        // D s_9_18: cast reint s_9_17 -> u8
        let s_9_18: bool = ((s_9_17.value()) != 0);
        // D s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 1u16);
        // C s_9_20: const #0u : u8
        let s_9_20: bool = false;
        // C s_9_21: cast zx s_9_20 -> bv
        let s_9_21: Bits = Bits::new(s_9_20 as u128, 1u16);
        // D s_9_22: cmp-eq s_9_19 s_9_21
        let s_9_22: bool = ((s_9_19) == (s_9_21));
        // N s_9_23: branch s_9_22 b55 b10
        if s_9_22 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#16515 <= s_10_0
        fn_state.gs_16515 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_11_0: read-var gs#16515:u8
        let s_11_0: bool = fn_state.gs_16515;
        // D s_11_1: not s_11_0
        let s_11_1: bool = !s_11_0;
        // N s_11_2: branch s_11_1 b54 b12
        if s_11_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#16510 <= s_12_0
        fn_state.gs_16510 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_13_0: read-var gs#16510:u8
        let s_13_0: bool = fn_state.gs_16510;
        // D s_13_1: write-var linked <= s_13_0
        fn_state.linked = s_13_0;
        // C s_13_2: const #() : ()
        let s_13_2: () = ();
        // S s_13_3: call Havev8p9Debug(s_13_2)
        let s_13_3: bool = Havev8p9Debug(state, tracer, s_13_2);
        // N s_13_4: branch s_13_3 b53 b14
        if s_13_3 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_14_0: const #0u : u8
        let s_14_0: u8 = 0;
        // D s_14_1: write-var lbnx <= s_14_0
        fn_state.lbnx = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_15_0: const #12184u : u32
        let s_15_0: u32 = 12184;
        // D s_15_1: read-reg s_15_0:[struct; 64]
        let s_15_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: read-var n:i64
        let s_15_2: i64 = fn_state.n;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: read-element s_15_1[s_15_3]
        let s_15_4: ProductType5c790c8ef59cc8b2 = s_15_1[(s_15_3) as usize];
        // D s_15_5: call _get_DBGBCR_EL1_Type_LBN(s_15_4)
        let s_15_5: u8 = u_get_DBGBCR_EL1_Type_LBN(state, tracer, s_15_4);
        // D s_15_6: read-var lbnx:u8
        let s_15_6: u8 = fn_state.lbnx;
        // D s_15_7: cast zx s_15_6 -> bv
        let s_15_7: Bits = Bits::new(s_15_6 as u128, 2u16);
        // D s_15_8: cast zx s_15_5 -> bv
        let s_15_8: Bits = Bits::new(s_15_5 as u128, 4u16);
        // D s_15_9: cast reint s_15_7 -> u128
        let s_15_9: u128 = (s_15_7.value() as u128);
        // D s_15_10: size-of s_15_7
        let s_15_10: u16 = s_15_7.length();
        // D s_15_11: cast reint s_15_8 -> u128
        let s_15_11: u128 = (s_15_8.value() as u128);
        // D s_15_12: size-of s_15_8
        let s_15_12: u16 = s_15_8.length();
        // D s_15_13: lsl s_15_9 s_15_12
        let s_15_13: u128 = s_15_9 << s_15_12;
        // D s_15_14: or s_15_13 s_15_11
        let s_15_14: u128 = ((s_15_13) | (s_15_11));
        // D s_15_15: add s_15_10 s_15_12
        let s_15_15: u16 = (s_15_10 + s_15_12);
        // D s_15_16: create-bits s_15_14 s_15_15
        let s_15_16: Bits = Bits::new(s_15_14, s_15_15);
        // D s_15_17: cast reint s_15_16 -> u8
        let s_15_17: u8 = (s_15_16.value() as u8);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 6u16);
        // D s_15_19: cast zx s_15_18 -> i
        let s_15_19: i128 = (s_15_18.value() as i128);
        // D s_15_20: cast reint s_15_19 -> i64
        let s_15_20: i64 = (s_15_19 as i64);
        // D s_15_21: write-var linked_n <= s_15_20
        fn_state.linked_n = s_15_20;
        // C s_15_22: const #() : ()
        let s_15_22: () = ();
        // S s_15_23: call HaveRME(s_15_22)
        let s_15_23: bool = HaveRME(state, tracer, s_15_22);
        // N s_15_24: branch s_15_23 b52 b16
        if s_15_23 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var ssce <= s_16_0
        fn_state.ssce = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_17_0: const #12184u : u32
        let s_17_0: u32 = 12184;
        // D s_17_1: read-reg s_17_0:[struct; 64]
        let s_17_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: read-var n:i64
        let s_17_2: i64 = fn_state.n;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: read-element s_17_1[s_17_3]
        let s_17_4: ProductType5c790c8ef59cc8b2 = s_17_1[(s_17_3) as usize];
        // D s_17_5: call _get_DBGBCR_EL1_Type_SSC(s_17_4)
        let s_17_5: u8 = u_get_DBGBCR_EL1_Type_SSC(state, tracer, s_17_4);
        // C s_17_6: const #12184u : u32
        let s_17_6: u32 = 12184;
        // D s_17_7: read-reg s_17_6:[struct; 64]
        let s_17_7: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_17_6 as isize);
            tracer.read_register(s_17_6 as isize, value);
            value
        };
        // D s_17_8: read-var n:i64
        let s_17_8: i64 = fn_state.n;
        // D s_17_9: cast zx s_17_8 -> i
        let s_17_9: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_10: read-element s_17_7[s_17_9]
        let s_17_10: ProductType5c790c8ef59cc8b2 = s_17_7[(s_17_9) as usize];
        // D s_17_11: call _get_DBGBCR_EL1_Type_HMC(s_17_10)
        let s_17_11: bool = u_get_DBGBCR_EL1_Type_HMC(state, tracer, s_17_10);
        // C s_17_12: const #12184u : u32
        let s_17_12: u32 = 12184;
        // D s_17_13: read-reg s_17_12:[struct; 64]
        let s_17_13: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_17_12 as isize);
            tracer.read_register(s_17_12 as isize, value);
            value
        };
        // D s_17_14: read-var n:i64
        let s_17_14: i64 = fn_state.n;
        // D s_17_15: cast zx s_17_14 -> i
        let s_17_15: i128 = (i128::try_from(s_17_14).unwrap());
        // D s_17_16: read-element s_17_13[s_17_15]
        let s_17_16: ProductType5c790c8ef59cc8b2 = s_17_13[(s_17_15) as usize];
        // D s_17_17: call _get_DBGBCR_EL1_Type_PMC(s_17_16)
        let s_17_17: u8 = u_get_DBGBCR_EL1_Type_PMC(state, tracer, s_17_16);
        // D s_17_18: read-var linked_n:i64
        let s_17_18: i64 = fn_state.linked_n;
        // D s_17_19: cast zx s_17_18 -> i
        let s_17_19: i128 = (i128::try_from(s_17_18).unwrap());
        // D s_17_20: read-var ssce:u8
        let s_17_20: bool = fn_state.ssce;
        // D s_17_21: read-var linked:u8
        let s_17_21: bool = fn_state.linked;
        // C s_17_22: const #1u : u8
        let s_17_22: bool = true;
        // D s_17_23: read-var vaddress:u64
        let s_17_23: u64 = fn_state.vaddress;
        // D s_17_24: read-var accdesc:struct
        let s_17_24: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_25: call AArch64_StateMatch(s_17_5, s_17_20, s_17_11, s_17_17, s_17_21, s_17_19, s_17_22, s_17_23, s_17_24)
        let s_17_25: bool = AArch64_StateMatch(
            state,
            tracer,
            s_17_5,
            s_17_20,
            s_17_11,
            s_17_17,
            s_17_21,
            s_17_19,
            s_17_22,
            s_17_23,
            s_17_24,
        );
        // D s_17_26: write-var state_match <= s_17_25
        fn_state.state_match = s_17_25;
        // D s_17_27: read-var n:i64
        let s_17_27: i64 = fn_state.n;
        // D s_17_28: cast zx s_17_27 -> i
        let s_17_28: i128 = (i128::try_from(s_17_27).unwrap());
        // D s_17_29: read-var vaddress:u64
        let s_17_29: u64 = fn_state.vaddress;
        // C s_17_30: const #0u : u8
        let s_17_30: bool = false;
        // C s_17_31: const #1u : u8
        let s_17_31: bool = true;
        // D s_17_32: call AArch64_BreakpointValueMatch(s_17_28, s_17_29, s_17_30, s_17_31)
        let s_17_32: ProductType8b847afc727d5818 = AArch64_BreakpointValueMatch(
            state,
            tracer,
            s_17_28,
            s_17_29,
            s_17_30,
            s_17_31,
        );
        // D s_17_33: write-var ga#12297 <= s_17_32
        fn_state.ga_12297 = s_17_32;
        // D s_17_34: read-var ga#12297.0:struct
        let s_17_34: bool = fn_state.ga_12297._0;
        // D s_17_35: read-var ga#12297.1:struct
        let s_17_35: bool = fn_state.ga_12297._1;
        // D s_17_36: write-var value_match_name <= s_17_34
        fn_state.value_match_name = s_17_34;
        // D s_17_37: write-var valid_mismatch_name <= s_17_35
        fn_state.valid_mismatch_name = s_17_35;
        // C s_17_38: const #() : ()
        let s_17_38: () = ();
        // S s_17_39: call HaveAArch32(s_17_38)
        let s_17_39: bool = HaveAArch32(state, tracer, s_17_38);
        // N s_17_40: branch s_17_39 b51 b18
        if s_17_39 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#16523 <= s_18_0
        fn_state.gs_16523 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_19_0: read-var gs#16523:u8
        let s_19_0: bool = fn_state.gs_16523;
        // N s_19_1: branch s_19_0 b44 b20
        if s_19_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_21_0: const #1s : i
        let s_21_0: i128 = 1;
        // D s_21_1: read-var vaddress:u64
        let s_21_1: u64 = fn_state.vaddress;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 64u16);
        // C s_21_3: const #1u : u64
        let s_21_3: u64 = 1;
        // D s_21_4: bit-extract s_21_2 s_21_0 s_21_3
        let s_21_4: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_3).unwrap(),
        ));
        // D s_21_5: cast reint s_21_4 -> u8
        let s_21_5: bool = ((s_21_4.value()) != 0);
        // C s_21_6: const #0s : i
        let s_21_6: i128 = 0;
        // C s_21_7: const #0u : u64
        let s_21_7: u64 = 0;
        // D s_21_8: cast zx s_21_5 -> u64
        let s_21_8: u64 = (s_21_5 as u64);
        // C s_21_9: const #1u : u64
        let s_21_9: u64 = 1;
        // D s_21_10: and s_21_8 s_21_9
        let s_21_10: u64 = ((s_21_8) & (s_21_9));
        // D s_21_11: cmp-eq s_21_10 s_21_9
        let s_21_11: bool = ((s_21_10) == (s_21_9));
        // D s_21_12: lsl s_21_8 s_21_6
        let s_21_12: u64 = s_21_8 << s_21_6;
        // D s_21_13: or s_21_7 s_21_12
        let s_21_13: u64 = ((s_21_7) | (s_21_12));
        // D s_21_14: cmpl s_21_12
        let s_21_14: u64 = !s_21_12;
        // D s_21_15: and s_21_7 s_21_14
        let s_21_15: u64 = ((s_21_7) & (s_21_14));
        // D s_21_16: select s_21_11 s_21_13 s_21_15
        let s_21_16: u64 = if s_21_11 { s_21_13 } else { s_21_15 };
        // D s_21_17: cast trunc s_21_16 -> u8
        let s_21_17: bool = ((s_21_16) != 0);
        // D s_21_18: cast zx s_21_17 -> bv
        let s_21_18: Bits = Bits::new(s_21_17 as u128, 1u16);
        // C s_21_19: const #1u : u8
        let s_21_19: bool = true;
        // C s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // D s_21_21: cmp-eq s_21_18 s_21_20
        let s_21_21: bool = ((s_21_18) == (s_21_20));
        // N s_21_22: branch s_21_21 b43 b22
        if s_21_21 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#16526 <= s_22_0
        fn_state.gs_16526 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_23_0: read-var gs#16526:u8
        let s_23_0: bool = fn_state.gs_16526;
        // N s_23_1: branch s_23_0 b39 b24
        if s_23_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_25_0: read-var value_match_name:u8
        let s_25_0: bool = fn_state.value_match_name;
        // N s_25_1: branch s_25_0 b38 b26
        if s_25_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#16537 <= s_26_0
        fn_state.gs_16537 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_27_0: read-var gs#16537:u8
        let s_27_0: bool = fn_state.gs_16537;
        // N s_27_1: branch s_27_0 b37 b28
        if s_27_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#16538 <= s_28_0
        fn_state.gs_16538 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_29_0: read-var gs#16538:u8
        let s_29_0: bool = fn_state.gs_16538;
        // D s_29_1: write-var val_match <= s_29_0
        fn_state.val_match = s_29_0;
        // D s_29_2: read-var valid_mismatch_name:u8
        let s_29_2: bool = fn_state.valid_mismatch_name;
        // N s_29_3: branch s_29_2 b36 b30
        if s_29_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#16539 <= s_30_0
        fn_state.gs_16539 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_31_0: read-var gs#16539:u8
        let s_31_0: bool = fn_state.gs_16539;
        // N s_31_1: branch s_31_0 b35 b32
        if s_31_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#16540 <= s_32_0
        fn_state.gs_16540 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_33_0: read-var gs#16540:u8
        let s_33_0: bool = fn_state.gs_16540;
        // D s_33_1: read-var val_match:u8
        let s_33_1: bool = fn_state.val_match;
        // D s_33_2: create-product struct = ["s_33_1", "s_33_0"]
        let s_33_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_33_1,
            _1: s_33_0,
        };
        // D s_33_3: write-var return_value <= s_33_2
        fn_state.return_value = s_33_2;
        // N s_33_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_34_0: read-var return_value:struct
        let s_34_0: ProductType8b847afc727d5818 = fn_state.return_value;
        // N s_34_1: return s_34_0
        return s_34_0;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_35_0: read-var enabled:u8
        let s_35_0: bool = fn_state.enabled;
        // D s_35_1: write-var gs#16540 <= s_35_0
        fn_state.gs_16540 = s_35_0;
        // N s_35_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_36_0: read-var state_match:u8
        let s_36_0: bool = fn_state.state_match;
        // D s_36_1: write-var gs#16539 <= s_36_0
        fn_state.gs_16539 = s_36_0;
        // N s_36_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_37_0: read-var enabled:u8
        let s_37_0: bool = fn_state.enabled;
        // D s_37_1: write-var gs#16538 <= s_37_0
        fn_state.gs_16538 = s_37_0;
        // N s_37_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_38_0: read-var state_match:u8
        let s_38_0: bool = fn_state.state_match;
        // D s_38_1: write-var gs#16537 <= s_38_0
        fn_state.gs_16537 = s_38_0;
        // N s_38_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_39_0: read-var value_match_name:u8
        let s_39_0: bool = fn_state.value_match_name;
        // N s_39_1: branch s_39_0 b42 b40
        if s_39_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_40_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_41_0: jump b25
        return block_25(state, tracer, fn_state);
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
        // N s_42_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_43_0: const #12184u : u32
        let s_43_0: u32 = 12184;
        // D s_43_1: read-reg s_43_0:[struct; 64]
        let s_43_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: read-var n:i64
        let s_43_2: i64 = fn_state.n;
        // D s_43_3: cast zx s_43_2 -> i
        let s_43_3: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_4: read-element s_43_1[s_43_3]
        let s_43_4: ProductType5c790c8ef59cc8b2 = s_43_1[(s_43_3) as usize];
        // D s_43_5: call _get_DBGBCR_EL1_Type_BAS(s_43_4)
        let s_43_5: u8 = u_get_DBGBCR_EL1_Type_BAS(state, tracer, s_43_4);
        // D s_43_6: cast zx s_43_5 -> bv
        let s_43_6: Bits = Bits::new(s_43_5 as u128, 4u16);
        // C s_43_7: const #15u : u8
        let s_43_7: u8 = 15;
        // C s_43_8: cast zx s_43_7 -> bv
        let s_43_8: Bits = Bits::new(s_43_7 as u128, 4u16);
        // D s_43_9: cmp-eq s_43_6 s_43_8
        let s_43_9: bool = ((s_43_6) == (s_43_8));
        // D s_43_10: write-var gs#16526 <= s_43_9
        fn_state.gs_16526 = s_43_9;
        // N s_43_11: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_44_0: const #2s : i
        let s_44_0: i128 = 2;
        // D s_44_1: read-var vaddress:u64
        let s_44_1: u64 = fn_state.vaddress;
        // D s_44_2: cast zx s_44_1 -> bv
        let s_44_2: Bits = Bits::new(s_44_1 as u128, 64u16);
        // C s_44_3: cast cvt s_44_0 -> bv
        let s_44_3: Bits = Bits::new(s_44_0 as u128, 128);
        // D s_44_4: add s_44_2 s_44_3
        let s_44_4: Bits = (s_44_2 + s_44_3);
        // D s_44_5: cast reint s_44_4 -> u64
        let s_44_5: u64 = (s_44_4.value() as u64);
        // D s_44_6: read-var n:i64
        let s_44_6: i64 = fn_state.n;
        // D s_44_7: cast zx s_44_6 -> i
        let s_44_7: i128 = (i128::try_from(s_44_6).unwrap());
        // C s_44_8: const #0u : u8
        let s_44_8: bool = false;
        // C s_44_9: const #1u : u8
        let s_44_9: bool = true;
        // D s_44_10: call AArch64_BreakpointValueMatch(s_44_7, s_44_5, s_44_8, s_44_9)
        let s_44_10: ProductType8b847afc727d5818 = AArch64_BreakpointValueMatch(
            state,
            tracer,
            s_44_7,
            s_44_5,
            s_44_8,
            s_44_9,
        );
        // D s_44_11: write-var ga#12301 <= s_44_10
        fn_state.ga_12301 = s_44_10;
        // D s_44_12: read-var ga#12301.0:struct
        let s_44_12: bool = fn_state.ga_12301._0;
        // D s_44_13: write-var match_i <= s_44_12
        fn_state.match_i = s_44_12;
        // D s_44_14: read-var value_match_name:u8
        let s_44_14: bool = fn_state.value_match_name;
        // D s_44_15: not s_44_14
        let s_44_15: bool = !s_44_14;
        // N s_44_16: branch s_44_15 b50 b45
        if s_44_15 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#16533 <= s_45_0
        fn_state.gs_16533 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_46_0: read-var gs#16533:u8
        let s_46_0: bool = fn_state.gs_16533;
        // N s_46_1: branch s_46_0 b49 b47
        if s_46_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_48_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_49_0: const #36u : u32
        let s_49_0: u32 = 36;
        // S s_49_1: call ConstrainUnpredictableBool(s_49_0)
        let s_49_1: bool = ConstrainUnpredictableBool(state, tracer, s_49_0);
        // D s_49_2: write-var value_match_name <= s_49_1
        fn_state.value_match_name = s_49_1;
        // N s_49_3: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_50_0: read-var match_i:u8
        let s_50_0: bool = fn_state.match_i;
        // D s_50_1: write-var gs#16533 <= s_50_0
        fn_state.gs_16533 = s_50_0;
        // N s_50_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_51_0: const #4s : i
        let s_51_0: i128 = 4;
        // D s_51_1: read-var size:i
        let s_51_1: i128 = fn_state.size;
        // D s_51_2: cmp-eq s_51_1 s_51_0
        let s_51_2: bool = ((s_51_1) == (s_51_0));
        // D s_51_3: write-var gs#16523 <= s_51_2
        fn_state.gs_16523 = s_51_2;
        // N s_51_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_52_0: const #12184u : u32
        let s_52_0: u32 = 12184;
        // D s_52_1: read-reg s_52_0:[struct; 64]
        let s_52_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: read-var n:i64
        let s_52_2: i64 = fn_state.n;
        // D s_52_3: cast zx s_52_2 -> i
        let s_52_3: i128 = (i128::try_from(s_52_2).unwrap());
        // D s_52_4: read-element s_52_1[s_52_3]
        let s_52_4: ProductType5c790c8ef59cc8b2 = s_52_1[(s_52_3) as usize];
        // D s_52_5: call _get_DBGBCR_EL1_Type_SSCE(s_52_4)
        let s_52_5: bool = u_get_DBGBCR_EL1_Type_SSCE(state, tracer, s_52_4);
        // D s_52_6: write-var ssce <= s_52_5
        fn_state.ssce = s_52_5;
        // N s_52_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_53_0: const #12184u : u32
        let s_53_0: u32 = 12184;
        // D s_53_1: read-reg s_53_0:[struct; 64]
        let s_53_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: read-var n:i64
        let s_53_2: i64 = fn_state.n;
        // D s_53_3: cast zx s_53_2 -> i
        let s_53_3: i128 = (i128::try_from(s_53_2).unwrap());
        // D s_53_4: read-element s_53_1[s_53_3]
        let s_53_4: ProductType5c790c8ef59cc8b2 = s_53_1[(s_53_3) as usize];
        // D s_53_5: call _get_DBGBCR_EL1_Type_LBNX(s_53_4)
        let s_53_5: u8 = u_get_DBGBCR_EL1_Type_LBNX(state, tracer, s_53_4);
        // D s_53_6: write-var lbnx <= s_53_5
        fn_state.lbnx = s_53_5;
        // N s_53_7: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#16510 <= s_54_0
        fn_state.gs_16510 = s_54_0;
        // N s_54_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_55_0: const #0s : i
        let s_55_0: i128 = 0;
        // D s_55_1: read-var b__0:u8
        let s_55_1: u8 = fn_state.b__0;
        // D s_55_2: cast zx s_55_1 -> bv
        let s_55_2: Bits = Bits::new(s_55_1 as u128, 4u16);
        // C s_55_3: const #1s : i64
        let s_55_3: i64 = 1;
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #1s : i
        let s_55_5: i128 = 1;
        // C s_55_6: add s_55_5 s_55_4
        let s_55_6: i128 = (s_55_5 + s_55_4);
        // D s_55_7: bit-extract s_55_2 s_55_0 s_55_6
        let s_55_7: Bits = (Bits::new(
            ((s_55_2) >> (s_55_0)).value(),
            u16::try_from(s_55_6).unwrap(),
        ));
        // D s_55_8: cast reint s_55_7 -> u8
        let s_55_8: u8 = (s_55_7.value() as u8);
        // D s_55_9: cast zx s_55_8 -> bv
        let s_55_9: Bits = Bits::new(s_55_8 as u128, 2u16);
        // C s_55_10: const #1u : u8
        let s_55_10: u8 = 1;
        // C s_55_11: cast zx s_55_10 -> bv
        let s_55_11: Bits = Bits::new(s_55_10 as u128, 2u16);
        // D s_55_12: cmp-eq s_55_9 s_55_11
        let s_55_12: bool = ((s_55_9) == (s_55_11));
        // D s_55_13: write-var gs#16515 <= s_55_12
        fn_state.gs_16515 = s_55_12;
        // N s_55_14: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // C s_56_1: const #0u : u8
        let s_56_1: bool = false;
        // D s_56_2: create-product struct = ["s_56_0", "s_56_1"]
        let s_56_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_56_0,
            _1: s_56_1,
        };
        // D s_56_3: write-var return_value <= s_56_2
        fn_state.return_value = s_56_2;
        // N s_56_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_57_0: const #12184u : u32
        let s_57_0: u32 = 12184;
        // D s_57_1: read-reg s_57_0:[struct; 64]
        let s_57_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: read-var n:i64
        let s_57_2: i64 = fn_state.n;
        // D s_57_3: cast zx s_57_2 -> i
        let s_57_3: i128 = (i128::try_from(s_57_2).unwrap());
        // D s_57_4: read-element s_57_1[s_57_3]
        let s_57_4: ProductType5c790c8ef59cc8b2 = s_57_1[(s_57_3) as usize];
        // D s_57_5: call _get_DBGBCR_EL1_Type_BT2(s_57_4)
        let s_57_5: bool = u_get_DBGBCR_EL1_Type_BT2(state, tracer, s_57_4);
        // D s_57_6: cast zx s_57_5 -> bv
        let s_57_6: Bits = Bits::new(s_57_5 as u128, 1u16);
        // C s_57_7: const #1u : u8
        let s_57_7: bool = true;
        // C s_57_8: cast zx s_57_7 -> bv
        let s_57_8: Bits = Bits::new(s_57_7 as u128, 1u16);
        // D s_57_9: cmp-eq s_57_6 s_57_8
        let s_57_9: bool = ((s_57_6) == (s_57_8));
        // D s_57_10: write-var gs#16506 <= s_57_9
        fn_state.gs_16506 = s_57_9;
        // N s_57_11: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#16507 <= s_58_0
        fn_state.gs_16507 = s_58_0;
        // N s_58_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_59_0: read-var ga#12328:u8
        let s_59_0: u8 = fn_state.ga_12328;
        // D s_59_1: write-var b__2 <= s_59_0
        fn_state.b__2 = s_59_0;
        // C s_59_2: const #3s : i
        let s_59_2: i128 = 3;
        // D s_59_3: read-var b__2:u8
        let s_59_3: u8 = fn_state.b__2;
        // D s_59_4: cast zx s_59_3 -> bv
        let s_59_4: Bits = Bits::new(s_59_3 as u128, 4u16);
        // C s_59_5: const #1s : i64
        let s_59_5: i64 = 1;
        // C s_59_6: cast zx s_59_5 -> i
        let s_59_6: i128 = (i128::try_from(s_59_5).unwrap());
        // C s_59_7: const #0s : i
        let s_59_7: i128 = 0;
        // C s_59_8: add s_59_7 s_59_6
        let s_59_8: i128 = (s_59_7 + s_59_6);
        // D s_59_9: bit-extract s_59_4 s_59_2 s_59_8
        let s_59_9: Bits = (Bits::new(
            ((s_59_4) >> (s_59_2)).value(),
            u16::try_from(s_59_8).unwrap(),
        ));
        // D s_59_10: cast reint s_59_9 -> u8
        let s_59_10: bool = ((s_59_9.value()) != 0);
        // D s_59_11: cast zx s_59_10 -> bv
        let s_59_11: Bits = Bits::new(s_59_10 as u128, 1u16);
        // C s_59_12: const #1u : u8
        let s_59_12: bool = true;
        // C s_59_13: cast zx s_59_12 -> bv
        let s_59_13: Bits = Bits::new(s_59_12 as u128, 1u16);
        // D s_59_14: cmp-eq s_59_11 s_59_13
        let s_59_14: bool = ((s_59_11) == (s_59_13));
        // N s_59_15: branch s_59_14 b64 b60
        if s_59_14 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#16503 <= s_60_0
        fn_state.gs_16503 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_61_0: read-var gs#16503:u8
        let s_61_0: bool = fn_state.gs_16503;
        // D s_61_1: not s_61_0
        let s_61_1: bool = !s_61_0;
        // N s_61_2: branch s_61_1 b63 b62
        if s_61_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#16492 <= s_62_0
        fn_state.gs_16492 = s_62_0;
        // N s_62_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#16492 <= s_63_0
        fn_state.gs_16492 = s_63_0;
        // N s_63_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_64_0: const #0s : i
        let s_64_0: i128 = 0;
        // D s_64_1: read-var b__2:u8
        let s_64_1: u8 = fn_state.b__2;
        // D s_64_2: cast zx s_64_1 -> bv
        let s_64_2: Bits = Bits::new(s_64_1 as u128, 4u16);
        // C s_64_3: const #1s : i64
        let s_64_3: i64 = 1;
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #0s : i
        let s_64_5: i128 = 0;
        // C s_64_6: add s_64_5 s_64_4
        let s_64_6: i128 = (s_64_5 + s_64_4);
        // D s_64_7: bit-extract s_64_2 s_64_0 s_64_6
        let s_64_7: Bits = (Bits::new(
            ((s_64_2) >> (s_64_0)).value(),
            u16::try_from(s_64_6).unwrap(),
        ));
        // D s_64_8: cast reint s_64_7 -> u8
        let s_64_8: bool = ((s_64_7.value()) != 0);
        // D s_64_9: cast zx s_64_8 -> bv
        let s_64_9: Bits = Bits::new(s_64_8 as u128, 1u16);
        // C s_64_10: const #1u : u8
        let s_64_10: bool = true;
        // C s_64_11: cast zx s_64_10 -> bv
        let s_64_11: Bits = Bits::new(s_64_10 as u128, 1u16);
        // D s_64_12: cmp-eq s_64_9 s_64_11
        let s_64_12: bool = ((s_64_9) == (s_64_11));
        // D s_64_13: write-var gs#16503 <= s_64_12
        fn_state.gs_16503 = s_64_12;
        // N s_64_14: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_65_0: const #0s : i
        let s_65_0: i128 = 0;
        // D s_65_1: read-var b__1:u8
        let s_65_1: u8 = fn_state.b__1;
        // D s_65_2: cast zx s_65_1 -> bv
        let s_65_2: Bits = Bits::new(s_65_1 as u128, 4u16);
        // C s_65_3: const #1s : i64
        let s_65_3: i64 = 1;
        // C s_65_4: cast zx s_65_3 -> i
        let s_65_4: i128 = (i128::try_from(s_65_3).unwrap());
        // C s_65_5: const #1s : i
        let s_65_5: i128 = 1;
        // C s_65_6: add s_65_5 s_65_4
        let s_65_6: i128 = (s_65_5 + s_65_4);
        // D s_65_7: bit-extract s_65_2 s_65_0 s_65_6
        let s_65_7: Bits = (Bits::new(
            ((s_65_2) >> (s_65_0)).value(),
            u16::try_from(s_65_6).unwrap(),
        ));
        // D s_65_8: cast reint s_65_7 -> u8
        let s_65_8: u8 = (s_65_7.value() as u8);
        // D s_65_9: cast zx s_65_8 -> bv
        let s_65_9: Bits = Bits::new(s_65_8 as u128, 2u16);
        // C s_65_10: const #3u : u8
        let s_65_10: u8 = 3;
        // C s_65_11: cast zx s_65_10 -> bv
        let s_65_11: Bits = Bits::new(s_65_10 as u128, 2u16);
        // D s_65_12: cmp-eq s_65_9 s_65_11
        let s_65_12: bool = ((s_65_9) == (s_65_11));
        // D s_65_13: write-var gs#16497 <= s_65_12
        fn_state.gs_16497 = s_65_12;
        // N s_65_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
