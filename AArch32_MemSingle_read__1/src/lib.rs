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
use IsFault__2::*;
use AArch32_Abort::*;
use PhysMemRead::*;
use HandleExternalReadAbort::*;
use IsAligned__1::*;
use IsFault::*;
use SPESampleLoadStore::*;
use AArch32_TranslateAddress::*;
use common::*;
pub fn AArch32_MemSingle_read__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
    accdesc_in: ProductType9878976b5bcce9c9,
    aligned: bool,
    ispair: bool,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_31089: bool,
        ga_24218: ProductType2b2aba4822138824,
        gs_31087: bool,
        gs_31086: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        memstatus: ProductTypef8c3639b88223255,
        value_name: Bits,
        gs_31088: bool,
        address: u32,
        size: i64,
        accdesc_in: ProductType9878976b5bcce9c9,
        aligned: bool,
        ispair: bool,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc_in,
        aligned,
        ispair,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var size:i64
        let s_0_1: i64 = fn_state.size;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b22 b1
        if s_0_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var size:i64
        let s_1_1: i64 = fn_state.size;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b21 b2
        if s_1_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var size:i64
        let s_2_1: i64 = fn_state.size;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b20 b3
        if s_2_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var size:i64
        let s_3_1: i64 = fn_state.size;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b19 b4
        if s_3_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // D s_4_1: read-var size:i64
        let s_4_1: i64 = fn_state.size;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // D s_4_4: write-var gs#31086 <= s_4_3
        fn_state.gs_31086 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var gs#31086:u8
        let s_5_0: bool = fn_state.gs_31086;
        // D s_5_1: write-var gs#31087 <= s_5_0
        fn_state.gs_31087 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#31087:u8
        let s_6_0: bool = fn_state.gs_31087;
        // D s_6_1: write-var gs#31088 <= s_6_0
        fn_state.gs_31088 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#31088:u8
        let s_7_0: bool = fn_state.gs_31088;
        // D s_7_1: write-var gs#31089 <= s_7_0
        fn_state.gs_31089 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#31089:u8
        let s_8_0: bool = fn_state.gs_31089;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var accdesc_in:struct
        let s_8_2: ProductType9878976b5bcce9c9 = fn_state.accdesc_in;
        // D s_8_3: write-var accdesc <= s_8_2
        fn_state.accdesc = s_8_2;
        // D s_8_4: read-var address:u32
        let s_8_4: u32 = fn_state.address;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 32u16);
        // D s_8_6: read-var size:i64
        let s_8_6: i64 = fn_state.size;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: call IsAligned__1(s_8_5, s_8_7)
        let s_8_8: bool = IsAligned__1(state, tracer, s_8_5, s_8_7);
        // N s_8_9: assert s_8_8
        let s_8_9: () = assert!(s_8_8);
        // D s_8_10: read-var size:i64
        let s_8_10: i64 = fn_state.size;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var address:u32
        let s_8_12: u32 = fn_state.address;
        // D s_8_13: read-var accdesc:struct
        let s_8_13: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_14: read-var aligned:u8
        let s_8_14: bool = fn_state.aligned;
        // D s_8_15: call AArch32_TranslateAddress(s_8_12, s_8_13, s_8_14, s_8_11)
        let s_8_15: ProductTypece7c66ccb2cab13e = AArch32_TranslateAddress(
            state,
            tracer,
            s_8_12,
            s_8_13,
            s_8_14,
            s_8_11,
        );
        // D s_8_16: write-var memaddrdesc <= s_8_15
        fn_state.memaddrdesc = s_8_15;
        // N s_8_17: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var memaddrdesc:struct
        let s_9_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_9_1: call IsFault(s_9_0)
        let s_9_1: bool = IsFault(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b18 b10
        if s_9_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #22416u : u32
        let s_11_0: u32 = 22416;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: bool = {
            let value = state.read_register::<bool>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // N s_11_2: branch s_11_1 b17 b12
        if s_11_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // D s_13_1: create-sum enum = 0:"s_13_0"
        let s_13_1: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_13_0);
        // D s_13_2: read-var size:i64
        let s_13_2: i64 = fn_state.size;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-var memaddrdesc:struct
        let s_13_4: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_13_5: read-var accdesc:struct
        let s_13_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_13_6: call PhysMemRead(s_13_4, s_13_3, s_13_5, s_13_1)
        let s_13_6: ProductType2b2aba4822138824 = PhysMemRead(
            state,
            tracer,
            s_13_4,
            s_13_3,
            s_13_5,
            s_13_1,
        );
        // D s_13_7: write-var ga#24218 <= s_13_6
        fn_state.ga_24218 = s_13_6;
        // D s_13_8: read-var ga#24218.0:struct
        let s_13_8: ProductTypef8c3639b88223255 = fn_state.ga_24218._0;
        // D s_13_9: read-var ga#24218.1:struct
        let s_13_9: Bits = fn_state.ga_24218._1;
        // D s_13_10: write-var memstatus <= s_13_8
        fn_state.memstatus = s_13_8;
        // D s_13_11: write-var value_name <= s_13_9
        fn_state.value_name = s_13_9;
        // D s_13_12: read-var memstatus:struct
        let s_13_12: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_13_13: call IsFault__2(s_13_12)
        let s_13_13: bool = IsFault__2(state, tracer, s_13_12);
        // N s_13_14: branch s_13_13 b16 b14
        if s_13_13 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_15_0: read-var value_name:bv
        let s_15_0: Bits = fn_state.value_name;
        // N s_15_1: return s_15_0
        return s_15_0;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_16_0: read-var size:i64
        let s_16_0: i64 = fn_state.size;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var memstatus:struct
        let s_16_2: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_16_3: read-var memaddrdesc:struct
        let s_16_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_16_4: read-var accdesc:struct
        let s_16_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_16_5: call HandleExternalReadAbort(s_16_2, s_16_3, s_16_1, s_16_4)
        let s_16_5: () = HandleExternalReadAbort(
            state,
            tracer,
            s_16_2,
            s_16_3,
            s_16_1,
            s_16_4,
        );
        // N s_16_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: read-var accdesc:struct
        let s_17_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_2: read-var memaddrdesc:struct
        let s_17_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_17_3: call SPESampleLoadStore(s_17_0, s_17_1, s_17_2)
        let s_17_3: () = SPESampleLoadStore(state, tracer, s_17_0, s_17_1, s_17_2);
        // N s_17_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_18_0: read-var memaddrdesc.0:struct
        let s_18_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_18_1: read-var address:u32
        let s_18_1: u32 = fn_state.address;
        // D s_18_2: call AArch32_Abort(s_18_1, s_18_0)
        let s_18_2: () = AArch32_Abort(state, tracer, s_18_1, s_18_0);
        // N s_18_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#31086 <= s_19_0
        fn_state.gs_31086 = s_19_0;
        // N s_19_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#31087 <= s_20_0
        fn_state.gs_31087 = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#31088 <= s_21_0
        fn_state.gs_31088 = s_21_0;
        // N s_21_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#31089 <= s_22_0
        fn_state.gs_31089 = s_22_0;
        // N s_22_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
