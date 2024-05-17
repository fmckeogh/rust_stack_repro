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
use u__ReadUART::*;
use read_request::*;
use sail_mem_read::*;
use ZeroExtend1::*;
use CreatePhysMemRetStatus::*;
use u__ReadGIC::*;
use common::*;
pub fn PhysMemRead<T: Tracer>(
    state: &mut State,
    tracer: &T,
    desc: ProductTypece7c66ccb2cab13e,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
    translation_info: SumTypeb20592b6489a79bd,
) -> ProductType2b2aba4822138824 {
    #[derive(Default)]
    struct FunctionState {
        gs_331545: ProductType2b2aba4822138824,
        ga_371007: SumTypebfdf2f926abd32c5,
        return_value: ProductType2b2aba4822138824,
        ga_371012: ProductTypeda0231e9dc169f81,
        gs_331539: bool,
        ga_371003: ProductTypeda0231e9dc169f81,
        paddress: u64,
        desc: ProductTypece7c66ccb2cab13e,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
        translation_info: SumTypeb20592b6489a79bd,
    }
    let fn_state = FunctionState {
        desc,
        size,
        accdesc,
        translation_info,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_0_0: read-var desc.3:struct
        let s_0_0: ProductTypeda0231e9dc169f81 = fn_state.desc._3;
        // D s_0_1: write-var ga#371012 <= s_0_0
        fn_state.ga_371012 = s_0_0;
        // D s_0_2: read-var ga#371012.0:struct
        let s_0_2: u64 = fn_state.ga_371012._0;
        // D s_0_3: write-var paddress <= s_0_2
        fn_state.paddress = s_0_2;
        // C s_0_4: const #4s : i
        let s_0_4: i128 = 4;
        // D s_0_5: read-var size:i
        let s_0_5: i128 = fn_state.size;
        // D s_0_6: cmp-ge s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) >= (s_0_4));
        // N s_0_7: branch s_0_6 b13 b1
        if s_0_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#331539 <= s_1_0
        fn_state.gs_331539 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_2_0: read-var gs#331539:u8
        let s_2_0: bool = fn_state.gs_331539;
        // N s_2_1: branch s_2_0 b12 b3
        if s_2_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // C s_3_1: const #40s : i
        let s_3_1: i128 = 40;
        // D s_3_2: read-var paddress:u56
        let s_3_2: u64 = fn_state.paddress;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 56u16);
        // D s_3_4: bit-extract s_3_3 s_3_0 s_3_1
        let s_3_4: Bits = (Bits::new(
            ((s_3_3) >> (s_3_0)).value(),
            u16::try_from(s_3_1).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u40
        let s_3_5: u64 = (s_3_4.value() as u64);
        // C s_3_6: const #39s : i64
        let s_3_6: i64 = 39;
        // C s_3_7: const #1s : i
        let s_3_7: i128 = 1;
        // C s_3_8: cast zx s_3_6 -> i
        let s_3_8: i128 = (i128::try_from(s_3_6).unwrap());
        // C s_3_9: add s_3_8 s_3_7
        let s_3_9: i128 = (s_3_8 + s_3_7);
        // C s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // C s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // C s_3_12: const #1352u : u32
        let s_3_12: u32 = 1352;
        // D s_3_13: read-reg s_3_12:u16
        let s_3_13: u16 = {
            let value = state.read_register::<u16>(s_3_12 as isize);
            tracer.read_register(s_3_12 as isize, value);
            value
        };
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 16u16);
        // D s_3_15: call ZeroExtend1(s_3_11, s_3_14)
        let s_3_15: Bits = ZeroExtend1(state, tracer, s_3_11, s_3_14);
        // D s_3_16: cast reint s_3_15 -> u40
        let s_3_16: u64 = (s_3_15.value() as u64);
        // D s_3_17: cast zx s_3_5 -> bv
        let s_3_17: Bits = Bits::new(s_3_5 as u128, 40u16);
        // D s_3_18: cast zx s_3_16 -> bv
        let s_3_18: Bits = Bits::new(s_3_16 as u128, 40u16);
        // D s_3_19: cmp-eq s_3_17 s_3_18
        let s_3_19: bool = ((s_3_17) == (s_3_18));
        // N s_3_20: branch s_3_19 b11 b4
        if s_3_19 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_4_0: read-var desc.7:struct
        let s_4_0: u64 = fn_state.desc._7;
        // D s_4_1: read-var desc.3:struct
        let s_4_1: ProductTypeda0231e9dc169f81 = fn_state.desc._3;
        // D s_4_2: write-var ga#371003 <= s_4_1
        fn_state.ga_371003 = s_4_1;
        // D s_4_3: read-var ga#371003.0:struct
        let s_4_3: u64 = fn_state.ga_371003._0;
        // D s_4_4: read-var accdesc:struct
        let s_4_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_5: read-var translation_info:enum
        let s_4_5: SumTypeb20592b6489a79bd = fn_state.translation_info;
        // D s_4_6: read-var size:i
        let s_4_6: i128 = fn_state.size;
        // D s_4_7: call read_request(s_4_4, s_4_5, s_4_6, s_4_0, s_4_3)
        let s_4_7: ProductTypeee12e330a5f80ce = read_request(
            state,
            tracer,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_0,
            s_4_3,
        );
        // D s_4_8: call sail_mem_read(s_4_7)
        let s_4_8: SumTypebfdf2f926abd32c5 = sail_mem_read(state, tracer, s_4_7);
        // D s_4_9: write-var ga#371007 <= s_4_8
        fn_state.ga_371007 = s_4_8;
        // D s_4_10: read-var ga#371007:enum
        let s_4_10: SumTypebfdf2f926abd32c5 = fn_state.ga_371007;
        // D s_4_11: matches-sum s_4_10 1
        let s_4_11: bool = matches!(s_4_10, SumTypebfdf2f926abd32c5::_1(_));
        // N s_4_12: branch s_4_11 b8 b5
        if s_4_11 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_5_0: read-var ga#371007:enum
        let s_5_0: SumTypebfdf2f926abd32c5 = fn_state.ga_371007;
        // D s_5_1: unwrap-sum s_5_0 1
        let s_5_1: ProductTypead6b611358cb4242 = match s_5_0 {
            SumTypebfdf2f926abd32c5::_1(inner) => inner,
            _ => panic!("unwrap sum failed"),
        };
        // D s_5_2: extract-field s_5_1.0
        let s_5_2: Bits = s_5_1._0;
        // C s_5_3: const #0u : u32
        let s_5_3: u32 = 0;
        // S s_5_4: call CreatePhysMemRetStatus(s_5_3)
        let s_5_4: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_5_3,
        );
        // D s_5_5: create-product struct = ["s_5_4", "s_5_2"]
        let s_5_5: ProductType2b2aba4822138824 = ProductType2b2aba4822138824 {
            _0: s_5_4,
            _1: s_5_2,
        };
        // D s_5_6: write-var gs#331545 <= s_5_5
        fn_state.gs_331545 = s_5_5;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_6_0: read-var gs#331545:struct
        let s_6_0: ProductType2b2aba4822138824 = fn_state.gs_331545;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_7_0: read-var return_value:struct
        let s_7_0: ProductType2b2aba4822138824 = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_8_0: read-var ga#371007:enum
        let s_8_0: SumTypebfdf2f926abd32c5 = fn_state.ga_371007;
        // D s_8_1: matches-sum s_8_0 0
        let s_8_1: bool = matches!(s_8_0, SumTypebfdf2f926abd32c5::_0(_));
        // N s_8_2: branch s_8_1 b10 b9
        if s_8_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // D s_9_0: read-var ga#371007:enum
        let s_9_0: SumTypebfdf2f926abd32c5 = fn_state.ga_371007;
        // D s_9_1: unwrap-sum s_9_0 0
        let s_9_1: u32 = match s_9_0 {
            SumTypebfdf2f926abd32c5::_0(inner) => inner,
            _ => panic!("unwrap sum failed"),
        };
        // D s_9_2: call CreatePhysMemRetStatus(s_9_1)
        let s_9_2: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_9_1,
        );
        // C s_9_3: const #8s : i
        let s_9_3: i128 = 8;
        // D s_9_4: read-var size:i
        let s_9_4: i128 = fn_state.size;
        // D s_9_5: mul s_9_3 s_9_4
        let s_9_5: i128 = ((s_9_3) * (s_9_4));
        // C s_9_6: const #0u : u8
        let s_9_6: u8 = 0;
        // C s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 8u16);
        // D s_9_8: bits-cast zx s_9_7 -> bv length s_9_5
        let s_9_8: Bits = s_9_7.zero_extend(s_9_5);
        // D s_9_9: create-product struct = ["s_9_2", "s_9_8"]
        let s_9_9: ProductType2b2aba4822138824 = ProductType2b2aba4822138824 {
            _0: s_9_2,
            _1: s_9_8,
        };
        // D s_9_10: write-var gs#331545 <= s_9_9
        fn_state.gs_331545 = s_9_9;
        // N s_9_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // N s_10_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // S s_11_1: call CreatePhysMemRetStatus(s_11_0)
        let s_11_1: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_11_0,
        );
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // D s_11_3: read-var paddress:u56
        let s_11_3: u64 = fn_state.paddress;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 56u16);
        // C s_11_5: const #1s : i64
        let s_11_5: i64 = 1;
        // C s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // C s_11_7: const #15s : i
        let s_11_7: i128 = 15;
        // C s_11_8: add s_11_7 s_11_6
        let s_11_8: i128 = (s_11_7 + s_11_6);
        // D s_11_9: bit-extract s_11_4 s_11_2 s_11_8
        let s_11_9: Bits = (Bits::new(
            ((s_11_4) >> (s_11_2)).value(),
            u16::try_from(s_11_8).unwrap(),
        ));
        // D s_11_10: cast reint s_11_9 -> u16
        let s_11_10: u16 = (s_11_9.value() as u16);
        // D s_11_11: read-var size:i
        let s_11_11: i128 = fn_state.size;
        // D s_11_12: call __ReadUART(s_11_10, s_11_11)
        let s_11_12: Bits = u__ReadUART(state, tracer, s_11_10, s_11_11);
        // D s_11_13: create-product struct = ["s_11_1", "s_11_12"]
        let s_11_13: ProductType2b2aba4822138824 = ProductType2b2aba4822138824 {
            _0: s_11_1,
            _1: s_11_12,
        };
        // D s_11_14: write-var return_value <= s_11_13
        fn_state.return_value = s_11_13;
        // N s_11_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // C s_12_0: const #0u : u32
        let s_12_0: u32 = 0;
        // S s_12_1: call CreatePhysMemRetStatus(s_12_0)
        let s_12_1: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_12_0,
        );
        // C s_12_2: const #0s : i
        let s_12_2: i128 = 0;
        // D s_12_3: read-var paddress:u56
        let s_12_3: u64 = fn_state.paddress;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 56u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #15s : i
        let s_12_7: i128 = 15;
        // C s_12_8: add s_12_7 s_12_6
        let s_12_8: i128 = (s_12_7 + s_12_6);
        // D s_12_9: bit-extract s_12_4 s_12_2 s_12_8
        let s_12_9: Bits = (Bits::new(
            ((s_12_4) >> (s_12_2)).value(),
            u16::try_from(s_12_8).unwrap(),
        ));
        // D s_12_10: cast reint s_12_9 -> u16
        let s_12_10: u16 = (s_12_9.value() as u16);
        // D s_12_11: call __ReadGIC(s_12_10)
        let s_12_11: u32 = u__ReadGIC(state, tracer, s_12_10);
        // C s_12_12: const #8s : i
        let s_12_12: i128 = 8;
        // D s_12_13: read-var size:i
        let s_12_13: i128 = fn_state.size;
        // D s_12_14: mul s_12_12 s_12_13
        let s_12_14: i128 = ((s_12_12) * (s_12_13));
        // D s_12_15: cast zx s_12_11 -> bv
        let s_12_15: Bits = Bits::new(s_12_11 as u128, 32u16);
        // D s_12_16: bits-cast zx s_12_15 -> bv length s_12_14
        let s_12_16: Bits = s_12_15.zero_extend(s_12_14);
        // D s_12_17: create-product struct = ["s_12_1", "s_12_16"]
        let s_12_17: ProductType2b2aba4822138824 = ProductType2b2aba4822138824 {
            _0: s_12_1,
            _1: s_12_16,
        };
        // D s_12_18: write-var return_value <= s_12_17
        fn_state.return_value = s_12_17;
        // N s_12_19: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2b2aba4822138824 {
        // C s_13_0: const #16s : i
        let s_13_0: i128 = 16;
        // C s_13_1: const #40s : i
        let s_13_1: i128 = 40;
        // D s_13_2: read-var paddress:u56
        let s_13_2: u64 = fn_state.paddress;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 56u16);
        // D s_13_4: bit-extract s_13_3 s_13_0 s_13_1
        let s_13_4: Bits = (Bits::new(
            ((s_13_3) >> (s_13_0)).value(),
            u16::try_from(s_13_1).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u40
        let s_13_5: u64 = (s_13_4.value() as u64);
        // C s_13_6: const #39s : i64
        let s_13_6: i64 = 39;
        // C s_13_7: const #1s : i
        let s_13_7: i128 = 1;
        // C s_13_8: cast zx s_13_6 -> i
        let s_13_8: i128 = (i128::try_from(s_13_6).unwrap());
        // C s_13_9: add s_13_8 s_13_7
        let s_13_9: i128 = (s_13_8 + s_13_7);
        // C s_13_10: cast reint s_13_9 -> i64
        let s_13_10: i64 = (s_13_9 as i64);
        // C s_13_11: cast zx s_13_10 -> i
        let s_13_11: i128 = (i128::try_from(s_13_10).unwrap());
        // C s_13_12: const #1360u : u32
        let s_13_12: u32 = 1360;
        // D s_13_13: read-reg s_13_12:u16
        let s_13_13: u16 = {
            let value = state.read_register::<u16>(s_13_12 as isize);
            tracer.read_register(s_13_12 as isize, value);
            value
        };
        // D s_13_14: cast zx s_13_13 -> bv
        let s_13_14: Bits = Bits::new(s_13_13 as u128, 16u16);
        // D s_13_15: call ZeroExtend1(s_13_11, s_13_14)
        let s_13_15: Bits = ZeroExtend1(state, tracer, s_13_11, s_13_14);
        // D s_13_16: cast reint s_13_15 -> u40
        let s_13_16: u64 = (s_13_15.value() as u64);
        // D s_13_17: cast zx s_13_5 -> bv
        let s_13_17: Bits = Bits::new(s_13_5 as u128, 40u16);
        // D s_13_18: cast zx s_13_16 -> bv
        let s_13_18: Bits = Bits::new(s_13_16 as u128, 40u16);
        // D s_13_19: cmp-eq s_13_17 s_13_18
        let s_13_19: bool = ((s_13_17) == (s_13_18));
        // D s_13_20: write-var gs#331539 <= s_13_19
        fn_state.gs_331539 = s_13_19;
        // N s_13_21: jump b2
        return block_2(state, tracer, fn_state);
    }
}
