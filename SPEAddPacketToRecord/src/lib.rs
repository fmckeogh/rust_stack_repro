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
use SPEAddByteToRecord::*;
use Unreachable::*;
use common::*;
pub fn SPEAddPacketToRecord<T: Tracer>(
    state: &mut State,
    tracer: &T,
    header_hi: u8,
    header_lo: u8,
    payload: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25911: i64,
        ga_20255: i64,
        sz: u8,
        i: i64,
        header_hi: u8,
        header_lo: u8,
        payload: Bits,
    }
    let fn_state = FunctionState {
        header_hi,
        header_lo,
        payload,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var payload:bv
        let s_0_0: Bits = fn_state.payload;
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_6: mod s_0_5 s_0_4
        let s_0_6: i128 = ((s_0_5) % (s_0_4));
        // D s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // D s_0_9: cast zx s_0_7 -> i
        let s_0_9: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_10: cmp-eq s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) == (s_0_8));
        // N s_0_11: assert s_0_10
        let s_0_11: () = assert!(s_0_10);
        // D s_0_12: read-var payload:bv
        let s_0_12: Bits = fn_state.payload;
        // D s_0_13: size-of s_0_12
        let s_0_13: u16 = s_0_12.length();
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: write-var ga#20255 <= s_0_15
        fn_state.ga_20255 = s_0_15;
        // D s_0_17: read-var ga#20255:i64
        let s_0_17: i64 = fn_state.ga_20255;
        // C s_0_18: const #8s : i
        let s_0_18: i128 = 8;
        // D s_0_19: cast zx s_0_17 -> i
        let s_0_19: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_20: cmp-eq s_0_19 s_0_18
        let s_0_20: bool = ((s_0_19) == (s_0_18));
        // D s_0_21: not s_0_20
        let s_0_21: bool = !s_0_20;
        // N s_0_22: branch s_0_21 b6 b1
        if s_0_21 {
            return block_6(state, tracer, fn_state);
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
        let s_1_0: u8 = 0;
        // D s_1_1: write-var sz <= s_1_0
        fn_state.sz = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var header_hi:u8
        let s_2_0: u8 = fn_state.header_hi;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // D s_2_2: read-var sz:u8
        let s_2_2: u8 = fn_state.sz;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // D s_2_14: read-var header_lo:u8
        let s_2_14: u8 = fn_state.header_lo;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 4u16);
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u8
        let s_2_24: u8 = (s_2_23.value() as u8);
        // D s_2_25: call SPEAddByteToRecord(s_2_24)
        let s_2_25: () = SPEAddByteToRecord(state, tracer, s_2_24);
        // C s_2_26: const #0s : i64
        let s_2_26: i64 = 0;
        // D s_2_27: read-var payload:bv
        let s_2_27: Bits = fn_state.payload;
        // D s_2_28: size-of s_2_27
        let s_2_28: u16 = s_2_27.length();
        // D s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (i128::try_from(s_2_28).unwrap());
        // D s_2_30: cast reint s_2_29 -> i64
        let s_2_30: i64 = (s_2_29 as i64);
        // C s_2_31: const #8s : i
        let s_2_31: i128 = 8;
        // D s_2_32: cast zx s_2_30 -> i
        let s_2_32: i128 = (i128::try_from(s_2_30).unwrap());
        // D s_2_33: div s_2_32 s_2_31
        let s_2_33: i128 = ((s_2_32) / (s_2_31));
        // D s_2_34: cast reint s_2_33 -> i64
        let s_2_34: i64 = (s_2_33 as i64);
        // C s_2_35: const #1s : i
        let s_2_35: i128 = 1;
        // D s_2_36: cast zx s_2_34 -> i
        let s_2_36: i128 = (i128::try_from(s_2_34).unwrap());
        // D s_2_37: sub s_2_36 s_2_35
        let s_2_37: i128 = ((s_2_36) - (s_2_35));
        // D s_2_38: cast reint s_2_37 -> i64
        let s_2_38: i64 = (s_2_37 as i64);
        // D s_2_39: write-var gs#25911 <= s_2_38
        fn_state.gs_25911 = s_2_38;
        // D s_2_40: write-var i <= s_2_26
        fn_state.i = s_2_26;
        // N s_2_41: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var i:i64
        let s_3_0: i64 = fn_state.i;
        // D s_3_1: read-var gs#25911:i64
        let s_3_1: i64 = fn_state.gs_25911;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b5 b4
        if s_3_2 {
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
        // C s_4_0: const #8s : i
        let s_4_0: i128 = 8;
        // D s_4_1: read-var i:i64
        let s_4_1: i64 = fn_state.i;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_2 s_4_0
        let s_4_3: i128 = ((s_4_2) * (s_4_0));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // C s_4_5: const #8s : i
        let s_4_5: i128 = 8;
        // D s_4_6: cast zx s_4_4 -> i
        let s_4_6: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_7: read-var payload:bv
        let s_4_7: Bits = fn_state.payload;
        // D s_4_8: bit-extract s_4_7 s_4_6 s_4_5
        let s_4_8: Bits = (Bits::new(
            ((s_4_7) >> (s_4_6)).value(),
            u16::try_from(s_4_5).unwrap(),
        ));
        // D s_4_9: cast reint s_4_8 -> u8
        let s_4_9: u8 = (s_4_8.value() as u8);
        // D s_4_10: call SPEAddByteToRecord(s_4_9)
        let s_4_10: () = SPEAddByteToRecord(state, tracer, s_4_9);
        // D s_4_11: read-var i:i64
        let s_4_11: i64 = fn_state.i;
        // C s_4_12: const #1s : i64
        let s_4_12: i64 = 1;
        // D s_4_13: add s_4_11 s_4_12
        let s_4_13: i64 = (s_4_11 + s_4_12);
        // D s_4_14: write-var i <= s_4_13
        fn_state.i = s_4_13;
        // N s_4_15: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#20255:i64
        let s_6_0: i64 = fn_state.ga_20255;
        // C s_6_1: const #16s : i
        let s_6_1: i128 = 16;
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
        // C s_7_0: const #1u : u8
        let s_7_0: u8 = 1;
        // D s_7_1: write-var sz <= s_7_0
        fn_state.sz = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#20255:i64
        let s_8_0: i64 = fn_state.ga_20255;
        // C s_8_1: const #32s : i
        let s_8_1: i128 = 32;
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
        // C s_9_0: const #2u : u8
        let s_9_0: u8 = 2;
        // D s_9_1: write-var sz <= s_9_0
        fn_state.sz = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#20255:i64
        let s_10_0: i64 = fn_state.ga_20255;
        // C s_10_1: const #64s : i
        let s_10_1: i128 = 64;
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
        // C s_11_0: const #3u : u8
        let s_11_0: u8 = 3;
        // D s_11_1: write-var sz <= s_11_0
        fn_state.sz = s_11_0;
        // N s_11_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call Unreachable(s_12_0)
        let s_12_1: () = Unreachable(state, tracer, s_12_0);
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
