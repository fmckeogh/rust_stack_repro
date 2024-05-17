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
use GPTL0Size::*;
use Unreachable::*;
use common::*;
pub fn DecodeGPTBlock<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pgs: u32,
    entry: u64,
) -> ProductType5b104d2f9e197511 {
    #[derive(Default)]
    struct FunctionState {
        result: ProductType9799615a3dcac2c0,
        return_value: ProductType5b104d2f9e197511,
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
        // C s_0_10: const #856u : u32
        let s_0_10: u32 = 856;
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
        // C s_0_15: const #8s : i
        let s_0_15: i128 = 8;
        // D s_0_16: read-var entry:u64
        let s_0_16: u64 = fn_state.entry;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: const #1s : i64
        let s_0_18: i64 = 1;
        // C s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // C s_0_20: const #55s : i
        let s_0_20: i128 = 55;
        // C s_0_21: add s_0_20 s_0_19
        let s_0_21: i128 = (s_0_20 + s_0_19);
        // D s_0_22: bit-extract s_0_17 s_0_15 s_0_21
        let s_0_22: Bits = (Bits::new(
            ((s_0_17) >> (s_0_15)).value(),
            u16::try_from(s_0_21).unwrap(),
        ));
        // D s_0_23: cast reint s_0_22 -> u56
        let s_0_23: u64 = (s_0_22.value() as u64);
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 56u16);
        // D s_0_25: call IsZero(s_0_24)
        let s_0_25: bool = IsZero(state, tracer, s_0_24);
        // D s_0_26: not s_0_25
        let s_0_26: bool = !s_0_25;
        // N s_0_27: branch s_0_26 b12 b1
        if s_0_26 {
            return block_12(state, tracer, fn_state);
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
        // D s_1_9: call GPIValid(s_1_8)
        let s_1_9: bool = GPIValid(state, tracer, s_1_8);
        // D s_1_10: not s_1_9
        let s_1_10: bool = !s_1_9;
        // N s_1_11: branch s_1_10 b11 b2
        if s_1_10 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var entry:u64
        let s_2_1: u64 = fn_state.entry;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #3s : i
        let s_2_5: i128 = 3;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_0 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: u8 = (s_2_7.value() as u8);
        // D s_2_9: write-var result.1 <= s_2_8
        fn_state.result._1 = s_2_8;
        // C s_2_10: const #0s : i
        let s_2_10: i128 = 0;
        // D s_2_11: write-var result.2 <= s_2_10
        fn_state.result._2 = s_2_10;
        // C s_2_12: const #0u : u32
        let s_2_12: u32 = 0;
        // D s_2_13: read-var pgs:u32
        let s_2_13: u32 = fn_state.pgs;
        // D s_2_14: cmp-eq s_2_12 s_2_13
        let s_2_14: bool = ((s_2_12) == (s_2_13));
        // D s_2_15: not s_2_14
        let s_2_15: bool = !s_2_14;
        // N s_2_16: branch s_2_15 b6 b3
        if s_2_15 {
            return block_6(state, tracer, fn_state);
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
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call GPTL0Size(s_4_0)
        let s_4_1: i128 = GPTL0Size(state, tracer, s_4_0);
        // D s_4_2: write-var result.0 <= s_4_1
        fn_state.result._0 = s_4_1;
        // C s_4_3: const #0u : u32
        let s_4_3: u32 = 0;
        // D s_4_4: read-var result:struct
        let s_4_4: ProductType9799615a3dcac2c0 = fn_state.result;
        // D s_4_5: create-product struct = ["s_4_3", "s_4_4"]
        let s_4_5: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_4_3,
            _1: s_4_4,
        };
        // D s_4_6: write-var return_value <= s_4_5
        fn_state.return_value = s_4_5;
        // N s_4_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_5_0: read-var return_value:struct
        let s_5_0: ProductType5b104d2f9e197511 = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: read-var pgs:u32
        let s_6_1: u32 = fn_state.pgs;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_7_0: const #920u : u32
        let s_7_0: u32 = 920;
        // D s_7_1: read-reg s_7_0:i64
        let s_7_1: i64 = {
            let value = state.read_register::<i64>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: write-var result.4 <= s_7_2
        fn_state.result._4 = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // D s_8_1: read-var pgs:u32
        let s_8_1: u32 = fn_state.pgs;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
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
        // C s_9_0: const #928u : u32
        let s_9_0: u32 = 928;
        // D s_9_1: read-reg s_9_0:i64
        let s_9_1: i64 = {
            let value = state.read_register::<i64>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: write-var result.4 <= s_9_2
        fn_state.result._4 = s_9_2;
        // N s_9_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Unreachable(s_10_0)
        let s_10_1: () = Unreachable(state, tracer, s_10_0);
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call __UNKNOWN_GPTEntry(s_11_0)
        let s_11_1: ProductType9799615a3dcac2c0 = u__UNKNOWN_GPTEntry(
            state,
            tracer,
            s_11_0,
        );
        // C s_11_2: const #2u : u32
        let s_11_2: u32 = 2;
        // D s_11_3: create-product struct = ["s_11_2", "s_11_1"]
        let s_11_3: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_11_2,
            _1: s_11_1,
        };
        // D s_11_4: write-var return_value <= s_11_3
        fn_state.return_value = s_11_3;
        // N s_11_5: jump b5
        return block_5(state, tracer, fn_state);
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
        // N s_12_5: jump b5
        return block_5(state, tracer, fn_state);
    }
}
