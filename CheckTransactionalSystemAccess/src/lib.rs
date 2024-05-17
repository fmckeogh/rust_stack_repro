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
use u_get_HCR_EL2_Type_E2H::*;
use u__IMPDEF_boolean::*;
use common::*;
pub fn CheckTransactionalSystemAccess<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: u8,
    op1: u8,
    crn: u8,
    crm: u8,
    op2: u8,
    read: bool,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        b__21: u32,
        gs_15710: bool,
        gs_15684: bool,
        b__27: u32,
        b__22: u32,
        gs_15697: bool,
        return_value: bool,
        gs_15648: bool,
        gs_15672: bool,
        gs_15691: bool,
        ga_11598: u32,
        gs_15709: bool,
        b__26: u32,
        b__24: u32,
        gs_15666: bool,
        b__23: u32,
        b__0: u32,
        gs_15645: bool,
        gs_15646: bool,
        gs_15678: bool,
        gs_15633: bool,
        b__30: u32,
        op0: u8,
        op1: u8,
        crn: u8,
        crm: u8,
        op2: u8,
        read: bool,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        crn,
        crm,
        op2,
        read,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var read:u8
        let s_0_0: bool = fn_state.read;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // D s_0_2: read-var op0:u8
        let s_0_2: u8 = fn_state.op0;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // D s_0_4: cast reint s_0_1 -> u128
        let s_0_4: u128 = (s_0_1.value() as u128);
        // D s_0_5: size-of s_0_1
        let s_0_5: u16 = s_0_1.length();
        // D s_0_6: cast reint s_0_3 -> u128
        let s_0_6: u128 = (s_0_3.value() as u128);
        // D s_0_7: size-of s_0_3
        let s_0_7: u16 = s_0_3.length();
        // D s_0_8: lsl s_0_4 s_0_7
        let s_0_8: u128 = s_0_4 << s_0_7;
        // D s_0_9: or s_0_8 s_0_6
        let s_0_9: u128 = ((s_0_8) | (s_0_6));
        // D s_0_10: add s_0_5 s_0_7
        let s_0_10: u16 = (s_0_5 + s_0_7);
        // D s_0_11: create-bits s_0_9 s_0_10
        let s_0_11: Bits = Bits::new(s_0_9, s_0_10);
        // D s_0_12: cast reint s_0_11 -> u8
        let s_0_12: u8 = (s_0_11.value() as u8);
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 3u16);
        // D s_0_14: read-var op1:u8
        let s_0_14: u8 = fn_state.op1;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 3u16);
        // D s_0_16: cast reint s_0_13 -> u128
        let s_0_16: u128 = (s_0_13.value() as u128);
        // D s_0_17: size-of s_0_13
        let s_0_17: u16 = s_0_13.length();
        // D s_0_18: cast reint s_0_15 -> u128
        let s_0_18: u128 = (s_0_15.value() as u128);
        // D s_0_19: size-of s_0_15
        let s_0_19: u16 = s_0_15.length();
        // D s_0_20: lsl s_0_16 s_0_19
        let s_0_20: u128 = s_0_16 << s_0_19;
        // D s_0_21: or s_0_20 s_0_18
        let s_0_21: u128 = ((s_0_20) | (s_0_18));
        // D s_0_22: add s_0_17 s_0_19
        let s_0_22: u16 = (s_0_17 + s_0_19);
        // D s_0_23: create-bits s_0_21 s_0_22
        let s_0_23: Bits = Bits::new(s_0_21, s_0_22);
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: u8 = (s_0_23.value() as u8);
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 6u16);
        // D s_0_26: read-var crn:u8
        let s_0_26: u8 = fn_state.crn;
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 4u16);
        // D s_0_28: cast reint s_0_25 -> u128
        let s_0_28: u128 = (s_0_25.value() as u128);
        // D s_0_29: size-of s_0_25
        let s_0_29: u16 = s_0_25.length();
        // D s_0_30: cast reint s_0_27 -> u128
        let s_0_30: u128 = (s_0_27.value() as u128);
        // D s_0_31: size-of s_0_27
        let s_0_31: u16 = s_0_27.length();
        // D s_0_32: lsl s_0_28 s_0_31
        let s_0_32: u128 = s_0_28 << s_0_31;
        // D s_0_33: or s_0_32 s_0_30
        let s_0_33: u128 = ((s_0_32) | (s_0_30));
        // D s_0_34: add s_0_29 s_0_31
        let s_0_34: u16 = (s_0_29 + s_0_31);
        // D s_0_35: create-bits s_0_33 s_0_34
        let s_0_35: Bits = Bits::new(s_0_33, s_0_34);
        // D s_0_36: cast reint s_0_35 -> u10
        let s_0_36: u16 = (s_0_35.value() as u16);
        // D s_0_37: cast zx s_0_36 -> bv
        let s_0_37: Bits = Bits::new(s_0_36 as u128, 10u16);
        // D s_0_38: read-var crm:u8
        let s_0_38: u8 = fn_state.crm;
        // D s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 4u16);
        // D s_0_40: cast reint s_0_37 -> u128
        let s_0_40: u128 = (s_0_37.value() as u128);
        // D s_0_41: size-of s_0_37
        let s_0_41: u16 = s_0_37.length();
        // D s_0_42: cast reint s_0_39 -> u128
        let s_0_42: u128 = (s_0_39.value() as u128);
        // D s_0_43: size-of s_0_39
        let s_0_43: u16 = s_0_39.length();
        // D s_0_44: lsl s_0_40 s_0_43
        let s_0_44: u128 = s_0_40 << s_0_43;
        // D s_0_45: or s_0_44 s_0_42
        let s_0_45: u128 = ((s_0_44) | (s_0_42));
        // D s_0_46: add s_0_41 s_0_43
        let s_0_46: u16 = (s_0_41 + s_0_43);
        // D s_0_47: create-bits s_0_45 s_0_46
        let s_0_47: Bits = Bits::new(s_0_45, s_0_46);
        // D s_0_48: cast reint s_0_47 -> u14
        let s_0_48: u16 = (s_0_47.value() as u16);
        // D s_0_49: cast zx s_0_48 -> bv
        let s_0_49: Bits = Bits::new(s_0_48 as u128, 14u16);
        // D s_0_50: read-var op2:u8
        let s_0_50: u8 = fn_state.op2;
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 3u16);
        // D s_0_52: cast reint s_0_49 -> u128
        let s_0_52: u128 = (s_0_49.value() as u128);
        // D s_0_53: size-of s_0_49
        let s_0_53: u16 = s_0_49.length();
        // D s_0_54: cast reint s_0_51 -> u128
        let s_0_54: u128 = (s_0_51.value() as u128);
        // D s_0_55: size-of s_0_51
        let s_0_55: u16 = s_0_51.length();
        // D s_0_56: lsl s_0_52 s_0_55
        let s_0_56: u128 = s_0_52 << s_0_55;
        // D s_0_57: or s_0_56 s_0_54
        let s_0_57: u128 = ((s_0_56) | (s_0_54));
        // D s_0_58: add s_0_53 s_0_55
        let s_0_58: u16 = (s_0_53 + s_0_55);
        // D s_0_59: create-bits s_0_57 s_0_58
        let s_0_59: Bits = Bits::new(s_0_57, s_0_58);
        // D s_0_60: cast reint s_0_59 -> u17
        let s_0_60: u32 = (s_0_59.value() as u32);
        // D s_0_61: write-var ga#11598 <= s_0_60
        fn_state.ga_11598 = s_0_60;
        // D s_0_62: read-var ga#11598:u17
        let s_0_62: u32 = fn_state.ga_11598;
        // D s_0_63: write-var b__0 <= s_0_62
        fn_state.b__0 = s_0_62;
        // C s_0_64: const #7s : i
        let s_0_64: i128 = 7;
        // D s_0_65: read-var b__0:u17
        let s_0_65: u32 = fn_state.b__0;
        // D s_0_66: cast zx s_0_65 -> bv
        let s_0_66: Bits = Bits::new(s_0_65 as u128, 17u16);
        // C s_0_67: const #1s : i64
        let s_0_67: i64 = 1;
        // C s_0_68: cast zx s_0_67 -> i
        let s_0_68: i128 = (i128::try_from(s_0_67).unwrap());
        // C s_0_69: const #9s : i
        let s_0_69: i128 = 9;
        // C s_0_70: add s_0_69 s_0_68
        let s_0_70: i128 = (s_0_69 + s_0_68);
        // D s_0_71: bit-extract s_0_66 s_0_64 s_0_70
        let s_0_71: Bits = (Bits::new(
            ((s_0_66) >> (s_0_64)).value(),
            u16::try_from(s_0_70).unwrap(),
        ));
        // D s_0_72: cast reint s_0_71 -> u10
        let s_0_72: u16 = (s_0_71.value() as u16);
        // D s_0_73: cast zx s_0_72 -> bv
        let s_0_73: Bits = Bits::new(s_0_72 as u128, 10u16);
        // C s_0_74: const #52u : u10
        let s_0_74: u16 = 52;
        // C s_0_75: cast zx s_0_74 -> bv
        let s_0_75: Bits = Bits::new(s_0_74 as u128, 10u16);
        // D s_0_76: cmp-eq s_0_73 s_0_75
        let s_0_76: bool = ((s_0_73) == (s_0_75));
        // N s_0_77: branch s_0_76 b99 b1
        if s_0_76 {
            return block_99(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#15633 <= s_1_0
        fn_state.gs_15633 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#15633:u8
        let s_2_0: bool = fn_state.gs_15633;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b5 b3
        if s_2_1 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var return_value:u8
        let s_4_0: bool = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var ga#11598:u17
        let s_5_0: u32 = fn_state.ga_11598;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 17u16);
        // C s_5_2: const #23457u : u17
        let s_5_2: u32 = 23457;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 17u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var ga#11598:u17
        let s_7_0: u32 = fn_state.ga_11598;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: cast zx s_7_0 -> bv
        let s_7_2: Bits = Bits::new(s_7_0 as u128, 17u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #15s : i
        let s_7_5: i128 = 15;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_1 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_1)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u16
        let s_7_8: u16 = (s_7_7.value() as u16);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 16u16);
        // C s_7_10: const #27912u : u16
        let s_7_10: u16 = 27912;
        // C s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 16u16);
        // D s_7_12: cmp-eq s_7_9 s_7_11
        let s_7_12: bool = ((s_7_9) == (s_7_11));
        // D s_7_13: not s_7_12
        let s_7_13: bool = !s_7_12;
        // N s_7_14: branch s_7_13 b9 b8
        if s_7_13 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var ga#11598:u17
        let s_9_0: u32 = fn_state.ga_11598;
        // C s_9_1: const #1s : i
        let s_9_1: i128 = 1;
        // D s_9_2: cast zx s_9_0 -> bv
        let s_9_2: Bits = Bits::new(s_9_0 as u128, 17u16);
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #15s : i
        let s_9_5: i128 = 15;
        // C s_9_6: add s_9_5 s_9_4
        let s_9_6: i128 = (s_9_5 + s_9_4);
        // D s_9_7: bit-extract s_9_2 s_9_1 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_2) >> (s_9_1)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u16
        let s_9_8: u16 = (s_9_7.value() as u16);
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 16u16);
        // C s_9_10: const #27920u : u16
        let s_9_10: u16 = 27920;
        // C s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 16u16);
        // D s_9_12: cmp-eq s_9_9 s_9_11
        let s_9_12: bool = ((s_9_9) == (s_9_11));
        // D s_9_13: not s_9_12
        let s_9_13: bool = !s_9_12;
        // N s_9_14: branch s_9_13 b11 b10
        if s_9_13 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var ga#11598:u17
        let s_11_0: u32 = fn_state.ga_11598;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 17u16);
        // C s_11_2: const #49712u : u17
        let s_11_2: u32 = 49712;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 17u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var ga#11598:u17
        let s_13_0: u32 = fn_state.ga_11598;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 17u16);
        // C s_13_2: const #56548u : u17
        let s_13_2: u32 = 56548;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 17u16);
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
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var ga#11598:u17
        let s_15_0: u32 = fn_state.ga_11598;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 17u16);
        // C s_15_2: const #121129u : u17
        let s_15_2: u32 = 121129;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 17u16);
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
    ) -> bool {
        // C s_16_0: const #16975u : u32
        let s_16_0: u32 = 16975;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // C s_16_3: const #448u : u32
        let s_16_3: u32 = 448;
        // D s_16_4: read-reg s_16_3:u8
        let s_16_4: u8 = {
            let value = state.read_register::<u8>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 2u16);
        // D s_16_6: cmp-eq s_16_2 s_16_5
        let s_16_6: bool = ((s_16_2) == (s_16_5));
        // D s_16_7: write-var return_value <= s_16_6
        fn_state.return_value = s_16_6;
        // N s_16_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var ga#11598:u17
        let s_17_0: u32 = fn_state.ga_11598;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 17u16);
        // C s_17_2: const #114985u : u17
        let s_17_2: u32 = 114985;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 17u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b25 b18
        if s_17_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #16975u : u32
        let s_18_0: u32 = 16975;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 2u16);
        // C s_18_3: const #440u : u32
        let s_18_3: u32 = 440;
        // D s_18_4: read-reg s_18_3:u8
        let s_18_4: u8 = {
            let value = state.read_register::<u8>(s_18_3 as isize);
            tracer.read_register(s_18_3 as isize, value);
            value
        };
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 2u16);
        // D s_18_6: cmp-eq s_18_2 s_18_5
        let s_18_6: bool = ((s_18_2) == (s_18_5));
        // N s_18_7: branch s_18_6 b24 b19
        if s_18_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #16975u : u32
        let s_19_0: u32 = 16975;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 2u16);
        // C s_19_3: const #432u : u32
        let s_19_3: u32 = 432;
        // D s_19_4: read-reg s_19_3:u8
        let s_19_4: u8 = {
            let value = state.read_register::<u8>(s_19_3 as isize);
            tracer.read_register(s_19_3 as isize, value);
            value
        };
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 2u16);
        // D s_19_6: cmp-eq s_19_2 s_19_5
        let s_19_6: bool = ((s_19_2) == (s_19_5));
        // N s_19_7: branch s_19_6 b23 b20
        if s_19_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#15645 <= s_20_0
        fn_state.gs_15645 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var gs#15645:u8
        let s_21_0: bool = fn_state.gs_15645;
        // D s_21_1: write-var gs#15646 <= s_21_0
        fn_state.gs_15646 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#15646:u8
        let s_22_0: bool = fn_state.gs_15646;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_E2H(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#15645 <= s_23_6
        fn_state.gs_15645 = s_23_6;
        // N s_23_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#15646 <= s_24_0
        fn_state.gs_15646 = s_24_0;
        // N s_24_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var ga#11598:u17
        let s_25_0: u32 = fn_state.ga_11598;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 17u16);
        // C s_25_2: const #123177u : u17
        let s_25_2: u32 = 123177;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 17u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b30 b26
        if s_25_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #16975u : u32
        let s_26_0: u32 = 16975;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 2u16);
        // C s_26_3: const #432u : u32
        let s_26_3: u32 = 432;
        // D s_26_4: read-reg s_26_3:u8
        let s_26_4: u8 = {
            let value = state.read_register::<u8>(s_26_3 as isize);
            tracer.read_register(s_26_3 as isize, value);
            value
        };
        // D s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 2u16);
        // D s_26_6: cmp-eq s_26_2 s_26_5
        let s_26_6: bool = ((s_26_2) == (s_26_5));
        // N s_26_7: branch s_26_6 b29 b27
        if s_26_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#15648 <= s_27_0
        fn_state.gs_15648 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var gs#15648:u8
        let s_28_0: bool = fn_state.gs_15648;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #102552u : u32
        let s_29_0: u32 = 102552;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_HCR_EL2_Type_E2H(s_29_1)
        let s_29_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#15648 <= s_29_6
        fn_state.gs_15648 = s_29_6;
        // N s_29_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var ga#11598:u17
        let s_30_0: u32 = fn_state.ga_11598;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 17u16);
        // C s_30_2: const #127273u : u17
        let s_30_2: u32 = 127273;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 17u16);
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
    ) -> bool {
        // C s_31_0: const #16975u : u32
        let s_31_0: u32 = 16975;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 2u16);
        // C s_31_3: const #424u : u32
        let s_31_3: u32 = 424;
        // D s_31_4: read-reg s_31_3:u8
        let s_31_4: u8 = {
            let value = state.read_register::<u8>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 2u16);
        // D s_31_6: cmp-eq s_31_2 s_31_5
        let s_31_6: bool = ((s_31_2) == (s_31_5));
        // D s_31_7: write-var return_value <= s_31_6
        fn_state.return_value = s_31_6;
        // N s_31_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var ga#11598:u17
        let s_32_0: u32 = fn_state.ga_11598;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 17u16);
        // C s_32_2: const #23480u : u17
        let s_32_2: u32 = 23480;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 17u16);
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
    ) -> bool {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var return_value <= s_33_0
        fn_state.return_value = s_33_0;
        // N s_33_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var ga#11598:u17
        let s_34_0: u32 = fn_state.ga_11598;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 17u16);
        // C s_34_2: const #89017u : u17
        let s_34_2: u32 = 89017;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 17u16);
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
    ) -> bool {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var return_value <= s_35_0
        fn_state.return_value = s_35_0;
        // N s_35_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var ga#11598:u17
        let s_36_0: u32 = fn_state.ga_11598;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 17u16);
        // C s_36_2: const #23482u : u17
        let s_36_2: u32 = 23482;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 17u16);
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
    ) -> bool {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var return_value <= s_37_0
        fn_state.return_value = s_37_0;
        // N s_37_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_38_0: read-var ga#11598:u17
        let s_38_0: u32 = fn_state.ga_11598;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 17u16);
        // C s_38_2: const #89019u : u17
        let s_38_2: u32 = 89019;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 17u16);
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
    ) -> bool {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var return_value <= s_39_0
        fn_state.return_value = s_39_0;
        // N s_39_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_40_0: read-var ga#11598:u17
        let s_40_0: u32 = fn_state.ga_11598;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 17u16);
        // C s_40_2: const #17342u : u17
        let s_40_2: u32 = 17342;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 17u16);
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
    ) -> bool {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var return_value <= s_41_0
        fn_state.return_value = s_41_0;
        // N s_41_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_42_0: read-var ga#11598:u17
        let s_42_0: u32 = fn_state.ga_11598;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 17u16);
        // C s_42_2: const #125225u : u17
        let s_42_2: u32 = 125225;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 17u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: not s_42_4
        let s_42_5: bool = !s_42_4;
        // N s_42_6: branch s_42_5 b44 b43
        if s_42_5 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var return_value <= s_43_0
        fn_state.return_value = s_43_0;
        // N s_43_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var ga#11598:u17
        let s_44_0: u32 = fn_state.ga_11598;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 17u16);
        // C s_44_2: const #114986u : u17
        let s_44_2: u32 = 114986;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 17u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: not s_44_4
        let s_44_5: bool = !s_44_4;
        // N s_44_6: branch s_44_5 b46 b45
        if s_44_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var return_value <= s_45_0
        fn_state.return_value = s_45_0;
        // N s_45_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_46_0: read-var ga#11598:u17
        let s_46_0: u32 = fn_state.ga_11598;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 17u16);
        // C s_46_2: const #114984u : u17
        let s_46_2: u32 = 114984;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 17u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: not s_46_4
        let s_46_5: bool = !s_46_4;
        // N s_46_6: branch s_46_5 b48 b47
        if s_46_5 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var return_value <= s_47_0
        fn_state.return_value = s_47_0;
        // N s_47_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var ga#11598:u17
        let s_48_0: u32 = fn_state.ga_11598;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 17u16);
        // C s_48_2: const #125224u : u17
        let s_48_2: u32 = 125224;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 17u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: not s_48_4
        let s_48_5: bool = !s_48_4;
        // N s_48_6: branch s_48_5 b50 b49
        if s_48_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var return_value <= s_49_0
        fn_state.return_value = s_49_0;
        // N s_49_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_50_0: read-var ga#11598:u17
        let s_50_0: u32 = fn_state.ga_11598;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 17u16);
        // C s_50_2: const #123176u : u17
        let s_50_2: u32 = 123176;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 17u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: not s_50_4
        let s_50_5: bool = !s_50_4;
        // N s_50_6: branch s_50_5 b52 b51
        if s_50_5 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var return_value <= s_51_0
        fn_state.return_value = s_51_0;
        // N s_51_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_52_0: read-var ga#11598:u17
        let s_52_0: u32 = fn_state.ga_11598;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 17u16);
        // C s_52_2: const #127272u : u17
        let s_52_2: u32 = 127272;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 17u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: not s_52_4
        let s_52_5: bool = !s_52_4;
        // N s_52_6: branch s_52_5 b54 b53
        if s_52_5 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var return_value <= s_53_0
        fn_state.return_value = s_53_0;
        // N s_53_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_54_0: read-var ga#11598:u17
        let s_54_0: u32 = fn_state.ga_11598;
        // D s_54_1: write-var b__21 <= s_54_0
        fn_state.b__21 = s_54_0;
        // C s_54_2: const #14s : i
        let s_54_2: i128 = 14;
        // D s_54_3: read-var b__21:u17
        let s_54_3: u32 = fn_state.b__21;
        // D s_54_4: cast zx s_54_3 -> bv
        let s_54_4: Bits = Bits::new(s_54_3 as u128, 17u16);
        // C s_54_5: const #1s : i64
        let s_54_5: i64 = 1;
        // C s_54_6: cast zx s_54_5 -> i
        let s_54_6: i128 = (i128::try_from(s_54_5).unwrap());
        // C s_54_7: const #2s : i
        let s_54_7: i128 = 2;
        // C s_54_8: add s_54_7 s_54_6
        let s_54_8: i128 = (s_54_7 + s_54_6);
        // D s_54_9: bit-extract s_54_4 s_54_2 s_54_8
        let s_54_9: Bits = (Bits::new(
            ((s_54_4) >> (s_54_2)).value(),
            u16::try_from(s_54_8).unwrap(),
        ));
        // D s_54_10: cast reint s_54_9 -> u8
        let s_54_10: u8 = (s_54_9.value() as u8);
        // D s_54_11: cast zx s_54_10 -> bv
        let s_54_11: Bits = Bits::new(s_54_10 as u128, 3u16);
        // C s_54_12: const #7u : u8
        let s_54_12: u8 = 7;
        // C s_54_13: cast zx s_54_12 -> bv
        let s_54_13: Bits = Bits::new(s_54_12 as u128, 3u16);
        // D s_54_14: cmp-eq s_54_11 s_54_13
        let s_54_14: bool = ((s_54_11) == (s_54_13));
        // N s_54_15: branch s_54_14 b98 b55
        if s_54_14 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#15666 <= s_55_0
        fn_state.gs_15666 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_56_0: read-var gs#15666:u8
        let s_56_0: bool = fn_state.gs_15666;
        // D s_56_1: not s_56_0
        let s_56_1: bool = !s_56_0;
        // N s_56_2: branch s_56_1 b58 b57
        if s_56_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var return_value <= s_57_0
        fn_state.return_value = s_57_0;
        // N s_57_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_58_0: read-var ga#11598:u17
        let s_58_0: u32 = fn_state.ga_11598;
        // D s_58_1: write-var b__22 <= s_58_0
        fn_state.b__22 = s_58_0;
        // C s_58_2: const #14s : i
        let s_58_2: i128 = 14;
        // D s_58_3: read-var b__22:u17
        let s_58_3: u32 = fn_state.b__22;
        // D s_58_4: cast zx s_58_3 -> bv
        let s_58_4: Bits = Bits::new(s_58_3 as u128, 17u16);
        // C s_58_5: const #1s : i64
        let s_58_5: i64 = 1;
        // C s_58_6: cast zx s_58_5 -> i
        let s_58_6: i128 = (i128::try_from(s_58_5).unwrap());
        // C s_58_7: const #2s : i
        let s_58_7: i128 = 2;
        // C s_58_8: add s_58_7 s_58_6
        let s_58_8: i128 = (s_58_7 + s_58_6);
        // D s_58_9: bit-extract s_58_4 s_58_2 s_58_8
        let s_58_9: Bits = (Bits::new(
            ((s_58_4) >> (s_58_2)).value(),
            u16::try_from(s_58_8).unwrap(),
        ));
        // D s_58_10: cast reint s_58_9 -> u8
        let s_58_10: u8 = (s_58_9.value() as u8);
        // D s_58_11: cast zx s_58_10 -> bv
        let s_58_11: Bits = Bits::new(s_58_10 as u128, 3u16);
        // C s_58_12: const #7u : u8
        let s_58_12: u8 = 7;
        // C s_58_13: cast zx s_58_12 -> bv
        let s_58_13: Bits = Bits::new(s_58_12 as u128, 3u16);
        // D s_58_14: cmp-eq s_58_11 s_58_13
        let s_58_14: bool = ((s_58_11) == (s_58_13));
        // N s_58_15: branch s_58_14 b97 b59
        if s_58_14 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#15672 <= s_59_0
        fn_state.gs_15672 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_60_0: read-var gs#15672:u8
        let s_60_0: bool = fn_state.gs_15672;
        // D s_60_1: not s_60_0
        let s_60_1: bool = !s_60_0;
        // N s_60_2: branch s_60_1 b62 b61
        if s_60_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var return_value <= s_61_0
        fn_state.return_value = s_61_0;
        // N s_61_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_62_0: read-var ga#11598:u17
        let s_62_0: u32 = fn_state.ga_11598;
        // D s_62_1: write-var b__23 <= s_62_0
        fn_state.b__23 = s_62_0;
        // C s_62_2: const #14s : i
        let s_62_2: i128 = 14;
        // D s_62_3: read-var b__23:u17
        let s_62_3: u32 = fn_state.b__23;
        // D s_62_4: cast zx s_62_3 -> bv
        let s_62_4: Bits = Bits::new(s_62_3 as u128, 17u16);
        // C s_62_5: const #1s : i64
        let s_62_5: i64 = 1;
        // C s_62_6: cast zx s_62_5 -> i
        let s_62_6: i128 = (i128::try_from(s_62_5).unwrap());
        // C s_62_7: const #2s : i
        let s_62_7: i128 = 2;
        // C s_62_8: add s_62_7 s_62_6
        let s_62_8: i128 = (s_62_7 + s_62_6);
        // D s_62_9: bit-extract s_62_4 s_62_2 s_62_8
        let s_62_9: Bits = (Bits::new(
            ((s_62_4) >> (s_62_2)).value(),
            u16::try_from(s_62_8).unwrap(),
        ));
        // D s_62_10: cast reint s_62_9 -> u8
        let s_62_10: u8 = (s_62_9.value() as u8);
        // D s_62_11: cast zx s_62_10 -> bv
        let s_62_11: Bits = Bits::new(s_62_10 as u128, 3u16);
        // C s_62_12: const #7u : u8
        let s_62_12: u8 = 7;
        // C s_62_13: cast zx s_62_12 -> bv
        let s_62_13: Bits = Bits::new(s_62_12 as u128, 3u16);
        // D s_62_14: cmp-eq s_62_11 s_62_13
        let s_62_14: bool = ((s_62_11) == (s_62_13));
        // N s_62_15: branch s_62_14 b96 b63
        if s_62_14 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#15678 <= s_63_0
        fn_state.gs_15678 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_64_0: read-var gs#15678:u8
        let s_64_0: bool = fn_state.gs_15678;
        // D s_64_1: not s_64_0
        let s_64_1: bool = !s_64_0;
        // N s_64_2: branch s_64_1 b66 b65
        if s_64_1 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var return_value <= s_65_0
        fn_state.return_value = s_65_0;
        // N s_65_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_66_0: read-var ga#11598:u17
        let s_66_0: u32 = fn_state.ga_11598;
        // D s_66_1: write-var b__24 <= s_66_0
        fn_state.b__24 = s_66_0;
        // C s_66_2: const #6s : i
        let s_66_2: i128 = 6;
        // D s_66_3: read-var b__24:u17
        let s_66_3: u32 = fn_state.b__24;
        // D s_66_4: cast zx s_66_3 -> bv
        let s_66_4: Bits = Bits::new(s_66_3 as u128, 17u16);
        // C s_66_5: const #1s : i64
        let s_66_5: i64 = 1;
        // C s_66_6: cast zx s_66_5 -> i
        let s_66_6: i128 = (i128::try_from(s_66_5).unwrap());
        // C s_66_7: const #10s : i
        let s_66_7: i128 = 10;
        // C s_66_8: add s_66_7 s_66_6
        let s_66_8: i128 = (s_66_7 + s_66_6);
        // D s_66_9: bit-extract s_66_4 s_66_2 s_66_8
        let s_66_9: Bits = (Bits::new(
            ((s_66_4) >> (s_66_2)).value(),
            u16::try_from(s_66_8).unwrap(),
        ));
        // D s_66_10: cast reint s_66_9 -> u11
        let s_66_10: u16 = (s_66_9.value() as u16);
        // D s_66_11: cast zx s_66_10 -> bv
        let s_66_11: Bits = Bits::new(s_66_10 as u128, 11u16);
        // C s_66_12: const #1817u : u11
        let s_66_12: u16 = 1817;
        // C s_66_13: cast zx s_66_12 -> bv
        let s_66_13: Bits = Bits::new(s_66_12 as u128, 11u16);
        // D s_66_14: cmp-eq s_66_11 s_66_13
        let s_66_14: bool = ((s_66_11) == (s_66_13));
        // N s_66_15: branch s_66_14 b95 b67
        if s_66_14 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#15684 <= s_67_0
        fn_state.gs_15684 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_68_0: read-var gs#15684:u8
        let s_68_0: bool = fn_state.gs_15684;
        // D s_68_1: not s_68_0
        let s_68_1: bool = !s_68_0;
        // N s_68_2: branch s_68_1 b70 b69
        if s_68_1 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var return_value <= s_69_0
        fn_state.return_value = s_69_0;
        // N s_69_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_70_0: read-var ga#11598:u17
        let s_70_0: u32 = fn_state.ga_11598;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 17u16);
        // C s_70_2: const #116315u : u17
        let s_70_2: u32 = 116315;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 17u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: not s_70_4
        let s_70_5: bool = !s_70_4;
        // N s_70_6: branch s_70_5 b72 b71
        if s_70_5 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var return_value <= s_71_0
        fn_state.return_value = s_71_0;
        // N s_71_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_72_0: read-var ga#11598:u17
        let s_72_0: u32 = fn_state.ga_11598;
        // D s_72_1: write-var b__26 <= s_72_0
        fn_state.b__26 = s_72_0;
        // C s_72_2: const #14s : i
        let s_72_2: i128 = 14;
        // D s_72_3: read-var b__26:u17
        let s_72_3: u32 = fn_state.b__26;
        // D s_72_4: cast zx s_72_3 -> bv
        let s_72_4: Bits = Bits::new(s_72_3 as u128, 17u16);
        // C s_72_5: const #1s : i64
        let s_72_5: i64 = 1;
        // C s_72_6: cast zx s_72_5 -> i
        let s_72_6: i128 = (i128::try_from(s_72_5).unwrap());
        // C s_72_7: const #2s : i
        let s_72_7: i128 = 2;
        // C s_72_8: add s_72_7 s_72_6
        let s_72_8: i128 = (s_72_7 + s_72_6);
        // D s_72_9: bit-extract s_72_4 s_72_2 s_72_8
        let s_72_9: Bits = (Bits::new(
            ((s_72_4) >> (s_72_2)).value(),
            u16::try_from(s_72_8).unwrap(),
        ));
        // D s_72_10: cast reint s_72_9 -> u8
        let s_72_10: u8 = (s_72_9.value() as u8);
        // D s_72_11: cast zx s_72_10 -> bv
        let s_72_11: Bits = Bits::new(s_72_10 as u128, 3u16);
        // C s_72_12: const #7u : u8
        let s_72_12: u8 = 7;
        // C s_72_13: cast zx s_72_12 -> bv
        let s_72_13: Bits = Bits::new(s_72_12 as u128, 3u16);
        // D s_72_14: cmp-eq s_72_11 s_72_13
        let s_72_14: bool = ((s_72_11) == (s_72_13));
        // N s_72_15: branch s_72_14 b94 b73
        if s_72_14 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#15691 <= s_73_0
        fn_state.gs_15691 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_74_0: read-var gs#15691:u8
        let s_74_0: bool = fn_state.gs_15691;
        // D s_74_1: not s_74_0
        let s_74_1: bool = !s_74_0;
        // N s_74_2: branch s_74_1 b76 b75
        if s_74_1 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var return_value <= s_75_0
        fn_state.return_value = s_75_0;
        // N s_75_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_76_0: read-var ga#11598:u17
        let s_76_0: u32 = fn_state.ga_11598;
        // D s_76_1: write-var b__27 <= s_76_0
        fn_state.b__27 = s_76_0;
        // C s_76_2: const #14s : i
        let s_76_2: i128 = 14;
        // D s_76_3: read-var b__27:u17
        let s_76_3: u32 = fn_state.b__27;
        // D s_76_4: cast zx s_76_3 -> bv
        let s_76_4: Bits = Bits::new(s_76_3 as u128, 17u16);
        // C s_76_5: const #1s : i64
        let s_76_5: i64 = 1;
        // C s_76_6: cast zx s_76_5 -> i
        let s_76_6: i128 = (i128::try_from(s_76_5).unwrap());
        // C s_76_7: const #2s : i
        let s_76_7: i128 = 2;
        // C s_76_8: add s_76_7 s_76_6
        let s_76_8: i128 = (s_76_7 + s_76_6);
        // D s_76_9: bit-extract s_76_4 s_76_2 s_76_8
        let s_76_9: Bits = (Bits::new(
            ((s_76_4) >> (s_76_2)).value(),
            u16::try_from(s_76_8).unwrap(),
        ));
        // D s_76_10: cast reint s_76_9 -> u8
        let s_76_10: u8 = (s_76_9.value() as u8);
        // D s_76_11: cast zx s_76_10 -> bv
        let s_76_11: Bits = Bits::new(s_76_10 as u128, 3u16);
        // C s_76_12: const #7u : u8
        let s_76_12: u8 = 7;
        // C s_76_13: cast zx s_76_12 -> bv
        let s_76_13: Bits = Bits::new(s_76_12 as u128, 3u16);
        // D s_76_14: cmp-eq s_76_11 s_76_13
        let s_76_14: bool = ((s_76_11) == (s_76_13));
        // N s_76_15: branch s_76_14 b93 b77
        if s_76_14 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#15697 <= s_77_0
        fn_state.gs_15697 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_78_0: read-var gs#15697:u8
        let s_78_0: bool = fn_state.gs_15697;
        // D s_78_1: not s_78_0
        let s_78_1: bool = !s_78_0;
        // N s_78_2: branch s_78_1 b80 b79
        if s_78_1 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var return_value <= s_79_0
        fn_state.return_value = s_79_0;
        // N s_79_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_80_0: read-var ga#11598:u17
        let s_80_0: u32 = fn_state.ga_11598;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 17u16);
        // C s_80_2: const #23455u : u17
        let s_80_2: u32 = 23455;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 17u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: not s_80_4
        let s_80_5: bool = !s_80_4;
        // N s_80_6: branch s_80_5 b82 b81
        if s_80_5 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var return_value <= s_81_0
        fn_state.return_value = s_81_0;
        // N s_81_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_82_0: read-var ga#11598:u17
        let s_82_0: u32 = fn_state.ga_11598;
        // C s_82_1: const #1s : i
        let s_82_1: i128 = 1;
        // D s_82_2: cast zx s_82_0 -> bv
        let s_82_2: Bits = Bits::new(s_82_0 as u128, 17u16);
        // C s_82_3: const #1s : i64
        let s_82_3: i64 = 1;
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #15s : i
        let s_82_5: i128 = 15;
        // C s_82_6: add s_82_5 s_82_4
        let s_82_6: i128 = (s_82_5 + s_82_4);
        // D s_82_7: bit-extract s_82_2 s_82_1 s_82_6
        let s_82_7: Bits = (Bits::new(
            ((s_82_2) >> (s_82_1)).value(),
            u16::try_from(s_82_6).unwrap(),
        ));
        // D s_82_8: cast reint s_82_7 -> u16
        let s_82_8: u16 = (s_82_7.value() as u16);
        // D s_82_9: cast zx s_82_8 -> bv
        let s_82_9: Bits = Bits::new(s_82_8 as u128, 16u16);
        // C s_82_10: const #11726u : u16
        let s_82_10: u16 = 11726;
        // C s_82_11: cast zx s_82_10 -> bv
        let s_82_11: Bits = Bits::new(s_82_10 as u128, 16u16);
        // D s_82_12: cmp-eq s_82_9 s_82_11
        let s_82_12: bool = ((s_82_9) == (s_82_11));
        // D s_82_13: not s_82_12
        let s_82_13: bool = !s_82_12;
        // N s_82_14: branch s_82_13 b84 b83
        if s_82_13 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // D s_83_1: write-var return_value <= s_83_0
        fn_state.return_value = s_83_0;
        // N s_83_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_84_0: read-var ga#11598:u17
        let s_84_0: u32 = fn_state.ga_11598;
        // D s_84_1: write-var b__30 <= s_84_0
        fn_state.b__30 = s_84_0;
        // C s_84_2: const #14s : i
        let s_84_2: i128 = 14;
        // D s_84_3: read-var b__30:u17
        let s_84_3: u32 = fn_state.b__30;
        // D s_84_4: cast zx s_84_3 -> bv
        let s_84_4: Bits = Bits::new(s_84_3 as u128, 17u16);
        // C s_84_5: const #1s : i64
        let s_84_5: i64 = 1;
        // C s_84_6: cast zx s_84_5 -> i
        let s_84_6: i128 = (i128::try_from(s_84_5).unwrap());
        // C s_84_7: const #1s : i
        let s_84_7: i128 = 1;
        // C s_84_8: add s_84_7 s_84_6
        let s_84_8: i128 = (s_84_7 + s_84_6);
        // D s_84_9: bit-extract s_84_4 s_84_2 s_84_8
        let s_84_9: Bits = (Bits::new(
            ((s_84_4) >> (s_84_2)).value(),
            u16::try_from(s_84_8).unwrap(),
        ));
        // D s_84_10: cast reint s_84_9 -> u8
        let s_84_10: u8 = (s_84_9.value() as u8);
        // D s_84_11: cast zx s_84_10 -> bv
        let s_84_11: Bits = Bits::new(s_84_10 as u128, 2u16);
        // C s_84_12: const #3u : u8
        let s_84_12: u8 = 3;
        // C s_84_13: cast zx s_84_12 -> bv
        let s_84_13: Bits = Bits::new(s_84_12 as u128, 2u16);
        // D s_84_14: cmp-eq s_84_11 s_84_13
        let s_84_14: bool = ((s_84_11) == (s_84_13));
        // N s_84_15: branch s_84_14 b89 b85
        if s_84_14 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#15710 <= s_85_0
        fn_state.gs_15710 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_86_0: read-var gs#15710:u8
        let s_86_0: bool = fn_state.gs_15710;
        // D s_86_1: not s_86_0
        let s_86_1: bool = !s_86_0;
        // N s_86_2: branch s_86_1 b88 b87
        if s_86_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_87_0: const #"" : str
        let s_87_0: &'static str = "";
        // S s_87_1: call __IMPDEF_boolean(s_87_0)
        let s_87_1: bool = u__IMPDEF_boolean(state, tracer, s_87_0);
        // D s_87_2: write-var return_value <= s_87_1
        fn_state.return_value = s_87_1;
        // N s_87_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var return_value <= s_88_0
        fn_state.return_value = s_88_0;
        // N s_88_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_89_0: const #10s : i
        let s_89_0: i128 = 10;
        // D s_89_1: read-var b__30:u17
        let s_89_1: u32 = fn_state.b__30;
        // D s_89_2: cast zx s_89_1 -> bv
        let s_89_2: Bits = Bits::new(s_89_1 as u128, 17u16);
        // C s_89_3: const #1s : i64
        let s_89_3: i64 = 1;
        // C s_89_4: cast zx s_89_3 -> i
        let s_89_4: i128 = (i128::try_from(s_89_3).unwrap());
        // C s_89_5: const #0s : i
        let s_89_5: i128 = 0;
        // C s_89_6: add s_89_5 s_89_4
        let s_89_6: i128 = (s_89_5 + s_89_4);
        // D s_89_7: bit-extract s_89_2 s_89_0 s_89_6
        let s_89_7: Bits = (Bits::new(
            ((s_89_2) >> (s_89_0)).value(),
            u16::try_from(s_89_6).unwrap(),
        ));
        // D s_89_8: cast reint s_89_7 -> u8
        let s_89_8: bool = ((s_89_7.value()) != 0);
        // D s_89_9: cast zx s_89_8 -> bv
        let s_89_9: Bits = Bits::new(s_89_8 as u128, 1u16);
        // C s_89_10: const #1u : u8
        let s_89_10: bool = true;
        // C s_89_11: cast zx s_89_10 -> bv
        let s_89_11: Bits = Bits::new(s_89_10 as u128, 1u16);
        // D s_89_12: cmp-eq s_89_9 s_89_11
        let s_89_12: bool = ((s_89_9) == (s_89_11));
        // N s_89_13: branch s_89_12 b92 b90
        if s_89_12 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#15709 <= s_90_0
        fn_state.gs_15709 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_91_0: read-var gs#15709:u8
        let s_91_0: bool = fn_state.gs_15709;
        // D s_91_1: write-var gs#15710 <= s_91_0
        fn_state.gs_15710 = s_91_0;
        // N s_91_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_92_0: const #7s : i
        let s_92_0: i128 = 7;
        // D s_92_1: read-var b__30:u17
        let s_92_1: u32 = fn_state.b__30;
        // D s_92_2: cast zx s_92_1 -> bv
        let s_92_2: Bits = Bits::new(s_92_1 as u128, 17u16);
        // C s_92_3: const #1s : i64
        let s_92_3: i64 = 1;
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // C s_92_5: const #1s : i
        let s_92_5: i128 = 1;
        // C s_92_6: add s_92_5 s_92_4
        let s_92_6: i128 = (s_92_5 + s_92_4);
        // D s_92_7: bit-extract s_92_2 s_92_0 s_92_6
        let s_92_7: Bits = (Bits::new(
            ((s_92_2) >> (s_92_0)).value(),
            u16::try_from(s_92_6).unwrap(),
        ));
        // D s_92_8: cast reint s_92_7 -> u8
        let s_92_8: u8 = (s_92_7.value() as u8);
        // D s_92_9: cast zx s_92_8 -> bv
        let s_92_9: Bits = Bits::new(s_92_8 as u128, 2u16);
        // C s_92_10: const #3u : u8
        let s_92_10: u8 = 3;
        // C s_92_11: cast zx s_92_10 -> bv
        let s_92_11: Bits = Bits::new(s_92_10 as u128, 2u16);
        // D s_92_12: cmp-eq s_92_9 s_92_11
        let s_92_12: bool = ((s_92_9) == (s_92_11));
        // D s_92_13: write-var gs#15709 <= s_92_12
        fn_state.gs_15709 = s_92_12;
        // N s_92_14: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_93_0: const #7s : i
        let s_93_0: i128 = 7;
        // D s_93_1: read-var b__27:u17
        let s_93_1: u32 = fn_state.b__27;
        // D s_93_2: cast zx s_93_1 -> bv
        let s_93_2: Bits = Bits::new(s_93_1 as u128, 17u16);
        // C s_93_3: const #1s : i64
        let s_93_3: i64 = 1;
        // C s_93_4: cast zx s_93_3 -> i
        let s_93_4: i128 = (i128::try_from(s_93_3).unwrap());
        // C s_93_5: const #3s : i
        let s_93_5: i128 = 3;
        // C s_93_6: add s_93_5 s_93_4
        let s_93_6: i128 = (s_93_5 + s_93_4);
        // D s_93_7: bit-extract s_93_2 s_93_0 s_93_6
        let s_93_7: Bits = (Bits::new(
            ((s_93_2) >> (s_93_0)).value(),
            u16::try_from(s_93_6).unwrap(),
        ));
        // D s_93_8: cast reint s_93_7 -> u8
        let s_93_8: u8 = (s_93_7.value() as u8);
        // D s_93_9: cast zx s_93_8 -> bv
        let s_93_9: Bits = Bits::new(s_93_8 as u128, 4u16);
        // C s_93_10: const #14u : u8
        let s_93_10: u8 = 14;
        // C s_93_11: cast zx s_93_10 -> bv
        let s_93_11: Bits = Bits::new(s_93_10 as u128, 4u16);
        // D s_93_12: cmp-eq s_93_9 s_93_11
        let s_93_12: bool = ((s_93_9) == (s_93_11));
        // D s_93_13: write-var gs#15697 <= s_93_12
        fn_state.gs_15697 = s_93_12;
        // N s_93_14: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_94_0: const #7s : i
        let s_94_0: i128 = 7;
        // D s_94_1: read-var b__26:u17
        let s_94_1: u32 = fn_state.b__26;
        // D s_94_2: cast zx s_94_1 -> bv
        let s_94_2: Bits = Bits::new(s_94_1 as u128, 17u16);
        // C s_94_3: const #1s : i64
        let s_94_3: i64 = 1;
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // C s_94_5: const #3s : i
        let s_94_5: i128 = 3;
        // C s_94_6: add s_94_5 s_94_4
        let s_94_6: i128 = (s_94_5 + s_94_4);
        // D s_94_7: bit-extract s_94_2 s_94_0 s_94_6
        let s_94_7: Bits = (Bits::new(
            ((s_94_2) >> (s_94_0)).value(),
            u16::try_from(s_94_6).unwrap(),
        ));
        // D s_94_8: cast reint s_94_7 -> u8
        let s_94_8: u8 = (s_94_7.value() as u8);
        // D s_94_9: cast zx s_94_8 -> bv
        let s_94_9: Bits = Bits::new(s_94_8 as u128, 4u16);
        // C s_94_10: const #13u : u8
        let s_94_10: u8 = 13;
        // C s_94_11: cast zx s_94_10 -> bv
        let s_94_11: Bits = Bits::new(s_94_10 as u128, 4u16);
        // D s_94_12: cmp-eq s_94_9 s_94_11
        let s_94_12: bool = ((s_94_9) == (s_94_11));
        // D s_94_13: write-var gs#15691 <= s_94_12
        fn_state.gs_15691 = s_94_12;
        // N s_94_14: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_95_0: const #0s : i
        let s_95_0: i128 = 0;
        // D s_95_1: read-var b__24:u17
        let s_95_1: u32 = fn_state.b__24;
        // D s_95_2: cast zx s_95_1 -> bv
        let s_95_2: Bits = Bits::new(s_95_1 as u128, 17u16);
        // C s_95_3: const #1s : i64
        let s_95_3: i64 = 1;
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // C s_95_5: const #4s : i
        let s_95_5: i128 = 4;
        // C s_95_6: add s_95_5 s_95_4
        let s_95_6: i128 = (s_95_5 + s_95_4);
        // D s_95_7: bit-extract s_95_2 s_95_0 s_95_6
        let s_95_7: Bits = (Bits::new(
            ((s_95_2) >> (s_95_0)).value(),
            u16::try_from(s_95_6).unwrap(),
        ));
        // D s_95_8: cast reint s_95_7 -> u8
        let s_95_8: u8 = (s_95_7.value() as u8);
        // D s_95_9: cast zx s_95_8 -> bv
        let s_95_9: Bits = Bits::new(s_95_8 as u128, 5u16);
        // C s_95_10: const #2u : u8
        let s_95_10: u8 = 2;
        // C s_95_11: cast zx s_95_10 -> bv
        let s_95_11: Bits = Bits::new(s_95_10 as u128, 5u16);
        // D s_95_12: cmp-eq s_95_9 s_95_11
        let s_95_12: bool = ((s_95_9) == (s_95_11));
        // D s_95_13: write-var gs#15684 <= s_95_12
        fn_state.gs_15684 = s_95_12;
        // N s_95_14: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_96_0: const #7s : i
        let s_96_0: i128 = 7;
        // D s_96_1: read-var b__23:u17
        let s_96_1: u32 = fn_state.b__23;
        // D s_96_2: cast zx s_96_1 -> bv
        let s_96_2: Bits = Bits::new(s_96_1 as u128, 17u16);
        // C s_96_3: const #1s : i64
        let s_96_3: i64 = 1;
        // C s_96_4: cast zx s_96_3 -> i
        let s_96_4: i128 = (i128::try_from(s_96_3).unwrap());
        // C s_96_5: const #3s : i
        let s_96_5: i128 = 3;
        // C s_96_6: add s_96_5 s_96_4
        let s_96_6: i128 = (s_96_5 + s_96_4);
        // D s_96_7: bit-extract s_96_2 s_96_0 s_96_6
        let s_96_7: Bits = (Bits::new(
            ((s_96_2) >> (s_96_0)).value(),
            u16::try_from(s_96_6).unwrap(),
        ));
        // D s_96_8: cast reint s_96_7 -> u8
        let s_96_8: u8 = (s_96_7.value() as u8);
        // D s_96_9: cast zx s_96_8 -> bv
        let s_96_9: Bits = Bits::new(s_96_8 as u128, 4u16);
        // C s_96_10: const #10u : u8
        let s_96_10: u8 = 10;
        // C s_96_11: cast zx s_96_10 -> bv
        let s_96_11: Bits = Bits::new(s_96_10 as u128, 4u16);
        // D s_96_12: cmp-eq s_96_9 s_96_11
        let s_96_12: bool = ((s_96_9) == (s_96_11));
        // D s_96_13: write-var gs#15678 <= s_96_12
        fn_state.gs_15678 = s_96_12;
        // N s_96_14: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_97_0: const #8s : i
        let s_97_0: i128 = 8;
        // D s_97_1: read-var b__22:u17
        let s_97_1: u32 = fn_state.b__22;
        // D s_97_2: cast zx s_97_1 -> bv
        let s_97_2: Bits = Bits::new(s_97_1 as u128, 17u16);
        // C s_97_3: const #1s : i64
        let s_97_3: i64 = 1;
        // C s_97_4: cast zx s_97_3 -> i
        let s_97_4: i128 = (i128::try_from(s_97_3).unwrap());
        // C s_97_5: const #2s : i
        let s_97_5: i128 = 2;
        // C s_97_6: add s_97_5 s_97_4
        let s_97_6: i128 = (s_97_5 + s_97_4);
        // D s_97_7: bit-extract s_97_2 s_97_0 s_97_6
        let s_97_7: Bits = (Bits::new(
            ((s_97_2) >> (s_97_0)).value(),
            u16::try_from(s_97_6).unwrap(),
        ));
        // D s_97_8: cast reint s_97_7 -> u8
        let s_97_8: u8 = (s_97_7.value() as u8);
        // D s_97_9: cast zx s_97_8 -> bv
        let s_97_9: Bits = Bits::new(s_97_8 as u128, 3u16);
        // C s_97_10: const #4u : u8
        let s_97_10: u8 = 4;
        // C s_97_11: cast zx s_97_10 -> bv
        let s_97_11: Bits = Bits::new(s_97_10 as u128, 3u16);
        // D s_97_12: cmp-eq s_97_9 s_97_11
        let s_97_12: bool = ((s_97_9) == (s_97_11));
        // D s_97_13: write-var gs#15672 <= s_97_12
        fn_state.gs_15672 = s_97_12;
        // N s_97_14: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_98_0: const #10s : i
        let s_98_0: i128 = 10;
        // D s_98_1: read-var b__21:u17
        let s_98_1: u32 = fn_state.b__21;
        // D s_98_2: cast zx s_98_1 -> bv
        let s_98_2: Bits = Bits::new(s_98_1 as u128, 17u16);
        // C s_98_3: const #1s : i64
        let s_98_3: i64 = 1;
        // C s_98_4: cast zx s_98_3 -> i
        let s_98_4: i128 = (i128::try_from(s_98_3).unwrap());
        // C s_98_5: const #0s : i
        let s_98_5: i128 = 0;
        // C s_98_6: add s_98_5 s_98_4
        let s_98_6: i128 = (s_98_5 + s_98_4);
        // D s_98_7: bit-extract s_98_2 s_98_0 s_98_6
        let s_98_7: Bits = (Bits::new(
            ((s_98_2) >> (s_98_0)).value(),
            u16::try_from(s_98_6).unwrap(),
        ));
        // D s_98_8: cast reint s_98_7 -> u8
        let s_98_8: bool = ((s_98_7.value()) != 0);
        // D s_98_9: cast zx s_98_8 -> bv
        let s_98_9: Bits = Bits::new(s_98_8 as u128, 1u16);
        // C s_98_10: const #0u : u8
        let s_98_10: bool = false;
        // C s_98_11: cast zx s_98_10 -> bv
        let s_98_11: Bits = Bits::new(s_98_10 as u128, 1u16);
        // D s_98_12: cmp-eq s_98_9 s_98_11
        let s_98_12: bool = ((s_98_9) == (s_98_11));
        // D s_98_13: write-var gs#15666 <= s_98_12
        fn_state.gs_15666 = s_98_12;
        // N s_98_14: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_99_0: const #1s : i
        let s_99_0: i128 = 1;
        // D s_99_1: read-var b__0:u17
        let s_99_1: u32 = fn_state.b__0;
        // D s_99_2: cast zx s_99_1 -> bv
        let s_99_2: Bits = Bits::new(s_99_1 as u128, 17u16);
        // C s_99_3: const #1s : i64
        let s_99_3: i64 = 1;
        // C s_99_4: cast zx s_99_3 -> i
        let s_99_4: i128 = (i128::try_from(s_99_3).unwrap());
        // C s_99_5: const #1s : i
        let s_99_5: i128 = 1;
        // C s_99_6: add s_99_5 s_99_4
        let s_99_6: i128 = (s_99_5 + s_99_4);
        // D s_99_7: bit-extract s_99_2 s_99_0 s_99_6
        let s_99_7: Bits = (Bits::new(
            ((s_99_2) >> (s_99_0)).value(),
            u16::try_from(s_99_6).unwrap(),
        ));
        // D s_99_8: cast reint s_99_7 -> u8
        let s_99_8: u8 = (s_99_7.value() as u8);
        // D s_99_9: cast zx s_99_8 -> bv
        let s_99_9: Bits = Bits::new(s_99_8 as u128, 2u16);
        // C s_99_10: const #3u : u8
        let s_99_10: u8 = 3;
        // C s_99_11: cast zx s_99_10 -> bv
        let s_99_11: Bits = Bits::new(s_99_10 as u128, 2u16);
        // D s_99_12: cmp-eq s_99_9 s_99_11
        let s_99_12: bool = ((s_99_9) == (s_99_11));
        // D s_99_13: write-var gs#15633 <= s_99_12
        fn_state.gs_15633 = s_99_12;
        // N s_99_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
