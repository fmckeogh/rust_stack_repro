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
use HaveSVE::*;
use CurrentVL_read::*;
use execute_ADR_Z_AZ_SD_same_scaled::*;
use common::*;
pub fn decode_ADR_Z_AZ_SD_same_scaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    sz: bool,
    Zm: u8,
    msz: u8,
    Zn: u8,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        VL: i64,
        esize: i64,
        osize: i64,
        n: i64,
        mbytes: i64,
        d: i64,
        sz: bool,
        Zm: u8,
        msz: u8,
        Zn: u8,
        Zd: u8,
    }
    let fn_state = FunctionState {
        sz,
        Zm,
        msz,
        Zn,
        Zd,
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
        // S s_0_4: call HaveSVE(s_0_3)
        let s_0_4: bool = HaveSVE(state, tracer, s_0_3);
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
        // D s_1_0: read-var sz:u8
        let s_1_0: bool = fn_state.sz;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #32s : i64
        let s_1_4: i64 = 32;
        // D s_1_5: lsl s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 << s_1_3;
        // D s_1_6: write-var esize <= s_1_5
        fn_state.esize = s_1_5;
        // D s_1_7: read-var Zn:u8
        let s_1_7: u8 = fn_state.Zn;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 5u16);
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (s_1_8.value() as i128);
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: write-var n <= s_1_10
        fn_state.n = s_1_10;
        // D s_1_12: read-var Zm:u8
        let s_1_12: u8 = fn_state.Zm;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 5u16);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (s_1_13.value() as i128);
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: write-var m <= s_1_15
        fn_state.m = s_1_15;
        // D s_1_17: read-var Zd:u8
        let s_1_17: u8 = fn_state.Zd;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 5u16);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (s_1_18.value() as i128);
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var d <= s_1_20
        fn_state.d = s_1_20;
        // D s_1_22: read-var esize:i64
        let s_1_22: i64 = fn_state.esize;
        // D s_1_23: write-var osize <= s_1_22
        fn_state.osize = s_1_22;
        // D s_1_24: read-var msz:u8
        let s_1_24: u8 = fn_state.msz;
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 2u16);
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (s_1_25.value() as i128);
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // C s_1_28: const #1s : i64
        let s_1_28: i64 = 1;
        // D s_1_29: lsl s_1_28 s_1_27
        let s_1_29: i64 = s_1_28 << s_1_27;
        // D s_1_30: write-var mbytes <= s_1_29
        fn_state.mbytes = s_1_29;
        // D s_1_31: read-var VL:i64
        let s_1_31: i64 = fn_state.VL;
        // C s_1_32: const #128s : i
        let s_1_32: i128 = 128;
        // D s_1_33: cast zx s_1_31 -> i
        let s_1_33: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_34: cmp-eq s_1_33 s_1_32
        let s_1_34: bool = ((s_1_33) == (s_1_32));
        // D s_1_35: not s_1_34
        let s_1_35: bool = !s_1_34;
        // N s_1_36: branch s_1_35 b3 b2
        if s_1_35 {
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
        // D s_2_1: read-var esize:i64
        let s_2_1: i64 = fn_state.esize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: read-var osize:i64
        let s_2_4: i64 = fn_state.osize;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: read-var d:i64
        let s_2_7: i64 = fn_state.d;
        // D s_2_8: read-var m:i64
        let s_2_8: i64 = fn_state.m;
        // D s_2_9: read-var mbytes:i64
        let s_2_9: i64 = fn_state.mbytes;
        // D s_2_10: read-var n:i64
        let s_2_10: i64 = fn_state.n;
        // C s_2_11: const #1u : u8
        let s_2_11: bool = true;
        // D s_2_12: call execute_ADR_Z_AZ_SD_same_scaled(s_2_0, s_2_7, s_2_3, s_2_8, s_2_9, s_2_10, s_2_6, s_2_11)
        let s_2_12: () = execute_ADR_Z_AZ_SD_same_scaled(
            state,
            tracer,
            s_2_0,
            s_2_7,
            s_2_3,
            s_2_8,
            s_2_9,
            s_2_10,
            s_2_6,
            s_2_11,
        );
        // N s_2_13: return
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
        // D s_4_1: read-var esize:i64
        let s_4_1: i64 = fn_state.esize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var osize:i64
        let s_4_4: i64 = fn_state.osize;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: read-var d:i64
        let s_4_7: i64 = fn_state.d;
        // D s_4_8: read-var m:i64
        let s_4_8: i64 = fn_state.m;
        // D s_4_9: read-var mbytes:i64
        let s_4_9: i64 = fn_state.mbytes;
        // D s_4_10: read-var n:i64
        let s_4_10: i64 = fn_state.n;
        // C s_4_11: const #1u : u8
        let s_4_11: bool = true;
        // D s_4_12: call execute_ADR_Z_AZ_SD_same_scaled(s_4_0, s_4_7, s_4_3, s_4_8, s_4_9, s_4_10, s_4_6, s_4_11)
        let s_4_12: () = execute_ADR_Z_AZ_SD_same_scaled(
            state,
            tracer,
            s_4_0,
            s_4_7,
            s_4_3,
            s_4_8,
            s_4_9,
            s_4_10,
            s_4_6,
            s_4_11,
        );
        // N s_4_13: return
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
        // D s_6_1: read-var esize:i64
        let s_6_1: i64 = fn_state.esize;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: read-var osize:i64
        let s_6_4: i64 = fn_state.osize;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // D s_6_7: read-var d:i64
        let s_6_7: i64 = fn_state.d;
        // D s_6_8: read-var m:i64
        let s_6_8: i64 = fn_state.m;
        // D s_6_9: read-var mbytes:i64
        let s_6_9: i64 = fn_state.mbytes;
        // D s_6_10: read-var n:i64
        let s_6_10: i64 = fn_state.n;
        // C s_6_11: const #1u : u8
        let s_6_11: bool = true;
        // D s_6_12: call execute_ADR_Z_AZ_SD_same_scaled(s_6_0, s_6_7, s_6_3, s_6_8, s_6_9, s_6_10, s_6_6, s_6_11)
        let s_6_12: () = execute_ADR_Z_AZ_SD_same_scaled(
            state,
            tracer,
            s_6_0,
            s_6_7,
            s_6_3,
            s_6_8,
            s_6_9,
            s_6_10,
            s_6_6,
            s_6_11,
        );
        // N s_6_13: return
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
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var osize:i64
        let s_8_4: i64 = fn_state.osize;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast reint s_8_5 -> i64
        let s_8_6: i64 = (s_8_5 as i64);
        // D s_8_7: read-var d:i64
        let s_8_7: i64 = fn_state.d;
        // D s_8_8: read-var m:i64
        let s_8_8: i64 = fn_state.m;
        // D s_8_9: read-var mbytes:i64
        let s_8_9: i64 = fn_state.mbytes;
        // D s_8_10: read-var n:i64
        let s_8_10: i64 = fn_state.n;
        // C s_8_11: const #1u : u8
        let s_8_11: bool = true;
        // D s_8_12: call execute_ADR_Z_AZ_SD_same_scaled(s_8_0, s_8_7, s_8_3, s_8_8, s_8_9, s_8_10, s_8_6, s_8_11)
        let s_8_12: () = execute_ADR_Z_AZ_SD_same_scaled(
            state,
            tracer,
            s_8_0,
            s_8_7,
            s_8_3,
            s_8_8,
            s_8_9,
            s_8_10,
            s_8_6,
            s_8_11,
        );
        // N s_8_13: return
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
        // D s_10_1: read-var esize:i64
        let s_10_1: i64 = fn_state.esize;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var osize:i64
        let s_10_4: i64 = fn_state.osize;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: cast reint s_10_5 -> i64
        let s_10_6: i64 = (s_10_5 as i64);
        // D s_10_7: read-var d:i64
        let s_10_7: i64 = fn_state.d;
        // D s_10_8: read-var m:i64
        let s_10_8: i64 = fn_state.m;
        // D s_10_9: read-var mbytes:i64
        let s_10_9: i64 = fn_state.mbytes;
        // D s_10_10: read-var n:i64
        let s_10_10: i64 = fn_state.n;
        // C s_10_11: const #1u : u8
        let s_10_11: bool = true;
        // D s_10_12: call execute_ADR_Z_AZ_SD_same_scaled(s_10_0, s_10_7, s_10_3, s_10_8, s_10_9, s_10_10, s_10_6, s_10_11)
        let s_10_12: () = execute_ADR_Z_AZ_SD_same_scaled(
            state,
            tracer,
            s_10_0,
            s_10_7,
            s_10_3,
            s_10_8,
            s_10_9,
            s_10_10,
            s_10_6,
            s_10_11,
        );
        // N s_10_13: return
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
