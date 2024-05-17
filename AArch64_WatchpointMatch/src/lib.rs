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
use NumWatchpointsImplemented::*;
use HaveRME::*;
use IsWatchpointEnabled::*;
use u_get_DBGWCR_EL1_Type_HMC::*;
use u_get_DBGWCR_EL1_Type_SSC::*;
use S1TranslationRegime__1::*;
use Havev8p9Debug::*;
use u_get_DBGWCR_EL1_Type_SSCE::*;
use u_get_DBGWCR_EL1_Type_WT::*;
use u_get_DBGWCR_EL1_Type_PAC::*;
use u_get_DBGWCR_EL1_Type_LBNX::*;
use AArch64_WatchpointByteMatch::*;
use PC_read::*;
use u_get_DBGWCR_EL1_Type_LSC::*;
use ELUsingAArch32::*;
use u_get_DBGWCR_EL1_Type_LBN::*;
use common::*;
pub fn AArch64_WatchpointMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    vaddress: u64,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        byte: i64,
        gs_16696: bool,
        linked_n: i64,
        state_match: bool,
        ga_12464: u8,
        enabled: bool,
        gs_16694: bool,
        gs_16688: i64,
        value_match_name: bool,
        gs_16695: bool,
        lbnx: u8,
        gs_16691: bool,
        gs_16680: bool,
        ls_match: bool,
        linked: bool,
        ssce: bool,
        n: i64,
        vaddress: u64,
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
        // S s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call NumWatchpointsImplemented(s_0_5)
        let s_0_6: i128 = NumWatchpointsImplemented(state, tracer, s_0_5);
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: cmp-lt s_0_8 s_0_6
        let s_0_9: bool = ((s_0_8) < (s_0_6));
        // N s_0_10: assert s_0_9
        let s_0_10: () = assert!(s_0_9);
        // D s_0_11: read-var n:i64
        let s_0_11: i64 = fn_state.n;
        // D s_0_12: call IsWatchpointEnabled(s_0_11)
        let s_0_12: bool = IsWatchpointEnabled(state, tracer, s_0_11);
        // D s_0_13: write-var enabled <= s_0_12
        fn_state.enabled = s_0_12;
        // C s_0_14: const #103984u : u32
        let s_0_14: u32 = 103984;
        // D s_0_15: read-reg s_0_14:[struct; 64]
        let s_0_15: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // D s_0_16: read-var n:i64
        let s_0_16: i64 = fn_state.n;
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_18: read-element s_0_15[s_0_17]
        let s_0_18: ProductType5c790c8ef59cc8b2 = s_0_15[(s_0_17) as usize];
        // D s_0_19: call _get_DBGWCR_EL1_Type_WT(s_0_18)
        let s_0_19: bool = u_get_DBGWCR_EL1_Type_WT(state, tracer, s_0_18);
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // C s_0_21: const #1u : u8
        let s_0_21: bool = true;
        // C s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 1u16);
        // D s_0_23: cmp-eq s_0_20 s_0_22
        let s_0_23: bool = ((s_0_20) == (s_0_22));
        // D s_0_24: write-var linked <= s_0_23
        fn_state.linked = s_0_23;
        // C s_0_25: const #() : ()
        let s_0_25: () = ();
        // S s_0_26: call Havev8p9Debug(s_0_25)
        let s_0_26: bool = Havev8p9Debug(state, tracer, s_0_25);
        // N s_0_27: branch s_0_26 b33 b1
        if s_0_26 {
            return block_33(state, tracer, fn_state);
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
        let s_1_0: u8 = 0;
        // D s_1_1: write-var lbnx <= s_1_0
        fn_state.lbnx = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #103984u : u32
        let s_2_0: u32 = 103984;
        // D s_2_1: read-reg s_2_0:[struct; 64]
        let s_2_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: read-var n:i64
        let s_2_2: i64 = fn_state.n;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: read-element s_2_1[s_2_3]
        let s_2_4: ProductType5c790c8ef59cc8b2 = s_2_1[(s_2_3) as usize];
        // D s_2_5: call _get_DBGWCR_EL1_Type_LBN(s_2_4)
        let s_2_5: u8 = u_get_DBGWCR_EL1_Type_LBN(state, tracer, s_2_4);
        // D s_2_6: read-var lbnx:u8
        let s_2_6: u8 = fn_state.lbnx;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cast zx s_2_5 -> bv
        let s_2_8: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_9: cast reint s_2_7 -> u128
        let s_2_9: u128 = (s_2_7.value() as u128);
        // D s_2_10: size-of s_2_7
        let s_2_10: u16 = s_2_7.length();
        // D s_2_11: cast reint s_2_8 -> u128
        let s_2_11: u128 = (s_2_8.value() as u128);
        // D s_2_12: size-of s_2_8
        let s_2_12: u16 = s_2_8.length();
        // D s_2_13: lsl s_2_9 s_2_12
        let s_2_13: u128 = s_2_9 << s_2_12;
        // D s_2_14: or s_2_13 s_2_11
        let s_2_14: u128 = ((s_2_13) | (s_2_11));
        // D s_2_15: add s_2_10 s_2_12
        let s_2_15: u16 = (s_2_10 + s_2_12);
        // D s_2_16: create-bits s_2_14 s_2_15
        let s_2_16: Bits = Bits::new(s_2_14, s_2_15);
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: u8 = (s_2_16.value() as u8);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 6u16);
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (s_2_18.value() as i128);
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: write-var linked_n <= s_2_20
        fn_state.linked_n = s_2_20;
        // C s_2_22: const #() : ()
        let s_2_22: () = ();
        // S s_2_23: call HaveRME(s_2_22)
        let s_2_23: bool = HaveRME(state, tracer, s_2_22);
        // N s_2_24: branch s_2_23 b32 b3
        if s_2_23 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var ssce <= s_3_0
        fn_state.ssce = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #103984u : u32
        let s_4_0: u32 = 103984;
        // D s_4_1: read-reg s_4_0:[struct; 64]
        let s_4_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: read-var n:i64
        let s_4_2: i64 = fn_state.n;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-element s_4_1[s_4_3]
        let s_4_4: ProductType5c790c8ef59cc8b2 = s_4_1[(s_4_3) as usize];
        // D s_4_5: call _get_DBGWCR_EL1_Type_SSC(s_4_4)
        let s_4_5: u8 = u_get_DBGWCR_EL1_Type_SSC(state, tracer, s_4_4);
        // C s_4_6: const #103984u : u32
        let s_4_6: u32 = 103984;
        // D s_4_7: read-reg s_4_6:[struct; 64]
        let s_4_7: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_4_6 as isize);
            tracer.read_register(s_4_6 as isize, value);
            value
        };
        // D s_4_8: read-var n:i64
        let s_4_8: i64 = fn_state.n;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_10: read-element s_4_7[s_4_9]
        let s_4_10: ProductType5c790c8ef59cc8b2 = s_4_7[(s_4_9) as usize];
        // D s_4_11: call _get_DBGWCR_EL1_Type_HMC(s_4_10)
        let s_4_11: bool = u_get_DBGWCR_EL1_Type_HMC(state, tracer, s_4_10);
        // C s_4_12: const #103984u : u32
        let s_4_12: u32 = 103984;
        // D s_4_13: read-reg s_4_12:[struct; 64]
        let s_4_13: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_4_12 as isize);
            tracer.read_register(s_4_12 as isize, value);
            value
        };
        // D s_4_14: read-var n:i64
        let s_4_14: i64 = fn_state.n;
        // D s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_16: read-element s_4_13[s_4_15]
        let s_4_16: ProductType5c790c8ef59cc8b2 = s_4_13[(s_4_15) as usize];
        // D s_4_17: call _get_DBGWCR_EL1_Type_PAC(s_4_16)
        let s_4_17: u8 = u_get_DBGWCR_EL1_Type_PAC(state, tracer, s_4_16);
        // C s_4_18: const #() : ()
        let s_4_18: () = ();
        // S s_4_19: call PC_read(s_4_18)
        let s_4_19: u64 = PC_read(state, tracer, s_4_18);
        // D s_4_20: read-var linked_n:i64
        let s_4_20: i64 = fn_state.linked_n;
        // D s_4_21: cast zx s_4_20 -> i
        let s_4_21: i128 = (i128::try_from(s_4_20).unwrap());
        // D s_4_22: read-var ssce:u8
        let s_4_22: bool = fn_state.ssce;
        // D s_4_23: read-var linked:u8
        let s_4_23: bool = fn_state.linked;
        // C s_4_24: const #0u : u8
        let s_4_24: bool = false;
        // D s_4_25: read-var accdesc:struct
        let s_4_25: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_26: call AArch64_StateMatch(s_4_5, s_4_22, s_4_11, s_4_17, s_4_23, s_4_21, s_4_24, s_4_19, s_4_25)
        let s_4_26: bool = AArch64_StateMatch(
            state,
            tracer,
            s_4_5,
            s_4_22,
            s_4_11,
            s_4_17,
            s_4_23,
            s_4_21,
            s_4_24,
            s_4_19,
            s_4_25,
        );
        // D s_4_27: write-var state_match <= s_4_26
        fn_state.state_match = s_4_26;
        // C s_4_28: const #103984u : u32
        let s_4_28: u32 = 103984;
        // D s_4_29: read-reg s_4_28:[struct; 64]
        let s_4_29: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_4_28 as isize);
            tracer.read_register(s_4_28 as isize, value);
            value
        };
        // D s_4_30: read-var n:i64
        let s_4_30: i64 = fn_state.n;
        // D s_4_31: cast zx s_4_30 -> i
        let s_4_31: i128 = (i128::try_from(s_4_30).unwrap());
        // D s_4_32: read-element s_4_29[s_4_31]
        let s_4_32: ProductType5c790c8ef59cc8b2 = s_4_29[(s_4_31) as usize];
        // D s_4_33: call _get_DBGWCR_EL1_Type_LSC(s_4_32)
        let s_4_33: u8 = u_get_DBGWCR_EL1_Type_LSC(state, tracer, s_4_32);
        // C s_4_34: const #0s : i
        let s_4_34: i128 = 0;
        // D s_4_35: cast zx s_4_33 -> bv
        let s_4_35: Bits = Bits::new(s_4_33 as u128, 2u16);
        // C s_4_36: const #1s : i64
        let s_4_36: i64 = 1;
        // C s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (i128::try_from(s_4_36).unwrap());
        // C s_4_38: const #1s : i
        let s_4_38: i128 = 1;
        // C s_4_39: add s_4_38 s_4_37
        let s_4_39: i128 = (s_4_38 + s_4_37);
        // D s_4_40: bit-extract s_4_35 s_4_34 s_4_39
        let s_4_40: Bits = (Bits::new(
            ((s_4_35) >> (s_4_34)).value(),
            u16::try_from(s_4_39).unwrap(),
        ));
        // D s_4_41: cast reint s_4_40 -> u8
        let s_4_41: u8 = (s_4_40.value() as u8);
        // D s_4_42: write-var ga#12464 <= s_4_41
        fn_state.ga_12464 = s_4_41;
        // D s_4_43: read-var ga#12464:u8
        let s_4_43: u8 = fn_state.ga_12464;
        // D s_4_44: cast zx s_4_43 -> bv
        let s_4_44: Bits = Bits::new(s_4_43 as u128, 2u16);
        // C s_4_45: const #0u : u8
        let s_4_45: u8 = 0;
        // C s_4_46: cast zx s_4_45 -> bv
        let s_4_46: Bits = Bits::new(s_4_45 as u128, 2u16);
        // D s_4_47: cmp-eq s_4_44 s_4_46
        let s_4_47: bool = ((s_4_44) == (s_4_46));
        // D s_4_48: not s_4_47
        let s_4_48: bool = !s_4_47;
        // N s_4_49: branch s_4_48 b22 b5
        if s_4_48 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var ls_match <= s_5_0
        fn_state.ls_match = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var value_match_name <= s_6_0
        fn_state.value_match_name = s_6_0;
        // C s_6_2: const #0s : i64
        let s_6_2: i64 = 0;
        // C s_6_3: const #1s : i
        let s_6_3: i128 = 1;
        // D s_6_4: read-var size:i
        let s_6_4: i128 = fn_state.size;
        // D s_6_5: sub s_6_4 s_6_3
        let s_6_5: i128 = ((s_6_4) - (s_6_3));
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // D s_6_7: write-var gs#16688 <= s_6_6
        fn_state.gs_16688 = s_6_6;
        // D s_6_8: write-var byte <= s_6_2
        fn_state.byte = s_6_2;
        // N s_6_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var byte:i64
        let s_7_0: i64 = fn_state.byte;
        // D s_7_1: read-var gs#16688:i64
        let s_7_1: i64 = fn_state.gs_16688;
        // D s_7_2: cmp-gt s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) > (s_7_1));
        // N s_7_3: branch s_7_2 b12 b8
        if s_7_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var value_match_name:u8
        let s_8_0: bool = fn_state.value_match_name;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var vaddress:u64
        let s_9_0: u64 = fn_state.vaddress;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var byte:i64
        let s_9_2: i64 = fn_state.byte;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cast cvt s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 128);
        // D s_9_5: add s_9_1 s_9_4
        let s_9_5: Bits = (s_9_1 + s_9_4);
        // D s_9_6: cast reint s_9_5 -> u64
        let s_9_6: u64 = (s_9_5.value() as u64);
        // D s_9_7: read-var n:i64
        let s_9_7: i64 = fn_state.n;
        // D s_9_8: call AArch64_WatchpointByteMatch(s_9_7, s_9_6)
        let s_9_8: bool = AArch64_WatchpointByteMatch(state, tracer, s_9_7, s_9_6);
        // D s_9_9: write-var gs#16691 <= s_9_8
        fn_state.gs_16691 = s_9_8;
        // N s_9_10: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#16691:u8
        let s_10_0: bool = fn_state.gs_16691;
        // D s_10_1: write-var value_match_name <= s_10_0
        fn_state.value_match_name = s_10_0;
        // D s_10_2: read-var byte:i64
        let s_10_2: i64 = fn_state.byte;
        // C s_10_3: const #1s : i64
        let s_10_3: i64 = 1;
        // D s_10_4: add s_10_2 s_10_3
        let s_10_4: i64 = (s_10_2 + s_10_3);
        // D s_10_5: write-var byte <= s_10_4
        fn_state.byte = s_10_4;
        // N s_10_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#16691 <= s_11_0
        fn_state.gs_16691 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var value_match_name:u8
        let s_12_0: bool = fn_state.value_match_name;
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
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#16694 <= s_13_0
        fn_state.gs_16694 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#16694:u8
        let s_14_0: bool = fn_state.gs_16694;
        // N s_14_1: branch s_14_0 b20 b15
        if s_14_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#16695 <= s_15_0
        fn_state.gs_16695 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#16695:u8
        let s_16_0: bool = fn_state.gs_16695;
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
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#16696 <= s_17_0
        fn_state.gs_16696 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#16696:u8
        let s_18_0: bool = fn_state.gs_16696;
        // N s_18_1: return s_18_0
        return s_18_0;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var enabled:u8
        let s_19_0: bool = fn_state.enabled;
        // D s_19_1: write-var gs#16696 <= s_19_0
        fn_state.gs_16696 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var ls_match:u8
        let s_20_0: bool = fn_state.ls_match;
        // D s_20_1: write-var gs#16695 <= s_20_0
        fn_state.gs_16695 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var state_match:u8
        let s_21_0: bool = fn_state.state_match;
        // D s_21_1: write-var gs#16694 <= s_21_0
        fn_state.gs_16694 = s_21_0;
        // N s_21_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var ga#12464:u8
        let s_22_0: u8 = fn_state.ga_12464;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #1u : u8
        let s_22_2: u8 = 1;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 2u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b24 b23
        if s_22_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var accdesc.23:struct
        let s_23_0: bool = fn_state.accdesc._23;
        // D s_23_1: write-var ls_match <= s_23_0
        fn_state.ls_match = s_23_0;
        // N s_23_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var ga#12464:u8
        let s_24_0: u8 = fn_state.ga_12464;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 2u16);
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 2u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: not s_24_4
        let s_24_5: bool = !s_24_4;
        // N s_24_6: branch s_24_5 b29 b25
        if s_24_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var accdesc.32:struct
        let s_25_0: bool = fn_state.accdesc._32;
        // N s_25_1: branch s_25_0 b28 b26
        if s_25_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var accdesc.1:struct
        let s_26_0: u32 = fn_state.accdesc._1;
        // C s_26_1: const #6u : u32
        let s_26_1: u32 = 6;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: write-var gs#16680 <= s_26_2
        fn_state.gs_16680 = s_26_2;
        // N s_26_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var gs#16680:u8
        let s_27_0: bool = fn_state.gs_16680;
        // D s_27_1: write-var ls_match <= s_27_0
        fn_state.ls_match = s_27_0;
        // N s_27_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#16680 <= s_28_0
        fn_state.gs_16680 = s_28_0;
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var ga#12464:u8
        let s_29_0: u8 = fn_state.ga_12464;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #3u : u8
        let s_29_2: u8 = 3;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 2u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: not s_29_4
        let s_29_5: bool = !s_29_4;
        // N s_29_6: branch s_29_5 b31 b30
        if s_29_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var ls_match <= s_30_0
        fn_state.ls_match = s_30_0;
        // N s_30_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_31_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #103984u : u32
        let s_32_0: u32 = 103984;
        // D s_32_1: read-reg s_32_0:[struct; 64]
        let s_32_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: read-var n:i64
        let s_32_2: i64 = fn_state.n;
        // D s_32_3: cast zx s_32_2 -> i
        let s_32_3: i128 = (i128::try_from(s_32_2).unwrap());
        // D s_32_4: read-element s_32_1[s_32_3]
        let s_32_4: ProductType5c790c8ef59cc8b2 = s_32_1[(s_32_3) as usize];
        // D s_32_5: call _get_DBGWCR_EL1_Type_SSCE(s_32_4)
        let s_32_5: bool = u_get_DBGWCR_EL1_Type_SSCE(state, tracer, s_32_4);
        // D s_32_6: write-var ssce <= s_32_5
        fn_state.ssce = s_32_5;
        // N s_32_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_33_0: const #103984u : u32
        let s_33_0: u32 = 103984;
        // D s_33_1: read-reg s_33_0:[struct; 64]
        let s_33_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: read-var n:i64
        let s_33_2: i64 = fn_state.n;
        // D s_33_3: cast zx s_33_2 -> i
        let s_33_3: i128 = (i128::try_from(s_33_2).unwrap());
        // D s_33_4: read-element s_33_1[s_33_3]
        let s_33_4: ProductType5c790c8ef59cc8b2 = s_33_1[(s_33_3) as usize];
        // D s_33_5: call _get_DBGWCR_EL1_Type_LBNX(s_33_4)
        let s_33_5: u8 = u_get_DBGWCR_EL1_Type_LBNX(state, tracer, s_33_4);
        // D s_33_6: write-var lbnx <= s_33_5
        fn_state.lbnx = s_33_5;
        // N s_33_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
