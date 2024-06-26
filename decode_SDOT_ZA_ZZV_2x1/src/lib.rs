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
use execute_SDOT_ZA_ZZV_2x1::*;
use HaveSMEI16I64::*;
use common::*;
pub fn decode_SDOT_ZA_ZZV_2x1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    sz: bool,
    Zm: u8,
    Rv: u8,
    Zn: u8,
    U: bool,
    off3: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        v: i64,
        VL: i64,
        offset: i64,
        gs_272888: bool,
        sz: bool,
        Zm: u8,
        Rv: u8,
        Zn: u8,
        U: bool,
        off3: u8,
    }
    let fn_state = FunctionState {
        sz,
        Zm,
        Rv,
        Zn,
        U,
        off3,
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
        // N s_0_6: branch s_0_5 b17 b1
        if s_0_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sz:u8
        let s_1_0: bool = fn_state.sz;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b16 b2
        if s_1_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#272888 <= s_2_0
        fn_state.gs_272888 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#272888:u8
        let s_3_0: bool = fn_state.gs_272888;
        // N s_3_1: branch s_3_0 b15 b4
        if s_3_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2u : u8
        let s_4_0: u8 = 2;
        // C s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 3u16);
        // D s_4_2: read-var Rv:u8
        let s_4_2: u8 = fn_state.Rv;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // C s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u8
        let s_4_12: u8 = (s_4_11.value() as u8);
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 5u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var v <= s_4_15
        fn_state.v = s_4_15;
        // D s_4_17: read-var sz:u8
        let s_4_17: bool = fn_state.sz;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // D s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (s_4_18.value() as i128);
        // D s_4_20: cast reint s_4_19 -> i64
        let s_4_20: i64 = (s_4_19 as i64);
        // C s_4_21: const #32s : i64
        let s_4_21: i64 = 32;
        // D s_4_22: lsl s_4_21 s_4_20
        let s_4_22: i64 = s_4_21 << s_4_20;
        // D s_4_23: write-var esize <= s_4_22
        fn_state.esize = s_4_22;
        // D s_4_24: read-var Zn:u8
        let s_4_24: u8 = fn_state.Zn;
        // D s_4_25: cast zx s_4_24 -> bv
        let s_4_25: Bits = Bits::new(s_4_24 as u128, 5u16);
        // D s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (s_4_25.value() as i128);
        // D s_4_27: cast reint s_4_26 -> i64
        let s_4_27: i64 = (s_4_26 as i64);
        // D s_4_28: write-var n <= s_4_27
        fn_state.n = s_4_27;
        // C s_4_29: const #0u : u8
        let s_4_29: bool = false;
        // C s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 1u16);
        // D s_4_31: read-var Zm:u8
        let s_4_31: u8 = fn_state.Zm;
        // D s_4_32: cast zx s_4_31 -> bv
        let s_4_32: Bits = Bits::new(s_4_31 as u128, 4u16);
        // C s_4_33: cast reint s_4_30 -> u128
        let s_4_33: u128 = (s_4_30.value() as u128);
        // D s_4_34: size-of s_4_30
        let s_4_34: u16 = s_4_30.length();
        // D s_4_35: cast reint s_4_32 -> u128
        let s_4_35: u128 = (s_4_32.value() as u128);
        // D s_4_36: size-of s_4_32
        let s_4_36: u16 = s_4_32.length();
        // D s_4_37: lsl s_4_33 s_4_36
        let s_4_37: u128 = s_4_33 << s_4_36;
        // D s_4_38: or s_4_37 s_4_35
        let s_4_38: u128 = ((s_4_37) | (s_4_35));
        // D s_4_39: add s_4_34 s_4_36
        let s_4_39: u16 = (s_4_34 + s_4_36);
        // D s_4_40: create-bits s_4_38 s_4_39
        let s_4_40: Bits = Bits::new(s_4_38, s_4_39);
        // D s_4_41: cast reint s_4_40 -> u8
        let s_4_41: u8 = (s_4_40.value() as u8);
        // D s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 5u16);
        // D s_4_43: cast zx s_4_42 -> i
        let s_4_43: i128 = (s_4_42.value() as i128);
        // D s_4_44: cast reint s_4_43 -> i64
        let s_4_44: i64 = (s_4_43 as i64);
        // D s_4_45: write-var m <= s_4_44
        fn_state.m = s_4_44;
        // D s_4_46: read-var off3:u8
        let s_4_46: u8 = fn_state.off3;
        // D s_4_47: cast zx s_4_46 -> bv
        let s_4_47: Bits = Bits::new(s_4_46 as u128, 3u16);
        // D s_4_48: cast zx s_4_47 -> i
        let s_4_48: i128 = (s_4_47.value() as i128);
        // D s_4_49: cast reint s_4_48 -> i64
        let s_4_49: i64 = (s_4_48 as i64);
        // D s_4_50: write-var offset <= s_4_49
        fn_state.offset = s_4_49;
        // D s_4_51: read-var VL:i64
        let s_4_51: i64 = fn_state.VL;
        // C s_4_52: const #128s : i
        let s_4_52: i128 = 128;
        // D s_4_53: cast zx s_4_51 -> i
        let s_4_53: i128 = (i128::try_from(s_4_51).unwrap());
        // D s_4_54: cmp-eq s_4_53 s_4_52
        let s_4_54: bool = ((s_4_53) == (s_4_52));
        // D s_4_55: not s_4_54
        let s_4_55: bool = !s_4_54;
        // N s_4_56: branch s_4_55 b6 b5
        if s_4_55 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #2s : i64
        let s_5_4: i64 = 2;
        // D s_5_5: read-var m:i64
        let s_5_5: i64 = fn_state.m;
        // D s_5_6: read-var n:i64
        let s_5_6: i64 = fn_state.n;
        // D s_5_7: read-var offset:i64
        let s_5_7: i64 = fn_state.offset;
        // D s_5_8: read-var v:i64
        let s_5_8: i64 = fn_state.v;
        // D s_5_9: call execute_SDOT_ZA_ZZV_2x1(s_5_0, s_5_3, s_5_5, s_5_6, s_5_4, s_5_7, s_5_8)
        let s_5_9: () = execute_SDOT_ZA_ZZV_2x1(
            state,
            tracer,
            s_5_0,
            s_5_3,
            s_5_5,
            s_5_6,
            s_5_4,
            s_5_7,
            s_5_8,
        );
        // N s_5_10: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VL:i64
        let s_6_0: i64 = fn_state.VL;
        // C s_6_1: const #256s : i
        let s_6_1: i128 = 256;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #256s : i64
        let s_7_0: i64 = 256;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #2s : i64
        let s_7_4: i64 = 2;
        // D s_7_5: read-var m:i64
        let s_7_5: i64 = fn_state.m;
        // D s_7_6: read-var n:i64
        let s_7_6: i64 = fn_state.n;
        // D s_7_7: read-var offset:i64
        let s_7_7: i64 = fn_state.offset;
        // D s_7_8: read-var v:i64
        let s_7_8: i64 = fn_state.v;
        // D s_7_9: call execute_SDOT_ZA_ZZV_2x1(s_7_0, s_7_3, s_7_5, s_7_6, s_7_4, s_7_7, s_7_8)
        let s_7_9: () = execute_SDOT_ZA_ZZV_2x1(
            state,
            tracer,
            s_7_0,
            s_7_3,
            s_7_5,
            s_7_6,
            s_7_4,
            s_7_7,
            s_7_8,
        );
        // N s_7_10: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #512s : i
        let s_8_1: i128 = 512;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #512s : i64
        let s_9_0: i64 = 512;
        // D s_9_1: read-var esize:i64
        let s_9_1: i64 = fn_state.esize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #2s : i64
        let s_9_4: i64 = 2;
        // D s_9_5: read-var m:i64
        let s_9_5: i64 = fn_state.m;
        // D s_9_6: read-var n:i64
        let s_9_6: i64 = fn_state.n;
        // D s_9_7: read-var offset:i64
        let s_9_7: i64 = fn_state.offset;
        // D s_9_8: read-var v:i64
        let s_9_8: i64 = fn_state.v;
        // D s_9_9: call execute_SDOT_ZA_ZZV_2x1(s_9_0, s_9_3, s_9_5, s_9_6, s_9_4, s_9_7, s_9_8)
        let s_9_9: () = execute_SDOT_ZA_ZZV_2x1(
            state,
            tracer,
            s_9_0,
            s_9_3,
            s_9_5,
            s_9_6,
            s_9_4,
            s_9_7,
            s_9_8,
        );
        // N s_9_10: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #1024s : i
        let s_10_1: i128 = 1024;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
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
        // C s_11_0: const #1024s : i64
        let s_11_0: i64 = 1024;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #2s : i64
        let s_11_4: i64 = 2;
        // D s_11_5: read-var m:i64
        let s_11_5: i64 = fn_state.m;
        // D s_11_6: read-var n:i64
        let s_11_6: i64 = fn_state.n;
        // D s_11_7: read-var offset:i64
        let s_11_7: i64 = fn_state.offset;
        // D s_11_8: read-var v:i64
        let s_11_8: i64 = fn_state.v;
        // D s_11_9: call execute_SDOT_ZA_ZZV_2x1(s_11_0, s_11_3, s_11_5, s_11_6, s_11_4, s_11_7, s_11_8)
        let s_11_9: () = execute_SDOT_ZA_ZZV_2x1(
            state,
            tracer,
            s_11_0,
            s_11_3,
            s_11_5,
            s_11_6,
            s_11_4,
            s_11_7,
            s_11_8,
        );
        // N s_11_10: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #2048s : i
        let s_12_1: i128 = 2048;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #2048s : i64
        let s_13_0: i64 = 2048;
        // D s_13_1: read-var esize:i64
        let s_13_1: i64 = fn_state.esize;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #2s : i64
        let s_13_4: i64 = 2;
        // D s_13_5: read-var m:i64
        let s_13_5: i64 = fn_state.m;
        // D s_13_6: read-var n:i64
        let s_13_6: i64 = fn_state.n;
        // D s_13_7: read-var offset:i64
        let s_13_7: i64 = fn_state.offset;
        // D s_13_8: read-var v:i64
        let s_13_8: i64 = fn_state.v;
        // D s_13_9: call execute_SDOT_ZA_ZZV_2x1(s_13_0, s_13_3, s_13_5, s_13_6, s_13_4, s_13_7, s_13_8)
        let s_13_9: () = execute_SDOT_ZA_ZZV_2x1(
            state,
            tracer,
            s_13_0,
            s_13_3,
            s_13_5,
            s_13_6,
            s_13_4,
            s_13_7,
            s_13_8,
        );
        // N s_13_10: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HaveSMEI16I64(s_16_0)
        let s_16_1: bool = HaveSMEI16I64(state, tracer, s_16_0);
        // S s_16_2: not s_16_1
        let s_16_2: bool = !s_16_1;
        // D s_16_3: write-var gs#272888 <= s_16_2
        fn_state.gs_272888 = s_16_2;
        // N s_16_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
}
