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
use HaveRME::*;
use Mk_ESRType::*;
use FAR_set::*;
use PFAR_set::*;
use IsSecureEL2Enabled::*;
use Zeros::*;
use CurrentSecurityState::*;
use ESR_set::*;
use HavePFAR::*;
use u__UNKNOWN_bits::*;
use set_subrange_zeros::*;
use integer_subrange::*;
use Bit::*;
use AArch64_ExceptionClass::*;
use common::*;
pub fn AArch64_ReportException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    except: ProductTypeb7f99f96751e17c4,
    target_el: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_3879: u32,
        gs_5846: bool,
        iss: u32,
        gs_5851: bool,
        gs_5843: bool,
        gs_5885: bool,
        ga_3833: ProductTypec1bd230b943b3b8c,
        ga_3878: ProductTypeda0231e9dc169f81,
        iss2: u32,
        ga_3884: bool,
        ga_3881: ProductTypeda0231e9dc169f81,
        gs_5844: bool,
        gs_5838: bool,
        ga_3886: ProductTypeda0231e9dc169f81,
        gs_5848: bool,
        gs_5845: bool,
        gs_5850: bool,
        faultaddr: u64,
        gs_5835: bool,
        il: bool,
        gs_5847: bool,
        exceptype: u32,
        ecshadow_67: i128,
        except: ProductTypeb7f99f96751e17c4,
        target_el: u8,
    }
    let fn_state = FunctionState {
        except,
        target_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var except.1:struct
        let s_0_0: u32 = fn_state.except._1;
        // D s_0_1: write-var exceptype <= s_0_0
        fn_state.exceptype = s_0_0;
        // D s_0_2: read-var exceptype:u32
        let s_0_2: u32 = fn_state.exceptype;
        // D s_0_3: read-var target_el:u8
        let s_0_3: u8 = fn_state.target_el;
        // D s_0_4: call AArch64_ExceptionClass(s_0_2, s_0_3)
        let s_0_4: ProductTypec1bd230b943b3b8c = AArch64_ExceptionClass(
            state,
            tracer,
            s_0_2,
            s_0_3,
        );
        // D s_0_5: write-var ga#3833 <= s_0_4
        fn_state.ga_3833 = s_0_4;
        // D s_0_6: read-var ga#3833.0:struct
        let s_0_6: i128 = fn_state.ga_3833._0;
        // D s_0_7: read-var ga#3833.1:struct
        let s_0_7: bool = fn_state.ga_3833._1;
        // D s_0_8: write-var il <= s_0_7
        fn_state.il = s_0_7;
        // D s_0_9: write-var ecshadow#67 <= s_0_6
        fn_state.ecshadow_67 = s_0_6;
        // D s_0_10: read-var except.6:struct
        let s_0_10: u32 = fn_state.except._6;
        // D s_0_11: write-var iss <= s_0_10
        fn_state.iss = s_0_10;
        // D s_0_12: read-var except.7:struct
        let s_0_12: u32 = fn_state.except._7;
        // D s_0_13: write-var iss2 <= s_0_12
        fn_state.iss2 = s_0_12;
        // C s_0_14: const #36u : u8
        let s_0_14: u8 = 36;
        // C s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 8u16);
        // C s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (s_0_15.value() as i128);
        // C s_0_17: cast reint s_0_16 -> i64
        let s_0_17: i64 = (s_0_16 as i64);
        // C s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: read-var ecshadow#67:i
        let s_0_19: i128 = fn_state.ecshadow_67;
        // D s_0_20: cmp-eq s_0_19 s_0_18
        let s_0_20: bool = ((s_0_19) == (s_0_18));
        // N s_0_21: branch s_0_20 b68 b1
        if s_0_20 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #37u : u8
        let s_1_0: u8 = 37;
        // C s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 8u16);
        // C s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // C s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var ecshadow#67:i
        let s_1_5: i128 = fn_state.ecshadow_67;
        // D s_1_6: cmp-eq s_1_5 s_1_4
        let s_1_6: bool = ((s_1_5) == (s_1_4));
        // D s_1_7: write-var gs#5835 <= s_1_6
        fn_state.gs_5835 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#5835:u8
        let s_2_0: bool = fn_state.gs_5835;
        // N s_2_1: branch s_2_0 b67 b3
        if s_2_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#5838 <= s_3_0
        fn_state.gs_5838 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#5838:u8
        let s_4_0: bool = fn_state.gs_5838;
        // N s_4_1: branch s_4_0 b66 b5
        if s_4_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var il:u8
        let s_6_0: bool = fn_state.il;
        // C s_6_1: const #8s : i
        let s_6_1: i128 = 8;
        // S s_6_2: call Zeros(s_6_1)
        let s_6_2: Bits = Zeros(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u8
        let s_6_3: u8 = (s_6_2.value() as u8);
        // S s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 8u16);
        // D s_6_5: read-var iss2:u24
        let s_6_5: u32 = fn_state.iss2;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 24u16);
        // S s_6_7: cast reint s_6_4 -> u128
        let s_6_7: u128 = (s_6_4.value() as u128);
        // D s_6_8: size-of s_6_4
        let s_6_8: u16 = s_6_4.length();
        // D s_6_9: cast reint s_6_6 -> u128
        let s_6_9: u128 = (s_6_6.value() as u128);
        // D s_6_10: size-of s_6_6
        let s_6_10: u16 = s_6_6.length();
        // D s_6_11: lsl s_6_7 s_6_10
        let s_6_11: u128 = s_6_7 << s_6_10;
        // D s_6_12: or s_6_11 s_6_9
        let s_6_12: u128 = ((s_6_11) | (s_6_9));
        // D s_6_13: add s_6_8 s_6_10
        let s_6_13: u16 = (s_6_8 + s_6_10);
        // D s_6_14: create-bits s_6_12 s_6_13
        let s_6_14: Bits = Bits::new(s_6_12, s_6_13);
        // D s_6_15: cast reint s_6_14 -> u32
        let s_6_15: u32 = (s_6_14.value() as u32);
        // C s_6_16: const #5s : i
        let s_6_16: i128 = 5;
        // C s_6_17: const #0s : i
        let s_6_17: i128 = 0;
        // D s_6_18: read-var ecshadow#67:i
        let s_6_18: i128 = fn_state.ecshadow_67;
        // D s_6_19: call integer_subrange(s_6_18, s_6_16, s_6_17)
        let s_6_19: Bits = integer_subrange(state, tracer, s_6_18, s_6_16, s_6_17);
        // D s_6_20: cast reint s_6_19 -> u8
        let s_6_20: u8 = (s_6_19.value() as u8);
        // D s_6_21: cast zx s_6_15 -> bv
        let s_6_21: Bits = Bits::new(s_6_15 as u128, 32u16);
        // D s_6_22: cast zx s_6_20 -> bv
        let s_6_22: Bits = Bits::new(s_6_20 as u128, 6u16);
        // D s_6_23: cast reint s_6_21 -> u128
        let s_6_23: u128 = (s_6_21.value() as u128);
        // D s_6_24: size-of s_6_21
        let s_6_24: u16 = s_6_21.length();
        // D s_6_25: cast reint s_6_22 -> u128
        let s_6_25: u128 = (s_6_22.value() as u128);
        // D s_6_26: size-of s_6_22
        let s_6_26: u16 = s_6_22.length();
        // D s_6_27: lsl s_6_23 s_6_26
        let s_6_27: u128 = s_6_23 << s_6_26;
        // D s_6_28: or s_6_27 s_6_25
        let s_6_28: u128 = ((s_6_27) | (s_6_25));
        // D s_6_29: add s_6_24 s_6_26
        let s_6_29: u16 = (s_6_24 + s_6_26);
        // D s_6_30: create-bits s_6_28 s_6_29
        let s_6_30: Bits = Bits::new(s_6_28, s_6_29);
        // D s_6_31: cast reint s_6_30 -> u38
        let s_6_31: u64 = (s_6_30.value() as u64);
        // D s_6_32: cast zx s_6_31 -> bv
        let s_6_32: Bits = Bits::new(s_6_31 as u128, 38u16);
        // D s_6_33: cast zx s_6_0 -> bv
        let s_6_33: Bits = Bits::new(s_6_0 as u128, 1u16);
        // D s_6_34: cast reint s_6_32 -> u128
        let s_6_34: u128 = (s_6_32.value() as u128);
        // D s_6_35: size-of s_6_32
        let s_6_35: u16 = s_6_32.length();
        // D s_6_36: cast reint s_6_33 -> u128
        let s_6_36: u128 = (s_6_33.value() as u128);
        // D s_6_37: size-of s_6_33
        let s_6_37: u16 = s_6_33.length();
        // D s_6_38: lsl s_6_34 s_6_37
        let s_6_38: u128 = s_6_34 << s_6_37;
        // D s_6_39: or s_6_38 s_6_36
        let s_6_39: u128 = ((s_6_38) | (s_6_36));
        // D s_6_40: add s_6_35 s_6_37
        let s_6_40: u16 = (s_6_35 + s_6_37);
        // D s_6_41: create-bits s_6_39 s_6_40
        let s_6_41: Bits = Bits::new(s_6_39, s_6_40);
        // D s_6_42: cast reint s_6_41 -> u39
        let s_6_42: u64 = (s_6_41.value() as u64);
        // D s_6_43: cast zx s_6_42 -> bv
        let s_6_43: Bits = Bits::new(s_6_42 as u128, 39u16);
        // D s_6_44: read-var iss:u25
        let s_6_44: u32 = fn_state.iss;
        // D s_6_45: cast zx s_6_44 -> bv
        let s_6_45: Bits = Bits::new(s_6_44 as u128, 25u16);
        // D s_6_46: cast reint s_6_43 -> u128
        let s_6_46: u128 = (s_6_43.value() as u128);
        // D s_6_47: size-of s_6_43
        let s_6_47: u16 = s_6_43.length();
        // D s_6_48: cast reint s_6_45 -> u128
        let s_6_48: u128 = (s_6_45.value() as u128);
        // D s_6_49: size-of s_6_45
        let s_6_49: u16 = s_6_45.length();
        // D s_6_50: lsl s_6_46 s_6_49
        let s_6_50: u128 = s_6_46 << s_6_49;
        // D s_6_51: or s_6_50 s_6_48
        let s_6_51: u128 = ((s_6_50) | (s_6_48));
        // D s_6_52: add s_6_47 s_6_49
        let s_6_52: u16 = (s_6_47 + s_6_49);
        // D s_6_53: create-bits s_6_51 s_6_52
        let s_6_53: Bits = Bits::new(s_6_51, s_6_52);
        // D s_6_54: cast reint s_6_53 -> u64
        let s_6_54: u64 = (s_6_53.value() as u64);
        // D s_6_55: call Mk_ESRType(s_6_54)
        let s_6_55: ProductType5c790c8ef59cc8b2 = Mk_ESRType(state, tracer, s_6_54);
        // D s_6_56: read-var target_el:u8
        let s_6_56: u8 = fn_state.target_el;
        // D s_6_57: call ESR_set(s_6_56, s_6_55)
        let s_6_57: () = ESR_set(state, tracer, s_6_56, s_6_55);
        // D s_6_58: read-var exceptype:u32
        let s_6_58: u32 = fn_state.exceptype;
        // C s_6_59: const #17u : u32
        let s_6_59: u32 = 17;
        // D s_6_60: cmp-eq s_6_58 s_6_59
        let s_6_60: bool = ((s_6_58) == (s_6_59));
        // N s_6_61: branch s_6_60 b65 b7
        if s_6_60 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var exceptype:u32
        let s_7_0: u32 = fn_state.exceptype;
        // C s_7_1: const #18u : u32
        let s_7_1: u32 = 18;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b64 b8
        if s_7_2 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var exceptype:u32
        let s_8_0: u32 = fn_state.exceptype;
        // C s_8_1: const #19u : u32
        let s_8_1: u32 = 19;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b63 b9
        if s_8_2 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var exceptype:u32
        let s_9_0: u32 = fn_state.exceptype;
        // C s_9_1: const #20u : u32
        let s_9_1: u32 = 20;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b62 b10
        if s_9_2 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var exceptype:u32
        let s_10_0: u32 = fn_state.exceptype;
        // C s_10_1: const #28u : u32
        let s_10_1: u32 = 28;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b61 b11
        if s_10_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var exceptype:u32
        let s_11_0: u32 = fn_state.exceptype;
        // C s_11_1: const #35u : u32
        let s_11_1: u32 = 35;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // N s_11_3: branch s_11_2 b60 b12
        if s_11_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var exceptype:u32
        let s_12_0: u32 = fn_state.exceptype;
        // C s_12_1: const #27u : u32
        let s_12_1: u32 = 27;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: write-var gs#5843 <= s_12_2
        fn_state.gs_5843 = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#5843:u8
        let s_13_0: bool = fn_state.gs_5843;
        // D s_13_1: write-var gs#5844 <= s_13_0
        fn_state.gs_5844 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#5844:u8
        let s_14_0: bool = fn_state.gs_5844;
        // D s_14_1: write-var gs#5845 <= s_14_0
        fn_state.gs_5845 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#5845:u8
        let s_15_0: bool = fn_state.gs_5845;
        // D s_15_1: write-var gs#5846 <= s_15_0
        fn_state.gs_5846 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#5846:u8
        let s_16_0: bool = fn_state.gs_5846;
        // D s_16_1: write-var gs#5847 <= s_16_0
        fn_state.gs_5847 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#5847:u8
        let s_17_0: bool = fn_state.gs_5847;
        // D s_17_1: write-var gs#5848 <= s_17_0
        fn_state.gs_5848 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#5848:u8
        let s_18_0: bool = fn_state.gs_5848;
        // N s_18_1: branch s_18_0 b59 b19
        if s_18_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // C s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // S s_19_2: call __UNKNOWN_bits(s_19_1)
        let s_19_2: Bits = u__UNKNOWN_bits(state, tracer, s_19_1);
        // S s_19_3: cast reint s_19_2 -> u64
        let s_19_3: u64 = (s_19_2.value() as u64);
        // D s_19_4: read-var target_el:u8
        let s_19_4: u8 = fn_state.target_el;
        // D s_19_5: call FAR_set(s_19_4, s_19_3)
        let s_19_5: () = FAR_set(state, tracer, s_19_4, s_19_3);
        // N s_19_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var except.3:struct
        let s_20_0: bool = fn_state.except._3;
        // N s_20_1: branch s_20_0 b53 b21
        if s_20_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var target_el:u8
        let s_21_0: u8 = fn_state.target_el;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #432u : u32
        let s_21_2: u32 = 432;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: u8 = {
            let value = state.read_register::<u8>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 2u16);
        // D s_21_5: cmp-eq s_21_1 s_21_4
        let s_21_5: bool = ((s_21_1) == (s_21_4));
        // N s_21_6: branch s_21_5 b52 b22
        if s_21_5 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var except.5:struct
        let s_23_0: bool = fn_state.except._5;
        // N s_23_1: branch s_23_0 b33 b24
        if s_23_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HavePFAR(s_24_0)
        let s_24_1: bool = HavePFAR(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b32 b25
        if s_24_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveRME(s_25_0)
        let s_25_1: bool = HaveRME(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b31 b26
        if s_25_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#5850 <= s_26_0
        fn_state.gs_5850 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#5850:u8
        let s_27_0: bool = fn_state.gs_5850;
        // D s_27_1: write-var gs#5851 <= s_27_0
        fn_state.gs_5851 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#5851:u8
        let s_28_0: bool = fn_state.gs_5851;
        // N s_28_1: branch s_28_0 b30 b29
        if s_28_0 {
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
        // N s_29_0: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #64s : i64
        let s_30_0: i64 = 64;
        // C s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // S s_30_2: call __UNKNOWN_bits(s_30_1)
        let s_30_2: Bits = u__UNKNOWN_bits(state, tracer, s_30_1);
        // S s_30_3: cast reint s_30_2 -> u64
        let s_30_3: u64 = (s_30_2.value() as u64);
        // D s_30_4: read-var target_el:u8
        let s_30_4: u8 = fn_state.target_el;
        // D s_30_5: call PFAR_set(s_30_4, s_30_3)
        let s_30_5: () = PFAR_set(state, tracer, s_30_4, s_30_3);
        // N s_30_6: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var target_el:u8
        let s_31_0: u8 = fn_state.target_el;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 2u16);
        // C s_31_2: const #424u : u32
        let s_31_2: u32 = 424;
        // D s_31_3: read-reg s_31_2:u8
        let s_31_3: u8 = {
            let value = state.read_register::<u8>(s_31_2 as isize);
            tracer.read_register(s_31_2 as isize, value);
            value
        };
        // D s_31_4: cast zx s_31_3 -> bv
        let s_31_4: Bits = Bits::new(s_31_3 as u128, 2u16);
        // D s_31_5: cmp-eq s_31_1 s_31_4
        let s_31_5: bool = ((s_31_1) == (s_31_4));
        // D s_31_6: write-var gs#5850 <= s_31_5
        fn_state.gs_5850 = s_31_5;
        // N s_31_7: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#5851 <= s_32_0
        fn_state.gs_5851 = s_32_0;
        // N s_32_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var except.4:struct
        let s_33_0: ProductTypeda0231e9dc169f81 = fn_state.except._4;
        // D s_33_1: write-var ga#3886 <= s_33_0
        fn_state.ga_3886 = s_33_0;
        // D s_33_2: read-var ga#3886.0:struct
        let s_33_2: u64 = fn_state.ga_3886._0;
        // C s_33_3: const #64s : i
        let s_33_3: i128 = 64;
        // D s_33_4: cast zx s_33_2 -> bv
        let s_33_4: Bits = Bits::new(s_33_2 as u128, 56u16);
        // D s_33_5: bits-cast zx s_33_4 -> bv length s_33_3
        let s_33_5: Bits = s_33_4.zero_extend(s_33_3);
        // D s_33_6: cast reint s_33_5 -> u64
        let s_33_6: u64 = (s_33_5.value() as u64);
        // D s_33_7: write-var faultaddr <= s_33_6
        fn_state.faultaddr = s_33_6;
        // C s_33_8: const #() : ()
        let s_33_8: () = ();
        // S s_33_9: call HaveRME(s_33_8)
        let s_33_9: bool = HaveRME(state, tracer, s_33_8);
        // N s_33_10: branch s_33_9 b39 b34
        if s_33_9 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var except.4:struct
        let s_34_0: ProductTypeda0231e9dc169f81 = fn_state.except._4;
        // D s_34_1: write-var ga#3881 <= s_34_0
        fn_state.ga_3881 = s_34_0;
        // D s_34_2: read-var ga#3881.1:struct
        let s_34_2: u32 = fn_state.ga_3881._1;
        // C s_34_3: const #0u : u32
        let s_34_3: u32 = 0;
        // D s_34_4: cmp-eq s_34_2 s_34_3
        let s_34_4: bool = ((s_34_2) == (s_34_3));
        // N s_34_5: branch s_34_4 b38 b35
        if s_34_4 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var ga#3884 <= s_35_0
        fn_state.ga_3884 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var ga#3884:u8
        let s_36_0: bool = fn_state.ga_3884;
        // D s_36_1: call Bit(s_36_0)
        let s_36_1: bool = Bit(state, tracer, s_36_0);
        // C s_36_2: const #63s : i
        let s_36_2: i128 = 63;
        // D s_36_3: read-var faultaddr:u64
        let s_36_3: u64 = fn_state.faultaddr;
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 64u16);
        // C s_36_5: const #1u : u64
        let s_36_5: u64 = 1;
        // D s_36_6: bit-insert s_36_4 s_36_4 s_36_2 s_36_5
        let s_36_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_36_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_36_4.length(),
            );
            (s_36_4 & mask) | (s_36_4 << s_36_2)
        };
        // D s_36_7: cast reint s_36_6 -> u64
        let s_36_7: u64 = (s_36_6.value() as u64);
        // D s_36_8: write-var faultaddr <= s_36_7
        fn_state.faultaddr = s_36_7;
        // N s_36_9: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var target_el:u8
        let s_37_0: u8 = fn_state.target_el;
        // D s_37_1: read-var faultaddr:u64
        let s_37_1: u64 = fn_state.faultaddr;
        // D s_37_2: call PFAR_set(s_37_0, s_37_1)
        let s_37_2: () = PFAR_set(state, tracer, s_37_0, s_37_1);
        // N s_37_3: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var ga#3884 <= s_38_0
        fn_state.ga_3884 = s_38_0;
        // N s_38_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var except.4:struct
        let s_39_0: ProductTypeda0231e9dc169f81 = fn_state.except._4;
        // D s_39_1: write-var ga#3878 <= s_39_0
        fn_state.ga_3878 = s_39_0;
        // D s_39_2: read-var ga#3878.1:struct
        let s_39_2: u32 = fn_state.ga_3878._1;
        // D s_39_3: write-var ga#3879 <= s_39_2
        fn_state.ga_3879 = s_39_2;
        // C s_39_4: const #1u : u32
        let s_39_4: u32 = 1;
        // D s_39_5: read-var ga#3879:u32
        let s_39_5: u32 = fn_state.ga_3879;
        // D s_39_6: cmp-eq s_39_4 s_39_5
        let s_39_6: bool = ((s_39_4) == (s_39_5));
        // D s_39_7: not s_39_6
        let s_39_7: bool = !s_39_6;
        // N s_39_8: branch s_39_7 b45 b40
        if s_39_7 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #62s : i
        let s_40_0: i128 = 62;
        // D s_40_1: read-var faultaddr:u64
        let s_40_1: u64 = fn_state.faultaddr;
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 64u16);
        // C s_40_3: const #0u : u8
        let s_40_3: u8 = 0;
        // C s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 2u16);
        // C s_40_5: const #1s : i
        let s_40_5: i128 = 1;
        // C s_40_6: const #1u : u64
        let s_40_6: u64 = 1;
        // C s_40_7: cast zx s_40_6 -> bv
        let s_40_7: Bits = Bits::new(s_40_6 as u128, 64u16);
        // C s_40_8: lsl s_40_7 s_40_5
        let s_40_8: Bits = s_40_7 << s_40_5;
        // C s_40_9: sub s_40_8 s_40_7
        let s_40_9: Bits = ((s_40_8) - (s_40_7));
        // C s_40_10: and s_40_4 s_40_9
        let s_40_10: Bits = ((s_40_4) & (s_40_9));
        // C s_40_11: lsl s_40_10 s_40_0
        let s_40_11: Bits = s_40_10 << s_40_0;
        // C s_40_12: lsl s_40_9 s_40_0
        let s_40_12: Bits = s_40_9 << s_40_0;
        // C s_40_13: cmpl s_40_12
        let s_40_13: Bits = !s_40_12;
        // D s_40_14: and s_40_2 s_40_13
        let s_40_14: Bits = ((s_40_2) & (s_40_13));
        // D s_40_15: or s_40_14 s_40_11
        let s_40_15: Bits = ((s_40_14) | (s_40_11));
        // D s_40_16: cast reint s_40_15 -> u64
        let s_40_16: u64 = (s_40_15.value() as u64);
        // D s_40_17: write-var faultaddr <= s_40_16
        fn_state.faultaddr = s_40_16;
        // N s_40_18: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var exceptype:u32
        let s_41_0: u32 = fn_state.exceptype;
        // C s_41_1: const #35u : u32
        let s_41_1: u32 = 35;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // N s_41_3: branch s_41_2 b44 b42
        if s_41_2 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #64s : i
        let s_44_0: i128 = 64;
        // C s_44_1: const #11s : i
        let s_44_1: i128 = 11;
        // C s_44_2: const #0s : i
        let s_44_2: i128 = 0;
        // D s_44_3: read-var faultaddr:u64
        let s_44_3: u64 = fn_state.faultaddr;
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 64u16);
        // D s_44_5: call set_subrange_zeros(s_44_0, s_44_4, s_44_1, s_44_2)
        let s_44_5: Bits = set_subrange_zeros(
            state,
            tracer,
            s_44_0,
            s_44_4,
            s_44_1,
            s_44_2,
        );
        // D s_44_6: cast reint s_44_5 -> u64
        let s_44_6: u64 = (s_44_5.value() as u64);
        // D s_44_7: write-var faultaddr <= s_44_6
        fn_state.faultaddr = s_44_6;
        // N s_44_8: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u32
        let s_45_0: u32 = 0;
        // D s_45_1: read-var ga#3879:u32
        let s_45_1: u32 = fn_state.ga_3879;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // D s_45_3: not s_45_2
        let s_45_3: bool = !s_45_2;
        // N s_45_4: branch s_45_3 b47 b46
        if s_45_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #62s : i
        let s_46_0: i128 = 62;
        // D s_46_1: read-var faultaddr:u64
        let s_46_1: u64 = fn_state.faultaddr;
        // D s_46_2: cast zx s_46_1 -> bv
        let s_46_2: Bits = Bits::new(s_46_1 as u128, 64u16);
        // C s_46_3: const #2u : u8
        let s_46_3: u8 = 2;
        // C s_46_4: cast zx s_46_3 -> bv
        let s_46_4: Bits = Bits::new(s_46_3 as u128, 2u16);
        // C s_46_5: const #1s : i
        let s_46_5: i128 = 1;
        // C s_46_6: const #1u : u64
        let s_46_6: u64 = 1;
        // C s_46_7: cast zx s_46_6 -> bv
        let s_46_7: Bits = Bits::new(s_46_6 as u128, 64u16);
        // C s_46_8: lsl s_46_7 s_46_5
        let s_46_8: Bits = s_46_7 << s_46_5;
        // C s_46_9: sub s_46_8 s_46_7
        let s_46_9: Bits = ((s_46_8) - (s_46_7));
        // C s_46_10: and s_46_4 s_46_9
        let s_46_10: Bits = ((s_46_4) & (s_46_9));
        // C s_46_11: lsl s_46_10 s_46_0
        let s_46_11: Bits = s_46_10 << s_46_0;
        // C s_46_12: lsl s_46_9 s_46_0
        let s_46_12: Bits = s_46_9 << s_46_0;
        // C s_46_13: cmpl s_46_12
        let s_46_13: Bits = !s_46_12;
        // D s_46_14: and s_46_2 s_46_13
        let s_46_14: Bits = ((s_46_2) & (s_46_13));
        // D s_46_15: or s_46_14 s_46_11
        let s_46_15: Bits = ((s_46_14) | (s_46_11));
        // D s_46_16: cast reint s_46_15 -> u64
        let s_46_16: u64 = (s_46_15.value() as u64);
        // D s_46_17: write-var faultaddr <= s_46_16
        fn_state.faultaddr = s_46_16;
        // N s_46_18: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #2u : u32
        let s_47_0: u32 = 2;
        // D s_47_1: read-var ga#3879:u32
        let s_47_1: u32 = fn_state.ga_3879;
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // D s_47_3: not s_47_2
        let s_47_3: bool = !s_47_2;
        // N s_47_4: branch s_47_3 b49 b48
        if s_47_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #62s : i
        let s_48_0: i128 = 62;
        // D s_48_1: read-var faultaddr:u64
        let s_48_1: u64 = fn_state.faultaddr;
        // D s_48_2: cast zx s_48_1 -> bv
        let s_48_2: Bits = Bits::new(s_48_1 as u128, 64u16);
        // C s_48_3: const #1u : u8
        let s_48_3: u8 = 1;
        // C s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 2u16);
        // C s_48_5: const #1s : i
        let s_48_5: i128 = 1;
        // C s_48_6: const #1u : u64
        let s_48_6: u64 = 1;
        // C s_48_7: cast zx s_48_6 -> bv
        let s_48_7: Bits = Bits::new(s_48_6 as u128, 64u16);
        // C s_48_8: lsl s_48_7 s_48_5
        let s_48_8: Bits = s_48_7 << s_48_5;
        // C s_48_9: sub s_48_8 s_48_7
        let s_48_9: Bits = ((s_48_8) - (s_48_7));
        // C s_48_10: and s_48_4 s_48_9
        let s_48_10: Bits = ((s_48_4) & (s_48_9));
        // C s_48_11: lsl s_48_10 s_48_0
        let s_48_11: Bits = s_48_10 << s_48_0;
        // C s_48_12: lsl s_48_9 s_48_0
        let s_48_12: Bits = s_48_9 << s_48_0;
        // C s_48_13: cmpl s_48_12
        let s_48_13: Bits = !s_48_12;
        // D s_48_14: and s_48_2 s_48_13
        let s_48_14: Bits = ((s_48_2) & (s_48_13));
        // D s_48_15: or s_48_14 s_48_11
        let s_48_15: Bits = ((s_48_14) | (s_48_11));
        // D s_48_16: cast reint s_48_15 -> u64
        let s_48_16: u64 = (s_48_15.value() as u64);
        // D s_48_17: write-var faultaddr <= s_48_16
        fn_state.faultaddr = s_48_16;
        // N s_48_18: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #3u : u32
        let s_49_0: u32 = 3;
        // D s_49_1: read-var ga#3879:u32
        let s_49_1: u32 = fn_state.ga_3879;
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // N s_49_4: branch s_49_3 b51 b50
        if s_49_3 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #62s : i
        let s_50_0: i128 = 62;
        // D s_50_1: read-var faultaddr:u64
        let s_50_1: u64 = fn_state.faultaddr;
        // D s_50_2: cast zx s_50_1 -> bv
        let s_50_2: Bits = Bits::new(s_50_1 as u128, 64u16);
        // C s_50_3: const #3u : u8
        let s_50_3: u8 = 3;
        // C s_50_4: cast zx s_50_3 -> bv
        let s_50_4: Bits = Bits::new(s_50_3 as u128, 2u16);
        // C s_50_5: const #1s : i
        let s_50_5: i128 = 1;
        // C s_50_6: const #1u : u64
        let s_50_6: u64 = 1;
        // C s_50_7: cast zx s_50_6 -> bv
        let s_50_7: Bits = Bits::new(s_50_6 as u128, 64u16);
        // C s_50_8: lsl s_50_7 s_50_5
        let s_50_8: Bits = s_50_7 << s_50_5;
        // C s_50_9: sub s_50_8 s_50_7
        let s_50_9: Bits = ((s_50_8) - (s_50_7));
        // C s_50_10: and s_50_4 s_50_9
        let s_50_10: Bits = ((s_50_4) & (s_50_9));
        // C s_50_11: lsl s_50_10 s_50_0
        let s_50_11: Bits = s_50_10 << s_50_0;
        // C s_50_12: lsl s_50_9 s_50_0
        let s_50_12: Bits = s_50_9 << s_50_0;
        // C s_50_13: cmpl s_50_12
        let s_50_13: Bits = !s_50_12;
        // D s_50_14: and s_50_2 s_50_13
        let s_50_14: Bits = ((s_50_2) & (s_50_13));
        // D s_50_15: or s_50_14 s_50_11
        let s_50_15: Bits = ((s_50_14) | (s_50_11));
        // D s_50_16: cast reint s_50_15 -> u64
        let s_50_16: u64 = (s_50_15.value() as u64);
        // D s_50_17: write-var faultaddr <= s_50_16
        fn_state.faultaddr = s_50_16;
        // N s_50_18: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #44s : i64
        let s_52_0: i64 = 44;
        // C s_52_1: cast zx s_52_0 -> i
        let s_52_1: i128 = (i128::try_from(s_52_0).unwrap());
        // S s_52_2: call __UNKNOWN_bits(s_52_1)
        let s_52_2: Bits = u__UNKNOWN_bits(state, tracer, s_52_1);
        // C s_52_3: const #11888u : u32
        let s_52_3: u32 = 11888;
        // D s_52_4: read-reg s_52_3:struct
        let s_52_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_52_3 as isize);
            tracer.read_register(s_52_3 as isize, value);
            value
        };
        // C s_52_5: const #11888u : u32
        let s_52_5: u32 = 11888;
        // N s_52_6: write-reg s_52_5 <= s_52_4
        let s_52_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_52_5 as isize, s_52_4);
            tracer.write_register(s_52_5 as isize, s_52_4);
        };
        // N s_52_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #11888u : u32
        let s_53_0: u32 = 11888;
        // D s_53_1: read-reg s_53_0:struct
        let s_53_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // C s_53_2: const #11888u : u32
        let s_53_2: u32 = 11888;
        // N s_53_3: write-reg s_53_2 <= s_53_1
        let s_53_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_53_2 as isize, s_53_1);
            tracer.write_register(s_53_2 as isize, s_53_1);
        };
        // C s_53_4: const #() : ()
        let s_53_4: () = ();
        // S s_53_5: call IsSecureEL2Enabled(s_53_4)
        let s_53_5: bool = IsSecureEL2Enabled(state, tracer, s_53_4);
        // N s_53_6: branch s_53_5 b58 b54
        if s_53_5 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#5885 <= s_54_0
        fn_state.gs_5885 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#5885:u8
        let s_55_0: bool = fn_state.gs_5885;
        // N s_55_1: branch s_55_0 b57 b56
        if s_55_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #11888u : u32
        let s_56_0: u32 = 11888;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // C s_56_2: const #11888u : u32
        let s_56_2: u32 = 11888;
        // N s_56_3: write-reg s_56_2 <= s_56_1
        let s_56_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_56_2 as isize, s_56_1);
            tracer.write_register(s_56_2 as isize, s_56_1);
        };
        // N s_56_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #11888u : u32
        let s_57_0: u32 = 11888;
        // D s_57_1: read-reg s_57_0:struct
        let s_57_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // C s_57_2: const #11888u : u32
        let s_57_2: u32 = 11888;
        // N s_57_3: write-reg s_57_2 <= s_57_1
        let s_57_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_57_2 as isize, s_57_1);
            tracer.write_register(s_57_2 as isize, s_57_1);
        };
        // N s_57_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call CurrentSecurityState(s_58_0)
        let s_58_1: u32 = CurrentSecurityState(state, tracer, s_58_0);
        // C s_58_2: const #3u : u32
        let s_58_2: u32 = 3;
        // S s_58_3: cmp-eq s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) == (s_58_2));
        // D s_58_4: write-var gs#5885 <= s_58_3
        fn_state.gs_5885 = s_58_3;
        // N s_58_5: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var except.9:struct
        let s_59_0: u64 = fn_state.except._9;
        // D s_59_1: read-var target_el:u8
        let s_59_1: u8 = fn_state.target_el;
        // D s_59_2: call FAR_set(s_59_1, s_59_0)
        let s_59_2: () = FAR_set(state, tracer, s_59_1, s_59_0);
        // N s_59_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#5843 <= s_60_0
        fn_state.gs_5843 = s_60_0;
        // N s_60_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#5844 <= s_61_0
        fn_state.gs_5844 = s_61_0;
        // N s_61_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#5845 <= s_62_0
        fn_state.gs_5845 = s_62_0;
        // N s_62_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#5846 <= s_63_0
        fn_state.gs_5846 = s_63_0;
        // N s_63_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#5847 <= s_64_0
        fn_state.gs_5847 = s_64_0;
        // N s_64_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#5848 <= s_65_0
        fn_state.gs_5848 = s_65_0;
        // N s_65_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var il <= s_66_0
        fn_state.il = s_66_0;
        // N s_66_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #24s : i
        let s_67_0: i128 = 24;
        // D s_67_1: read-var iss:u25
        let s_67_1: u32 = fn_state.iss;
        // D s_67_2: cast zx s_67_1 -> bv
        let s_67_2: Bits = Bits::new(s_67_1 as u128, 25u16);
        // C s_67_3: const #1u : u64
        let s_67_3: u64 = 1;
        // D s_67_4: bit-extract s_67_2 s_67_0 s_67_3
        let s_67_4: Bits = (Bits::new(
            ((s_67_2) >> (s_67_0)).value(),
            u16::try_from(s_67_3).unwrap(),
        ));
        // D s_67_5: cast reint s_67_4 -> u8
        let s_67_5: bool = ((s_67_4.value()) != 0);
        // C s_67_6: const #0s : i
        let s_67_6: i128 = 0;
        // C s_67_7: const #0u : u64
        let s_67_7: u64 = 0;
        // D s_67_8: cast zx s_67_5 -> u64
        let s_67_8: u64 = (s_67_5 as u64);
        // C s_67_9: const #1u : u64
        let s_67_9: u64 = 1;
        // D s_67_10: and s_67_8 s_67_9
        let s_67_10: u64 = ((s_67_8) & (s_67_9));
        // D s_67_11: cmp-eq s_67_10 s_67_9
        let s_67_11: bool = ((s_67_10) == (s_67_9));
        // D s_67_12: lsl s_67_8 s_67_6
        let s_67_12: u64 = s_67_8 << s_67_6;
        // D s_67_13: or s_67_7 s_67_12
        let s_67_13: u64 = ((s_67_7) | (s_67_12));
        // D s_67_14: cmpl s_67_12
        let s_67_14: u64 = !s_67_12;
        // D s_67_15: and s_67_7 s_67_14
        let s_67_15: u64 = ((s_67_7) & (s_67_14));
        // D s_67_16: select s_67_11 s_67_13 s_67_15
        let s_67_16: u64 = if s_67_11 { s_67_13 } else { s_67_15 };
        // D s_67_17: cast trunc s_67_16 -> u8
        let s_67_17: bool = ((s_67_16) != 0);
        // D s_67_18: cast zx s_67_17 -> bv
        let s_67_18: Bits = Bits::new(s_67_17 as u128, 1u16);
        // C s_67_19: const #0u : u8
        let s_67_19: bool = false;
        // C s_67_20: cast zx s_67_19 -> bv
        let s_67_20: Bits = Bits::new(s_67_19 as u128, 1u16);
        // D s_67_21: cmp-eq s_67_18 s_67_20
        let s_67_21: bool = ((s_67_18) == (s_67_20));
        // D s_67_22: write-var gs#5838 <= s_67_21
        fn_state.gs_5838 = s_67_21;
        // N s_67_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#5835 <= s_68_0
        fn_state.gs_5835 = s_68_0;
        // N s_68_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
