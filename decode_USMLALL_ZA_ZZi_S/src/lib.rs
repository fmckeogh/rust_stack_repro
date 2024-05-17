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
use CurrentVL_read::*;
use HaveSME2::*;
use execute_USMLALL_ZA_ZZi_S::*;
use common::*;
pub fn decode_USMLALL_ZA_ZZi_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Zm: u8,
    i4h: bool,
    Rv: u8,
    i4l: u8,
    Zn: u8,
    U: bool,
    S: bool,
    off2: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        VL: i64,
        n: i64,
        index: i64,
        v: i64,
        offset: i64,
        Zm: u8,
        i4h: bool,
        Rv: u8,
        i4l: u8,
        Zn: u8,
        U: bool,
        S: bool,
        off2: u8,
    }
    let fn_state = FunctionState {
        Zm,
        i4h,
        Rv,
        i4l,
        Zn,
        U,
        S,
        off2,
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
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSME2(s_0_3)
        let s_0_4: bool = HaveSME2(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b12 b1
        if s_0_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2u : u8
        let s_1_0: u8 = 2;
        // C s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 3u16);
        // D s_1_2: read-var Rv:u8
        let s_1_2: u8 = fn_state.Rv;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // C s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u8
        let s_1_12: u8 = (s_1_11.value() as u8);
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 5u16);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (s_1_13.value() as i128);
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: write-var v <= s_1_15
        fn_state.v = s_1_15;
        // D s_1_17: read-var Zn:u8
        let s_1_17: u8 = fn_state.Zn;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 5u16);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (s_1_18.value() as i128);
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var n <= s_1_20
        fn_state.n = s_1_20;
        // C s_1_22: const #0u : u8
        let s_1_22: bool = false;
        // C s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 1u16);
        // D s_1_24: read-var Zm:u8
        let s_1_24: u8 = fn_state.Zm;
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 4u16);
        // C s_1_26: cast reint s_1_23 -> u128
        let s_1_26: u128 = (s_1_23.value() as u128);
        // D s_1_27: size-of s_1_23
        let s_1_27: u16 = s_1_23.length();
        // D s_1_28: cast reint s_1_25 -> u128
        let s_1_28: u128 = (s_1_25.value() as u128);
        // D s_1_29: size-of s_1_25
        let s_1_29: u16 = s_1_25.length();
        // D s_1_30: lsl s_1_26 s_1_29
        let s_1_30: u128 = s_1_26 << s_1_29;
        // D s_1_31: or s_1_30 s_1_28
        let s_1_31: u128 = ((s_1_30) | (s_1_28));
        // D s_1_32: add s_1_27 s_1_29
        let s_1_32: u16 = (s_1_27 + s_1_29);
        // D s_1_33: create-bits s_1_31 s_1_32
        let s_1_33: Bits = Bits::new(s_1_31, s_1_32);
        // D s_1_34: cast reint s_1_33 -> u8
        let s_1_34: u8 = (s_1_33.value() as u8);
        // D s_1_35: cast zx s_1_34 -> bv
        let s_1_35: Bits = Bits::new(s_1_34 as u128, 5u16);
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (s_1_35.value() as i128);
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: write-var m <= s_1_37
        fn_state.m = s_1_37;
        // D s_1_39: read-var off2:u8
        let s_1_39: u8 = fn_state.off2;
        // D s_1_40: cast zx s_1_39 -> bv
        let s_1_40: Bits = Bits::new(s_1_39 as u128, 2u16);
        // C s_1_41: const #0u : u8
        let s_1_41: u8 = 0;
        // C s_1_42: cast zx s_1_41 -> bv
        let s_1_42: Bits = Bits::new(s_1_41 as u128, 2u16);
        // D s_1_43: cast reint s_1_40 -> u128
        let s_1_43: u128 = (s_1_40.value() as u128);
        // D s_1_44: size-of s_1_40
        let s_1_44: u16 = s_1_40.length();
        // C s_1_45: cast reint s_1_42 -> u128
        let s_1_45: u128 = (s_1_42.value() as u128);
        // D s_1_46: size-of s_1_42
        let s_1_46: u16 = s_1_42.length();
        // D s_1_47: lsl s_1_43 s_1_46
        let s_1_47: u128 = s_1_43 << s_1_46;
        // D s_1_48: or s_1_47 s_1_45
        let s_1_48: u128 = ((s_1_47) | (s_1_45));
        // D s_1_49: add s_1_44 s_1_46
        let s_1_49: u16 = (s_1_44 + s_1_46);
        // D s_1_50: create-bits s_1_48 s_1_49
        let s_1_50: Bits = Bits::new(s_1_48, s_1_49);
        // D s_1_51: cast reint s_1_50 -> u8
        let s_1_51: u8 = (s_1_50.value() as u8);
        // D s_1_52: cast zx s_1_51 -> bv
        let s_1_52: Bits = Bits::new(s_1_51 as u128, 4u16);
        // D s_1_53: cast zx s_1_52 -> i
        let s_1_53: i128 = (s_1_52.value() as i128);
        // D s_1_54: cast reint s_1_53 -> i64
        let s_1_54: i64 = (s_1_53 as i64);
        // D s_1_55: write-var offset <= s_1_54
        fn_state.offset = s_1_54;
        // D s_1_56: read-var i4h:u8
        let s_1_56: bool = fn_state.i4h;
        // D s_1_57: cast zx s_1_56 -> bv
        let s_1_57: Bits = Bits::new(s_1_56 as u128, 1u16);
        // D s_1_58: read-var i4l:u8
        let s_1_58: u8 = fn_state.i4l;
        // D s_1_59: cast zx s_1_58 -> bv
        let s_1_59: Bits = Bits::new(s_1_58 as u128, 3u16);
        // D s_1_60: cast reint s_1_57 -> u128
        let s_1_60: u128 = (s_1_57.value() as u128);
        // D s_1_61: size-of s_1_57
        let s_1_61: u16 = s_1_57.length();
        // D s_1_62: cast reint s_1_59 -> u128
        let s_1_62: u128 = (s_1_59.value() as u128);
        // D s_1_63: size-of s_1_59
        let s_1_63: u16 = s_1_59.length();
        // D s_1_64: lsl s_1_60 s_1_63
        let s_1_64: u128 = s_1_60 << s_1_63;
        // D s_1_65: or s_1_64 s_1_62
        let s_1_65: u128 = ((s_1_64) | (s_1_62));
        // D s_1_66: add s_1_61 s_1_63
        let s_1_66: u16 = (s_1_61 + s_1_63);
        // D s_1_67: create-bits s_1_65 s_1_66
        let s_1_67: Bits = Bits::new(s_1_65, s_1_66);
        // D s_1_68: cast reint s_1_67 -> u8
        let s_1_68: u8 = (s_1_67.value() as u8);
        // D s_1_69: cast zx s_1_68 -> bv
        let s_1_69: Bits = Bits::new(s_1_68 as u128, 4u16);
        // D s_1_70: cast zx s_1_69 -> i
        let s_1_70: i128 = (s_1_69.value() as i128);
        // D s_1_71: cast reint s_1_70 -> i64
        let s_1_71: i64 = (s_1_70 as i64);
        // D s_1_72: write-var index <= s_1_71
        fn_state.index = s_1_71;
        // D s_1_73: read-var VL:i64
        let s_1_73: i64 = fn_state.VL;
        // C s_1_74: const #128s : i
        let s_1_74: i128 = 128;
        // D s_1_75: cast zx s_1_73 -> i
        let s_1_75: i128 = (i128::try_from(s_1_73).unwrap());
        // D s_1_76: cmp-eq s_1_75 s_1_74
        let s_1_76: bool = ((s_1_75) == (s_1_74));
        // D s_1_77: not s_1_76
        let s_1_77: bool = !s_1_76;
        // N s_1_78: branch s_1_77 b3 b2
        if s_1_77 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #128s : i64
        let s_2_0: i64 = 128;
        // C s_2_1: const #32s : i64
        let s_2_1: i64 = 32;
        // D s_2_2: read-var index:i64
        let s_2_2: i64 = fn_state.index;
        // D s_2_3: read-var m:i64
        let s_2_3: i64 = fn_state.m;
        // D s_2_4: read-var n:i64
        let s_2_4: i64 = fn_state.n;
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // D s_2_6: read-var offset:i64
        let s_2_6: i64 = fn_state.offset;
        // D s_2_7: read-var v:i64
        let s_2_7: i64 = fn_state.v;
        // D s_2_8: call execute_USMLALL_ZA_ZZi_S(s_2_0, s_2_1, s_2_2, s_2_3, s_2_4, s_2_5, s_2_6, s_2_7)
        let s_2_8: () = execute_USMLALL_ZA_ZZi_S(
            state,
            tracer,
            s_2_0,
            s_2_1,
            s_2_2,
            s_2_3,
            s_2_4,
            s_2_5,
            s_2_6,
            s_2_7,
        );
        // N s_2_9: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VL:i64
        let s_3_0: i64 = fn_state.VL;
        // C s_3_1: const #256s : i
        let s_3_1: i128 = 256;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) == (s_3_1));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #256s : i64
        let s_4_0: i64 = 256;
        // C s_4_1: const #32s : i64
        let s_4_1: i64 = 32;
        // D s_4_2: read-var index:i64
        let s_4_2: i64 = fn_state.index;
        // D s_4_3: read-var m:i64
        let s_4_3: i64 = fn_state.m;
        // D s_4_4: read-var n:i64
        let s_4_4: i64 = fn_state.n;
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // D s_4_6: read-var offset:i64
        let s_4_6: i64 = fn_state.offset;
        // D s_4_7: read-var v:i64
        let s_4_7: i64 = fn_state.v;
        // D s_4_8: call execute_USMLALL_ZA_ZZi_S(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4, s_4_5, s_4_6, s_4_7)
        let s_4_8: () = execute_USMLALL_ZA_ZZi_S(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_7,
        );
        // N s_4_9: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
        // C s_5_1: const #512s : i
        let s_5_1: i128 = 512;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #512s : i64
        let s_6_0: i64 = 512;
        // C s_6_1: const #32s : i64
        let s_6_1: i64 = 32;
        // D s_6_2: read-var index:i64
        let s_6_2: i64 = fn_state.index;
        // D s_6_3: read-var m:i64
        let s_6_3: i64 = fn_state.m;
        // D s_6_4: read-var n:i64
        let s_6_4: i64 = fn_state.n;
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // D s_6_6: read-var offset:i64
        let s_6_6: i64 = fn_state.offset;
        // D s_6_7: read-var v:i64
        let s_6_7: i64 = fn_state.v;
        // D s_6_8: call execute_USMLALL_ZA_ZZi_S(s_6_0, s_6_1, s_6_2, s_6_3, s_6_4, s_6_5, s_6_6, s_6_7)
        let s_6_8: () = execute_USMLALL_ZA_ZZi_S(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_7,
        );
        // N s_6_9: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #1024s : i
        let s_7_1: i128 = 1024;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1024s : i64
        let s_8_0: i64 = 1024;
        // C s_8_1: const #32s : i64
        let s_8_1: i64 = 32;
        // D s_8_2: read-var index:i64
        let s_8_2: i64 = fn_state.index;
        // D s_8_3: read-var m:i64
        let s_8_3: i64 = fn_state.m;
        // D s_8_4: read-var n:i64
        let s_8_4: i64 = fn_state.n;
        // C s_8_5: const #1s : i64
        let s_8_5: i64 = 1;
        // D s_8_6: read-var offset:i64
        let s_8_6: i64 = fn_state.offset;
        // D s_8_7: read-var v:i64
        let s_8_7: i64 = fn_state.v;
        // D s_8_8: call execute_USMLALL_ZA_ZZi_S(s_8_0, s_8_1, s_8_2, s_8_3, s_8_4, s_8_5, s_8_6, s_8_7)
        let s_8_8: () = execute_USMLALL_ZA_ZZi_S(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
            s_8_7,
        );
        // N s_8_9: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #2048s : i
        let s_9_1: i128 = 2048;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #2048s : i64
        let s_10_0: i64 = 2048;
        // C s_10_1: const #32s : i64
        let s_10_1: i64 = 32;
        // D s_10_2: read-var index:i64
        let s_10_2: i64 = fn_state.index;
        // D s_10_3: read-var m:i64
        let s_10_3: i64 = fn_state.m;
        // D s_10_4: read-var n:i64
        let s_10_4: i64 = fn_state.n;
        // C s_10_5: const #1s : i64
        let s_10_5: i64 = 1;
        // D s_10_6: read-var offset:i64
        let s_10_6: i64 = fn_state.offset;
        // D s_10_7: read-var v:i64
        let s_10_7: i64 = fn_state.v;
        // D s_10_8: call execute_USMLALL_ZA_ZZi_S(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5, s_10_6, s_10_7)
        let s_10_8: () = execute_USMLALL_ZA_ZZi_S(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_7,
        );
        // N s_10_9: return
        return;
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
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
