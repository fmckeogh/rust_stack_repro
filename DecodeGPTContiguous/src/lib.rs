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
use GPIValid::*;
use IsZero::*;
use u__UNKNOWN_GPTEntry::*;
use Unreachable::*;
use common::*;
pub fn DecodeGPTContiguous<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pgs: u32,
    entry: u64,
) -> ProductType5b104d2f9e197511 {
    #[derive(Default)]
    struct FunctionState {
        result: ProductType9799615a3dcac2c0,
        return_value: ProductType5b104d2f9e197511,
        ga_9783: u8,
        pgs: u32,
        entry: u64,
    }
    let fn_state = FunctionState {
        pgs,
        entry,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var entry:u64
        let s_0_1: u64 = fn_state.entry;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 64u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #3s : i
        let s_0_5: i128 = 3;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 4u16);
        // C s_0_10: const #864u : u32
        let s_0_10: u32 = 864;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 4u16);
        // D s_0_13: cmp-eq s_0_9 s_0_12
        let s_0_13: bool = ((s_0_9) == (s_0_12));
        // N s_0_14: assert s_0_13
        let s_0_14: () = assert!(s_0_13);
        // C s_0_15: const #10s : i
        let s_0_15: i128 = 10;
        // D s_0_16: read-var entry:u64
        let s_0_16: u64 = fn_state.entry;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: const #1s : i64
        let s_0_18: i64 = 1;
        // C s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // C s_0_20: const #53s : i
        let s_0_20: i128 = 53;
        // C s_0_21: add s_0_20 s_0_19
        let s_0_21: i128 = (s_0_20 + s_0_19);
        // D s_0_22: bit-extract s_0_17 s_0_15 s_0_21
        let s_0_22: Bits = (Bits::new(
            ((s_0_17) >> (s_0_15)).value(),
            u16::try_from(s_0_21).unwrap(),
        ));
        // D s_0_23: cast reint s_0_22 -> u54
        let s_0_23: u64 = (s_0_22.value() as u64);
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 54u16);
        // D s_0_25: call IsZero(s_0_24)
        let s_0_25: bool = IsZero(state, tracer, s_0_24);
        // D s_0_26: not s_0_25
        let s_0_26: bool = !s_0_25;
        // N s_0_27: branch s_0_26 b19 b1
        if s_0_26 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_1_0: const #4s : i
        let s_1_0: i128 = 4;
        // D s_1_1: read-var entry:u64
        let s_1_1: u64 = fn_state.entry;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 64u16);
        // C s_1_3: const #1s : i64
        let s_1_3: i64 = 1;
        // C s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // C s_1_5: const #3s : i
        let s_1_5: i128 = 3;
        // C s_1_6: add s_1_5 s_1_4
        let s_1_6: i128 = (s_1_5 + s_1_4);
        // D s_1_7: bit-extract s_1_2 s_1_0 s_1_6
        let s_1_7: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_6).unwrap(),
        ));
        // D s_1_8: cast reint s_1_7 -> u8
        let s_1_8: u8 = (s_1_7.value() as u8);
        // D s_1_9: write-var result.1 <= s_1_8
        fn_state.result._1 = s_1_8;
        // D s_1_10: read-var result.1:struct
        let s_1_10: u8 = fn_state.result._1;
        // D s_1_11: call GPIValid(s_1_10)
        let s_1_11: bool = GPIValid(state, tracer, s_1_10);
        // D s_1_12: not s_1_11
        let s_1_12: bool = !s_1_11;
        // N s_1_13: branch s_1_12 b18 b2
        if s_1_12 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: read-var pgs:u32
        let s_2_1: u32 = fn_state.pgs;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b13 b3
        if s_2_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_3_0: const #912u : u32
        let s_3_0: u32 = 912;
        // D s_3_1: read-reg s_3_0:i64
        let s_3_1: i64 = {
            let value = state.read_register::<i64>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: write-var result.4 <= s_3_2
        fn_state.result._4 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_4_0: const #8s : i
        let s_4_0: i128 = 8;
        // D s_4_1: read-var entry:u64
        let s_4_1: u64 = fn_state.entry;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: write-var ga#9783 <= s_4_8
        fn_state.ga_9783 = s_4_8;
        // D s_4_10: read-var ga#9783:u8
        let s_4_10: u8 = fn_state.ga_9783;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // C s_4_12: const #1u : u8
        let s_4_12: u8 = 1;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 2u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // D s_4_15: not s_4_14
        let s_4_15: bool = !s_4_14;
        // N s_4_16: branch s_4_15 b8 b5
        if s_4_15 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_5_0: const #936u : u32
        let s_5_0: u32 = 936;
        // D s_5_1: read-reg s_5_0:i64
        let s_5_1: i64 = {
            let value = state.read_register::<i64>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: write-var result.0 <= s_5_2
        fn_state.result._0 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: write-var result.2 <= s_6_0
        fn_state.result._2 = s_6_0;
        // C s_6_2: const #0u : u32
        let s_6_2: u32 = 0;
        // D s_6_3: read-var result:struct
        let s_6_3: ProductType9799615a3dcac2c0 = fn_state.result;
        // D s_6_4: create-product struct = ["s_6_2", "s_6_3"]
        let s_6_4: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_6_2,
            _1: s_6_3,
        };
        // D s_6_5: write-var return_value <= s_6_4
        fn_state.return_value = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_7_0: read-var return_value:struct
        let s_7_0: ProductType5b104d2f9e197511 = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_8_0: read-var ga#9783:u8
        let s_8_0: u8 = fn_state.ga_9783;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_9_0: const #944u : u32
        let s_9_0: u32 = 944;
        // D s_9_1: read-reg s_9_0:i64
        let s_9_1: i64 = {
            let value = state.read_register::<i64>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: write-var result.0 <= s_9_2
        fn_state.result._0 = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_10_0: read-var ga#9783:u8
        let s_10_0: u8 = fn_state.ga_9783;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #3u : u8
        let s_10_2: u8 = 3;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_11_0: const #952u : u32
        let s_11_0: u32 = 952;
        // D s_11_1: read-reg s_11_0:i64
        let s_11_1: i64 = {
            let value = state.read_register::<i64>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: write-var result.0 <= s_11_2
        fn_state.result._0 = s_11_2;
        // N s_11_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call __UNKNOWN_GPTEntry(s_12_0)
        let s_12_1: ProductType9799615a3dcac2c0 = u__UNKNOWN_GPTEntry(
            state,
            tracer,
            s_12_0,
        );
        // C s_12_2: const #2u : u32
        let s_12_2: u32 = 2;
        // D s_12_3: create-product struct = ["s_12_2", "s_12_1"]
        let s_12_3: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_12_2,
            _1: s_12_1,
        };
        // D s_12_4: write-var return_value <= s_12_3
        fn_state.return_value = s_12_3;
        // N s_12_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_13_0: const #1u : u32
        let s_13_0: u32 = 1;
        // D s_13_1: read-var pgs:u32
        let s_13_1: u32 = fn_state.pgs;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_14_0: const #920u : u32
        let s_14_0: u32 = 920;
        // D s_14_1: read-reg s_14_0:i64
        let s_14_1: i64 = {
            let value = state.read_register::<i64>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: write-var result.4 <= s_14_2
        fn_state.result._4 = s_14_2;
        // N s_14_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_15_0: const #2u : u32
        let s_15_0: u32 = 2;
        // D s_15_1: read-var pgs:u32
        let s_15_1: u32 = fn_state.pgs;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b17 b16
        if s_15_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_16_0: const #928u : u32
        let s_16_0: u32 = 928;
        // D s_16_1: read-reg s_16_0:i64
        let s_16_1: i64 = {
            let value = state.read_register::<i64>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: write-var result.4 <= s_16_2
        fn_state.result._4 = s_16_2;
        // N s_16_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call Unreachable(s_17_0)
        let s_17_1: () = Unreachable(state, tracer, s_17_0);
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: read-var result:struct
        let s_18_1: ProductType9799615a3dcac2c0 = fn_state.result;
        // D s_18_2: create-product struct = ["s_18_0", "s_18_1"]
        let s_18_2: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_18_0,
            _1: s_18_1,
        };
        // D s_18_3: write-var return_value <= s_18_2
        fn_state.return_value = s_18_2;
        // N s_18_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_19_0: const #2u : u32
        let s_19_0: u32 = 2;
        // D s_19_1: read-var result:struct
        let s_19_1: ProductType9799615a3dcac2c0 = fn_state.result;
        // D s_19_2: create-product struct = ["s_19_0", "s_19_1"]
        let s_19_2: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_19_0,
            _1: s_19_1,
        };
        // D s_19_3: write-var return_value <= s_19_2
        fn_state.return_value = s_19_2;
        // N s_19_4: jump b7
        return block_7(state, tracer, fn_state);
    }
}
